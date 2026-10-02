use super::*;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use std::sync::Arc;
use tower::ServiceExt;

struct Fixture {
    admin: sqlx::PgPool,
    scoped: sqlx::PgPool,
    schema: String,
    role: String,
    store: Arc<server_auth::Store>,
}
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
impl Fixture {
    async fn new() -> Self {
        let url =
            std::env::var("OHC_SEARCH_TEST_DATABASE_URL").expect("isolated PostgreSQL required");
        let schema = format!("search_{}", uuid::Uuid::new_v4().simple());
        let role = format!("search_reader_{}", uuid::Uuid::new_v4().simple());
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
        let encoding: String = sqlx::query_scalar("SHOW server_encoding")
            .fetch_one(&admin)
            .await
            .unwrap();
        assert_eq!(
            encoding, "UTF8",
            "The search fixture requires a UTF8 database"
        );
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
            sqlx::query("INSERT INTO users(id,username,email,roles,tenant_id)VALUES($1,$2,$3,ARRAY['ADMIN'],$4)")
                .bind(&u.id).bind(&u.username).bind(&u.email).bind(tenant).execute(&admin).await.unwrap();
        }
        for (tenant, suffix) in [("tenant-a", "a"), ("tenant-b", "b")] {
            sqlx::query(
                "INSERT INTO customers(id,tenant_id,name,email,phone)VALUES($1,$2,$3,$4,'12345')",
            )
            .bind(format!("customer-{suffix}"))
            .bind(tenant)
            .bind(format!("Shared {suffix}"))
            .bind(format!("{suffix}@example.test"))
            .execute(&admin)
            .await
            .unwrap();
            sqlx::query(
                "INSERT INTO orders(id,tenant_id,customer_id,status)VALUES($1,$2,$3,'paid')",
            )
            .bind(format!("order-{suffix}"))
            .bind(tenant)
            .bind(format!("customer-{suffix}"))
            .execute(&admin)
            .await
            .unwrap();
            sqlx::query("INSERT INTO omni_inbox_messages(id,tenant_id,source,original_content,translated_content,target_language,sender_id)VALUES($1,$2,'email',$3,$3,'English',$4)")
                .bind(format!("message-{suffix}")).bind(tenant).bind(format!("Shared message {suffix}"))
                .bind(format!("sender-{suffix}")).execute(&admin).await.unwrap();
        }
        sqlx::query(&format!("CREATE ROLE {role} LOGIN NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE PASSWORD '{password}'")).execute(&admin).await.unwrap();
        for table in ["customers", "orders", "omni_inbox_messages"] {
            sqlx::query(&format!("ALTER TABLE {table} FORCE ROW LEVEL SECURITY"))
                .execute(&admin)
                .await
                .unwrap();
        }
        sqlx::raw_sql(&format!("GRANT USAGE ON SCHEMA {schema} TO {role}; GRANT SELECT ON customers,orders,omni_inbox_messages TO {role};")).execute(&admin).await.unwrap();
        let scoped = secure_pg_pool_options()
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
        let identity: (String,String,bool,bool) = sqlx::query_as("SELECT session_user::text,current_user::text,rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user").fetch_one(&scoped).await.unwrap();
        assert_eq!(identity, (role.clone(), role.clone(), false, false));
        let store = Arc::new(server_auth::Store::with_repo(Arc::new(
            server_auth::postgres_store::PgUserRepository::new(admin.clone()),
        )));
        Self {
            admin,
            scoped,
            schema,
            role,
            store,
        }
    }
    async fn read(
        &self,
        tenant: Option<&str>,
        query: &str,
    ) -> (StatusCode, serde_json::Value, String) {
        let app = search::router(self.scoped.clone(), self.store.clone());
        let mut request = Request::builder()
            .uri(format!("/api/v1/search?{query}"))
            .header("x-tenant-id", "tenant-b")
            .header("x-spiffe-id", "spiffe://ohc/org/tenant-b/agent/forged");
        if let Some(tenant) = tenant {
            request = request.header(
                "authorization",
                format!("Bearer {}", self.store.issue_token(&user(tenant)).unwrap()),
            );
        }
        let response = app
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
        (
            status,
            serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null),
            cache,
        )
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
async fn searches_three_actual_owned_record_types_under_forced_rls() {
    let f = Fixture::new().await;
    let response = f.read(Some("tenant-a"), "q=shared").await;
    f.finish().await;
    assert_eq!(response.0, StatusCode::OK);
    assert_eq!(response.2, "private, no-store");
    assert_eq!(response.1["success"], true);
    let rows = response.1["results"].as_array().unwrap();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0]["id"], "customer-a");
    assert_eq!(
        rows[0]["route"],
        "/customer/memory-graph?customerId=customer-a"
    );
    assert_eq!(rows[1]["id"], "order-a");
    assert_eq!(rows[1]["route"], "/orders/order-a");
    assert_eq!(rows[2]["id"], "message-a");
    assert_eq!(rows[2]["route"], "/inbox?messageId=message-a");
    assert!(
        rows.iter()
            .all(|v| !v["id"].as_str().unwrap().ends_with("-b"))
    );
}

