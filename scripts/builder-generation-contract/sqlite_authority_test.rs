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

fn isolated_startup_case(name: &str, values: &[(&str, &str)]) -> bool {
    if std::env::var("OHC_SQLITE_STARTUP_CHILD").as_deref() == Ok(name) {
        return false;
    }
    let directory = OwnedDirectory::new();
    let mut child = std::process::Command::new(std::env::current_exe().unwrap());
    child
        .args([
            "--exact",
            &format!("tests::sqlite_authority_test::{name}"),
            "--nocapture",
        ])
        .env("OHC_SQLITE_STARTUP_CHILD", name)
        .env("TEST_WORKSPACE", env!("CARGO_MANIFEST_DIR"))
        .env("TEST_TMPDIR", &directory.0);
    for key in [
        "DATABASE_URL",
        "DATABASE_URL_FILE",
        "OMNISOLO_DATABASE_URL",
        "OMNISOLO_DATABASE_URL_FILE",
        "OHC_DATABASE_URL",
        "OMNISOLO_SQLITE_KEY",
        "JWT_SECRET_FILE",
        "REDIS_URL",
        "REDIS_URL_FILE",
        "OMNISOLO_REDIS_URL",
    ] {
        child.env_remove(key);
    }
    for (key, value) in values {
        child.env(key, value);
    }
    let result = child.output().unwrap();
    assert!(
        result.status.success(),
        "isolated startup case failed: {}{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    true
}

fn configured_sqlite_store(pool: &SqlitePool) -> crate::startup_db::DB {
    crate::startup_db::DB {
        pool: sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://unused@127.0.0.1:1/unused")
            .unwrap(),
        store: crate::startup_db::DbStore::Sqlite(pool.clone()),
    }
}

#[tokio::test]
async fn sqlite_startup_without_environment_keeps_existing_account_and_paid_data() {
    if isolated_startup_case(
        "sqlite_startup_without_environment_keeps_existing_account_and_paid_data",
        &[],
    ) {
        return;
    }
    let f = Fixture::new(false).await;
    sqlx::query("INSERT INTO tenants(id,name,plan_tier,tier,default_currency) VALUES('owned-sqlite-tenant','Existing fixture business','pro','pro','EUR') ON CONFLICT(id) DO UPDATE SET plan_tier='pro',tier='pro',default_currency='EUR'")
        .execute(&f.pool).await.unwrap();
    sqlx::query(
        "INSERT INTO authority_business VALUES(1,'owned-sqlite-tenant','owner','existing data')",
    )
    .execute(&f.pool)
    .await
    .unwrap();
    let selected = crate::application_startup_database(configured_sqlite_store(&f.pool), true)
        .await.unwrap().expect("a configured standalone SQLite store must not lose authentication when no environment URL is supplied");
    assert!(std::ptr::eq(
        selected.connection().get_sqlite_connection_pool().options(),
        f.pool.options()
    ));
    crate::persistence::migration::migrate(&selected)
        .await
        .unwrap();
    let reopened = Arc::new(server_auth::Store::with_portable_repo(Arc::new(
        server_auth::seaorm_store::SeaOrmAuthRepository::new(selected.connection().clone()),
    )));
    let capability = CanonicalSqliteAuthority::bind(reopened, &f.pool).unwrap();
    capability.authorize(&f.claims, &f.headers).await.unwrap();
    let plan: (String, String, String) = sqlx::query_as(
        "SELECT plan_tier,tier,default_currency FROM tenants WHERE id='owned-sqlite-tenant'",
    )
    .fetch_one(&f.pool)
    .await
    .unwrap();
    assert_eq!(plan, ("pro".into(), "pro".into(), "EUR".into()));
    assert_eq!(f.count().await, 1);
}

#[tokio::test]
async fn sqlite_startup_configured_memory_keeps_the_actual_pool_options_and_hooks() {
    if isolated_startup_case(
        "sqlite_startup_configured_memory_keeps_the_actual_pool_options_and_hooks",
        &[
            ("DATABASE_URL", "sqlite::memory:"),
            ("OMNISOLO_SQLITE_KEY", "public-owned-startup-fixture-key"),
        ],
    ) {
        return;
    }
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .after_connect(|connection, _| {
            Box::pin(async move {
                sqlx::query("PRAGMA cache_size=-12345")
                    .execute(connection)
                    .await?;
                Ok(())
            })
        })
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .in_memory(true)
                .foreign_keys(true)
                .busy_timeout(Duration::from_secs(7)),
        )
        .await
        .unwrap();
    sqlx::query("CREATE TABLE existing_pool_witness(value TEXT)")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO existing_pool_witness VALUES('same configured database')")
        .execute(&pool)
        .await
        .unwrap();
    let selected = crate::application_startup_database(configured_sqlite_store(&pool), true)
        .await
        .unwrap()
        .unwrap();
    let actual = selected.connection().get_sqlite_connection_pool();
    assert!(
        std::ptr::eq(actual.options(), pool.options()),
        "equal memory URLs must not create an unrelated auth database"
    );
    let cache: i64 = sqlx::query_scalar("PRAGMA cache_size")
        .fetch_one(actual)
        .await
        .unwrap();
    let timeout: i64 = sqlx::query_scalar("PRAGMA busy_timeout")
        .fetch_one(actual)
        .await
        .unwrap();
    let value: String = sqlx::query_scalar("SELECT value FROM existing_pool_witness")
        .fetch_one(actual)
        .await
        .unwrap();
    assert_eq!((cache, timeout), (-12345, 7000));
    assert_eq!(value, "same configured database");
}

