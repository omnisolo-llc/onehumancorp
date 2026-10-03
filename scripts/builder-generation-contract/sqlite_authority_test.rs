//! Real portable owner-write contracts; no provider or substitute authority.
use axum::http::{HeaderMap, header::AUTHORIZATION};
use server_auth::commit_authority::{AuthorityError, CanonicalSqliteAuthority};
use server_common::Claims;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::{str::FromStr, sync::Arc, time::Duration};

struct OwnedDirectory(std::path::PathBuf);
impl OwnedDirectory {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("ohc-authority-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for OwnedDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

struct Fixture {
    pool: SqlitePool,
    store: Arc<server_auth::Store>,
    claims: Claims,
    headers: HeaderMap,
    url: String,
    _directory: OwnedDirectory,
}
impl Fixture {
    async fn new(in_memory: bool) -> Self {
        let directory = OwnedDirectory::new();
        let url = if in_memory {
            "sqlite::memory:".to_string()
        } else {
            format!(
                "sqlite://{}?mode=rwc",
                directory.0.join("owned.sqlite").display()
            )
        };
        let options = sqlx::sqlite::SqliteConnectOptions::from_str(&url)
            .unwrap()
            .foreign_keys(true)
            .busy_timeout(Duration::from_secs(7));
        let pool = SqlitePoolOptions::new()
            .max_connections(if in_memory { 1 } else { 4 })
            .connect_with(options)
            .await
            .unwrap();
        let database = crate::persistence::AppDatabase::from_connection(
            sea_orm::SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone()),
        );
        crate::persistence::migration::migrate(&database)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE authority_business(id INTEGER PRIMARY KEY, tenant_id TEXT NOT NULL, actor_id TEXT NOT NULL, value TEXT NOT NULL)")
            .execute(&pool).await.unwrap();
        let store = Arc::new(server_auth::Store::with_portable_repo(Arc::new(
            server_auth::seaorm_store::SeaOrmAuthRepository::new(database.connection().clone()),
        )));
        let user = store
            .create_user(
                "owner".into(),
                format!("{}@example.test", uuid::Uuid::new_v4()),
                "public-local-owner-fixture".into(),
                vec![server_auth::ROLE_ADMIN.into()],
                "owned-sqlite-tenant".into(),
            )
            .await
            .unwrap();
        let token = store.issue_token(&user).unwrap();
        let claims = store.validate_token(&token).await.unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
        Self {
            pool,
            store,
            claims,
            headers,
            url,
            _directory: directory,
        }
    }
    fn capability(&self) -> CanonicalSqliteAuthority {
        CanonicalSqliteAuthority::bind(self.store.clone(), &self.pool).unwrap()
    }
    async fn count(&self) -> i64 {
        tokio::time::timeout(
            Duration::from_secs(3),
            sqlx::query_scalar("SELECT COUNT(*) FROM authority_business").fetch_one(&self.pool),
        )
        .await
        .unwrap()
        .unwrap()
    }
}

#[tokio::test]
async fn sqlite_authority_same_pool_commits_the_signed_raw_tenant_and_actor() {
    let f = Fixture::new(false).await;
    let owner = f
        .capability()
        .authorize(&f.claims, &f.headers)
        .await
        .unwrap();
    assert_eq!(owner.tenant_id(), "owned-sqlite-tenant");
    assert_eq!(owner.actor_id(), f.claims.sub);
    let tenant = owner.tenant_id().to_owned();
    let actor = owner.actor_id().to_owned();
    let mut tx = owner.begin().await.unwrap();
    sqlx::query("INSERT INTO authority_business VALUES(1,?,?, 'accepted')")
        .bind(&tenant)
        .bind(&actor)
        .execute(tx.connection())
        .await
        .unwrap();
    tx.commit().await.unwrap();
    assert_eq!(f.count().await, 1);
}

#[tokio::test]
async fn sqlite_authority_independent_same_file_pool_and_copied_store_cannot_bind() {
    let f = Fixture::new(false).await;
    let second = SqlitePool::connect(&f.url).await.unwrap();
    assert!(matches!(
        CanonicalSqliteAuthority::bind(f.store.clone(), &second),
        Err(AuthorityError::Unavailable)
    ));
    let other = Fixture::new(false).await;
    assert!(matches!(
        CanonicalSqliteAuthority::bind(f.store.clone(), &other.pool),
        Err(AuthorityError::Unavailable)
    ));
    assert_eq!(f.count().await, 0);
    assert_eq!(other.count().await, 0);
}

#[tokio::test]
async fn sqlite_authority_rechecks_revocation_committed_before_write_intent() {
    let f = Fixture::new(false).await;
    let owner = f
        .capability()
        .authorize(&f.claims, &f.headers)
        .await
        .unwrap();
    f.store
        .revoke_token(
            f.claims.jti.clone(),
            chrono::DateTime::from_timestamp(f.claims.exp, 0).unwrap(),
            "owned-sqlite-tenant",
        )
        .await
        .unwrap();
    assert!(matches!(
        owner.begin().await,
        Err(AuthorityError::Forbidden)
    ));
    assert_eq!(f.count().await, 0);
}

#[tokio::test]
async fn sqlite_authority_rechecks_canonical_roles_after_authorization() {
    let f = Fixture::new(false).await;
    let owner = f
        .capability()
        .authorize(&f.claims, &f.headers)
        .await
        .unwrap();
    f.store
        .update_user(
            &f.claims.sub,
            None,
            Some(vec!["MEMBER".into()]),
            None,
            "owned-sqlite-tenant",
        )
        .await
        .unwrap();
    assert!(matches!(
        owner.begin().await,
        Err(AuthorityError::Forbidden)
    ));
    assert_eq!(f.count().await, 0);
}

#[tokio::test]
async fn sqlite_authority_canonical_revocation_writer_cannot_commit_inside_accepted_write() {
    let f = Fixture::new(false).await;
    let owner = f
        .capability()
        .authorize(&f.claims, &f.headers)
        .await
        .unwrap();
    let mut tx = owner.begin().await.unwrap();
    sqlx::query(
        "INSERT INTO authority_business VALUES(1,'owned-sqlite-tenant','owner','accepted')",
    )
    .execute(tx.connection())
    .await
    .unwrap();
    let store = f.store.clone();
    let claims = f.claims.clone();
    let mut revocation = tokio::spawn(async move {
        store
            .revoke_token(
                claims.jti,
                chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
                "owned-sqlite-tenant",
            )
            .await
    });
    let before_commit = tokio::time::timeout(Duration::from_millis(100), &mut revocation).await;
    if let Ok(result) = &before_commit {
        assert!(
            result.as_ref().unwrap().is_err(),
            "canonical revocation must not commit through SQLite write intent"
        );
    }
    tx.commit().await.unwrap();
    assert_eq!(f.count().await, 1);
    if before_commit.is_err() {
        let _ = revocation.await.unwrap();
    }
    f.store
        .revoke_token(
            f.claims.jti.clone(),
            chrono::DateTime::from_timestamp(f.claims.exp, 0).unwrap(),
            "owned-sqlite-tenant",
        )
        .await
        .unwrap();
    assert!(matches!(
        f.capability().authorize(&f.claims, &f.headers).await,
        Err(AuthorityError::Forbidden)
    ));
}

#[tokio::test]
async fn sqlite_authority_dropped_write_preserves_single_connection_memory_and_options() {
    let f = Fixture::new(true).await;
    let owner = f
        .capability()
        .authorize(&f.claims, &f.headers)
        .await
        .unwrap();
    let mut tx = owner.begin().await.unwrap();
    sqlx::query(
        "INSERT INTO authority_business VALUES(1,'owned-sqlite-tenant','owner','discarded')",
    )
    .execute(tx.connection())
    .await
    .unwrap();
    drop(tx);
    assert_eq!(f.count().await, 0);
    let timeout: i64 = sqlx::query_scalar("PRAGMA busy_timeout")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(timeout, 7000);
    assert!(
        f.store
            .validate_token(
                f.headers[AUTHORIZATION]
                    .to_str()
                    .unwrap()
                    .strip_prefix("Bearer ")
                    .unwrap()
            )
            .await
            .is_ok()
    );
    let mut tx = f
        .capability()
        .authorize(&f.claims, &f.headers)
        .await
        .unwrap()
        .begin()
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO authority_business VALUES(2,'owned-sqlite-tenant','owner','retained')",
    )
    .execute(tx.connection())
    .await
    .unwrap();
    tx.commit().await.unwrap();
    assert_eq!(f.count().await, 1);
    let timeout: i64 = sqlx::query_scalar("PRAGMA busy_timeout")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(timeout, 7000);
}

