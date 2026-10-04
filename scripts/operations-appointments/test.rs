use super::*;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use std::sync::Arc;
use tower::ServiceExt;

fn user(tenant: &str) -> server_auth::User {
    server_auth::User {
        id: format!("owner-{tenant}"),
        username: format!("owner-{tenant}"),
        email: format!("{tenant}@example.test"),
        password_hash: String::new(),
        roles: vec![server_auth::ROLE_ADMIN.into()],
        active: true,
        organization_id: Some(tenant.into()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        oidc_subject: None,
    }
}
fn guarded_url(raw: &str) -> bool {
    let Ok(url) = url::Url::parse(raw) else {
        return false;
    };
    let name = url.path().trim_start_matches('/');
    matches!(url.scheme(), "postgres" | "postgresql")
        && url
            .host_str()
            .and_then(|h| h.trim_matches(['[', ']']).parse::<std::net::IpAddr>().ok())
            .is_some_and(|h| h.is_loopback())
        && url.query().is_none()
        && url.fragment().is_none()
        && !raw.chars().any(char::is_control)
        && name.starts_with("ohc_")
        && name.ends_with("_test")
        && name.len() > 10
        && name.len() <= 63
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}
struct Fixture {
    admin: sqlx::PgPool,
    scoped: sqlx::PgPool,
    store: Arc<server_auth::Store>,
    schema: String,
    role: String,
}
impl Fixture {
    async fn new() -> Self {
        let url = std::env::var("OHC_APPOINTMENTS_TEST_DATABASE_URL")
            .expect("Explicit owned PostgreSQL prerequisite required");
        assert!(
            guarded_url(&url),
            "Use an owned loopback ohc_*_test database before creating isolated resources"
        );
        let schema = format!("appointments_{}", uuid::Uuid::new_v4().simple());
        let role = format!("appointments_reader_{}", uuid::Uuid::new_v4().simple());
        let password = uuid::Uuid::new_v4().simple().to_string();
        let options = url
            .parse::<sqlx::postgres::PgConnectOptions>()
            .unwrap()
            .options([("search_path", schema.as_str())]);
        let admin = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect_with(options.clone())
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
        for tenant in ["tenant-a", "tenant-b", "tenant-empty", "system"] {
            sqlx::query("INSERT INTO tenants(id,name)VALUES($1,$1)")
                .bind(tenant)
                .execute(&admin)
                .await
                .unwrap();
            let u = user(tenant);
            sqlx::query("INSERT INTO users(id,username,email,roles,tenant_id)VALUES($1,$2,$3,ARRAY['ADMIN'],$4)").bind(u.id).bind(u.username).bind(u.email).bind(tenant).execute(&admin).await.unwrap();
        }
        for (tenant, suffix) in [("tenant-a", "a"), ("tenant-b", "b")] {
            sqlx::query("INSERT INTO customers(id,tenant_id,name)VALUES($1,$2,$3)")
                .bind(format!("customer-{suffix}"))
                .bind(tenant)
                .bind(format!("Recorded customer {suffix}"))
                .execute(&admin)
                .await
                .unwrap();
            sqlx::query("INSERT INTO job_templates(id,tenant_id,name)VALUES($1,$2,$3)")
                .bind(format!("template-{suffix}"))
                .bind(tenant)
                .bind(format!("Recorded service {suffix}"))
                .execute(&admin)
                .await
                .unwrap();
            sqlx::query("INSERT INTO appointments(id,tenant_id,customer_id,job_template_id,status,scheduled_start_time,scheduled_end_time,location_address,location_lat,location_lng,notes)VALUES($1,$2,$3,$4,'Confirmed','2026-10-02T11:00:00Z','2026-10-02T12:00:00Z',$5,1.5,2.5,$6)").bind(format!("appointment-{suffix}")).bind(tenant).bind(format!("customer-{suffix}")).bind(format!("template-{suffix}")).bind(format!("Private address {suffix}")).bind(format!("Owner note {suffix}")).execute(&admin).await.unwrap();
            sqlx::query(
                "INSERT INTO service_routes(id,tenant_id,route_date)VALUES($1,$2,'2026-10-02')",
            )
            .bind(format!("route-{suffix}"))
            .bind(tenant)
            .execute(&admin)
            .await
            .unwrap();
        }
        sqlx::query(&format!("CREATE ROLE {role} LOGIN NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE PASSWORD '{password}'")).execute(&admin).await.unwrap();
        for table in [
            "customers",
            "job_templates",
            "appointments",
            "job_locations",
            "service_routes",
        ] {
            sqlx::query(&format!("ALTER TABLE {table} FORCE ROW LEVEL SECURITY"))
                .execute(&admin)
                .await
                .unwrap();
        }
        sqlx::raw_sql(&format!("GRANT USAGE ON SCHEMA {schema} TO {role}; GRANT SELECT ON customers,job_templates,appointments,job_locations,service_routes TO {role}")).execute(&admin).await.unwrap();
        let scoped = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect_with(options.username(&role).password(&password))
            .await
            .unwrap();
        let identity:(String,String,bool,bool)=sqlx::query_as("SELECT session_user::text,current_user::text,rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user").fetch_one(&scoped).await.unwrap();
        assert_eq!(identity, (role.clone(), role.clone(), false, false));
        let store = Arc::new(server_auth::Store::with_repo(Arc::new(
            server_auth::postgres_store::PgUserRepository::new(admin.clone()),
        )));
        Self {
            admin,
            scoped,
            store,
            schema,
            role,
        }
    }
    async fn read(
        &self,
        tenant: Option<&str>,
        query: &str,
        owner_pool: bool,
    ) -> (StatusCode, serde_json::Value, String) {
        let pool = if owner_pool {
            self.admin.clone()
        } else {
            self.scoped.clone()
        };
        let mut request = Request::builder()
            .uri(format!("/appointments{query}"))
            .header("x-tenant-id", "tenant-b")
            .header("x-spiffe-id", "spiffe://ohc/org/tenant-b/agent/forged");
        if let Some(tenant) = tenant {
            request = request.header(
                "authorization",
                format!("Bearer {}", self.store.issue_token(&user(tenant)).unwrap()),
            );
        }
        let response = app(pool, self.store.clone())
            .oneshot(request.body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let cache = response
            .headers()
            .get("cache-control")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_owned();
        let bytes = to_bytes(response.into_body(), 1_000_000).await.unwrap();
        let body = serde_json::from_slice(&bytes).unwrap_or_else(|_| {
            serde_json::Value::String(String::from_utf8_lossy(&bytes).into_owned())
        });
        (status, body, cache)
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
async fn actual_signed_owner_reads_real_fields_under_restricted_rls() {
    let f = Fixture::new().await;
    let r = f.read(Some("tenant-a"), "?tenant_id=tenant-a", false).await;
    f.finish().await;
    assert_eq!(r.0, StatusCode::OK);
    assert_eq!(r.2, "private, no-store");
    let rows = r.1["appointments"].as_array().unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["id"], "appointment-a");
    assert_eq!(rows[0]["customer_name"], "Recorded customer a");
    assert_eq!(rows[0]["job_name"], "Recorded service a");
    assert_eq!(rows[0]["status"], "Confirmed");
    assert_eq!(rows[0]["notes"], "Owner note a");
    assert_eq!(rows[0]["location_lat"], 1.5);
}
#[tokio::test]
async fn signed_identity_is_sufficient_without_a_caller_tenant() {
    let f = Fixture::new().await;
    let r = f.read(Some("tenant-a"), "", false).await;
    f.finish().await;
    assert_eq!(r.0, StatusCode::OK);
    assert_eq!(r.1["appointments"][0]["id"], "appointment-a");
}
#[tokio::test]
async fn forged_query_cannot_select_foreign_records_even_with_a_bypass_pool() {
    let f = Fixture::new().await;
    let r = f.read(Some("tenant-a"), "?tenant_id=tenant-b", true).await;
    f.finish().await;
    assert_eq!(r.0, StatusCode::FORBIDDEN);
    assert!(!r.1.to_string().contains("Recorded customer b"));
}
#[tokio::test]
async fn anonymous_and_system_identity_are_denied() {
    let f = Fixture::new().await;
    let a = f.read(None, "?tenant_id=tenant-a", true).await;
    let b = f.read(Some("system"), "?tenant_id=tenant-a", true).await;
    f.finish().await;
    assert_eq!(a.0, StatusCode::UNAUTHORIZED);
    assert_eq!(b.0, StatusCode::UNAUTHORIZED);
}
#[tokio::test]
async fn one_connection_alternates_tenants_and_leaves_no_session_context() {
    let f = Fixture::new().await;
    let pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&f.scoped)
        .await
        .unwrap();
    let a = f.read(Some("tenant-a"), "?tenant_id=tenant-a", false).await;
    let b = f.read(Some("tenant-b"), "?tenant_id=tenant-b", false).await;
    let empty = f
        .read(Some("tenant-empty"), "?tenant_id=tenant-empty", false)
        .await;
    let again = f.read(Some("tenant-a"), "?tenant_id=tenant-a", false).await;
    let context: (i32, Option<String>) =
        sqlx::query_as("SELECT pg_backend_pid(),current_setting('app.current_tenant',true)")
            .fetch_one(&f.scoped)
            .await
            .unwrap();
    f.finish().await;
    assert_eq!(context.0, pid);
    assert!(context.1.as_deref().unwrap_or("").is_empty());
    assert_eq!(a, again);
    assert_eq!(b.1["appointments"][0]["id"], "appointment-b");
    assert_eq!(empty.0, StatusCode::OK);
    assert_eq!(empty.1, serde_json::json!({"appointments":[]}));
}
#[tokio::test]
async fn mobile_projection_omits_private_details_without_losing_real_identity() {
    let f = Fixture::new().await;
    let r = f
        .read(
            Some("tenant-a"),
            "?tenant_id=tenant-a&mobile_optimized=true",
            false,
        )
        .await;
    f.finish().await;
    assert_eq!(r.0, StatusCode::OK);
    let row = &r.1["appointments"][0];
    assert_eq!(row["id"], "appointment-a");
    for key in ["notes", "location_address", "location_lat", "location_lng"] {
        assert!(row.get(key).is_none(), "{key}");
    }
}
#[tokio::test]
async fn foreign_linked_names_are_not_exposed_by_legacy_cross_tenant_rows() {
    let f = Fixture::new().await;
    sqlx::query("UPDATE appointments SET customer_id='customer-b',job_template_id='template-b' WHERE id='appointment-a'").execute(&f.admin).await.unwrap();
    let r = f.read(Some("tenant-a"), "?tenant_id=tenant-a", true).await;
    f.finish().await;
    assert_eq!(r.0, StatusCode::OK);
    let row = &r.1["appointments"][0];
    assert_eq!(row["customer_name"], "");
    assert_eq!(row["job_name"], "");
    assert!(!r.1.to_string().contains("Recorded customer b"));
}
#[tokio::test]
async fn multiple_route_locations_do_not_duplicate_an_appointment() {
    let f = Fixture::new().await;
    for (id, tenant, route, order) in [
        ("loc-1", "tenant-a", "route-a", 2),
        ("loc-2", "tenant-a", "route-a", 3),
        ("foreign-loc", "tenant-b", "route-b", 1),
    ] {
        sqlx::query("INSERT INTO job_locations(id,tenant_id,service_route_id,appointment_id,sequence_order)VALUES($1,$2,$3,'appointment-a',$4)").bind(id).bind(tenant).bind(route).bind(order).execute(&f.admin).await.unwrap();
    }
    let r = f.read(Some("tenant-a"), "?tenant_id=tenant-a", true).await;
    f.finish().await;
    assert_eq!(r.0, StatusCode::OK);
    assert_eq!(r.1["appointments"].as_array().unwrap().len(), 1);
}
#[tokio::test]
async fn closed_database_is_unavailable_not_empty_or_private_error_text() {
    let f = Fixture::new().await;
    f.scoped.close().await;
    let r = f.read(Some("tenant-a"), "?tenant_id=tenant-a", false).await;
    f.finish().await;
    assert_eq!(r.0, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(
        r.1,
        serde_json::json!({"error":"Appointments are temporarily unavailable"})
    );
    assert_eq!(r.2, "private, no-store");
}
#[test]
fn unsafe_database_targets_are_rejected_before_any_connection() {
    for raw in [
        "",
        "postgres://remote.example/ohc_appointments_test",
        "postgres://127.0.0.1/production",
        "postgres://127.0.0.1/ohc_appointments_test?options=x",
    ] {
        assert!(!guarded_url(raw));
    }
    assert!(guarded_url(
        "postgres://127.0.0.1:55439/ohc_appointments_test"
    ));
}

#[tokio::test]
async fn actual_parent_router_keeps_get_authentication_when_post_is_merged() {
    let f = Fixture::new().await;
    let router = axum::Router::new().nest(
        "/api/v1/field-ops",
        mounted_parent::router(f.scoped.clone(), Arc::new(()), f.store.clone()),
    );
    let unauthorized = router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/field-ops/appointments")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let bearer = format!("Bearer {}", f.store.issue_token(&user("tenant-a")).unwrap());
    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/field-ops/appointments")
                .header("authorization", &bearer)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1_000_000).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    // A terminal sentinel proves the sibling method remains mounted without
    // invoking appointment mutation, mesh dispatch, messaging or providers.
    let sibling = router
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/field-ops/appointments")
                .header("authorization", &bearer)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    f.finish().await;
    assert_eq!(unauthorized.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["appointments"][0]["id"], "appointment-a");
    assert_eq!(sibling.status(), StatusCode::IM_A_TEAPOT);
}
