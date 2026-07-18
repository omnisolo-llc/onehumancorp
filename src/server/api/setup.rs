use crate::db::DB;
use axum::{
    Json, Router,
    body::Bytes,
    extract::{DefaultBodyLimit, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::sync::Arc;

const MIN_SETUP_TOKEN_BYTES: usize = 32;
const MAX_SETUP_REQUEST_BYTES: usize = 4 * 1024;
const MIN_USERNAME_BYTES: usize = 3;
const MAX_USERNAME_BYTES: usize = 64;
const MAX_EMAIL_BYTES: usize = 254;
const MIN_PASSWORD_BYTES: usize = 12;
const MAX_PASSWORD_BYTES: usize = 72;
const MAX_ORGANIZATION_ID_BYTES: usize = 64;
const POSTGRES_BOOTSTRAP_LOCK_ID: i64 = 0x4f48_435f_5345_5455;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SetupAdminRequest {
    username: String,
    email: String,
    password: String,
    organization_id: String,
}

#[derive(Serialize)]
struct SetupAdminResponse {
    success: bool,
    message: &'static str,
}

#[derive(Clone)]
struct SetupState {
    db: Arc<DB>,
    token_key: Arc<[u8]>,
    expected_tag: [u8; 32],
    password_hash_slots: Arc<tokio::sync::Semaphore>,
}

#[derive(Clone, Copy)]
enum BootstrapOutcome {
    Created,
    Conflict,
}

enum BootstrapError {
    Database,
    PasswordHash,
}

impl From<sqlx::Error> for BootstrapError {
    fn from(_error: sqlx::Error) -> Self {
        Self::Database
    }
}

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {
    let token = match ::server_common::secret_source::load_optional_secret(
        "OHC_SETUP_TOKEN",
        "OHC_SETUP_TOKEN_FILE",
    ) {
        Ok(Some(token)) => token,
        Ok(None) | Err(_) => return Router::new(),
    };
    if token.len() < MIN_SETUP_TOKEN_BYTES {
        return Router::new();
    }

    let token_key: Arc<[u8]> = Arc::from(token);
    let mut expected =
        Hmac::<Sha256>::new_from_slice(&token_key).expect("HMAC accepts keys of every length");
    expected.update(&token_key);
    let expected_tag = expected.finalize().into_bytes().into();

    Router::new()
        .route("/admin", post(create_initial_admin))
        .layer(DefaultBodyLimit::max(MAX_SETUP_REQUEST_BYTES))
        .with_state(SetupState {
            db,
            token_key,
            expected_tag,
            password_hash_slots: Arc::new(tokio::sync::Semaphore::new(1)),
        })
}

fn token_is_authorized(state: &SetupState, headers: &HeaderMap) -> bool {
    let Some(candidate) = headers
        .get(axum::http::header::AUTHORIZATION)
        .map(|value| value.as_bytes())
        .and_then(|value| value.strip_prefix(b"Bearer "))
    else {
        return false;
    };

    let Ok(mut verifier) = Hmac::<Sha256>::new_from_slice(&state.token_key) else {
        return false;
    };
    verifier.update(candidate);
    verifier.verify_slice(&state.expected_tag).is_ok()
}

fn valid_username(value: &str) -> bool {
    (MIN_USERNAME_BYTES..=MAX_USERNAME_BYTES).contains(&value.len())
        && value
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_alphanumeric)
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

fn valid_email(value: &str) -> bool {
    if value.is_empty() || value.len() > MAX_EMAIL_BYTES || !value.is_ascii() {
        return false;
    }
    let Some((local, domain)) = value.split_once('@') else {
        return false;
    };
    if local.is_empty()
        || local.len() > 64
        || local.starts_with('.')
        || local.ends_with('.')
        || local.contains("..")
        || domain.len() > 253
        || !domain.contains('.')
        || domain.contains('@')
    {
        return false;
    }
    if !local.bytes().all(|byte| {
        byte.is_ascii_alphanumeric()
            || matches!(
                byte,
                b'.' | b'!'
                    | b'#'
                    | b'$'
                    | b'%'
                    | b'&'
                    | b'\''
                    | b'*'
                    | b'+'
                    | b'-'
                    | b'/'
                    | b'='
                    | b'?'
                    | b'^'
                    | b'_'
                    | b'`'
                    | b'{'
                    | b'|'
                    | b'}'
                    | b'~'
            )
    }) {
        return false;
    }
    domain.split('.').all(|label| {
        !label.is_empty()
            && label.len() <= 63
            && label
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_alphanumeric)
            && label
                .as_bytes()
                .last()
                .is_some_and(u8::is_ascii_alphanumeric)
            && label
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    })
}

fn valid_password(value: &str) -> bool {
    (MIN_PASSWORD_BYTES..=MAX_PASSWORD_BYTES).contains(&value.len())
        && !value.bytes().any(|byte| byte.is_ascii_control())
}

fn valid_organization_id(value: &str) -> bool {
    !value.eq_ignore_ascii_case("system")
        && (1..=MAX_ORGANIZATION_ID_BYTES).contains(&value.len())
        && value
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_alphanumeric)
        && value
            .as_bytes()
            .last()
            .is_some_and(u8::is_ascii_alphanumeric)
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

fn valid_request(request: &SetupAdminRequest) -> bool {
    valid_username(&request.username)
        && valid_email(&request.email)
        && valid_password(&request.password)
        && valid_organization_id(&request.organization_id)
}

fn setup_response(status: StatusCode, success: bool, message: &'static str) -> Response {
    let mut response = (status, Json(SetupAdminResponse { success, message })).into_response();
    response.headers_mut().insert(
        axum::http::header::CACHE_CONTROL,
        axum::http::HeaderValue::from_static("no-store"),
    );
    response
}

async fn hash_setup_password(
    password: &str,
    password_hash_slots: Arc<tokio::sync::Semaphore>,
) -> Result<String, BootstrapError> {
    let permit = password_hash_slots
        .acquire_owned()
        .await
        .map_err(|_| BootstrapError::PasswordHash)?;
    let password = password.to_string();
    tokio::task::spawn_blocking(move || {
        let _permit = permit;
        bcrypt::hash(password, crate::auth::DEFAULT_COST)
    })
    .await
    .map_err(|_| BootstrapError::PasswordHash)?
    .map_err(|_| BootstrapError::PasswordHash)
}

async fn create_initial_admin(
    State(state): State<SetupState>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    if !token_is_authorized(&state, &headers) {
        return setup_response(StatusCode::UNAUTHORIZED, false, "request failed");
    }

    let Ok(request) = serde_json::from_slice::<SetupAdminRequest>(&body) else {
        return setup_response(StatusCode::BAD_REQUEST, false, "invalid request");
    };
    if !valid_request(&request) {
        return setup_response(StatusCode::BAD_REQUEST, false, "invalid request");
    }

    let result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            bootstrap_postgres(&state.db, &request, state.password_hash_slots.clone()).await
        }
        crate::db::DbStore::Sqlite(pool) => {
            bootstrap_sqlite(pool, &request, state.password_hash_slots.clone()).await
        }
    };

    match result {
        Ok(BootstrapOutcome::Created) => {
            setup_response(StatusCode::CREATED, true, "admin account created")
        }
        Ok(BootstrapOutcome::Conflict) => {
            setup_response(StatusCode::CONFLICT, false, "admin already exists")
        }
        Err(_) => {
            tracing::error!("Admin bootstrap operation failed");
            setup_response(StatusCode::INTERNAL_SERVER_ERROR, false, "request failed")
        }
    }
}