#[tokio::test]
async fn sqlite_authority_busy_begin_is_bounded_and_never_enqueues_a_commit() {
    let f = Fixture::new(false).await;
    let owner = f
        .capability()
        .authorize(&f.claims, &f.headers)
        .await
        .unwrap();
    let blocker = f.pool.begin_with("BEGIN IMMEDIATE").await.unwrap();
    let start = std::time::Instant::now();
    assert!(owner.begin().await.is_err());
    assert!(start.elapsed() < Duration::from_secs(3));
    blocker.rollback().await.unwrap();
    assert_eq!(f.count().await, 0);
    let owner = f
        .capability()
        .authorize(&f.claims, &f.headers)
        .await
        .unwrap();
    let blocker = f.pool.begin_with("BEGIN IMMEDIATE").await.unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(30), owner.begin())
            .await
            .is_err()
    );
    blocker.rollback().await.unwrap();
    let mut tx = f
        .capability()
        .authorize(&f.claims, &f.headers)
        .await
        .unwrap()
        .begin()
        .await
        .unwrap();
    sqlx::query("INSERT INTO authority_business VALUES(1,'owned-sqlite-tenant','owner','after cancellation')")
        .execute(tx.connection()).await.unwrap();
    tx.commit().await.unwrap();
    assert_eq!(f.count().await, 1);
}