#[tokio::test]
async fn sqlite_startup_conflicting_database_selectors_are_not_silently_accepted() {
    if isolated_startup_case(
        "sqlite_startup_conflicting_database_selectors_are_not_silently_accepted",
        &[
            ("DATABASE_URL", "sqlite::memory:"),
            ("OMNISOLO_DATABASE_URL", "sqlite::memory:"),
        ],
    ) {
        return;
    }
    assert!(crate::startup_db::configured_url().is_err());
}

#[tokio::test]
async fn sqlite_startup_database_url_file_aliases_keep_the_selected_value() {
    if std::env::var("OHC_SQLITE_STARTUP_CHILD").as_deref()
        == Ok("sqlite_startup_database_url_file_aliases_keep_the_selected_value")
    {
        assert_eq!(
            crate::startup_db::configured_url().unwrap().as_deref(),
            Some("sqlite::memory:")
        );
        return;
    }
    let directory = OwnedDirectory::new();
    let file = directory.0.join("public-fixture-url");
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    use std::io::Write;
    options
        .open(&file)
        .unwrap()
        .write_all(b"sqlite::memory:\n")
        .unwrap();
    for key in ["DATABASE_URL_FILE", "OMNISOLO_DATABASE_URL_FILE"] {
        assert!(isolated_startup_case(
            "sqlite_startup_database_url_file_aliases_keep_the_selected_value",
            &[(key, file.to_str().unwrap())]
        ));
    }
}

#[tokio::test]
async fn sqlite_startup_requested_encryption_requires_an_actual_cipher_engine() {
    let directory = OwnedDirectory::new();
    let path = directory.0.join("encrypted.sqlite");
    let url = format!("sqlite://{}?mode=rwc", path.display());
    let probe = SqlitePool::connect("sqlite::memory:").await.unwrap();
    let version: Option<String> = sqlx::query_scalar("PRAGMA cipher_version")
        .fetch_optional(&probe)
        .await
        .unwrap();
    probe.close().await;
    #[cfg(feature = "production-sqlcipher")]
    assert!(
        version.as_deref().is_some_and(|value| !value.is_empty()),
        "production-sqlcipher must exercise a real cipher engine, not the plain-engine rejection branch"
    );
    let opened = crate::persistence::AppDatabase::connect_with_sqlcipher_key(
        &url,
        "public-owned-cipher-fixture-key",
    )
    .await;
    if version.as_deref().is_none_or(str::is_empty) {
        match opened {
            Ok(_) => panic!(
                "requested encryption must fail clearly when this SQLite engine has no SQLCipher"
            ),
            Err(error) => assert!(error.to_string().contains("SQLCipher")),
        }
        return;
    }
    let opened = opened.unwrap();
    let pool = opened.connection().get_sqlite_connection_pool();
    sqlx::query("CREATE TABLE encrypted_witness(value INTEGER)")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO encrypted_witness VALUES(1)")
        .execute(pool)
        .await
        .unwrap();
    pool.close().await;
    let stored_bytes = std::fs::read(&path).unwrap();
    assert!(stored_bytes.len() >= 4096);
    assert!(!stored_bytes.starts_with(b"SQLite format 3\0"));
    let reopened = crate::persistence::AppDatabase::connect_with_sqlcipher_key(
        &url,
        "public-owned-cipher-fixture-key",
    )
    .await
    .unwrap();
    let value: i64 = sqlx::query_scalar("SELECT value FROM encrypted_witness")
        .fetch_one(reopened.connection().get_sqlite_connection_pool())
        .await
        .unwrap();
    assert_eq!(value, 1);
    reopened
        .connection()
        .get_sqlite_connection_pool()
        .close()
        .await;
    assert!(
        crate::persistence::AppDatabase::connect_with_sqlcipher_key(
            &url,
            "public-wrong-fixture-key"
        )
        .await
        .is_err()
    );
    eprintln!(
        "actual cipher verified: nonempty encrypted file, original data reopened, wrong key rejected"
    );
}