async fn bootstrap_postgres(
    db: &DB,
    request: &SetupAdminRequest,
    password_hash_slots: Arc<tokio::sync::Semaphore>,
) -> Result<BootstrapOutcome, BootstrapError> {
    let mut transaction = db.pool.begin().await?;
    ::server_common::auth_utils::set_system_context(&mut *transaction).await?;
    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(POSTGRES_BOOTSTRAP_LOCK_ID)
        .execute(&mut *transaction)
        .await?;
    let admin_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM users AS u CROSS JOIN LATERAL unnest(COALESCE(u.roles, ARRAY[]::TEXT[])) AS role(value) WHERE UPPER(role.value) = $1)",
    )
    .bind(crate::auth::ROLE_ADMIN)
    .fetch_one(&mut *transaction)
    .await?;
    if admin_exists {
        transaction.commit().await?;
        return Ok(BootstrapOutcome::Conflict);
    }

    let password_hash = hash_setup_password(&request.password, password_hash_slots).await?;
    sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, $2) ON CONFLICT (id) DO NOTHING")
        .bind(&request.organization_id)
        .bind(&request.organization_id)
        .execute(&mut *transaction)
        .await?;
    let now = chrono::Utc::now();
    sqlx::query(
        "INSERT INTO users (id, username, email, password_hash, roles, active, tenant_id, created_at, updated_at) VALUES ($1, $2, $3, $4, ARRAY[$5]::TEXT[], TRUE, $6, $7, $7)",
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(&request.username)
    .bind(&request.email)
    .bind(&password_hash)
    .bind(crate::auth::ROLE_ADMIN)
    .bind(&request.organization_id)
    .bind(now)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(BootstrapOutcome::Created)
}

