//! Request-owned writes whose identity and data use one canonical PostgreSQL pool.
//! Accepted background work has its own durable approval and is not a bearer write.
use std::{sync::Arc, time::Duration};

use axum::http::{HeaderMap, header::AUTHORIZATION};
use sea_orm::ConnectionTrait;
use server_common::Claims;
use sqlx::{PgConnection, PgPool, Postgres, Transaction};

use crate::Store;

mod sqlite;
pub use sqlite::{AuthorizedSqliteOwner, CanonicalSqliteAuthority, OwnerSqliteTransaction};

#[derive(Debug)]
pub enum AuthorityError {
    Forbidden,
    Unavailable,
    Database(sqlx::Error),
}
impl std::fmt::Display for AuthorityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Forbidden => "Current canonical owner authority is required",
            Self::Unavailable => "Canonical transaction authority is unavailable",
            Self::Database(_) => "Canonical authority persistence failed",
        })
    }
}
impl std::error::Error for AuthorityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Database(error) => Some(error),
            _ => None,
        }
    }
}
impl From<sqlx::Error> for AuthorityError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

#[derive(Clone)]
pub struct VerifiedOwner {
    tenant_id: String,
    actor_id: String,
    token_id: String,
    expires_at: i64,
    session_id: Option<String>,
}
impl VerifiedOwner {
    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }
    pub fn actor_id(&self) -> &str {
        &self.actor_id
    }
    pub fn token_id(&self) -> &str {
        &self.token_id
    }
    pub fn expires_at(&self) -> i64 {
        self.expires_at
    }
    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }
}

/// Recheck a previously authenticated identity. This function alone grants no
/// request capability; callers must first authenticate the exact bearer below.
pub async fn require_current_owner(
    store: &Store,
    tenant: &str,
    actor: &str,
    token: &str,
    expires_at: i64,
) -> Result<(), AuthorityError> {
    if expires_at <= chrono::Utc::now().timestamp() {
        return Err(AuthorityError::Forbidden);
    }
    tokio::time::timeout(Duration::from_secs(3), async {
        if store
            .is_revoked(token, tenant)
            .await
            .map_err(|_| AuthorityError::Unavailable)?
        {
            return Err(AuthorityError::Forbidden);
        }
        let user = store
            .get_user(actor, tenant)
            .await
            .ok_or(AuthorityError::Forbidden)?;
        if !user.active
            || user.organization_id.as_deref() != Some(tenant)
            || !user.roles.iter().any(|role| {
                role.eq_ignore_ascii_case("owner") || role.eq_ignore_ascii_case("admin")
            })
        {
            return Err(AuthorityError::Forbidden);
        }
        Ok(())
    })
    .await
    .map_err(|_| AuthorityError::Unavailable)?
}

/// Verify the exact signed token and current canonical role. No caller-supplied
/// Claims value, copied user row, or tenant header can construct this snapshot.
pub async fn verify_owner(
    store: &Store,
    claims: &Claims,
    headers: &HeaderMap,
) -> Result<VerifiedOwner, AuthorityError> {
    let mut values = headers.get_all(AUTHORIZATION).iter();
    let token = values
        .next()
        .filter(|_| values.next().is_none())
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|value| {
            !value.is_empty()
                && value.len() <= crate::MAX_ACCESS_TOKEN_BYTES
                && !value
                    .chars()
                    .any(|c| c.is_whitespace() || c.is_ascii_control())
        })
        .ok_or(AuthorityError::Forbidden)?;
    let signed = tokio::time::timeout(Duration::from_secs(3), store.validate_token(token))
        .await
        .map_err(|_| AuthorityError::Unavailable)?
        .map_err(|_| AuthorityError::Forbidden)?;
    if signed.sub != claims.sub
        || signed.organization_id != claims.organization_id
        || signed.jti != claims.jti
        || signed.exp != claims.exp
        || signed.session_id != claims.session_id
    {
        return Err(AuthorityError::Forbidden);
    }
    let tenant_id =
        server_common::auth_utils::signed_tenant_id(&signed).ok_or(AuthorityError::Forbidden)?;
    if signed.organization_id.as_deref() != Some(tenant_id.as_str())
        || tenant_id.len() > 512
        || signed.sub.trim().is_empty()
        || signed.sub.len() > 512
        || signed.jti.trim().is_empty()
        || signed.jti.len() > 512
        || signed
            .session_id
            .as_ref()
            .is_some_and(|value| value.trim().is_empty() || value.len() > 512)
    {
        return Err(AuthorityError::Forbidden);
    }
    require_current_owner(store, &tenant_id, &signed.sub, &signed.jti, signed.exp).await?;
    Ok(VerifiedOwner {
        tenant_id,
        actor_id: signed.sub,
        token_id: signed.jti,
        expires_at: signed.exp,
        session_id: signed.session_id,
    })
}

