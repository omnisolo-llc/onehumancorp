use crate::{agent_definitions::DefinitionStore, definitions_api};
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use serde_json::{Value, json};
use std::sync::Arc;
use tower::ServiceExt;
use uuid::Uuid;

async fn sqlite_schema(pool: &sqlx::SqlitePool, url: &str) {
    sqlx::raw_sql(include_str!("core_sqlite.sql"))
        .execute(pool)
        .await
        .unwrap();
    sqlx::raw_sql(include_str!(
        "../../src/server/persistence/agent_definitions_sqlite.sql"
    ))
    .execute(pool)
    .await
    .unwrap();
    let database = crate::persistence::AppDatabase::connect(url).await.unwrap();
    crate::persistence::migration::migrate(&database)
        .await
        .unwrap();
    for tenant in ["definition-a", "definition-b", "disk-tenant"] {
        sqlx::query("INSERT INTO tenants(id,name) VALUES($1,$1) ON CONFLICT(id) DO NOTHING")
            .bind(tenant)
            .execute(pool)
            .await
            .unwrap();
    }
    let repository =
        server_auth::seaorm_store::SeaOrmAuthRepository::new(database.connection().clone());
    create_identity(&repository, "disk-user", "disk-tenant", "OWNER").await;
}
async fn create_identity(
    repository: &server_auth::seaorm_store::SeaOrmAuthRepository,
    id: &str,
    tenant: &str,
    role: &str,
) {
    use server_auth::user_repository::UserRepository;
    let now = chrono::Utc::now();
    repository
        .create_user(
            server_auth::User {
                id: id.into(),
                username: id.into(),
                email: format!("{id}@example.test"),
                password_hash: String::new(),
                roles: vec![role.into()],
                active: true,
                organization_id: Some(tenant.into()),
                created_at: now,
                updated_at: now,
                oidc_subject: None,
            },
            tenant,
        )
        .await
        .unwrap();
}

struct Fixture {
    app: Router,
    store: DefinitionStore,
    auth: Arc<server_auth::Store>,
    owner: String,
    same_tenant: String,
    foreign: String,
    staff: String,
    sqlite: sqlx::SqlitePool,
}
impl Fixture {
    async fn sqlite() -> Self {
        let url = format!(
            "sqlite:file:definitions_{}?mode=memory&cache=shared",
            Uuid::new_v4()
        );
        let sqlite = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(4)
            .connect(&url)
            .await
            .unwrap();
        sqlite_schema(&sqlite, &url).await;
        Self::with_store(DefinitionStore::Sqlite(sqlite.clone()), sqlite).await
    }
    async fn with_store(store: DefinitionStore, sqlite: sqlx::SqlitePool) -> Self {
        let connection = match &store {
            DefinitionStore::Sqlite(pool) => {
                sea_orm::SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone())
            }
            DefinitionStore::Postgres(pool) => {
                sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone())
            }
            DefinitionStore::Unavailable => unreachable!(),
        };
        let auth = Arc::new(server_auth::Store::with_portable_repo(Arc::new(
            server_auth::seaorm_store::SeaOrmAuthRepository::new(connection),
        )));
        let mut tokens = vec![];
        for (name, tenant, role) in [
            ("owner-a", "definition-a", "OWNER"),
            ("owner-b", "definition-a", "admin"),
            ("owner-c", "definition-b", "OWNER"),
            ("staff", "definition-a", "STAFF"),
        ] {
            let user = auth
                .create_user(
                    name.into(),
                    format!("{name}@example.test"),
                    "public-synthetic-test-password".into(),
                    vec![role.into()],
                    tenant.into(),
                )
                .await
                .unwrap();
            tokens.push(auth.issue_token(&user).unwrap());
        }
        let app = Router::new().merge(definitions_api::router(store.clone(), auth.clone()));
        Self {
            app,
            store,
            auth,
            owner: tokens.remove(0),
            same_tenant: tokens.remove(0),
            foreign: tokens.remove(0),
            staff: tokens.remove(0),
            sqlite,
        }
    }
    async fn request(
        &self,
        method: &str,
        path: &str,
        token: Option<&str>,
        body: Value,
    ) -> (StatusCode, Value) {
        let mut req = Request::builder()
            .method(method)
            .uri(path)
            .header("content-type", "application/json")
            .header("x-tenant-id", "definition-b")
            .header("x-spiffe-id", "spiffe://ohc/org/definition-b/agent/forged");
        if let Some(token) = token {
            req = req.header("authorization", format!("Bearer {token}"));
        }
        let res = self
            .app
            .clone()
            .oneshot(
                req.body(if method == "GET" {
                    Body::empty()
                } else {
                    Body::from(body.to_string())
                })
                .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            res.headers().get("cache-control").unwrap(),
            "private, no-store"
        );
        let status = res.status();
        let bytes = to_bytes(res.into_body(), 2_097_152).await.unwrap();
        (
            status,
            if bytes.is_empty() {
                Value::Null
            } else {
                serde_json::from_slice(&bytes).unwrap()
            },
        )
    }
    async fn publish(&self, request: Value) -> Value {
        let (status, body) = self
            .request(
                "POST",
                "/api/v1/agents/definitions",
                Some(&self.owner),
                request,
            )
            .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert_eq!(body["success"], true);
        assert_eq!(body["status"], "published");
        body
    }
}
fn publish_request() -> Value {
    json!({"request_id":Uuid::new_v4(),"name":"Reviewed Analyst","description":"Exact reviewed public description","role":"Research analyst","system_prompt":"Read authorized sources and report uncertainty. Do not execute actions.","visibility":"public"})
}
fn install_request(receipt: &Value) -> Value {
    json!({"request_id":Uuid::new_v4(),"version":receipt["definition"]["version"],"digest":receipt["definition"]["digest"]})
}

