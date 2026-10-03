//! SQLite writes use its canonical single-writer boundary, including revocation.
use super::{AuthorityError, VerifiedOwner, verify_owner};
use crate::Store;
use axum::http::HeaderMap;
use sea_orm::ConnectionTrait;
use server_common::Claims;
use sqlx::{
    Sqlite, SqliteConnection, SqlitePool, TransactionManager, pool::PoolConnection,
    sqlite::SqliteTransactionManager,
};
use std::{sync::Arc, time::Duration};

const DEADLINE: Duration = Duration::from_secs(3);

#[derive(Clone)]
pub struct CanonicalSqliteAuthority {
    pool: SqlitePool,
    store: Arc<Store>,
}
impl CanonicalSqliteAuthority {
    pub fn bind(store: Arc<Store>, data_pool: &SqlitePool) -> Result<Self, AuthorityError> {
        let repository = store.portable_repo().ok_or(AuthorityError::Unavailable)?;
        let connection = repository.connection();
        if connection.get_database_backend() != sea_orm::DatabaseBackend::Sqlite {
            return Err(AuthorityError::Unavailable);
        }
        let canonical = connection.get_sqlite_connection_pool();
        if !std::ptr::eq(canonical.options(), data_pool.options()) {
            return Err(AuthorityError::Unavailable);
        }
        Ok(Self {
            pool: canonical.clone(),
            store,
        })
    }

    pub async fn authorize(
        &self,
        claims: &Claims,
        headers: &HeaderMap,
    ) -> Result<AuthorizedSqliteOwner, AuthorityError> {
        let owner = verify_owner(&self.store, claims, headers).await?;
        // Confirm the actual write capability before callers incur provider work.
        // Roll back this preflight; never hold a SQLite write lock across I/O.
        begin_owner_transaction(&self.pool, &owner)
            .await?
            .restore()
            .await?;
        Ok(AuthorizedSqliteOwner {
            pool: self.pool.clone(),
            owner,
        })
    }
}

pub struct AuthorizedSqliteOwner {
    pool: SqlitePool,
    owner: VerifiedOwner,
}
impl AuthorizedSqliteOwner {
    pub fn tenant_id(&self) -> &str {
        self.owner.tenant_id()
    }
    pub fn actor_id(&self) -> &str {
        self.owner.actor_id()
    }
    pub async fn begin(self) -> Result<OwnerSqliteTransaction, AuthorityError> {
        let connection = begin_owner_transaction(&self.pool, &self.owner).await?;
        Ok(OwnerSqliteTransaction {
            connection,
            owner: self.owner,
        })
    }
}

pub struct OwnerSqliteTransaction {
    connection: WriteConnection,
    owner: VerifiedOwner,
}
impl OwnerSqliteTransaction {
    pub fn connection(&mut self) -> &mut SqliteConnection {
        self.connection.connection()
    }
    pub async fn commit(mut self) -> Result<(), AuthorityError> {
        tokio::time::timeout(DEADLINE, async {
            check_database_owner(self.connection.connection(), &self.owner).await?;
            // After submission, cancellation or missing acknowledgement is an
            // unconfirmed save. Cleanup never submits a replacement COMMIT.
            SqliteTransactionManager::commit(self.connection.connection()).await?;
            self.connection.restore().await
        })
        .await
        .map_err(|_| AuthorityError::Unavailable)?
    }
}