async fn bootstrap_sqlite(
    pool: &sqlx::SqlitePool,
    request: &SetupAdminRequest,
    password_hash_slots: Arc<tokio::sync::Semaphore>,
) -> Result<BootstrapOutcome, BootstrapError> {
    let mut transaction = pool.begin_with("BEGIN IMMEDIATE").await?;
    let admin_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM users AS u JOIN json_each(CASE WHEN json_valid(u.roles) THEN u.roles ELSE '[]' END) AS role WHERE UPPER(CAST(role.value AS TEXT)) = ?)",
    )
    .bind(crate::auth::ROLE_ADMIN)
    .fetch_one(&mut *transaction)
    .await?;
    if admin_exists {
        transaction.commit().await?;
        return Ok(BootstrapOutcome::Conflict);
    }

    let password_hash = hash_setup_password(&request.password, password_hash_slots).await?;
    sqlx::query("INSERT INTO tenants (id, name) VALUES (?, ?) ON CONFLICT (id) DO NOTHING")
        .bind(&request.organization_id)
        .bind(&request.organization_id)
        .execute(&mut *transaction)
        .await?;
    let now = chrono::Utc::now();
    sqlx::query(
        "INSERT INTO users (id, username, email, password_hash, roles, active, tenant_id, created_at, updated_at) VALUES (?, ?, ?, ?, json(?), TRUE, ?, ?, ?)",
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(&request.username)
    .bind(&request.email)
    .bind(&password_hash)
    .bind(serde_json::json!([crate::auth::ROLE_ADMIN]).to_string())
    .bind(&request.organization_id)
    .bind(now)
    .bind(now)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(BootstrapOutcome::Created)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{Body, to_bytes},
        http::Request,
    };
    use sqlx::Row;
    use tower::ServiceExt;

    const SETUP_TOKEN: &str = "0123456789abcdef0123456789abcdef";
    const VALID_REQUEST: &str = r#"{
        "username":"bootstrap-admin",
        "email":"admin@example.test",
        "password":"correct horse battery staple",
        "organizationId":"tenant-bootstrap"
    }"#;

    async fn sqlite_db() -> (Arc<DB>, sqlx::SqlitePool) {
        let pool = crate::db::create_sqlite_pool_for_test().await;
        let db = Arc::new(DB {
            pool: crate::db::create_dummy_pg_pool().await,
            store: crate::db::DbStore::Sqlite(pool.clone()),
        });
        db.run_migrations().await.unwrap();
        (db, pool)
    }

    fn request(token: Option<&str>, body: &str) -> Request<Body> {
        let mut builder = Request::post("/admin").header("content-type", "application/json");
        if let Some(token) = token {
            builder = builder.header("authorization", format!("Bearer {token}"));
        }
        builder.body(Body::from(body.to_owned())).unwrap()
    }

    async fn response_json(response: axum::response::Response) -> serde_json::Value {
        serde_json::from_slice(&to_bytes(response.into_body(), 4096).await.unwrap()).unwrap()
    }

    #[tokio::test]
    async fn setup_route_is_disabled_for_a_short_configured_token() {
        let (db, _) = sqlite_db().await;
        temp_env::async_with_vars(
            [("OHC_SETUP_TOKEN", Some("short-setup-token"))],
            async move {
                let response = router(db)
                    .oneshot(request(Some("short-setup-token"), VALID_REQUEST))
                    .await
                    .unwrap();
                assert_eq!(response.status(), StatusCode::NOT_FOUND);
            },
        )
        .await;
    }

    #[tokio::test]
    async fn setup_route_is_absent_without_a_configured_token() {
        let (db, _) = sqlite_db().await;
        temp_env::async_with_vars([("OHC_SETUP_TOKEN", None::<&str>)], async move {
            let response = router(db)
                .oneshot(request(None, VALID_REQUEST))
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::NOT_FOUND);
        })
        .await;
    }

    #[tokio::test]
    async fn setup_route_accepts_secure_file_token_and_rejects_ambiguous_sources() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("setup-token");
        std::fs::write(&path, format!("{SETUP_TOKEN}\n")).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        }

        let (db, _) = sqlite_db().await;
        temp_env::async_with_vars(
            [
                ("OHC_SETUP_TOKEN", None),
                ("OHC_SETUP_TOKEN_FILE", Some(path.to_str().unwrap())),
            ],
            async move {
                let response = router(db)
                    .oneshot(request(Some(SETUP_TOKEN), VALID_REQUEST))
                    .await
                    .unwrap();
                assert_eq!(response.status(), StatusCode::CREATED);
            },
        )
        .await;

        let (db, _) = sqlite_db().await;
        temp_env::async_with_vars(
            [
                ("OHC_SETUP_TOKEN", Some(SETUP_TOKEN)),
                ("OHC_SETUP_TOKEN_FILE", Some(path.to_str().unwrap())),
            ],
            async move {
                let response = router(db)
                    .oneshot(request(Some(SETUP_TOKEN), VALID_REQUEST))
                    .await
                    .unwrap();
                assert_eq!(response.status(), StatusCode::NOT_FOUND);
            },
        )
        .await;

        let missing_path = directory.path().join("missing-setup-token");
        let (db, _) = sqlite_db().await;
        temp_env::async_with_vars(
            [
                ("OHC_SETUP_TOKEN", None),
                ("OHC_SETUP_TOKEN_FILE", Some(missing_path.to_str().unwrap())),
            ],
            async move {
                let response = router(db)
                    .oneshot(request(Some(SETUP_TOKEN), VALID_REQUEST))
                    .await
                    .unwrap();
                assert_eq!(response.status(), StatusCode::NOT_FOUND);
            },
        )
        .await;
    }

    #[tokio::test]
    async fn setup_route_requires_the_exact_bearer_token() {
        for authorization in [None, Some("wrong-wrong-wrong-wrong-wrong-token")] {
            let (db, pool) = sqlite_db().await;
            temp_env::async_with_vars([("OHC_SETUP_TOKEN", Some(SETUP_TOKEN))], async move {
                let response = router(db)
                    .oneshot(request(authorization, VALID_REQUEST))
                    .await
                    .unwrap();
                assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
                assert_eq!(
                    response_json(response).await,
                    serde_json::json!({"success": false, "message": "request failed"})
                );
                let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
                    .fetch_one(&pool)
                    .await
                    .unwrap();
                assert_eq!(count, 0);
            })
            .await;
        }
    }

    #[tokio::test]
    async fn concurrent_setup_requests_create_one_admin_and_conflict_the_rest() {
        let (db, pool) = sqlite_db().await;
        temp_env::async_with_vars([("OHC_SETUP_TOKEN", Some(SETUP_TOKEN))], async move {
            let app = router(db);
            let mut tasks = Vec::new();
            for _ in 0..4 {
                let app = app.clone();
                tasks.push(tokio::spawn(async move {
                    app.oneshot(request(Some(SETUP_TOKEN), VALID_REQUEST))
                        .await
                        .unwrap()
                        .status()
                }));
            }
            let mut statuses = Vec::new();
            for task in tasks {
                statuses.push(task.await.unwrap());
            }
            assert_eq!(
                statuses
                    .iter()
                    .filter(|status| **status == StatusCode::CREATED)
                    .count(),
                1
            );
            assert_eq!(
                statuses
                    .iter()
                    .filter(|status| **status == StatusCode::CONFLICT)
                    .count(),
                3,
                "statuses: {statuses:?}"
            );
            let admin_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM users AS u JOIN json_each(u.roles) AS role WHERE UPPER(CAST(role.value AS TEXT)) = 'ADMIN'",
            )
            .fetch_one(&pool)
            .await
            .unwrap();
            assert_eq!(admin_count, 1);
        })
        .await;
    }

    #[tokio::test]
    async fn setup_creates_the_requested_tenant_and_a_fixed_admin() {
        let (db, pool) = sqlite_db().await;
        temp_env::async_with_vars([("OHC_SETUP_TOKEN", Some(SETUP_TOKEN))], async move {
            let response = router(db)
                .oneshot(request(Some(SETUP_TOKEN), VALID_REQUEST))
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::CREATED);

            let tenant_name: String =
                sqlx::query_scalar("SELECT name FROM tenants WHERE id = 'tenant-bootstrap'")
                    .fetch_one(&pool)
                    .await
                    .unwrap();
            assert_eq!(tenant_name, "tenant-bootstrap");

            let user = sqlx::query(
                "SELECT username, email, password_hash, roles, active, tenant_id FROM users",
            )
            .fetch_one(&pool)
            .await
            .unwrap();
            assert_eq!(user.get::<String, _>("username"), "bootstrap-admin");
            assert_eq!(user.get::<String, _>("email"), "admin@example.test");
            assert_eq!(user.get::<String, _>("tenant_id"), "tenant-bootstrap");
            assert!(user.get::<bool, _>("active"));
            assert_eq!(
                serde_json::from_str::<Vec<String>>(&user.get::<String, _>("roles")).unwrap(),
                vec![crate::auth::ROLE_ADMIN.to_string()]
            );
            let password_hash = user.get::<String, _>("password_hash");
            assert!(password_hash.starts_with("$2"));
            assert_eq!(password_hash.split('$').nth(2), Some("10"));
            assert!(bcrypt::verify("correct horse battery staple", &password_hash).unwrap());

            let auth_repository: Arc<dyn crate::auth::user_repository::UserRepository> =
                Arc::new(crate::auth::sqlite_store::SqliteUserRepository::new(
                    pool.clone(),
                ));
            let auth_store = Arc::new(crate::auth::Store::with_repo(auth_repository));
            let mut login_request = Request::post("/api/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"username":"bootstrap-admin","password":"correct horse battery staple","organization_id":"tenant-bootstrap"}"#,
                ))
                .unwrap();
            login_request.extensions_mut().insert(axum::extract::ConnectInfo(
                std::net::SocketAddr::from(([127, 0, 0, 1], 12345)),
            ));
            let login_response = crate::auth::http::router(auth_store.clone())
                .unwrap()
                .oneshot(login_request)
                .await
                .unwrap();
            assert_eq!(login_response.status(), StatusCode::OK);
            let login_body: serde_json::Value = serde_json::from_slice(
                &to_bytes(login_response.into_body(), 4096).await.unwrap(),
            )
            .unwrap();
            let access_token = login_body["token"].as_str().unwrap();
            let protected = Router::new()
                .route("/protected", axum::routing::get(|| async { "ok" }))
                .route_layer(axum::middleware::from_fn_with_state(
                    auth_store,
                    ::server_auth::strict_bearer_auth_middleware,
                ));
            let protected_response = protected
                .oneshot(
                    Request::get("/protected")
                        .header("authorization", format!("Bearer {access_token}"))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(protected_response.status(), StatusCode::OK);
        })
        .await;
    }

    #[tokio::test]
    async fn sqlite_migrations_create_tenant_scoped_auth_schema() {
        let (_, pool) = sqlite_db().await;
        let tables: Vec<String> = sqlx::query_scalar(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name IN ('users', 'revoked_tokens') ORDER BY name",
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(tables, vec!["revoked_tokens", "users"]);

        let user_columns: Vec<String> =
            sqlx::query_scalar("SELECT name FROM pragma_table_info('users') ORDER BY cid")
                .fetch_all(&pool)
                .await
                .unwrap();
        for required in [
            "id",
            "username",
            "email",
            "password_hash",
            "roles",
            "active",
            "tenant_id",
            "oidc_subject",
            "created_at",
            "updated_at",
        ] {
            assert!(
                user_columns.iter().any(|column| column == required),
                "missing {required}"
            );
        }
        let user_foreign_key: Option<String> = sqlx::query_scalar(
            "SELECT \"table\" FROM pragma_foreign_key_list('users') WHERE \"from\" = 'tenant_id'",
        )
        .fetch_optional(&pool)
        .await
        .unwrap();
        assert_eq!(user_foreign_key.as_deref(), Some("tenants"));

        for tenant in ["tenant-a", "tenant-b"] {
            sqlx::query("INSERT INTO tenants (id, name) VALUES (?, ?)")
                .bind(tenant)
                .bind(tenant)
                .execute(&pool)
                .await
                .unwrap();
        }
        for (id, tenant) in [("user-a", "tenant-a"), ("user-b", "tenant-b")] {
            sqlx::query("INSERT INTO users (id, username, email, password_hash, roles, active, tenant_id, oidc_subject) VALUES (?, 'same-name', 'same@example.test', 'hash', json('[\"ADMIN\"]'), TRUE, ?, 'same-subject')")
                .bind(id)
                .bind(tenant)
                .execute(&pool)
                .await
                .unwrap();
        }
        for (username, email, subject) in [
            ("same-name", "other@example.test", "other-subject"),
            ("other-name", "same@example.test", "other-subject"),
            ("other-name", "other@example.test", "same-subject"),
        ] {
            let result = sqlx::query("INSERT INTO users (id, username, email, password_hash, roles, active, tenant_id, oidc_subject) VALUES (?, ?, ?, 'hash', json('[]'), TRUE, 'tenant-a', ?)")
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(username)
                .bind(email)
                .bind(subject)
                .execute(&pool)
                .await;
            assert!(result.is_err());
        }

        for tenant in ["tenant-a", "tenant-b"] {
            sqlx::query("INSERT INTO revoked_tokens (jti, tenant_id, expires_at) VALUES ('same-jti', ?, CURRENT_TIMESTAMP)")
                .bind(tenant)
                .execute(&pool)
                .await
                .unwrap();
        }
        assert!(sqlx::query("INSERT INTO revoked_tokens (jti, tenant_id, expires_at) VALUES ('same-jti', 'tenant-a', CURRENT_TIMESTAMP)")
            .execute(&pool)
            .await
            .is_err());
    }

    #[tokio::test]
    async fn setup_rejects_an_existing_admin_across_tenants() {
        let (db, pool) = sqlite_db().await;
        sqlx::query("INSERT INTO tenants (id, name) VALUES ('existing', 'existing')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO users (id, username, email, password_hash, roles, active, tenant_id, created_at, updated_at) VALUES ('existing-admin', 'existing-admin', 'existing@example.test', 'hash', '[\"admin\"]', TRUE, 'existing', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
            .execute(&pool)
            .await
            .unwrap();

        temp_env::async_with_vars([("OHC_SETUP_TOKEN", Some(SETUP_TOKEN))], async move {
            let response = router(db)
                .oneshot(request(Some(SETUP_TOKEN), VALID_REQUEST))
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::CONFLICT);
            let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
                .fetch_one(&pool)
                .await
                .unwrap();
            assert_eq!(count, 1);
        })
        .await;
    }

    #[tokio::test]
    async fn setup_rolls_back_tenant_creation_when_user_creation_fails() {
        let (db, pool) = sqlite_db().await;
        sqlx::query("CREATE TRIGGER reject_bootstrap_admin BEFORE INSERT ON users WHEN NEW.username = 'bootstrap-admin' BEGIN SELECT RAISE(ABORT, 'forced user failure'); END")
            .execute(&pool)
            .await
            .unwrap();

        temp_env::async_with_vars([("OHC_SETUP_TOKEN", Some(SETUP_TOKEN))], async move {
            let response = router(db)
                .oneshot(request(Some(SETUP_TOKEN), VALID_REQUEST))
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
            assert_eq!(
                response_json(response).await,
                serde_json::json!({"success": false, "message": "request failed"})
            );
            let tenant_count: i64 =
                sqlx::query_scalar("SELECT COUNT(*) FROM tenants WHERE id = 'tenant-bootstrap'")
                    .fetch_one(&pool)
                    .await
                    .unwrap();
            assert_eq!(tenant_count, 0);
        })
        .await;
    }

    #[tokio::test]
    async fn setup_rejects_unknown_fields_and_invalid_values() {
        let invalid_requests = [
            r#"{"username":"ab","email":"admin@example.test","password":"correct horse battery staple","organizationId":"tenant-bootstrap"}"#,
            r#"{"username":"bootstrap-admin","email":"not-an-email","password":"correct horse battery staple","organizationId":"tenant-bootstrap"}"#,
            r#"{"username":"bootstrap-admin","email":"admin@example.test","password":"too-short","organizationId":"tenant-bootstrap"}"#,
            r#"{"username":"bootstrap-admin","email":"admin@example.test","password":"correct horse battery staple","organizationId":"system"}"#,
            r#"{"username":"bootstrap-admin","email":"admin@example.test","password":"correct horse battery staple","organizationId":"tenant-bootstrap","role":"VIEWER"}"#,
        ];

        for body in invalid_requests {
            let (db, _) = sqlite_db().await;
            temp_env::async_with_vars([("OHC_SETUP_TOKEN", Some(SETUP_TOKEN))], async move {
                let response = router(db)
                    .oneshot(request(Some(SETUP_TOKEN), body))
                    .await
                    .unwrap();
                assert_eq!(response.status(), StatusCode::BAD_REQUEST, "body: {body}");
            })
            .await;
        }
    }

    #[tokio::test]
    async fn public_setup_and_oauth_are_merged_after_protected_routes() {
        use axum::routing::get;

        let source = include_str!("../lib.rs");
        let production_source = source
            .rsplit_once("\n#[cfg(test)]\nmod tests {")
            .expect("server source must retain its final test-module boundary")
            .0;
        let app_source = production_source
            .split_once("let app = axum::Router::new()")
            .expect("real HTTP router must remain identifiable")
            .1;
        assert!(!production_source.contains(".nest(\"/oauth\""));
        assert!(
            production_source.contains(".nest(\"/api/v1/oauth\", api::oauth::proxy::router())")
        );
        let protected_layers_end = app_source
            .find(".with_state(mesh_transport)")
            .expect("protected router must install its state after route layers");
        let setup_merge = app_source
            .find(".merge(setup_router)")
            .expect("setup router must be merged into the real HTTP router");
        let oauth_merge = app_source
            .find(".merge(oauth_callback_router)")
            .expect("OAuth callback router must be merged into the real HTTP router");
        assert!(setup_merge > protected_layers_end);
        assert!(oauth_merge > protected_layers_end);

        type MeshState = Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>;
        let auth_store = Arc::new(crate::auth::Store::new());
        let protected = Router::<MeshState>::new()
            .route("/api/v1/protected", get(|| async { "protected" }))
            .route_layer(axum::middleware::from_fn_with_state(
                auth_store,
                ::server_auth::strict_bearer_auth_middleware,
            ));
        let (db, _) = sqlite_db().await;
        let transport: MeshState =
            Arc::new(ohc_builtin_agent::mesh::transport::InProcessTransport::new());

        temp_env::async_with_vars([("OHC_SETUP_TOKEN", Some(SETUP_TOKEN))], async move {
            let setup_router = Router::<MeshState>::new().nest("/api/v1/setup", router(db));
            let oauth_router = Router::<MeshState>::new()
                .nest("/api/v1/oauth", crate::api::oauth::proxy::router());
            let app = protected
                .merge(setup_router)
                .merge(oauth_router)
                .with_state(transport);

            let setup_response = app
                .clone()
                .oneshot(
                    Request::post("/api/v1/setup/admin")
                        .header("content-type", "application/json")
                        .header("authorization", format!("Bearer {SETUP_TOKEN}"))
                        .body(Body::from(VALID_REQUEST))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(setup_response.status(), StatusCode::CREATED);

            let protected_response = app
                .clone()
                .oneshot(
                    Request::get("/api/v1/protected")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(protected_response.status(), StatusCode::UNAUTHORIZED);

            let legacy_setup_response = app
                .clone()
                .oneshot(
                    Request::post("/api/setup/admin")
                        .header("content-type", "application/json")
                        .header("authorization", format!("Bearer {SETUP_TOKEN}"))
                        .body(Body::from(VALID_REQUEST))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(legacy_setup_response.status(), StatusCode::NOT_FOUND);

            let oauth_response = app
                .oneshot(
                    Request::get("/api/v1/oauth/callback?code=test&state=cloud")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(oauth_response.status(), StatusCode::OK);
        })
        .await;
    }

    #[test]
    fn postgres_bootstrap_uses_established_system_context() {
        let source = include_str!("setup.rs");
        let production_source = source
            .rsplit_once("\n#[cfg(test)]\nmod tests {")
            .expect("setup source must retain its test boundary")
            .0;
        assert!(
            production_source
                .contains("::server_common::auth_utils::set_system_context(&mut *transaction)")
        );
        assert!(!production_source.contains("SET LOCAL row_security = off"));
    }

    #[test]
    fn bootstrap_hashes_only_after_cancellation_safe_backend_lock_and_admin_check() {
        let source = include_str!("setup.rs");
        let production_source = source
            .rsplit_once("\n#[cfg(test)]\nmod tests {")
            .expect("setup source must retain its test boundary")
            .0;
        let handler = production_source
            .split_once("async fn create_initial_admin")
            .unwrap()
            .1
            .split_once("async fn bootstrap_postgres")
            .unwrap()
            .0;
        assert!(!handler.contains("bcrypt::hash"));
        assert!(production_source.contains("password_hash_slots: Arc<tokio::sync::Semaphore>"));
        assert!(production_source.contains("Arc::new(tokio::sync::Semaphore::new(1))"));
        let hash_helper = production_source
            .split_once("async fn hash_setup_password")
            .unwrap()
            .1
            .split_once("async fn create_initial_admin")
            .unwrap()
            .0;
        assert!(hash_helper.contains("acquire_owned"));
        assert!(hash_helper.contains("let _permit = permit"));
        assert!(hash_helper.contains("spawn_blocking"));

        let postgres = production_source
            .split_once("async fn bootstrap_postgres")
            .unwrap()
            .1
            .split_once("async fn bootstrap_sqlite")
            .unwrap()
            .0;
        let postgres_lock = postgres.find("pg_advisory_xact_lock").unwrap();
        let postgres_admin_check = postgres.find("let admin_exists").unwrap();
        let postgres_hash = postgres.find("hash_setup_password").unwrap();
        assert!(postgres_lock < postgres_admin_check && postgres_admin_check < postgres_hash);

        let sqlite = production_source
            .split_once("async fn bootstrap_sqlite")
            .unwrap()
            .1;
        assert!(sqlite.contains("pool.begin_with(\"BEGIN IMMEDIATE\")"));
        assert!(!sqlite.contains("sqlx::query(\"BEGIN IMMEDIATE\")"));
        let sqlite_lock = sqlite.find("pool.begin_with").unwrap();
        let sqlite_admin_check = sqlite.find("let admin_exists").unwrap();
        let sqlite_hash = sqlite.find("hash_setup_password").unwrap();
        assert!(sqlite_lock < sqlite_admin_check && sqlite_admin_check < sqlite_hash);
    }
}
