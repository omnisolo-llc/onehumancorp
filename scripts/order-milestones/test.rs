use super::*;
use axum::{
    Router,
    body::{Body, to_bytes},
    extract::Extension,
    http::{Request as HttpRequest, StatusCode},
    routing::get,
};
use serde_json::{Value, json};
use std::sync::Arc;
use tower::ServiceExt;

struct Fixture {
    admin: sqlx::PgPool,
    scoped: sqlx::PgPool,
    schema: String,
    role: String,
    auth: Arc<server_auth::Store>,
}
fn user(tenant: &str) -> server_auth::User {
    server_auth::User {
        id: format!("member-{tenant}"),
        username: format!("member-{tenant}"),
        email: format!("{tenant}@example.test"),
        password_hash: String::new(),
        roles: vec!["OPERATOR".into()],
        active: true,
        organization_id: Some(tenant.into()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        oidc_subject: None,
    }
}
fn validate_database_url(raw: &str) -> Result<(), &'static str> {
    const ERROR: &str =
        "Use an explicit loopback PostgreSQL URL with an ohc_*_test database and no URL options";
    if raw.chars().any(char::is_control) {
        return Err(ERROR);
    }
    let parsed = url::Url::parse(raw).map_err(|_| ERROR)?;
    let loopback = match parsed.host() {
        Some(url::Host::Ipv4(host)) => host.is_loopback(),
        Some(url::Host::Ipv6(host)) => host.is_loopback(),
        // PostgreSQL is a non-special URL scheme; the URL parser keeps an IPv4 literal as an opaque host.
        Some(url::Host::Domain(host)) => host
            .parse::<std::net::IpAddr>()
            .is_ok_and(|ip| ip.is_loopback()),
        _ => false,
    };
    let database = parsed.path().strip_prefix('/').ok_or(ERROR)?;
    if !matches!(parsed.scheme(), "postgres" | "postgresql")
        || !loopback
        || parsed.query().is_some()
        || parsed.fragment().is_some()
        || !database.starts_with("ohc_")
        || !database.ends_with("_test")
        || database.len() <= "ohc__test".len()
        || database.len() > 63
        || !database
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_')
    {
        return Err(ERROR);
    }
    Ok(())
}
#[test]
fn disposable_database_guard_rejects_unsafe_targets_without_connecting() {
    for url in [
        "postgres://127.0.0.1/production",
        "postgres://db.example.test/ohc_milestone_test",
        "postgres://127.0.0.1/ohc__test",
        "postgres://127.0.0.1/ohc_milestone_test?host=db.example.test",
        "postgres://127.0.0.1/ohc_milestone_test#fragment",
        "postgres://127.0.0.1/ohc_milestone_test/extra",
        "postgres:///ohc_milestone_test",
        "postgres://127.0.0.1/ohc_%2F_test",
        "mysql://127.0.0.1/ohc_milestone_test",
    ] {
        assert!(
            validate_database_url(url).is_err(),
            "unsafe disposable database target accepted"
        );
    }
    for url in [
        "postgres://127.0.0.1:55439/ohc_milestone_test",
        "postgresql://[::1]:55439/ohc_milestone_test",
    ] {
        assert!(validate_database_url(url).is_ok());
    }
}
impl Fixture {
    async fn new() -> Self {
        let url =
            std::env::var("OHC_MILESTONE_TEST_DATABASE_URL").expect("isolated PostgreSQL required");
        validate_database_url(&url)
            .expect("unsafe disposable PostgreSQL target rejected before connection");
        let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
        let schema = format!("milestone_{}", uuid::Uuid::new_v4().simple());
        let role = format!("milestone_member_{}", uuid::Uuid::new_v4().simple());
        let password = uuid::Uuid::new_v4().simple().to_string();
        let admin = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect_with(options.clone().options([("search_path", schema.as_str())]))
            .await
            .unwrap();
        let encoding: String = sqlx::query_scalar("SHOW server_encoding")
            .fetch_one(&admin)
            .await
            .unwrap();
        assert_eq!(encoding, "UTF8");
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        sqlx::raw_sql(include_str!("schema.sql"))
            .execute(&admin)
            .await
            .unwrap();
        for tenant in ["tenant-a", "tenant-b", "tenant-empty", "raw-雪-tenant"] {
            sqlx::query("INSERT INTO tenants(id,name)VALUES($1,$1)")
                .bind(tenant)
                .execute(&admin)
                .await
                .unwrap();
            let u = user(tenant);
            sqlx::query("INSERT INTO users(id,username,email,roles,tenant_id)VALUES($1,$2,$3,ARRAY['OPERATOR'],$4)").bind(u.id).bind(u.username).bind(u.email).bind(tenant).execute(&admin).await.unwrap();
        }
        sqlx::query(&format!(
            "CREATE ROLE {role} LOGIN NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE PASSWORD '{password}'"
        ))
        .execute(&admin)
        .await
        .unwrap();
        sqlx::raw_sql(&format!("ALTER TABLE orders FORCE ROW LEVEL SECURITY; ALTER TABLE business_milestones FORCE ROW LEVEL SECURITY; GRANT USAGE ON SCHEMA {schema} TO {role}; GRANT SELECT ON orders,business_milestones TO {role};")).execute(&admin).await.unwrap();
        let scoped = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect_with(
                options
                    .clone()
                    .username(&role)
                    .password(&password)
                    .options([("search_path", schema.as_str())]),
            )
            .await
            .unwrap();
        let identity:(String,bool,bool)=sqlx::query_as("SELECT current_user::text,rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user").fetch_one(&scoped).await.unwrap();
        assert_eq!(identity, (role.clone(), false, false));
        let auth = Arc::new(server_auth::Store::with_repo(Arc::new(
            server_auth::postgres_store::PgUserRepository::new(admin.clone()),
        )));
        Self {
            admin,
            scoped,
            schema,
            role,
            auth,
        }
    }
    fn app(&self, restricted: bool, authenticated: bool) -> Router {
        let app = Router::new()
            .route("/milestone", get(handle_get_milestone))
            .layer(Extension(GrowthState {
                pool: if restricted {
                    self.scoped.clone()
                } else {
                    self.admin.clone()
                },
            }))
            .layer(axum::middleware::from_fn(growth_auth_fallback_middleware));
        if authenticated {
            app.layer(axum::middleware::from_fn_with_state(
                self.auth.clone(),
                server_auth::strict_bearer_auth_middleware,
            ))
        } else {
            app
        }
    }
    async fn request(
        &self,
        tenant: Option<&str>,
        method: &str,
        path: &str,
        body: Value,
        restricted: bool,
    ) -> (StatusCode, Value) {
        self.call(self.app(restricted, true), tenant, method, path, body)
            .await
    }
    async fn call(
        &self,
        app: Router,
        tenant: Option<&str>,
        method: &str,
        path: &str,
        body: Value,
    ) -> (StatusCode, Value) {
        let mut req = HttpRequest::builder()
            .method(method)
            .uri(path)
            .header("content-type", "application/json")
            .header("x-tenant-id", "tenant-b")
            .header("x-spiffe-id", "spiffe://forged/org/tenant-b");
        if let Some(t) = tenant {
            req = req.header(
                "authorization",
                format!("Bearer {}", self.auth.issue_token(&user(t)).unwrap()),
            );
        }
        let res = app
            .oneshot(req.body(Body::from(body.to_string())).unwrap())
            .await
            .unwrap();
        let status = res.status();
        let bytes = to_bytes(res.into_body(), 1_000_000).await.unwrap();
        (
            status,
            serde_json::from_slice(&bytes).unwrap_or(Value::Null),
        )
    }
    async fn seed(&self, tenant: &str, count: i64) {
        sqlx::query("INSERT INTO orders(id,tenant_id,status,total_amount) SELECT $1 || '-' || n::text,$1,'pending',0 FROM generate_series(1,$2) n")
            .bind(tenant).bind(count).execute(&self.admin).await.unwrap();
    }
    async fn read(&self, tenant: &str) -> (StatusCode, Value) {
        self.request(Some(tenant), "GET", "/milestone", Value::Null, true)
            .await
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

fn assert_recorded(body: &Value, tenant: &str, count: i64, threshold: Option<i64>) {
    assert_eq!(body["success"], true, "{body}");
    assert_eq!(body["metric"], "recorded_orders", "{body}");
    assert_eq!(body["tenant_id"], tenant, "{body}");
    assert_eq!(body["recorded_orders"], count, "{body}");
    assert_eq!(body["highest_threshold"], json!(threshold), "{body}");
    for unsupported in ["reward", "revenue", "delivered_orders", "paid_orders"] {
        assert!(
            body.get(unsupported).is_none(),
            "{unsupported} must not be invented: {body}"
        );
    }
}
#[tokio::test]
async fn a_database_owner_connection_still_respects_the_signed_tenant_predicate() {
    let f = Fixture::new().await;
    let bypasses: bool = sqlx::query_scalar(
        "SELECT rolsuper OR rolbypassrls FROM pg_roles WHERE rolname=current_user",
    )
    .fetch_one(&f.admin)
    .await
    .unwrap();
    assert!(
        bypasses,
        "This control must bypass RLS to test the explicit predicate independently"
    );
    f.seed("tenant-a", 3).await;
    f.seed("tenant-b", 100).await;
    let result = f
        .request(Some("tenant-a"), "GET", "/milestone", Value::Null, false)
        .await;
    f.finish().await;
    assert_eq!(result.0, StatusCode::OK);
    assert_recorded(&result.1, "tenant-a", 3, Some(1));
}
#[tokio::test]
async fn actual_mounted_read_rejects_anonymous_and_forged_header_authority() {
    let f = Fixture::new().await;
    let protected = f
        .request(
            None,
            "GET",
            "/milestone?tenant_id=tenant-a",
            Value::Null,
            true,
        )
        .await;
    let fallback = f
        .call(
            f.app(true, false),
            None,
            "GET",
            "/milestone?tenant_id=tenant-a",
            Value::Null,
        )
        .await;
    f.finish().await;
    assert_eq!(protected.0, StatusCode::UNAUTHORIZED);
    assert_eq!(fallback.0, StatusCode::UNAUTHORIZED);
}
#[tokio::test]
async fn restricted_signed_tenants_and_opaque_ids_read_only_their_actual_rows() {
    let f = Fixture::new().await;
    f.seed("tenant-a", 3).await;
    f.seed("tenant-b", 7).await;
    f.seed("raw-雪-tenant", 11).await;
    let a = f.read("tenant-a").await;
    let b = f.read("tenant-b").await;
    let raw = f.read("raw-雪-tenant").await;
    let context: Option<String> =
        sqlx::query_scalar("SELECT current_setting('app.current_tenant',true)")
            .fetch_one(&f.scoped)
            .await
            .unwrap();
    f.finish().await;
    assert_eq!(
        (a.0, b.0, raw.0),
        (StatusCode::OK, StatusCode::OK, StatusCode::OK)
    );
    assert_recorded(&a.1, "tenant-a", 3, Some(1));
    assert_recorded(&b.1, "tenant-b", 7, Some(1));
    assert_recorded(&raw.1, "raw-雪-tenant", 11, Some(10));
    assert!(
        context.as_deref().is_none_or(str::is_empty),
        "transaction context leaked: {context:?}"
    );
}
#[tokio::test]
async fn an_empty_business_has_a_real_zero_and_no_invented_century_milestone() {
    let f = Fixture::new().await;
    let result = f.read("tenant-empty").await;
    f.finish().await;
    assert_eq!(result.0, StatusCode::OK);
    assert_recorded(&result.1, "tenant-empty", 0, None);
    assert_eq!(result.1["reached_thresholds"], json!([]));
}
#[tokio::test]
async fn a_foreign_query_is_a_denied_precondition_not_tenant_authority() {
    let f = Fixture::new().await;
    f.seed("tenant-b", 100).await;
    let result = f
        .request(
            Some("tenant-a"),
            "GET",
            "/milestone?tenant_id=tenant-b",
            Value::Null,
            true,
        )
        .await;
    f.finish().await;
    assert_eq!(result.0, StatusCode::FORBIDDEN);
    assert!(result.1.get("recorded_orders").is_none());
}
#[tokio::test]
async fn every_existing_order_threshold_remains_reached_after_the_exact_transition() {
    let f = Fixture::new().await;
    let mut results = Vec::new();
    for count in [0, 1, 9, 10, 11, 49, 50, 51, 99, 100, 101, 999, 1000, 1001] {
        sqlx::query("DELETE FROM orders WHERE tenant_id='tenant-a'")
            .execute(&f.admin)
            .await
            .unwrap();
        f.seed("tenant-a", count).await;
        results.push((count, f.read("tenant-a").await));
    }
    f.finish().await;
    for (count, (status, body)) in results {
        let reached: Vec<i64> = [1, 10, 50, 100, 1000]
            .into_iter()
            .filter(|limit| count >= *limit)
            .collect();
        assert_eq!(status, StatusCode::OK);
        assert_recorded(&body, "tenant-a", count, reached.last().copied());
        assert_eq!(body["reached_thresholds"], json!(reached));
    }
}
#[tokio::test]
async fn all_recorded_statuses_count_without_summing_unrelated_currencies_or_claiming_sales() {
    let f = Fixture::new().await;
    for (id, status, amount, currency) in [
        ("pending", "pending", "900000.50", "USD"),
        ("canceled", "cancelled", "NaN", "NZD"),
        ("fulfilled", "fulfilled", "42", "JPY"),
    ] {
        sqlx::query("INSERT INTO orders(id,tenant_id,status,total_amount,base_currency,transaction_currency)VALUES($1,'tenant-a',$2,$3::numeric,$4,$4)")
            .bind(id).bind(status).bind(amount).bind(currency).execute(&f.admin).await.unwrap();
    }
    let result = f.read("tenant-a").await;
    f.finish().await;
    assert_eq!(result.0, StatusCode::OK);
    assert_recorded(&result.1, "tenant-a", 3, Some(1));
    assert_eq!(result.1["included_statuses"], "all_recorded_statuses");
}
#[tokio::test]
async fn stale_worker_markers_cannot_supply_an_unearned_order_count() {
    let f = Fixture::new().await;
    sqlx::query("INSERT INTO business_milestones(id,tenant_id,milestone_type)VALUES('stale-marker','tenant-a','1000_orders')").execute(&f.admin).await.unwrap();
    let result = f.read("tenant-a").await;
    let remaining: i64 = sqlx::query_scalar("SELECT count(*) FROM business_milestones")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    f.finish().await;
    assert_eq!(result.0, StatusCode::OK);
    assert_recorded(&result.1, "tenant-a", 0, None);
    assert_eq!(remaining, 1);
}
#[tokio::test]
async fn a_real_closed_read_pool_is_unavailable_not_a_zero_or_fake_milestone() {
    let f = Fixture::new().await;
    f.scoped.close().await;
    let result = f.read("tenant-a").await;
    f.finish().await;
    assert_eq!(result.0, StatusCode::SERVICE_UNAVAILABLE);
    assert!(result.1.get("recorded_orders").is_none());
}
#[tokio::test]
async fn denied_table_reads_are_distinct_from_an_empty_tenant() {
    let f = Fixture::new().await;
    sqlx::query(&format!("REVOKE SELECT ON orders FROM {}", f.role))
        .execute(&f.admin)
        .await
        .unwrap();
    let result = f.read("tenant-a").await;
    f.finish().await;
    assert_eq!(result.0, StatusCode::SERVICE_UNAVAILABLE);
    assert!(result.1.get("recorded_orders").is_none());
}
#[tokio::test]
async fn signed_reads_preserve_order_rows_and_do_not_write_worker_milestones() {
    let f = Fixture::new().await;
    f.seed("tenant-a", 11).await;
    let before: Value =
        sqlx::query_scalar("SELECT jsonb_agg(to_jsonb(o) ORDER BY id) FROM orders o")
            .fetch_one(&f.admin)
            .await
            .unwrap();
    let _ = f.read("tenant-a").await;
    let after: Value =
        sqlx::query_scalar("SELECT jsonb_agg(to_jsonb(o) ORDER BY id) FROM orders o")
            .fetch_one(&f.admin)
            .await
            .unwrap();
    let milestones: i64 = sqlx::query_scalar("SELECT count(*) FROM business_milestones")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    f.finish().await;
    assert_eq!(before, after);
    assert_eq!(milestones, 0);
}
#[tokio::test]
async fn a_matching_opaque_query_remains_a_precondition_on_the_signed_tenant() {
    let f = Fixture::new().await;
    f.seed("raw-雪-tenant", 10).await;
    let result = f
        .request(
            Some("raw-雪-tenant"),
            "GET",
            &format!(
                "/milestone?tenant_id={}",
                urlencoding::encode("raw-雪-tenant")
            ),
            Value::Null,
            true,
        )
        .await;
    f.finish().await;
    assert_eq!(result.0, StatusCode::OK);
    assert_recorded(&result.1, "raw-雪-tenant", 10, Some(10));
}