#[derive(Clone)]
pub struct CanonicalPgAuthority {
    pool: PgPool,
    store: Arc<Store>,
}
impl CanonicalPgAuthority {
    pub fn bind(store: Arc<Store>, data_pool: &PgPool) -> Result<Self, AuthorityError> {
        let repository = store.portable_repo().ok_or(AuthorityError::Unavailable)?;
        let connection = repository.connection();
        if connection.get_database_backend() != sea_orm::DatabaseBackend::Postgres {
            return Err(AuthorityError::Unavailable);
        }
        let canonical = connection.get_postgres_connection_pool();
        // SQLx stores these immutable options inside the shared pool allocation.
        // Equal URLs/options or copied rows are insufficient: clones share this
        // exact reference; independent pools do not.
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
    ) -> Result<AuthorizedPgOwner, AuthorityError> {
        let owner = verify_owner(&self.store, claims, headers).await?;
        // Reject unavailable/read-only/unsupported data authority before callers
        // incur provider work. No database lock is held across provider I/O.
        let tx = begin_owner_transaction(&self.pool, &owner).await?;
        tx.rollback().await?;
        Ok(AuthorizedPgOwner {
            pool: self.pool.clone(),
            owner,
        })
    }
}

pub struct AuthorizedPgOwner {
    pool: PgPool,
    owner: VerifiedOwner,
}
impl AuthorizedPgOwner {
    pub fn tenant_id(&self) -> &str {
        self.owner.tenant_id()
    }
    pub fn actor_id(&self) -> &str {
        self.owner.actor_id()
    }
    pub async fn begin(self) -> Result<OwnerPgTransaction, AuthorityError> {
        let tx = begin_owner_transaction(&self.pool, &self.owner).await?;
        Ok(OwnerPgTransaction {
            tx,
            owner: self.owner,
        })
    }
}

pub struct OwnerPgTransaction {
    tx: Transaction<'static, Postgres>,
    owner: VerifiedOwner,
}
impl OwnerPgTransaction {
    pub fn connection(&mut self) -> &mut PgConnection {
        &mut self.tx
    }
    pub async fn commit(mut self) -> Result<(), AuthorityError> {
        tokio::time::timeout(Duration::from_secs(3), async {
            // Keep a legacy data table's UUID context for deferred constraints;
            // all authority reads always use the canonical raw tenant instead.
            let data_tenant: Option<String> = sqlx::query_scalar("SELECT pg_catalog.current_setting('app.current_tenant',true)")
                .fetch_one(&mut *self.tx).await?;
            set_tenant(&mut self.tx, self.owner.tenant_id()).await?;
            sqlx::query("SELECT pg_catalog.pg_advisory_xact_lock_shared(pg_catalog.hashtextextended(pg_catalog.jsonb_build_array('ohc-token-fence-v1',$1::text,$2::text)::text,0))")
                .bind(self.owner.tenant_id()).bind(self.owner.token_id()).execute(&mut *self.tx).await?;
            check_database_owner(&mut self.tx, &self.owner).await?;
            if let Some(tenant) = data_tenant {
                set_tenant(&mut self.tx, &tenant).await?;
            }
            self.tx.commit().await?;
            Ok(())
        }).await.map_err(|_| AuthorityError::Unavailable)?
    }
}