#[tokio::test]
async fn same_connection_switches_tenants_and_resets_context() {
    let f = Fixture::new().await;
    let pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&f.scoped)
        .await
        .unwrap();
    let a = f.read(Some("tenant-a"), "q=shared").await;
    let b = f.read(Some("tenant-b"), "q=shared").await;
    let empty = f.read(Some("tenant-empty"), "q=shared").await;
    let again = f.read(Some("tenant-a"), "q=shared").await;
    let current: (i32, Option<String>) =
        sqlx::query_as("SELECT pg_backend_pid(),current_setting('app.current_tenant',true)")
            .fetch_one(&f.scoped)
            .await
            .unwrap();
    f.finish().await;
    assert_eq!(current.0, pid);
    assert!(current.1.as_deref().unwrap_or("").is_empty());
    assert_eq!(a, again);
    assert_eq!(b.1["results"][0]["id"], "customer-b");
    assert_eq!(empty.0, StatusCode::OK);
    assert_eq!(empty.1, serde_json::json!({"success":true,"results":[]}));
}

#[tokio::test]
async fn forged_headers_and_unscoped_signed_identity_cannot_read() {
    let f = Fixture::new().await;
    let anonymous = f.read(None, "q=shared").await;
    let system = f.read(Some("system"), "q=shared").await;
    f.finish().await;
    assert_eq!(anonymous.0, StatusCode::UNAUTHORIZED);
    // The existing strict bearer middleware rejects unscoped identities first.
    assert_eq!(system.0, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn literal_wildcards_unicode_and_ids_are_encoded_without_changing_text() {
    let f = Fixture::new().await;
    let id = "id /?&雪";
    let name = "Shared %_\\ Café 雪";
    sqlx::query("INSERT INTO customers(id,tenant_id,name)VALUES($1,'tenant-a',$2)")
        .bind(id)
        .bind(name)
        .execute(&f.admin)
        .await
        .unwrap();
    let literal = f.read(Some("tenant-a"), "q=%25_%5C").await;
    let unicode = f.read(Some("tenant-a"), "q=Caf%C3%A9%20%E9%9B%AA").await;
    f.finish().await;
    assert_eq!(literal.0, StatusCode::OK);
    assert_eq!(literal.1["results"].as_array().unwrap().len(), 1);
    assert_eq!(unicode.1, literal.1);
    assert_eq!(literal.1["results"][0]["id"], id);
    assert_eq!(literal.1["results"][0]["title"], name);
    assert_eq!(
        literal.1["results"][0]["route"],
        "/customer/memory-graph?customerId=id%20%2F%3F%26%E9%9B%AA"
    );
}

#[tokio::test]
async fn cross_tenant_customer_reference_never_leaks_joined_name() {
    let f = Fixture::new().await;
    sqlx::query("UPDATE customers SET name='Foreign Secret' WHERE id='customer-b'")
        .execute(&f.admin)
        .await
        .unwrap();
    sqlx::query("INSERT INTO orders(id,tenant_id,customer_id,status)VALUES('cross-order','tenant-a','customer-b','pending')").execute(&f.admin).await.unwrap();
    let secret = f.read(Some("tenant-a"), "q=Foreign").await;
    let owned = f.read(Some("tenant-a"), "q=cross-order").await;
    f.finish().await;
    assert_eq!(secret.1["results"], serde_json::json!([]));
    assert_eq!(owned.1["results"].as_array().unwrap().len(), 1);
    assert!(!owned.1.to_string().contains("Foreign"));
}

#[tokio::test]
async fn query_contract_rejects_unbounded_and_unknown_inputs() {
    let f = Fixture::new().await;
    let mut statuses = Vec::new();
    for query in [
        "q=",
        "q=%20%20",
        "q=x",
        "q=shared&limit=0",
        "q=shared&limit=51",
        "q=shared&limit=-1",
        "q=shared&tenant_id=tenant-b",
        "q=shared&offset=1",
    ] {
        statuses.push(f.read(Some("tenant-a"), query).await.0);
    }
    statuses.push(
        f.read(Some("tenant-a"), &format!("q={}", "a".repeat(201)))
            .await
            .0,
    );
    f.finish().await;
    assert!(
        statuses.iter().all(|s| *s == StatusCode::BAD_REQUEST),
        "{statuses:?}"
    );
}

#[tokio::test]
async fn result_limit_is_total_and_stable_across_categories() {
    let f = Fixture::new().await;
    let first = f.read(Some("tenant-a"), "q=shared&limit=2").await;
    let again = f.read(Some("tenant-a"), "q=shared&limit=2").await;
    f.finish().await;
    assert_eq!(first, again);
    assert_eq!(first.1["results"].as_array().unwrap().len(), 2);
    assert_eq!(first.1["results"][0]["entity_type"], "customer");
    assert_eq!(first.1["results"][1]["entity_type"], "order");
}

#[tokio::test]
async fn database_failure_is_unavailable_and_does_not_claim_empty_success() {
    let f = Fixture::new().await;
    f.scoped.close().await;
    let response = f.read(Some("tenant-a"), "q=private-search-text").await;
    f.finish().await;
    assert_eq!(response.0, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(
        response.1,
        serde_json::json!({"success":false,"error":"Search is temporarily unavailable"})
    );
    assert_eq!(response.2, "private, no-store");
}

#[tokio::test]
async fn partial_table_failure_is_not_a_partial_success() {
    let f = Fixture::new().await;
    sqlx::query("ALTER TABLE omni_inbox_messages RENAME TO absent_messages")
        .execute(&f.admin)
        .await
        .unwrap();
    let response = f.read(Some("tenant-a"), "q=shared").await;
    f.finish().await;
    assert_eq!(response.0, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(response.1["success"], false);
    assert!(response.1.get("results").is_none());
}

#[tokio::test]
async fn default_and_maximum_limits_bound_the_combined_result_set() {
    let f = Fixture::new().await;
    for index in 0..60 {
        sqlx::query(
            "INSERT INTO customers(id,tenant_id,name)VALUES($1,'tenant-a','Shared additional')",
        )
        .bind(format!("extra-{index:03}"))
        .execute(&f.admin)
        .await
        .unwrap();
    }
    let default = f.read(Some("tenant-a"), "q=shared").await;
    let maximum = f.read(Some("tenant-a"), "q=shared&limit=50").await;
    f.finish().await;
    assert_eq!(default.0, StatusCode::OK);
    assert_eq!(default.1["results"].as_array().unwrap().len(), 20);
    assert_eq!(maximum.1["results"].as_array().unwrap().len(), 50);
    assert_eq!(
        default.1["results"],
        serde_json::json!(&maximum.1["results"].as_array().unwrap()[..20])
    );
}

#[tokio::test]
async fn message_and_order_routes_encode_ids_and_snippets_bound_unicode_text() {
    let f = Fixture::new().await;
    let id = "encoded /?&雪";
    sqlx::query("INSERT INTO orders(id,tenant_id,status)VALUES($1,'tenant-a','pending')")
        .bind(id)
        .execute(&f.admin)
        .await
        .unwrap();
    sqlx::query("INSERT INTO omni_inbox_messages(id,tenant_id,source,original_content,translated_content,target_language,sender_id)VALUES($1,'tenant-a','email',$2,$2,'English',$3)")
        .bind(id).bind(format!("encoded {}", "雪".repeat(1000))).bind("雪".repeat(500)).execute(&f.admin).await.unwrap();
    let response = f.read(Some("tenant-a"), "q=encoded").await;
    f.finish().await;
    assert_eq!(response.0, StatusCode::OK);
    let rows = response.1["results"].as_array().unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["route"], "/orders/encoded%20%2F%3F%26%E9%9B%AA");
    assert_eq!(
        rows[1]["route"],
        "/inbox?messageId=encoded%20%2F%3F%26%E9%9B%AA"
    );
    assert_eq!(rows[1]["title"].as_str().unwrap().chars().count(), 200);
    assert_eq!(rows[1]["subtitle"].as_str().unwrap().chars().count(), 240);
}

#[tokio::test]
async fn dot_segment_order_ids_cannot_create_misdirected_result_routes() {
    let f = Fixture::new().await;
    sqlx::query("INSERT INTO orders(id,tenant_id,status)VALUES('..','tenant-a','route-edge')")
        .execute(&f.admin)
        .await
        .unwrap();
    let response = f.read(Some("tenant-a"), "q=route-edge").await;
    f.finish().await;
    assert_eq!(response.0, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(response.1["success"], false);
    assert!(response.1.get("results").is_none());
}

#[tokio::test]
async fn customer_id_beyond_maintained_destination_limit_is_not_returned() {
    let f = Fixture::new().await;
    sqlx::query("INSERT INTO customers(id,tenant_id,name)VALUES($1,'tenant-a','long-route-edge')")
        .bind("x".repeat(201))
        .execute(&f.admin)
        .await
        .unwrap();
    let response = f.read(Some("tenant-a"), "q=long-route-edge").await;
    f.finish().await;
    assert_eq!(response.0, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(response.1["success"], false);
    assert!(response.1.get("results").is_none());
}