/// Own the connection before changing its settings. SQLx's SQLite worker may
/// outlive the caller's future; cleanup retains this connection until rollback
/// and restoration are acknowledged, including for an in-memory/max1 pool.
struct WriteConnection {
    connection: Option<PoolConnection<Sqlite>>,
    busy_timeout: Option<i64>,
    runtime: tokio::runtime::Handle,
}
impl WriteConnection {
    fn connection(&mut self) -> &mut SqliteConnection {
        self.connection.as_mut().expect("owned SQLite connection")
    }
    fn cleanup(&mut self) -> Option<CleanupConnection> {
        let mut connection = self.connection.take()?;
        // SQLx's queued rollback also handles an unacknowledged COMMIT without
        // rolling back a later transaction. Queue it before any restoration.
        SqliteTransactionManager::start_rollback(&mut connection);
        Some(CleanupConnection {
            connection: Some(connection),
            busy_timeout: self.busy_timeout,
        })
    }
    async fn restore(mut self) -> Result<(), AuthorityError> {
        self.cleanup()
            .expect("owned SQLite cleanup")
            .restore()
            .await
    }
}
impl Drop for WriteConnection {
    fn drop(&mut self) {
        if let Some(cleanup) = self.cleanup() {
            // A resource-cleanup task only: it owns the connection and can issue
            // rollback/restoration, never business writes or COMMIT.
            self.runtime.spawn(async move {
                if let Err(error) = cleanup.restore().await {
                    tracing::warn!(error=%error, database_failure=std::error::Error::source(&error).is_some(),
                        "canonical SQLite cleanup unconfirmed; connection discarded, in-memory state not confirmed");
                }
            });
        }
    }
}

struct CleanupConnection {
    connection: Option<PoolConnection<Sqlite>>,
    busy_timeout: Option<i64>,
}
impl CleanupConnection {
    async fn restore(mut self) -> Result<(), AuthorityError> {
        tokio::time::timeout(DEADLINE, async {
            let connection = self.connection.as_mut().expect("owned cleanup connection");
            if let Some(timeout) = self.busy_timeout {
                sqlx::query(&format!("PRAGMA busy_timeout={timeout}"))
                    .execute(&mut **connection)
                    .await?;
                let actual: i64 = sqlx::query_scalar("PRAGMA busy_timeout")
                    .fetch_one(&mut **connection)
                    .await?;
                if actual != timeout {
                    return Err(AuthorityError::Unavailable);
                }
            } else {
                // Flush the preceding rollback even if cancellation preceded
                // reading/changing the original timeout.
                sqlx::query("SELECT 1").execute(&mut **connection).await?;
            }
            if SqliteTransactionManager::get_transaction_depth(connection) != 0 {
                return Err(AuthorityError::Unavailable);
            }
            Ok(())
        })
        .await
        .map_err(|_| AuthorityError::Unavailable)??;
        // Only a confirmed clean connection can return to the original pool.
        drop(self.connection.take());
        Ok(())
    }
}
impl Drop for CleanupConnection {
    fn drop(&mut self) {
        if let Some(connection) = self.connection.as_mut() {
            // Also covers cleanup-task cancellation and runtime shutdown. Do
            // not return an unverified transaction or altered timeout to users.
            connection.close_on_drop();
            tracing::warn!(
                "canonical SQLite connection cleanup did not complete; in-memory state is unconfirmed"
            );
        }
    }
}

async fn begin_owner_transaction(
    pool: &SqlitePool,
    owner: &VerifiedOwner,
) -> Result<WriteConnection, AuthorityError> {
    tokio::time::timeout(DEADLINE, async {
        let mut connection = WriteConnection {
            connection: Some(pool.acquire().await?),
            busy_timeout: None,
            runtime: tokio::runtime::Handle::try_current()
                .map_err(|_| AuthorityError::Unavailable)?,
        };
        let original: i64 = sqlx::query_scalar("PRAGMA busy_timeout")
            .fetch_one(connection.connection())
            .await?;
        if !(0..=i32::MAX as i64).contains(&original) {
            return Err(AuthorityError::Unavailable);
        }
        connection.busy_timeout = Some(original);
        sqlx::query(&format!("PRAGMA busy_timeout={}", original.min(1000)))
            .execute(connection.connection())
            .await?;
        for (pragma, expected) in [
            ("PRAGMA foreign_keys", 1),
            ("PRAGMA read_uncommitted", 0),
            ("PRAGMA query_only", 0),
        ] {
            let actual: i64 = sqlx::query_scalar(pragma)
                .fetch_one(connection.connection())
                .await?;
            if actual != expected {
                return Err(AuthorityError::Unavailable);
            }
        }
        SqliteTransactionManager::begin(connection.connection(), Some("BEGIN IMMEDIATE".into()))
            .await?;
        check_database_owner(connection.connection(), owner).await?;
        Ok(connection)
    })
    .await
    .map_err(|_| AuthorityError::Unavailable)?
}