async fn set_tenant(connection: &mut PgConnection, tenant: &str) -> Result<(), AuthorityError> {
    if tenant.trim().is_empty() || tenant.trim() != tenant || tenant.eq_ignore_ascii_case("system")
    {
        return Err(AuthorityError::Forbidden);
    }
    sqlx::query("SELECT pg_catalog.set_config('role','none',true),pg_catalog.set_config('app.current_tenant',$1,true)")
        .bind(tenant).execute(connection).await?;
    Ok(())
}
async fn check_database_owner(
    connection: &mut PgConnection,
    owner: &VerifiedOwner,
) -> Result<(), AuthorityError> {
    let current:Option<String> = sqlx::query_scalar("SELECT u.id FROM users u JOIN identity_user_roles r ON r.user_id=u.id AND r.tenant_id=u.tenant_id WHERE u.id=$1 AND u.tenant_id=$2 AND u.active=TRUE AND pg_catalog.translate(r.role_name,'ABCDEFGHIJKLMNOPQRSTUVWXYZ','abcdefghijklmnopqrstuvwxyz') IN ('admin','owner') FOR SHARE OF u,r")
        .bind(owner.actor_id()).bind(owner.tenant_id()).fetch_optional(&mut *connection).await?;
    if current.is_none() {
        return Err(AuthorityError::Forbidden);
    }
    let now: i64 = sqlx::query_scalar(
        "SELECT floor(extract(epoch FROM pg_catalog.clock_timestamp()))::bigint",
    )
    .fetch_one(&mut *connection)
    .await?;
    let revoked: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM auth_revoked_tokens WHERE tenant_id=$1 AND jti=$2)",
    )
    .bind(owner.tenant_id())
    .bind(owner.token_id())
    .fetch_one(connection)
    .await?;
    if owner.expires_at() <= now || revoked {
        return Err(AuthorityError::Forbidden);
    }
    Ok(())
}
async fn begin_owner_transaction(
    pool: &PgPool,
    owner: &VerifiedOwner,
) -> Result<Transaction<'static, Postgres>, AuthorityError> {
    tokio::time::timeout(Duration::from_secs(3), async {
        let mut tx = pool.begin().await?;
        set_tenant(&mut tx, owner.tenant_id()).await?;
        sqlx::query("SET LOCAL statement_timeout='3000ms'")
            .execute(&mut *tx)
            .await?;
        sqlx::query("SET LOCAL lock_timeout='1000ms'")
            .execute(&mut *tx)
            .await?;
        let isolation: String = sqlx::query_scalar("SHOW transaction_isolation")
            .fetch_one(&mut *tx)
            .await?;
        if isolation != "read committed" {
            return Err(AuthorityError::Unavailable);
        }
        check_database_owner(&mut tx, owner).await?;
        Ok(tx)
    })
    .await
    .map_err(|_| AuthorityError::Unavailable)?
}

