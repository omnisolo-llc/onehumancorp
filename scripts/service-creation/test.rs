use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use serde_json::{Value, json};
use sqlx::Row;
use std::sync::Arc;
use tower::ServiceExt;

struct Fixture {
    admin: sqlx::PgPool,
    scoped: sqlx::PgPool,
    schema: String,
    role: String,
    app: Router,
    a: String,
    b: String,
}
impl Fixture {
    async fn new() -> Self {
        let url =
            std::env::var("OHC_SERVICE_TEST_DATABASE_URL").expect("owned PostgreSQL required");
        let schema = format!("service_test_{}", uuid::Uuid::new_v4().simple());
        let role = format!("service_login_{}", uuid::Uuid::new_v4().simple());
        let password = uuid::Uuid::new_v4().simple().to_string();
        let admin = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect_with(
                url.parse::<sqlx::postgres::PgConnectOptions>()
                    .unwrap()
                    .options([("search_path", schema.as_str())]),
            )
            .await
            .unwrap();
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        // Only the canonical tenant/service table statements are prerequisites.
        // Rewriting the service columns here would conceal the production bug.
        let initial = include_str!("../../src/server/migrations/001_initial.sql");
        let start = initial
            .find("CREATE TABLE IF NOT EXISTS tenants (")
            .or_else(|| initial.find("CREATE TABLE tenants ("))
            .unwrap();
        let end = initial[start..].find(");").unwrap() + start + 2;
        sqlx::raw_sql(&initial[start..end])
            .execute(&admin)
            .await
            .unwrap();
        let migration = include_str!("../../src/server/migrations/008_data_model_architecture.sql");
        let start = migration
            .find("CREATE TABLE IF NOT EXISTS services (")
            .unwrap();
        let end = migration[start..].find(");").unwrap() + start + 2;
        sqlx::raw_sql(&migration[start..end])
            .execute(&admin)
            .await
            .unwrap();
        sqlx::query("INSERT INTO tenants(id,name) VALUES('tenant-a','A'),('tenant-b','B')")
            .execute(&admin)
            .await
            .unwrap();
        sqlx::raw_sql("ALTER TABLE services ENABLE ROW LEVEL SECURITY; ALTER TABLE services FORCE ROW LEVEL SECURITY; CREATE POLICY service_tenant ON services USING (tenant_id=current_setting('app.current_tenant',true)) WITH CHECK (tenant_id=current_setting('app.current_tenant',true));").execute(&admin).await.unwrap();
        sqlx::query(&format!("CREATE ROLE {role} LOGIN NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE PASSWORD '{password}'")).execute(&admin).await.unwrap();
        sqlx::raw_sql(&format!(
            "GRANT USAGE ON SCHEMA {schema} TO {role}; GRANT SELECT,INSERT ON services TO {role};"
        ))
        .execute(&admin)
        .await
        .unwrap();
        let scoped = crate::db::secure_pg_pool_options()
            .max_connections(1)
            .connect_with(
                url.parse::<sqlx::postgres::PgConnectOptions>()
                    .unwrap()
                    .username(&role)
                    .password(&password)
                    .options([("search_path", schema.as_str())]),
            )
            .await
            .unwrap();
        let identity:(String,String,bool,bool)=sqlx::query_as("SELECT session_user::text,current_user::text,rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user").fetch_one(&scoped).await.unwrap();
        assert_eq!(identity, (role.clone(), role.clone(), false, false));
        let store = Arc::new(server_auth::Store::new());
        let mut tokens = Vec::new();
        for tenant in ["tenant-a", "tenant-b"] {
            let user = store
                .create_user(
                    format!("owner-{tenant}"),
                    format!("{tenant}@example.test"),
                    "public-test-password".into(),
                    vec!["ADMIN".into()],
                    tenant.into(),
                )
                .await
                .unwrap();
            tokens.push(store.issue_token(&user).unwrap());
        }
        let db = Arc::new(crate::db::DB {
            pool: scoped.clone(),
            store: crate::db::DbStore::Postgres,
        });
        let app = Router::new()
            .nest(
                "/api/v1/booking/services",
                crate::create_service::router(db),
            )
            .route_layer(axum::middleware::from_fn_with_state(
                store,
                server_auth::strict_bearer_auth_middleware,
            ));
        Self {
            admin,
            scoped,
            schema,
            role,
            app,
            a: tokens.remove(0),
            b: tokens.remove(0),
        }
    }
    async fn create(&self, token: Option<&str>, value: Value) -> (StatusCode, Value) {
        let mut request = Request::builder()
            .method("POST")
            .uri("/api/v1/booking/services")
            .header("content-type", "application/json")
            .header("x-tenant-id", "tenant-b");
        if let Some(token) = token {
            request = request.header("authorization", format!("Bearer {token}"));
        }
        let response = self
            .app
            .clone()
            .oneshot(request.body(Body::from(value.to_string())).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        (status, serde_json::from_slice(&body).unwrap_or(Value::Null))
    }
    async fn finish(self) {
        self.scoped.close().await;
        sqlx::query(&format!("DROP SCHEMA {} CASCADE", self.schema))
            .execute(&self.admin)
            .await
            .unwrap();
        sqlx::query(&format!("DROP ROLE {}", self.role))
            .execute(&self.admin)
            .await
            .unwrap();
        self.admin.close().await;
    }
}

#[tokio::test]
async fn postgres_creation_uses_canonical_schema_exact_cents_and_signed_tenant() {
    let f = Fixture::new().await;
    let mut results = Vec::new();
    for (index, cents) in [0_i64, 1, 5001, 1_000_000_000].into_iter().enumerate() {
        let title = format!("Exact service {index}");
        let (status, body) = f
            .create(
                Some(&f.a),
                json!({"title":title,"description":"Reviewed description","price_cents":cents}),
            )
            .await;
        let row=sqlx::query("SELECT tenant_id,name,description,(price*100)::BIGINT AS cents FROM services WHERE id=$1").bind(body["service_id"].as_str().unwrap_or("")).fetch_optional(&f.admin).await.unwrap();
        results.push((status, body, row, cents, title));
    }
    let (other_status, other) = f
        .create(Some(&f.b), json!({"title":"Other tenant","price_cents":42}))
        .await;
    let other_tenant: Option<String> =
        sqlx::query_scalar("SELECT tenant_id FROM services WHERE id=$1")
            .bind(other["service_id"].as_str().unwrap_or(""))
            .fetch_optional(&f.admin)
            .await
            .unwrap();
    let identity: (String, String) = sqlx::query_as("SELECT current_user::text,session_user::text")
        .fetch_one(&f.scoped)
        .await
        .unwrap();
    let unscoped: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM services")
        .fetch_one(&f.scoped)
        .await
        .unwrap();
    let expected_role = f.role.clone();
    f.finish().await;
    for (status, body, row, cents, title) in results {
        assert_eq!(status, StatusCode::OK, "{body}");
        assert_eq!(body["success"], true);
        uuid::Uuid::parse_str(body["service_id"].as_str().unwrap()).unwrap();
        let row = row.expect("acknowledged service must be committed");
        assert_eq!(row.get::<String, _>("tenant_id"), "tenant-a");
        assert_eq!(row.get::<String, _>("name"), title);
        assert_eq!(row.get::<String, _>("description"), "Reviewed description");
        assert_eq!(row.get::<i64, _>("cents"), cents);
    }
    assert_eq!(other_status, StatusCode::OK);
    assert_eq!(other_tenant.as_deref(), Some("tenant-b"));
    assert_eq!(identity, (expected_role.clone(), expected_role));
    assert_eq!(unscoped, 0);
}

#[tokio::test]
async fn invalid_or_unsigned_service_requests_leave_no_rows() {
    let f = Fixture::new().await;
    let mut statuses = Vec::new();
    for value in [
        json!({"title":"","price_cents":1}),
        json!({"title":"Invalid","price_cents":-1}),
        json!({"title":"Invalid","price_cents":1_000_000_001}),
        json!({"title":"Valid","description":"x".repeat(10_001)}),
    ] {
        statuses.push(f.create(Some(&f.a), value).await.0);
    }
    let unsigned = f
        .create(
            None,
            json!({"title":"Forged tenant header","price_cents":42}),
        )
        .await
        .0;
    let unknown = f
        .create(
            Some(&f.a),
            json!({"title":"Unknown","tenant_id":"tenant-b"}),
        )
        .await
        .0;
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM services")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    f.finish().await;
    assert!(
        statuses.iter().all(|s| *s == StatusCode::BAD_REQUEST),
        "{statuses:?}"
    );
    assert_eq!(unsigned, StatusCode::UNAUTHORIZED);
    assert_eq!(unknown, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(count, 0);
}

#[tokio::test]
async fn deferred_commit_failure_cannot_acknowledge_a_service() {
    let f = Fixture::new().await;
    sqlx::raw_sql("CREATE SEQUENCE commit_attempt; CREATE FUNCTION reject_service_commit() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN PERFORM nextval('commit_attempt'); RAISE EXCEPTION 'owned deferred commit rejection'; END; $$; CREATE CONSTRAINT TRIGGER reject_service_commit AFTER INSERT ON services DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION reject_service_commit();").execute(&f.admin).await.unwrap();
    sqlx::query(&format!(
        "GRANT USAGE ON SEQUENCE commit_attempt TO {}",
        f.role
    ))
    .execute(&f.admin)
    .await
    .unwrap();
    let (status, body) = f
        .create(
            Some(&f.a),
            json!({"title":"Must roll back","price_cents":123}),
        )
        .await;
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM services")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    let attempted: bool = sqlx::query_scalar("SELECT is_called FROM commit_attempt")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    f.finish().await;
    assert!(
        attempted,
        "the actual deferred commit constraint must execute"
    );
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body["success"], false);
    assert!(body["service_id"].is_null());
    assert_eq!(count, 0);
}