async fn check_database_owner(
    connection: &mut SqliteConnection,
    owner: &VerifiedOwner,
) -> Result<(), AuthorityError> {
    let current: Option<String> = sqlx::query_scalar("SELECT u.id FROM main.users u JOIN main.identity_user_roles r ON r.user_id=u.id AND r.tenant_id=u.tenant_id WHERE u.id=? AND u.tenant_id=? AND u.active=1 AND lower(r.role_name) IN ('admin','owner') LIMIT 1")
        .bind(owner.actor_id()).bind(owner.tenant_id()).fetch_optional(&mut *connection).await?;
    let now: i64 = sqlx::query_scalar("SELECT CAST(strftime('%s','now') AS INTEGER)")
        .fetch_one(&mut *connection)
        .await?;
    let revoked: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM main.auth_revoked_tokens WHERE tenant_id=? AND jti=?)",
    )
    .bind(owner.tenant_id())
    .bind(owner.token_id())
    .fetch_one(connection)
    .await?;
    if current.is_none() || owner.expires_at() <= now || revoked {
        return Err(AuthorityError::Forbidden);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

    async fn memory_pool() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(
                SqliteConnectOptions::new()
                    .in_memory(true)
                    .foreign_keys(true)
                    .busy_timeout(Duration::from_secs(7)),
            )
            .await
            .unwrap();
        sqlx::query("CREATE TABLE retained(id INTEGER PRIMARY KEY)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO retained VALUES(1)")
            .execute(&pool)
            .await
            .unwrap();
        pool
    }

    async fn changed_connection(pool: &SqlitePool) -> WriteConnection {
        let mut connection = WriteConnection {
            connection: Some(pool.acquire().await.unwrap()),
            busy_timeout: Some(7000),
            runtime: tokio::runtime::Handle::current(),
        };
        sqlx::query("PRAGMA busy_timeout=1000")
            .execute(connection.connection())
            .await
            .unwrap();
        SqliteTransactionManager::begin(connection.connection(), Some("BEGIN IMMEDIATE".into()))
            .await
            .unwrap();
        sqlx::query("INSERT INTO retained VALUES(2)")
            .execute(connection.connection())
            .await
            .unwrap();
        connection
    }

    #[tokio::test]
    async fn sqlite_cleanup_retains_single_memory_connection_until_rollback_and_restore() {
        let pool = memory_pool().await;
        drop(changed_connection(&pool).await);
        let count: i64 = tokio::time::timeout(
            DEADLINE,
            sqlx::query_scalar("SELECT COUNT(*) FROM retained").fetch_one(&pool),
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(count, 1);
        let timeout: i64 = sqlx::query_scalar("PRAGMA busy_timeout")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(timeout, 7000);
    }

    #[tokio::test]
    async fn sqlite_cleanup_failed_restoration_discards_instead_of_returning_changed_connection() {
        let pool = memory_pool().await;
        let mut changed = changed_connection(&pool).await;
        let mut cleanup = changed.cleanup().unwrap();
        // SQLite cannot restore this value: its actual PRAGMA clamps it. Prove
        // failure of the post-write verification against the real engine.
        cleanup.busy_timeout = Some(i64::MAX);
        assert!(matches!(
            cleanup.restore().await,
            Err(AuthorityError::Unavailable)
        ));
        let connection = tokio::time::timeout(DEADLINE, pool.acquire())
            .await
            .unwrap()
            .unwrap();
        let mut connection = connection;
        let timeout: i64 = sqlx::query_scalar("PRAGMA busy_timeout")
            .fetch_one(&mut *connection)
            .await
            .unwrap();
        assert_eq!(
            timeout, 7000,
            "replacement uses the original configured options"
        );
        assert!(
            sqlx::query("SELECT * FROM retained")
                .execute(&mut *connection)
                .await
                .is_err(),
            "failed cleanup cannot claim the discarded in-memory database was preserved"
        );
    }

    #[tokio::test]
    async fn sqlite_cleanup_task_cancellation_never_returns_unconfirmed_connection() {
        let pool = memory_pool().await;
        let mut changed = changed_connection(&pool).await;
        let cleanup = changed.cleanup().unwrap();
        let (ready_tx, ready_rx) = tokio::sync::oneshot::channel();
        let task = tokio::spawn(async move {
            let cleanup = cleanup;
            ready_tx.send(()).unwrap();
            std::future::pending::<()>().await;
            cleanup.restore().await
        });
        ready_rx.await.unwrap();
        task.abort();
        assert!(task.await.unwrap_err().is_cancelled());
        let mut connection = tokio::time::timeout(DEADLINE, pool.acquire())
            .await
            .unwrap()
            .unwrap();
        let timeout: i64 = sqlx::query_scalar("PRAGMA busy_timeout")
            .fetch_one(&mut *connection)
            .await
            .unwrap();
        assert_eq!(timeout, 7000);
        assert!(
            sqlx::query("SELECT * FROM retained")
                .execute(&mut *connection)
                .await
                .is_err()
        );
    }

    #[test]
    fn sqlite_cleanup_runtime_shutdown_does_not_recycle_an_unconfirmed_transaction() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let pool = runtime.block_on(async {
            let pool = memory_pool().await;
            let mut changed = changed_connection(&pool).await;
            let cleanup = changed.cleanup().unwrap();
            runtime.spawn(async move {
                let cleanup = cleanup;
                std::future::pending::<()>().await;
                cleanup.restore().await
            });
            pool
        });
        drop(runtime);
        let next = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        next.block_on(async {
            let mut connection = tokio::time::timeout(DEADLINE, pool.acquire())
                .await
                .unwrap()
                .unwrap();
            let timeout: i64 = sqlx::query_scalar("PRAGMA busy_timeout")
                .fetch_one(&mut *connection)
                .await
                .unwrap();
            assert_eq!(timeout, 7000);
            assert!(
                sqlx::query("SELECT * FROM retained")
                    .execute(&mut *connection)
                    .await
                    .is_err()
            );
            drop(connection);
            pool.close().await;
        });
    }

    #[tokio::test]
    async fn sqlite_final_commit_rechecks_database_clock_and_discards_expired_write() {
        let pool = memory_pool().await;
        sqlx::query("CREATE TABLE users(id TEXT,tenant_id TEXT,active INTEGER)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE identity_user_roles(user_id TEXT,tenant_id TEXT,role_name TEXT)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE auth_revoked_tokens(tenant_id TEXT,jti TEXT)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO users VALUES('owner','tenant',1)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO identity_user_roles VALUES('owner','tenant','ADMIN')")
            .execute(&pool)
            .await
            .unwrap();
        let mut owner = VerifiedOwner {
            tenant_id: "tenant".into(),
            actor_id: "owner".into(),
            token_id: "token".into(),
            expires_at: chrono::Utc::now().timestamp() + 60,
            session_id: None,
        };
        let mut connection = begin_owner_transaction(&pool, &owner).await.unwrap();
        sqlx::query("INSERT INTO retained VALUES(2)")
            .execute(connection.connection())
            .await
            .unwrap();
        owner.expires_at = chrono::Utc::now().timestamp() - 1;
        assert!(matches!(
            (OwnerSqliteTransaction { connection, owner })
                .commit()
                .await,
            Err(AuthorityError::Forbidden)
        ));
        let count: i64 = tokio::time::timeout(
            DEADLINE,
            sqlx::query_scalar("SELECT COUNT(*) FROM retained").fetch_one(&pool),
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(count, 1);
    }
}