/// Reuse the canonical pool only after proving that the configured business pool
/// names the same actual database objects. This performs no schema/data writes.
/// Public reads and previously accepted workers keep their configured data pool.
pub async fn canonical_pg_data_pool(
    authority: &sea_orm::DatabaseConnection,
    data: &PgPool,
    business_relations: &[&str],
) -> Result<PgPool, AuthorityError> {
    if authority.get_database_backend() != sea_orm::DatabaseBackend::Postgres {
        return Err(AuthorityError::Unavailable);
    }
    let canonical = authority.get_postgres_connection_pool();
    // In particular, do not acquire the same one-connection pool twice.
    if std::ptr::eq(canonical.options(), data.options()) {
        return Ok(canonical.clone());
    }
    if business_relations.is_empty() || business_relations.len() > 32 {
        return Err(AuthorityError::Unavailable);
    }
    let mut relations = vec![
        "users".to_string(),
        "identity_user_roles".to_string(),
        "auth_revoked_tokens".to_string(),
    ];
    relations.extend(business_relations.iter().map(|name| (*name).to_owned()));
    let verified = tokio::time::timeout(Duration::from_secs(3), async {
        let mut canonical_tx = canonical.begin().await?;
        limit_probe(&mut canonical_tx).await?;
        let mut data_tx = data.begin().await?;
        limit_probe(&mut data_tx).await?;
        let proof = async {
            let nonce = uuid::Uuid::new_v4();
            let key = i64::from_be_bytes(
                nonce.as_bytes()[..8]
                    .try_into()
                    .map_err(|_| AuthorityError::Unavailable)?,
            );
            sqlx::query("SELECT pg_catalog.pg_advisory_xact_lock($1)")
                .bind(key)
                .execute(&mut *canonical_tx)
                .await?;
            let acquired: bool =
                sqlx::query_scalar("SELECT pg_catalog.pg_try_advisory_xact_lock($1)")
                    .bind(key)
                    .fetch_one(&mut *data_tx)
                    .await?;
            if acquired {
                // A different database/cluster has an independent lock namespace,
                // even if URLs, database names and cloned relation OIDs match.
                return Err(AuthorityError::Unavailable);
            }
            let canonical_path: Vec<String> =
                sqlx::query_scalar("SELECT pg_catalog.current_schemas(false)::text[]")
                    .fetch_one(&mut *canonical_tx)
                    .await?;
            let data_path: Vec<String> =
                sqlx::query_scalar("SELECT pg_catalog.current_schemas(false)::text[]")
                    .fetch_one(&mut *data_tx)
                    .await?;
            if canonical_path != data_path || canonical_path.is_empty() {
                return Err(AuthorityError::Unavailable);
            }
            let canonical_ids = relation_ids(&mut canonical_tx, &relations).await?;
            let data_ids = relation_ids(&mut data_tx, &relations).await?;
            if canonical_ids != data_ids || canonical_ids.iter().any(Option::is_none) {
                return Err(AuthorityError::Unavailable);
            }
            Ok(())
        }
        .await;
        // Confirm ordinary cleanup even for a rejected proof. On timeout,
        // SQLx's transaction drop queues rollback; the server also bounds idle
        // transactions so a lost connection cannot retain the random probe.
        let data_cleanup = data_tx.rollback().await;
        let canonical_cleanup = canonical_tx.rollback().await;
        proof?;
        data_cleanup?;
        canonical_cleanup?;
        Ok::<(), AuthorityError>(())
    })
    .await
    .map_err(|_| AuthorityError::Unavailable)?;
    verified?;
    Ok(canonical.clone())
}

async fn limit_probe(connection: &mut PgConnection) -> Result<(), AuthorityError> {
    sqlx::query("SET LOCAL statement_timeout='1000ms'")
        .execute(&mut *connection)
        .await?;
    sqlx::query("SET LOCAL lock_timeout='1000ms'")
        .execute(&mut *connection)
        .await?;
    sqlx::query("SET LOCAL idle_in_transaction_session_timeout='3000ms'")
        .execute(connection)
        .await?;
    Ok(())
}
async fn relation_ids(
    connection: &mut PgConnection,
    names: &[String],
) -> Result<Vec<Option<i64>>, AuthorityError> {
    Ok(sqlx::query_scalar("SELECT ARRAY(SELECT pg_catalog.to_regclass(r.name)::oid::bigint FROM pg_catalog.unnest($1::text[]) WITH ORDINALITY AS r(name,position) ORDER BY r.position)")
        .bind(names).fetch_one(connection).await?)
}
