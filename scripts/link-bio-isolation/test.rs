use super::*;
use axum::{
    Router,
    body::{Body, to_bytes},
    extract::Extension,
    http::{Request as HttpRequest, StatusCode},
    routing::{get, post},
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
fn config(title: &str) -> Value {
    json!({"store_name":title,"bio":"Owner-entered description","theme":"dark","links":[{"id":"1","title":"Owner link","url":"https://example.test/business"}],"remove_branding":false})
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
        "postgres://db.example.test/ohc_bio_test",
        "postgres://127.0.0.1/ohc__test",
        "postgres://127.0.0.1/ohc_bio_test?host=db.example.test",
        "postgres://127.0.0.1/ohc_bio_test#fragment",
        "postgres://127.0.0.1/ohc_bio_test/extra",
        "postgres:///ohc_bio_test",
        "postgres://127.0.0.1/ohc_%2F_test",
        "mysql://127.0.0.1/ohc_bio_test",
    ] {
        assert!(
            validate_database_url(url).is_err(),
            "unsafe disposable database target accepted"
        );
    }
    for url in [
        "postgres://127.0.0.1:55439/ohc_bio_test",
        "postgresql://[::1]:55439/ohc_bio_test",
    ] {
        assert!(validate_database_url(url).is_ok());
    }
}
impl Fixture {
    async fn new() -> Self {
        let url = std::env::var("OHC_BIO_TEST_DATABASE_URL").expect("isolated PostgreSQL required");
        validate_database_url(&url)
            .expect("unsafe disposable PostgreSQL target rejected before connection");
        let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
        let schema = format!("bio_{}", uuid::Uuid::new_v4().simple());
        let role = format!("bio_member_{}", uuid::Uuid::new_v4().simple());
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
        for tenant in ["tenant-a", "tenant-b", "tenant-empty", "my-store"] {
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
        sqlx::raw_sql(&format!("ALTER TABLE agent_kv_store FORCE ROW LEVEL SECURITY; GRANT USAGE ON SCHEMA {schema} TO {role}; GRANT SELECT,INSERT,UPDATE ON agent_kv_store TO {role};")).execute(&admin).await.unwrap();
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
            .route("/link-in-bio", post(handle_post_link_in_bio))
            .route("/link-in-bio/{tenant}", get(handle_get_link_in_bio))
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
    async fn seed(&self, tenant: &str, value: &str) {
        sqlx::query("INSERT INTO agent_kv_store(tenant_id,kv_key,kv_value)VALUES($1,'link_in_bio_config',$2)").bind(tenant).bind(value).execute(&self.admin).await.unwrap();
    }
    async fn stored(&self, tenant: &str) -> Option<String> {
        sqlx::query_scalar("SELECT kv_value FROM agent_kv_store WHERE tenant_id=$1 AND kv_key='link_in_bio_config'").bind(tenant).fetch_optional(&self.admin).await.unwrap()
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
async fn signed_member_save_uses_claims_instead_of_forged_header_and_never_mirrors() {
    let f = Fixture::new().await;
    let payload = config("Actual member business");
    let saved = f
        .request(
            Some("tenant-a"),
            "POST",
            "/link-in-bio",
            payload.clone(),
            true,
        )
        .await;
    let a = f.stored("tenant-a").await;
    let b = f.stored("tenant-b").await;
    let alias = f.stored("my-store").await;
    let read = f
        .request(
            Some("tenant-a"),
            "GET",
            "/link-in-bio/tenant-a",
            json!({}),
            true,
        )
        .await;
    f.finish().await;
    assert_eq!(saved.0, StatusCode::OK);
    assert_eq!(serde_json::from_str::<Value>(&a.unwrap()).unwrap(), payload);
    assert!(b.is_none());
    assert!(alias.is_none());
    assert_eq!(read, (StatusCode::OK, payload));
}
#[tokio::test]
async fn body_tenant_cannot_overwrite_another_signed_tenants_profile() {
    let f = Fixture::new().await;
    let original = config("Private business B").to_string();
    f.seed("tenant-b", &original).await;
    let mut payload = config("Attacker replacement");
    payload["tenant_id"] = json!("tenant-b");
    let result = f
        .request(Some("tenant-a"), "POST", "/link-in-bio", payload, true)
        .await;
    let b = f.stored("tenant-b").await;
    let a = f.stored("tenant-a").await;
    f.finish().await;
    assert_eq!(result.0, StatusCode::FORBIDDEN);
    assert_eq!(b.as_deref(), Some(original.as_str()));
    assert!(a.is_none());
}
#[tokio::test]
async fn signed_tenant_cannot_read_another_private_profile() {
    let f = Fixture::new().await;
    f.seed("tenant-b", &config("Private B").to_string()).await;
    let result = f
        .request(
            Some("tenant-a"),
            "GET",
            "/link-in-bio/tenant-b",
            json!({}),
            true,
        )
        .await;
    f.finish().await;
    assert_eq!(result.0, StatusCode::FORBIDDEN);
    assert!(!result.1.to_string().contains("Private B"));
}
#[tokio::test]
async fn missing_own_profile_is_not_the_latest_other_tenant_profile() {
    let f = Fixture::new().await;
    f.seed("tenant-b", &config("Latest foreign profile").to_string())
        .await;
    let result = f
        .request(
            Some("tenant-empty"),
            "GET",
            "/link-in-bio/tenant-empty",
            json!({}),
            false,
        )
        .await;
    f.finish().await;
    assert_eq!(result.0, StatusCode::NOT_FOUND);
    assert!(!result.1.to_string().contains("Latest foreign"));
}
#[tokio::test]
async fn my_store_is_an_owned_identifier_not_a_global_write_alias() {
    let f = Fixture::new().await;
    let original = config("Actual alias owner").to_string();
    f.seed("my-store", &original).await;
    let mut payload = config("Tenant A");
    payload["tenant_id"] = json!("tenant-a");
    let saved = f
        .request(Some("tenant-a"), "POST", "/link-in-bio", payload, true)
        .await;
    let alias = f.stored("my-store").await;
    let foreign_read = f
        .request(
            Some("tenant-a"),
            "GET",
            "/link-in-bio/my-store",
            json!({}),
            true,
        )
        .await;
    let own_read = f
        .request(
            Some("my-store"),
            "GET",
            "/link-in-bio/my-store",
            json!({}),
            true,
        )
        .await;
    f.finish().await;
    assert_eq!(saved.0, StatusCode::OK);
    assert_eq!(alias.as_deref(), Some(original.as_str()));
    assert_eq!(foreign_read.0, StatusCode::FORBIDDEN);
    assert_eq!(
        own_read.1,
        serde_json::from_str::<Value>(&original).unwrap()
    );
}
#[tokio::test]
async fn malformed_stored_profile_is_unavailable_and_bytes_are_preserved() {
    let f = Fixture::new().await;
    f.seed("tenant-a", "not valid stored JSON").await;
    let result = f
        .request(
            Some("tenant-a"),
            "GET",
            "/link-in-bio/tenant-a",
            json!({}),
            true,
        )
        .await;
    let bytes = f.stored("tenant-a").await;
    f.finish().await;
    assert_eq!(result.0, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(bytes.as_deref(), Some("not valid stored JSON"));
}
#[tokio::test]
async fn anonymous_requests_cannot_read_or_write_private_profiles() {
    let f = Fixture::new().await;
    f.seed("tenant-a", &config("Private A").to_string()).await;
    let get = f
        .request(None, "GET", "/link-in-bio/tenant-a", json!({}), true)
        .await;
    let post = f
        .request(None, "POST", "/link-in-bio", config("No authority"), true)
        .await;
    f.finish().await;
    assert_eq!(get.0, StatusCode::UNAUTHORIZED);
    assert_eq!(post.0, StatusCode::UNAUTHORIZED);
}
#[tokio::test]
async fn fallback_authinfo_alone_cannot_authorize_a_handler_write() {
    let f = Fixture::new().await;
    let result = f
        .call(
            f.app(true, false),
            None,
            "POST",
            "/link-in-bio",
            config("No signed identity"),
        )
        .await;
    let b = f.stored("tenant-b").await;
    f.finish().await;
    assert_eq!(result.0, StatusCode::UNAUTHORIZED);
    assert!(b.is_none());
}
#[tokio::test]
async fn one_real_connection_switches_signed_tenants_without_residual_context() {
    let f = Fixture::new().await;
    let pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&f.scoped)
        .await
        .unwrap();
    let mut results = Vec::new();
    for t in ["tenant-a", "tenant-b"] {
        let mut p = config(t);
        p["tenant_id"] = json!(t);
        results.push(f.request(Some(t), "POST", "/link-in-bio", p, true).await.0);
    }
    let a = f
        .request(
            Some("tenant-a"),
            "GET",
            "/link-in-bio/tenant-a",
            json!({}),
            true,
        )
        .await;
    let b = f
        .request(
            Some("tenant-b"),
            "GET",
            "/link-in-bio/tenant-b",
            json!({}),
            true,
        )
        .await;
    let final_context: (i32, Option<String>) =
        sqlx::query_as("SELECT pg_backend_pid(),current_setting('app.current_tenant',true)")
            .fetch_one(&f.scoped)
            .await
            .unwrap();
    f.finish().await;
    assert_eq!(results, vec![StatusCode::OK; 2]);
    assert_eq!(a.1["store_name"], "tenant-a");
    assert_eq!(b.1["store_name"], "tenant-b");
    assert_eq!(final_context.0, pid);
    assert!(final_context.1.as_deref().unwrap_or("").is_empty());
}

#[tokio::test]
async fn successful_private_profile_response_disables_shared_caching() {
    let f = Fixture::new().await;
    f.seed("tenant-a", &config("Private profile").to_string())
        .await;
    let response = f
        .app(true, true)
        .oneshot(
            HttpRequest::builder()
                .uri("/link-in-bio/tenant-a")
                .header(
                    "authorization",
                    format!("Bearer {}", f.auth.issue_token(&user("tenant-a")).unwrap()),
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let cache = response
        .headers()
        .get("cache-control")
        .and_then(|h| h.to_str().ok())
        .map(str::to_string);
    f.finish().await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(cache.as_deref(), Some("private, no-store"));
}

fn unsafe_urls() -> Vec<&'static str> {
    vec![
        "javascript:alert(1)",
        "JaVaScRiPt:alert(1)",
        "data:text/html,<script>alert(1)</script>",
        "blob:https://example.test/id",
        "mailto:a@example.test",
        "tel:+12025550123",
        "/relative",
        "//example.test",
        "https:example.test",
        "https://",
        " https://example.test",
        "https://exa\nmple.test",
        "https://example.test/white space",
        "https://example.test/\0x",
        "https:\\example.test",
    ]
}
#[tokio::test]
async fn unsafe_url_writes_never_replace_the_stored_private_profile() {
    let f = Fixture::new().await;
    let original = config("Original valid profile").to_string();
    f.seed("tenant-a", &original).await;
    let mut observed = Vec::new();
    for url in unsafe_urls() {
        let mut payload = config("Unaccepted change");
        payload["links"][0]["url"] = json!(url);
        let result = f
            .request(Some("tenant-a"), "POST", "/link-in-bio", payload, true)
            .await;
        observed.push((url, result.0, f.stored("tenant-a").await));
    }
    f.finish().await;
    for (url, status, stored) in observed {
        assert_eq!(
            status,
            StatusCode::BAD_REQUEST,
            "unsafe URL accepted: {url:?}"
        );
        assert_eq!(stored.as_deref(), Some(original.as_str()));
    }
}
#[tokio::test]
async fn unsafe_historical_urls_are_unavailable_on_read_without_modifying_bytes() {
    let f = Fixture::new().await;
    let mut observed = Vec::new();
    for url in unsafe_urls() {
        let mut payload = config("Historical private profile");
        payload["links"][0]["url"] = json!(url);
        let bytes = payload.to_string();
        sqlx::query("INSERT INTO agent_kv_store(tenant_id,kv_key,kv_value)VALUES('tenant-a','link_in_bio_config',$1) ON CONFLICT(tenant_id,kv_key) DO UPDATE SET kv_value=$1").bind(&bytes).execute(&f.admin).await.unwrap();
        let result = f
            .request(
                Some("tenant-a"),
                "GET",
                "/link-in-bio/tenant-a",
                json!({}),
                true,
            )
            .await;
        observed.push((url, bytes, result, f.stored("tenant-a").await));
    }
    f.finish().await;
    for (url, bytes, result, stored) in observed {
        assert_eq!(
            result.0,
            StatusCode::INTERNAL_SERVER_ERROR,
            "unsafe stored URL returned: {url:?}"
        );
        assert_eq!(stored.as_deref(), Some(bytes.as_str()));
        assert_eq!(result.1, Value::Null);
    }
}
#[tokio::test]
async fn valid_web_destinations_round_trip_exactly_without_sanitizing_titles_or_urls() {
    let f = Fixture::new().await;
    let mut observed = Vec::new();
    for url in [
        "http://example.test/",
        "https://example.test/路径?q='\"&x=%26#✓",
        "HTTPS://example.test/Case",
    ] {
        let mut payload = config("Verified web profile");
        payload["links"][0]["url"] = json!(url);
        payload["links"][0]["title"] = json!("<img src=x onerror=throw(1)>");
        let saved = f
            .request(
                Some("tenant-a"),
                "POST",
                "/link-in-bio",
                payload.clone(),
                true,
            )
            .await;
        let read = f
            .request(
                Some("tenant-a"),
                "GET",
                "/link-in-bio/tenant-a",
                json!({}),
                true,
            )
            .await;
        observed.push((payload, saved, read));
    }
    f.finish().await;
    for (payload, saved, read) in observed {
        assert_eq!(saved.0, StatusCode::OK);
        assert_eq!(read, (StatusCode::OK, payload));
    }
}