#[tokio::test]
async fn publication_replay_binds_exact_reviewed_content_and_owner() {
    let f = Fixture::sqlite().await;
    let request = publish_request();
    let first = f.publish(request.clone()).await;
    let second = f.publish(request.clone()).await;
    assert_eq!(first["definition"], second["definition"]);
    assert_eq!(first["replayed"], false);
    assert_eq!(second["replayed"], true);
    assert_eq!(first["organization_id"], "definition-a");
    assert_eq!(
        first["definition"]["system_prompt"],
        request["system_prompt"]
    );
    assert_eq!(first["definition"]["digest"].as_str().unwrap().len(), 64);
    let mut changed = request.clone();
    changed["system_prompt"] = json!("Different reviewed instructions");
    assert_eq!(
        f.request(
            "POST",
            "/api/v1/agents/definitions",
            Some(&f.owner),
            changed
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    let path = format!(
        "/api/v1/agents/definitions/operations/{}",
        request["request_id"].as_str().unwrap()
    );
    let (status, recovered) = f.request("GET", &path, Some(&f.owner), Value::Null).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(recovered["definition"], first["definition"]);
    for token in [&f.same_tenant, &f.foreign] {
        assert_eq!(
            f.request("GET", &path, Some(token), Value::Null).await.0,
            StatusCode::NOT_FOUND
        );
    }
}

#[tokio::test]
async fn install_is_durable_inactive_namespaced_and_idempotent_across_requests() {
    let f = Fixture::sqlite().await;
    let published = f.publish(publish_request()).await;
    let id = published["definition"]["id"].as_str().unwrap();
    let path = format!("/api/v1/agents/definitions/{id}/install");
    let request = install_request(&published);
    let (status, first) = f
        .request("POST", &path, Some(&f.owner), request.clone())
        .await;
    assert_eq!(status, StatusCode::OK, "{first}");
    assert_eq!(first["status"], "installed_inactive");
    let (_, second) = f.request("POST", &path, Some(&f.owner), request).await;
    assert_eq!(first["installation"], second["installation"]);
    assert_eq!(second["replayed"], true);
    let (_, third) = f
        .request("POST", &path, Some(&f.owner), install_request(&published))
        .await;
    assert_eq!(first["installation"], third["installation"]);
    assert!(
        first["installation"]["role_key"]
            .as_str()
            .unwrap()
            .starts_with("marketplace/")
    );
    assert_eq!(
        first["installation"]["system_prompt"],
        published["definition"]["system_prompt"]
    );
    let (_, own) = f
        .request(
            "GET",
            "/api/v1/agents/definitions",
            Some(&f.owner),
            Value::Null,
        )
        .await;
    assert_eq!(own["installations"].as_array().unwrap().len(), 1);
    for token in [&f.same_tenant, &f.foreign] {
        let (_, other) = f
            .request(
                "GET",
                "/api/v1/agents/definitions",
                Some(token),
                Value::Null,
            )
            .await;
        assert_eq!(other["installations"], json!([]));
        assert!(
            other["definitions"]
                .as_array()
                .unwrap()
                .iter()
                .any(|d| d["id"] == id)
        );
        assert!(!other["definitions"].to_string().contains("definition-a"));
        assert!(!other["definitions"].to_string().contains("user_id"));
    }
}

#[tokio::test]
async fn authority_unknown_fields_and_stale_installation_fail_before_writes() {
    let f = Fixture::sqlite().await;
    assert_eq!(
        f.request(
            "POST",
            "/api/v1/agents/definitions",
            None,
            publish_request()
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        f.request(
            "POST",
            "/api/v1/agents/definitions",
            Some(&f.staff),
            publish_request()
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    let mut injected = publish_request();
    injected["organization_id"] = json!("definition-b");
    assert_eq!(
        f.request(
            "POST",
            "/api/v1/agents/definitions",
            Some(&f.owner),
            injected
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    let published = f.publish(publish_request()).await;
    let path = format!(
        "/api/v1/agents/definitions/{}/install",
        published["definition"]["id"].as_str().unwrap()
    );
    let mut stale = install_request(&published);
    stale["digest"] = json!("0".repeat(64));
    assert_eq!(
        f.request("POST", &path, Some(&f.owner), stale).await.0,
        StatusCode::CONFLICT
    );
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_definition_installations")
        .fetch_one(&f.sqlite)
        .await
        .unwrap();
    assert_eq!(count, 0);
}

#[tokio::test]
async fn parallel_same_request_commits_only_one_publication() {
    let f = Arc::new(Fixture::sqlite().await);
    let request = publish_request();
    let mut pending = vec![];
    for _ in 0..8 {
        let f = f.clone();
        let request = request.clone();
        pending.push(tokio::spawn(async move { f.publish(request).await }));
    }
    let mut definitions = vec![];
    for pending in pending {
        definitions.push(pending.await.unwrap()["definition"].clone());
    }
    assert!(
        definitions
            .iter()
            .all(|definition| definition == &definitions[0])
    );
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_definition_operations")
        .fetch_one(&f.sqlite)
        .await
        .unwrap();
    assert_eq!(count, 1);
}

#[tokio::test]
async fn explicit_seed_and_get_are_inert_and_immutable() {
    let f = Fixture::sqlite().await;
    let (_, catalogue) = f
        .request(
            "GET",
            "/api/v1/agents/definitions",
            Some(&f.owner),
            Value::Null,
        )
        .await;
    let names: Vec<_> = catalogue["definitions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, vec!["Senior Rust Developer", "Technical Writer"]);
    assert!(
        catalogue["definitions"]
            .as_array()
            .unwrap()
            .iter()
            .all(|d| d["source"] == "first_party")
    );
    assert_eq!(catalogue["installations"], json!([]));
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_definition_operations")
            .fetch_one(&f.sqlite)
            .await
            .unwrap(),
        0
    );
    assert!(
        sqlx::query("UPDATE agent_definitions SET document='{}'")
            .execute(&f.sqlite)
            .await
            .is_err()
    );
    assert!(
        sqlx::query("DELETE FROM agent_definitions")
            .execute(&f.sqlite)
            .await
            .is_err()
    );
    let (_, again) = f
        .request(
            "GET",
            "/api/v1/agents/definitions",
            Some(&f.owner),
            Value::Null,
        )
        .await;
    assert_eq!(catalogue, again);
}

#[tokio::test]
async fn revoked_and_expired_real_bearers_cannot_mutate_definitions() {
    let f = Fixture::sqlite().await;
    let signing_key = b"public-local-definition-regression-signing-key-only";
    let validated = jsonwebtoken::decode::<server_common::Claims>(
        &f.owner,
        &jsonwebtoken::DecodingKey::from_secret(signing_key),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .unwrap();
    let mut claims = validated.claims;
    claims.exp -= 172800;
    claims.iat -= 172800;
    let expired = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(signing_key),
    )
    .unwrap();
    assert_eq!(
        f.request(
            "POST",
            "/api/v1/agents/definitions",
            Some(&expired),
            publish_request()
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    f.auth.logout_token(&f.owner).await.unwrap();
    assert_eq!(
        f.request(
            "POST",
            "/api/v1/agents/definitions",
            Some(&f.owner),
            publish_request()
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_definition_operations")
            .fetch_one(&f.sqlite)
            .await
            .unwrap(),
        0
    );
}

#[tokio::test]
async fn expected_owner_headers_are_paired_singular_preconditions() {
    let f = Fixture::sqlite().await;
    let claims = f.auth.validate_token(&f.owner).await.unwrap();
    for headers in [
        vec![("x-ohc-expected-user", claims.sub.as_str())],
        vec![
            ("x-ohc-expected-user", claims.sub.as_str()),
            ("x-ohc-expected-tenant", "definition-b"),
        ],
        vec![
            ("x-ohc-expected-user", claims.sub.as_str()),
            ("x-ohc-expected-user", claims.sub.as_str()),
            ("x-ohc-expected-tenant", "definition-a"),
        ],
    ] {
        let mut request = Request::builder()
            .method("POST")
            .uri("/api/v1/agents/definitions")
            .header("authorization", format!("Bearer {}", f.owner))
            .header("content-type", "application/json");
        for (name, value) in headers {
            request.headers_mut().unwrap().append(
                axum::http::header::HeaderName::from_bytes(name.as_bytes()).unwrap(),
                value.parse().unwrap(),
            );
        }
        let response = f
            .app
            .clone()
            .oneshot(
                request
                    .body(Body::from(publish_request().to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_definition_operations")
            .fetch_one(&f.sqlite)
            .await
            .unwrap(),
        0
    );
    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/agents/definitions")
        .header("authorization", format!("Bearer {}", f.owner))
        .header("content-type", "application/json")
        .header("x-ohc-expected-user", claims.sub)
        .header("x-ohc-expected-tenant", "definition-a")
        .body(Body::from(publish_request().to_string()))
        .unwrap();
    assert_eq!(
        f.app.clone().oneshot(request).await.unwrap().status(),
        StatusCode::OK
    );
}

#[tokio::test]
async fn transaction_failure_leaves_no_public_or_private_partial_record() {
    let f = Fixture::sqlite().await;
    sqlx::raw_sql("CREATE TRIGGER reject_receipt BEFORE INSERT ON agent_definition_operations BEGIN SELECT RAISE(ABORT,'synthetic receipt failure'); END;").execute(&f.sqlite).await.unwrap();
    let request = publish_request();
    let (status, body) = f
        .request(
            "POST",
            "/api/v1/agents/definitions",
            Some(&f.owner),
            request.clone(),
        )
        .await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body["success"], false);
    assert_eq!(body["reason"], "database_operation_failed");
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_definitions")
            .fetch_one(&f.sqlite)
            .await
            .unwrap(),
        2
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_definition_publishers")
            .fetch_one(&f.sqlite)
            .await
            .unwrap(),
        0
    );
    let path = format!(
        "/api/v1/agents/definitions/operations/{}",
        request["request_id"].as_str().unwrap()
    );
    assert_eq!(
        f.request("GET", &path, Some(&f.owner), Value::Null).await.0,
        StatusCode::NOT_FOUND
    );
}

#[tokio::test]
async fn independent_store_recovery_returns_the_existing_receipt_after_lost_response() {
    let f = Fixture::sqlite().await;
    let request = publish_request();
    let saved = f.publish(request.clone()).await;
    let store = match &f.store {
        DefinitionStore::Sqlite(pool) => DefinitionStore::Sqlite(pool.clone()),
        _ => unreachable!(),
    };
    let app: Router = definitions_api::router(store, f.auth.clone());
    let path = format!(
        "/api/v1/agents/definitions/operations/{}",
        request["request_id"].as_str().unwrap()
    );
    let response = app
        .oneshot(
            Request::builder()
                .uri(path)
                .header("authorization", format!("Bearer {}", f.owner))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let recovered: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 262144).await.unwrap()).unwrap();
    assert_eq!(recovered["definition"], saved["definition"]);
    assert_eq!(recovered["replayed"], true);
}

#[tokio::test]
async fn paged_catalogue_has_literal_search_and_owner_bound_cursors() {
    let f = Fixture::sqlite().await;
    let mut request = publish_request();
    request["name"] = json!("100% Ready_notes");
    f.publish(request).await;
    let (_, literal) = f
        .request(
            "GET",
            "/api/v1/agents/definitions?q=%25",
            Some(&f.owner),
            Value::Null,
        )
        .await;
    assert_eq!(literal["definitions"].as_array().unwrap().len(), 1);
    assert_eq!(literal["definitions"][0]["name"], "100% Ready_notes");
    let (_, first) = f
        .request(
            "GET",
            "/api/v1/agents/definitions?limit=1",
            Some(&f.owner),
            Value::Null,
        )
        .await;
    let cursor = first["next_cursor"].as_str().unwrap();
    let path = format!("/api/v1/agents/definitions?limit=1&cursor={cursor}");
    let (_, second) = f.request("GET", &path, Some(&f.owner), Value::Null).await;
    assert_ne!(
        first["definitions"][0]["id"],
        second["definitions"][0]["id"]
    );
    assert_eq!(
        f.request("GET", &path, Some(&f.same_tenant), Value::Null)
            .await
            .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        f.request(
            "GET",
            &format!("{path}&q=changed"),
            Some(&f.owner),
            Value::Null
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
}

struct PgFixture {
    admin: sqlx::PgPool,
    pool: sqlx::PgPool,
    store: DefinitionStore,
    schema: String,
    role: String,
    created_migration_role: bool,
}
impl PgFixture {
    async fn new() -> Self {
        use std::str::FromStr;
        let url = std::env::var("OHC_AGENT_DEFINITION_TEST_DATABASE_URL")
            .expect("explicit disposable PostgreSQL URL required");
        let admin_options = sqlx::postgres::PgConnectOptions::from_str(&url).unwrap();
        assert!(
            admin_options
                .get_host()
                .parse::<std::net::IpAddr>()
                .is_ok_and(|host| host.is_loopback()),
            "Only an explicit loopback disposable PostgreSQL service is accepted"
        );
        assert!(
            !url.contains('?') && !url.contains('#') && !url.chars().any(char::is_control),
            "Disposable URLs must not override connection/session settings"
        );
        let database = admin_options
            .get_database()
            .expect("explicit test database required");
        assert!(
            database.starts_with("ohc_")
                && database.ends_with("_test")
                && database.len() > 8
                && database.len() <= 63
                && database
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'_'),
            "A dedicated ohc_*_test database is required"
        );
        let schema = format!("definition_{}", Uuid::new_v4().simple());
        let role = format!("definition_role_{}", Uuid::new_v4().simple());
        let password = Uuid::new_v4().simple().to_string();
        let admin = crate::db::secure_pg_pool_options()
            .max_connections(2)
            .connect_with(
                admin_options
                    .clone()
                    .options([("search_path", schema.as_str())]),
            )
            .await
            .unwrap();
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        // The complete production setup/global registration writers require
        // the same NOLOGIN migration role as migrations003/1003. It is never
        // granted to the restricted marketplace role and is removed if owned.
        let role_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM pg_roles WHERE rolname='ohc_bypassrls')",
        )
        .fetch_one(&admin)
        .await
        .unwrap();
        if !role_exists {
            sqlx::query("CREATE ROLE ohc_bypassrls NOLOGIN BYPASSRLS")
                .execute(&admin)
                .await
                .unwrap();
        }
        let (bypass, login): (bool, bool) = sqlx::query_as(
            "SELECT rolbypassrls,rolcanlogin FROM pg_roles WHERE rolname='ohc_bypassrls'",
        )
        .fetch_one(&admin)
        .await
        .unwrap();
        assert!(
            bypass && !login,
            "The existing global auth boundary requires its designated NOLOGIN migration role"
        );
        sqlx::raw_sql(&format!("GRANT USAGE ON SCHEMA {schema} TO ohc_bypassrls; ALTER DEFAULT PRIVILEGES IN SCHEMA {schema} GRANT ALL ON TABLES TO ohc_bypassrls;")).execute(&admin).await.unwrap();
        sqlx::raw_sql(include_str!("core_pg.sql"))
            .execute(&admin)
            .await
            .unwrap();
        sqlx::raw_sql(include_str!(
            "../../src/server/migrations/1018_agent_definition_marketplace.sql"
        ))
        .execute(&admin)
        .await
        .unwrap();
        assert!(
            !url.contains('?'),
            "The owned fixture DSN must not supply session options"
        );
        let database = crate::persistence::AppDatabase::connect(&format!(
            "{url}?options=-csearch_path%3D{schema}"
        ))
        .await
        .unwrap();
        crate::persistence::migration::migrate(&database)
            .await
            .unwrap();
        for tenant in ["pg-a", "pg-b", "definition-a", "definition-b"] {
            sqlx::query("INSERT INTO tenants(id,name) VALUES($1,$1)")
                .bind(tenant)
                .execute(&admin)
                .await
                .unwrap();
        }
        let repository =
            server_auth::seaorm_store::SeaOrmAuthRepository::new(database.connection().clone());
        for (user, tenant) in [
            ("user-a", "pg-a"),
            ("user-b", "pg-a"),
            ("user-c", "pg-b"),
            ("owner", "pg-a"),
        ] {
            create_identity(&repository, user, tenant, "OWNER").await;
        }
        sqlx::raw_sql(&format!("ALTER TABLE users ENABLE ROW LEVEL SECURITY; ALTER TABLE users FORCE ROW LEVEL SECURITY; CREATE POLICY definition_user_tenant ON users USING(tenant_id=current_setting('app.current_tenant',true)) WITH CHECK(tenant_id=current_setting('app.current_tenant',true)); CREATE ROLE {role} LOGIN NOSUPERUSER NOBYPASSRLS PASSWORD '{password}'; GRANT USAGE ON SCHEMA {schema} TO {role}; GRANT SELECT,INSERT,UPDATE,DELETE ON ALL TABLES IN SCHEMA {schema} TO {role}; REVOKE UPDATE,DELETE ON agent_definition_authorities FROM {role};")).execute(&admin).await.unwrap();
        drop(repository);
        database.connection().clone().close().await.unwrap();
        let options = sqlx::postgres::PgConnectOptions::from_str(&url)
            .unwrap()
            .username(&role)
            .password(&password)
            .options([("search_path", schema.as_str())]);
        let pool = crate::db::secure_pg_pool_options()
            .max_connections(4)
            .connect_with(options)
            .await
            .unwrap();
        let (current,session,superuser,bypass):(String,String,bool,bool)=sqlx::query_as("SELECT current_user::text,session_user::text,rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user").fetch_one(&pool).await.unwrap();
        assert_eq!(current, role);
        assert_eq!(session, role);
        assert!(!superuser && !bypass);
        let forced:i64=sqlx::query_scalar("SELECT count(*) FROM pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace WHERE n.nspname=$1 AND c.relname IN ('agent_definitions','agent_definition_publishers','agent_definition_installations','agent_definition_operations') AND c.relrowsecurity AND c.relforcerowsecurity").bind(&schema).fetch_one(&pool).await.unwrap();
        assert_eq!(forced, 4);
        Self {
            admin,
            pool: pool.clone(),
            store: DefinitionStore::Postgres(pool),
            schema,
            role,
            created_migration_role: !role_exists,
        }
    }
    async fn close(self) {
        self.pool.close().await;
        sqlx::raw_sql(&format!(
            "DROP SCHEMA {} CASCADE; DROP ROLE {};",
            self.schema, self.role
        ))
        .execute(&self.admin)
        .await
        .unwrap();
        if self.created_migration_role {
            sqlx::query("DROP ROLE ohc_bypassrls")
                .execute(&self.admin)
                .await
                .unwrap();
        }
        self.admin.close().await;
    }
}
fn owner(tenant: &str, user: &str) -> crate::agent_definitions::Owner {
    crate::agent_definitions::Owner {
        tenant: tenant.into(),
        user: user.into(),
    }
}
fn typed_publish() -> crate::agent_definitions::PublishRequest {
    serde_json::from_value(publish_request()).unwrap()
}

#[tokio::test]
async fn postgres_forced_rls_scopes_private_rows_and_reused_connections() {
    let pg = PgFixture::new().await;
    let a = owner("pg-a", "user-a");
    let b = owner("pg-b", "user-c");
    let same_tenant = owner("pg-a", "user-b");
    let request = typed_publish();
    let published = pg.store.publish(&a, &request).await.unwrap();
    let definition = published.definition.unwrap();
    let install = crate::agent_definitions::InstallRequest {
        request_id: Uuid::new_v4(),
        version: definition.version,
        digest: definition.digest.clone(),
    };
    pg.store
        .install(&a, Uuid::parse_str(&definition.id).unwrap(), &install)
        .await
        .unwrap();
    for foreign in [&b, &same_tenant] {
        assert!(matches!(
            pg.store.operation(foreign, request.request_id).await,
            Err(crate::agent_definitions::Error::NotFound)
        ));
        let catalog = pg.store.list(foreign, &Default::default()).await.unwrap();
        assert!(catalog.installations.is_empty());
        assert!(catalog.definitions.iter().any(|d| d.id == definition.id));
        let mut tx = pg.pool.begin().await.unwrap();
        server_common::auth_utils::set_org_context(&mut *tx, &foreign.tenant)
            .await
            .unwrap();
        sqlx::query("SELECT set_config('app.current_actor',$1,true)")
            .bind(&foreign.user)
            .execute(&mut *tx)
            .await
            .unwrap();
        for table in [
            "agent_definition_publishers",
            "agent_definition_installations",
            "agent_definition_operations",
        ] {
            let count: i64 = sqlx::query_scalar(&format!("SELECT count(*) FROM {table}"))
                .fetch_one(&mut *tx)
                .await
                .unwrap();
            assert_eq!(count, 0, "{table}");
        }
        tx.commit().await.unwrap();
    }
    let (tenant,actor):(Option<String>,Option<String>)=sqlx::query_as("SELECT current_setting('app.current_tenant',true),current_setting('app.current_actor',true)").fetch_one(&pg.pool).await.unwrap();
    assert!(tenant.is_none_or(|v| v.is_empty()));
    assert!(actor.is_none_or(|v| v.is_empty()));
    let own = pg.store.list(&a, &Default::default()).await.unwrap();
    assert_eq!(own.installations.len(), 1);
    let identity:(String,String,bool,bool)=sqlx::query_as("SELECT current_user::text,session_user::text,rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user").fetch_one(&pg.pool).await.unwrap();
    assert_eq!(identity, (pg.role.clone(), pg.role.clone(), false, false));
    pg.close().await;
}

#[tokio::test]
async fn postgres_concurrent_replay_and_distinct_install_requests_share_one_snapshot() {
    let pg = PgFixture::new().await;
    let a = owner("pg-a", "owner");
    let request = typed_publish();
    let mut pending = vec![];
    for _ in 0..8 {
        let store = pg.store.clone();
        let a = a.clone();
        let request = request.clone();
        pending.push(tokio::spawn(
            async move { store.publish(&a, &request).await },
        ));
    }
    let mut saved = vec![];
    for p in pending {
        saved.push(p.await.unwrap().unwrap().definition.unwrap());
    }
    assert!(saved.iter().all(|d| d == &saved[0]));
    let mut pending = vec![];
    for _ in 0..8 {
        let store = pg.store.clone();
        let a = a.clone();
        let d = saved[0].clone();
        pending.push(tokio::spawn(async move {
            store
                .install(
                    &a,
                    Uuid::parse_str(&d.id).unwrap(),
                    &crate::agent_definitions::InstallRequest {
                        request_id: Uuid::new_v4(),
                        version: d.version,
                        digest: d.digest,
                    },
                )
                .await
        }));
    }
    let mut installed = vec![];
    for p in pending {
        installed.push(p.await.unwrap().unwrap().installation.unwrap());
    }
    assert!(installed.iter().all(|i| i == &installed[0]));
    assert_eq!(
        pg.store
            .list(&a, &Default::default())
            .await
            .unwrap()
            .installations
            .len(),
        1
    );
    pg.close().await;
}

#[tokio::test]
async fn postgres_deferred_commit_failure_rolls_back_definition_and_receipt() {
    let pg = PgFixture::new().await;
    let a = owner("pg-a", "owner");
    sqlx::raw_sql(&format!("CREATE FUNCTION {}.reject_definition_commit() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'synthetic deferred failure'; END $$; CREATE CONSTRAINT TRIGGER reject_definition_commit AFTER INSERT ON {}.agent_definition_operations DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION {}.reject_definition_commit();",pg.schema,pg.schema,pg.schema)).execute(&pg.admin).await.unwrap();
    let request = typed_publish();
    assert!(matches!(
        pg.store.publish(&a, &request).await,
        Err(crate::agent_definitions::Error::Database(_))
    ));
    assert!(matches!(
        pg.store.operation(&a, request.request_id).await,
        Err(crate::agent_definitions::Error::NotFound)
    ));
    let rows = pg.store.list(&a, &Default::default()).await.unwrap();
    assert_eq!(rows.definitions.len(), 2);
    assert!(rows.installations.is_empty());
    pg.close().await;
}

#[tokio::test]
async fn real_bearer_http_publication_uses_the_postgres_store() {
    let pg = PgFixture::new().await;
    let temporary = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    let f = Fixture::with_store(pg.store.clone(), temporary).await;
    let request = publish_request();
    let receipt = f.publish(request.clone()).await;
    assert_eq!(receipt["organization_id"], "definition-a");
    let (_, rows) = f
        .request(
            "GET",
            "/api/v1/agents/definitions",
            Some(&f.foreign),
            Value::Null,
        )
        .await;
    assert!(
        rows["definitions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["id"] == receipt["definition"]["id"])
    );
    assert_eq!(
        f.request(
            "GET",
            &format!(
                "/api/v1/agents/definitions/operations/{}",
                request["request_id"].as_str().unwrap()
            ),
            Some(&f.foreign),
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    drop(f);
    pg.close().await;
}

#[tokio::test]
async fn sqlite_file_reopen_preserves_receipts_and_inactive_installations() {
    let path =
        std::env::temp_dir().join(format!("ohc-definition-reopen-{}.sqlite", Uuid::new_v4()));
    let url = format!("sqlite://{}?mode=rwc", path.display());
    let pool = sqlx::SqlitePool::connect(&url).await.unwrap();
    sqlite_schema(&pool, &url).await;
    let a = owner("disk-tenant", "disk-user");
    let request = typed_publish();
    let store = DefinitionStore::Sqlite(pool.clone());
    let receipt = store.publish(&a, &request).await.unwrap();
    let definition = receipt.definition.unwrap();
    let installation = store
        .install(
            &a,
            Uuid::parse_str(&definition.id).unwrap(),
            &crate::agent_definitions::InstallRequest {
                request_id: Uuid::new_v4(),
                version: 1,
                digest: definition.digest.clone(),
            },
        )
        .await
        .unwrap()
        .installation
        .unwrap();
    drop(store);
    pool.close().await;
    drop(pool);
    let reopened = sqlx::SqlitePool::connect(&url).await.unwrap();
    let second = DefinitionStore::Sqlite(reopened.clone());
    assert_eq!(
        second
            .operation(&a, request.request_id)
            .await
            .unwrap()
            .definition
            .unwrap(),
        definition
    );
    assert_eq!(
        second
            .list(&a, &Default::default())
            .await
            .unwrap()
            .installations,
        vec![installation]
    );
    drop(second);
    reopened.close().await;
    drop(reopened);
    std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn concurrent_conflicting_bodies_cannot_adopt_another_receipt() {
    let f = Arc::new(Fixture::sqlite().await);
    let a = publish_request();
    let mut b = a.clone();
    b["system_prompt"] = json!("A different explicitly reviewed definition");
    let (one, two) = tokio::join!(
        f.request("POST", "/api/v1/agents/definitions", Some(&f.owner), a),
        f.request("POST", "/api/v1/agents/definitions", Some(&f.owner), b)
    );
    assert!(
        (one.0 == StatusCode::OK && two.0 == StatusCode::CONFLICT)
            || (two.0 == StatusCode::OK && one.0 == StatusCode::CONFLICT)
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_definition_operations")
            .fetch_one(&f.sqlite)
            .await
            .unwrap(),
        1
    );
}

#[tokio::test]
async fn request_limits_preserve_unicode_and_reject_unsupported_content_before_writes() {
    let f = Fixture::sqlite().await;
    for (field, value) in [
        ("name", json!("a".repeat(121))),
        ("role", json!(" ")),
        ("description", json!("a".repeat(2001))),
        ("system_prompt", json!("x".repeat(16001))),
        ("system_prompt", json!("before\u{0}after")),
        ("visibility", json!("private")),
        ("source", json!("first_party")),
    ] {
        let mut body = publish_request();
        body[field] = value;
        let (status, error) = f
            .request("POST", "/api/v1/agents/definitions", Some(&f.owner), body)
            .await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{field}: {error}");
        assert_eq!(error["success"], false);
    }
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_definition_operations")
            .fetch_one(&f.sqlite)
            .await
            .unwrap(),
        0
    );
    let mut unicode = publish_request();
    unicode["name"] = json!("界".repeat(120));
    let receipt = f.publish(unicode.clone()).await;
    assert_eq!(receipt["definition"]["name"], unicode["name"]);
}

#[tokio::test]
async fn encoded_page_budget_returns_whole_prompts_and_recoverable_cursors() {
    let f = Fixture::sqlite().await;
    for index in 0..26 {
        let mut body = publish_request();
        body["name"] = json!(format!("Budget entry {index}"));
        body["system_prompt"] = json!("🚀".repeat(16000));
        let published = f.publish(body).await;
        let path = format!(
            "/api/v1/agents/definitions/{}/install",
            published["definition"]["id"].as_str().unwrap()
        );
        assert_eq!(
            f.request("POST", &path, Some(&f.owner), install_request(&published))
                .await
                .0,
            StatusCode::OK
        );
    }
    let mut public_cursor = None;
    let mut installation_cursor = None;
    let mut definitions = std::collections::BTreeSet::new();
    let mut installations = std::collections::BTreeSet::new();
    let mut pages = 0;
    loop {
        let mut path = "/api/v1/agents/definitions?limit=100&q=Budget".to_string();
        if let Some(cursor) = &public_cursor {
            path.push_str(&format!("&cursor={cursor}"));
        }
        if let Some(cursor) = &installation_cursor {
            path.push_str(&format!("&installation_cursor={cursor}"));
        }
        let (status, page) = f.request("GET", &path, Some(&f.owner), Value::Null).await;
        assert_eq!(status, StatusCode::OK);
        assert!(serde_json::to_vec(&page).unwrap().len() < 2_097_152);
        for definition in page["definitions"].as_array().unwrap() {
            assert_eq!(
                definition["system_prompt"]
                    .as_str()
                    .unwrap()
                    .chars()
                    .count(),
                16000
            );
            definitions.insert(definition["id"].as_str().unwrap().to_string());
        }
        for installation in page["installations"].as_array().unwrap() {
            assert_eq!(
                installation["system_prompt"]
                    .as_str()
                    .unwrap()
                    .chars()
                    .count(),
                16000
            );
            assert_eq!(installation["status"], "installed_inactive");
            installations.insert(installation["id"].as_str().unwrap().to_string());
        }
        public_cursor = page["next_cursor"].as_str().map(str::to_string);
        installation_cursor = page["next_installation_cursor"]
            .as_str()
            .map(str::to_string);
        pages += 1;
        if public_cursor.is_none() && installation_cursor.is_none() {
            break;
        }
        assert!(pages <= 10, "Pagination failed to make progress");
    }
    assert!(pages > 1);
    assert_eq!(definitions.len(), 26);
    assert_eq!(installations.len(), 26);
}

#[tokio::test]
async fn operation_id_conflicts_across_publish_and_install_kinds() {
    let f = Fixture::sqlite().await;
    let request = publish_request();
    let published = f.publish(request.clone()).await;
    let mut install = install_request(&published);
    install["request_id"] = request["request_id"].clone();
    let path = format!(
        "/api/v1/agents/definitions/{}/install",
        published["definition"]["id"].as_str().unwrap()
    );
    assert_eq!(
        f.request("POST", &path, Some(&f.owner), install).await.0,
        StatusCode::CONFLICT
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_definition_installations")
            .fetch_one(&f.sqlite)
            .await
            .unwrap(),
        0
    );
}

#[tokio::test]
async fn missing_schema_read_does_not_seed_or_claim_an_empty_catalogue() {
    let empty = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    let mut f = Fixture::sqlite().await;
    f.app = definitions_api::router(DefinitionStore::Sqlite(empty.clone()), f.auth.clone());
    let (status, value) = f
        .request(
            "GET",
            "/api/v1/agents/definitions",
            Some(&f.owner),
            Value::Null,
        )
        .await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(value["success"], false);
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM sqlite_master WHERE type='table'")
            .fetch_one(&empty)
            .await
            .unwrap(),
        0
    );
}

#[tokio::test]
async fn postgres_in_flight_original_and_exact_replay_wait_for_one_committed_receipt() {
    let pg = PgFixture::new().await;
    let a = owner("pg-a", "owner");
    let request = typed_publish();
    let barrier: i64 = 77_031;
    let mut blocker = pg.admin.acquire().await.unwrap();
    sqlx::query("SELECT pg_advisory_lock($1)")
        .bind(barrier)
        .execute(&mut *blocker)
        .await
        .unwrap();
    sqlx::raw_sql(&format!("CREATE FUNCTION {}.hold_definition_receipt() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN PERFORM pg_advisory_xact_lock({barrier}); RETURN NEW; END $$; CREATE TRIGGER hold_definition_receipt BEFORE INSERT ON {}.agent_definition_operations FOR EACH ROW EXECUTE FUNCTION {}.hold_definition_receipt();",pg.schema,pg.schema,pg.schema)).execute(&pg.admin).await.unwrap();
    let store = pg.store.clone();
    let actor = a.clone();
    let original_request = request.clone();
    let original = tokio::spawn(async move { store.publish(&actor, &original_request).await });
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            let waiting: i64 = sqlx::query_scalar(
                "SELECT count(*) FROM pg_locks WHERE locktype='advisory' AND NOT granted AND pid IN (SELECT pid FROM pg_stat_activity WHERE usename=$1)",
            )
            .bind(&pg.role)
            .fetch_one(&pg.admin)
            .await
            .unwrap();
            if waiting >= 1 {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    let store = pg.store.clone();
    let actor = a.clone();
    let replay_request = request.clone();
    let replay = tokio::spawn(async move { store.publish(&actor, &replay_request).await });
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            let waiting: i64 = sqlx::query_scalar(
                "SELECT count(*) FROM pg_locks WHERE locktype='advisory' AND NOT granted AND pid IN (SELECT pid FROM pg_stat_activity WHERE usename=$1)",
            )
            .bind(&pg.role)
            .fetch_one(&pg.admin)
            .await
            .unwrap();
            if waiting >= 2 {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    assert!(!original.is_finished() && !replay.is_finished());
    assert!(
        matches!(
            pg.store.operation(&a, request.request_id).await,
            Err(crate::agent_definitions::Error::NotFound)
        ),
        "An in-flight receipt must not be fabricated"
    );
    sqlx::query("SELECT pg_advisory_unlock($1)")
        .bind(barrier)
        .execute(&mut *blocker)
        .await
        .unwrap();
    drop(blocker);
    let original = original.await.unwrap().unwrap();
    let replay = replay.await.unwrap().unwrap();
    assert!(!original.replayed);
    assert!(replay.replayed);
    assert_eq!(original.definition, replay.definition);
    assert_eq!(
        pg.store
            .operation(&a, request.request_id)
            .await
            .unwrap()
            .definition,
        original.definition
    );
    pg.close().await;
}

#[tokio::test]
async fn independent_sqlite_connections_share_an_atomic_request_claim() {
    let path = std::env::temp_dir().join(format!(
        "ohc-definition-concurrent-{}.sqlite",
        Uuid::new_v4()
    ));
    let url = format!("sqlite://{}?mode=rwc", path.display());
    let one = sqlx::SqlitePool::connect(&url).await.unwrap();
    sqlite_schema(&one, &url).await;
    let two = sqlx::SqlitePool::connect(&url).await.unwrap();
    let left = DefinitionStore::Sqlite(one.clone());
    let right = DefinitionStore::Sqlite(two.clone());
    let a = owner("disk-tenant", "disk-user");
    let request = typed_publish();
    let (a, b) = tokio::join!(left.publish(&a, &request), right.publish(&a, &request));
    let a = a.unwrap();
    let b = b.unwrap();
    assert_eq!(a.definition, b.definition);
    assert_ne!(a.replayed, b.replayed);
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_definition_operations")
            .fetch_one(&one)
            .await
            .unwrap(),
        1
    );
    drop(left);
    drop(right);
    one.close().await;
    two.close().await;
    drop(one);
    drop(two);
    std::fs::remove_file(path).unwrap();
}

#[path = "authority_test.rs"]
mod authority;

#[path = "lifecycle_test.rs"]
mod lifecycle;

#[path = "writer_test.rs"]
mod writers;