#[tokio::test]
async fn sqlite_authority_failed_deferred_commit_rolls_back_without_claiming_success() {
    let f = Fixture::new(false).await;
    sqlx::query("CREATE TABLE authority_parent(id INTEGER PRIMARY KEY)")
        .execute(&f.pool)
        .await
        .unwrap();
    sqlx::query("CREATE TABLE authority_child(id INTEGER REFERENCES authority_parent(id) DEFERRABLE INITIALLY DEFERRED)").execute(&f.pool).await.unwrap();
    let mut tx = f
        .capability()
        .authorize(&f.claims, &f.headers)
        .await
        .unwrap()
        .begin()
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO authority_business VALUES(1,'owned-sqlite-tenant','owner','unconfirmed')",
    )
    .execute(tx.connection())
    .await
    .unwrap();
    sqlx::query("INSERT INTO authority_child VALUES(91)")
        .execute(tx.connection())
        .await
        .unwrap();
    assert!(matches!(
        tx.commit().await,
        Err(AuthorityError::Database(_))
    ));
    assert_eq!(f.count().await, 0);
}

#[tokio::test]
async fn sqlite_authority_forbidden_identity_and_disabled_foreign_keys_create_no_write() {
    let f = Fixture::new(true).await;
    let mut forged = f.claims.clone();
    forged.organization_id = Some("different-tenant".into());
    assert!(matches!(
        f.capability().authorize(&forged, &f.headers).await,
        Err(AuthorityError::Forbidden)
    ));
    sqlx::query("PRAGMA foreign_keys=OFF")
        .execute(&f.pool)
        .await
        .unwrap();
    assert!(matches!(
        f.capability().authorize(&f.claims, &f.headers).await,
        Err(AuthorityError::Unavailable)
    ));
    assert_eq!(f.count().await, 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn sqlite_authority_submitted_commit_timeout_is_unconfirmed_and_never_replays() {
    let f = Fixture::new(true).await;
    let mut tx = f
        .capability()
        .authorize(&f.claims, &f.headers)
        .await
        .unwrap()
        .begin()
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO authority_business VALUES(1,'owned-sqlite-tenant','owner','actual commit')",
    )
    .execute(tx.connection())
    .await
    .unwrap();
    let (entered_tx, entered_rx) = tokio::sync::oneshot::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    let mut entered_tx = Some(entered_tx);
    tx.connection()
        .lock_handle()
        .await
        .unwrap()
        .set_commit_hook(move || {
            if let Some(sender) = entered_tx.take() {
                sender.send(()).unwrap();
            }
            // This is the actual SQLite COMMIT callback, not an alternate store.
            // SQLx's safe hook returns true to allow the submitted commit.
            release_rx.recv_timeout(Duration::from_secs(5)).is_ok()
        });
    let commit = tokio::spawn(tx.commit());
    tokio::time::timeout(Duration::from_secs(1), entered_rx)
        .await
        .unwrap()
        .unwrap();
    let outcome = commit.await.unwrap();
    release_tx.send(()).unwrap();
    assert!(
        matches!(outcome, Err(AuthorityError::Unavailable)),
        "the submitted COMMIT was not acknowledged, so success/no-effect are both unproven"
    );
    assert_eq!(
        f.count().await,
        1,
        "the original submitted commit can complete after its caller's deadline"
    );
    let mut connection = f.pool.acquire().await.unwrap();
    connection.lock_handle().await.unwrap().remove_commit_hook();
    let timeout: i64 = sqlx::query_scalar("PRAGMA busy_timeout")
        .fetch_one(&mut *connection)
        .await
        .unwrap();
    assert_eq!(timeout, 7000);
    drop(connection);
    assert_eq!(
        f.count().await,
        1,
        "cleanup must never execute a second business effect"
    );
}
