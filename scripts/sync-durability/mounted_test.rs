use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    routing::post,
};
use tower::ServiceExt;
#[tokio::test]
async fn forged_headers_do_not_authorize_any_mounted_sync_route() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_millis(20))
        .connect_lazy("postgres://localhost/unused")
        .unwrap();
    let app = router(pool);
    for (route, body) in [
        ("events", r#"{"events":[]}"#),
        ("operation-intents", r#"{"intents":[]}"#),
    ] {
        let request = Request::builder()
            .method("POST")
            .uri(format!("/api/v1/sync/{route}"))
            .header("content-type", "application/json")
            .header("x-spiffe-id", "spiffe://ohc/org/tenant-a/agent/owner")
            .header("x-tenant-id", "tenant-a")
            .body(Body::from(body))
            .unwrap();
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}

async fn fixture() -> (sqlx::PgPool, server_auth::Store) {
    let url = std::env::var("OHC_SYNC_TEST_DATABASE_URL").expect("isolated test database required");
    let schema = format!("http_sync_test_{}", uuid::Uuid::new_v4().simple());
    let admin = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await
        .unwrap();
    sqlx::query(&format!("CREATE SCHEMA {schema}"))
        .execute(&admin)
        .await
        .unwrap();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(8)
        .after_connect(move |connection, _| {
            let schema = schema.clone();
            Box::pin(async move {
                sqlx::query(&format!("SET search_path TO {schema}"))
                    .execute(connection)
                    .await?;
                Ok(())
            })
        })
        .connect(&url)
        .await
        .unwrap();
    sqlx::raw_sql(include_str!(
        "../../src/server/api/durable_sync_test_schema.sql"
    ))
    .execute(&pool)
    .await
    .unwrap();
    sqlx::raw_sql(include_str!(
        "../../src/server/migrations/234_sync_durable_receipts.sql"
    ))
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query("CREATE TABLE revoked_tokens (jti TEXT,tenant_id TEXT,expires_at TIMESTAMPTZ,UNIQUE(jti,tenant_id))").execute(&pool).await.unwrap();
    let store = server_auth::Store::with_repo(std::sync::Arc::new(
        server_auth::postgres_store::PgUserRepository::new(pool.clone()),
    ));
    (pool, store)
}
fn user(tenant: Option<&str>) -> server_auth::User {
    server_auth::User {
        id: "owner".into(),
        username: "owner".into(),
        email: "owner@example.test".into(),
        password_hash: String::new(),
        roles: vec!["owner".into()],
        active: true,
        organization_id: tenant.map(str::to_owned),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        oidc_subject: None,
    }
}
fn router(pool: sqlx::PgPool) -> Router {
    let store = std::sync::Arc::new(server_auth::Store::with_repo(std::sync::Arc::new(
        server_auth::postgres_store::PgUserRepository::new(pool.clone()),
    )));
    let state = crate::offline_sync::SyncEventsState {
        pool: pool.clone(),
        access: crate::api::field_ops::records::FieldAccess { pool: None, store },
    };
    let write_state = crate::offline_sync::SyncWriteState {
        mutations: state.access.clone(),
        intents: state.access.clone(),
    };
    Router::new()
        .route(
            "/api/v1/sync/events",
            post(crate::offline_sync::sync_events_handler),
        )
        .with_state(state)
        .merge(
            Router::new()
                .route(
                    "/api/v1/sync/operation-intents",
                    post(crate::offline_sync::operation_intents_handler),
                )
                .with_state(write_state),
        )
}
async fn request(
    app: &Router,
    route: &str,
    token: Option<&str>,
    body: serde_json::Value,
) -> axum::response::Response {
    let mut request = Request::builder()
        .method("POST")
        .uri(route)
        .header("content-type", "application/json")
        .header("x-spiffe-id", "spiffe://ohc/org/forged/agent/owner")
        .header("x-tenant-id", "forged");
    if let Some(token) = token {
        request = request.header("authorization", format!("Bearer {token}"));
    }
    app.clone()
        .oneshot(request.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap()
}
#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn signed_mounted_event_commits_only_in_signed_tenant_and_preserves_ids() {
    let (pool, store) = fixture().await;
    sqlx::query("INSERT INTO products (id,tenant_id) VALUES ('p','tenant-a'),('foreign','forged')")
        .execute(&pool)
        .await
        .unwrap();
    let token = store.issue_token(&user(Some("tenant-a"))).unwrap();
    let app = router(pool.clone());
    let body = serde_json::json!({"events":[{"id":"accepted-id","entity_id":"p","entity_type":"product","action_type":"ToggleSoldOut","payload":{"is_sold_out":true,"expected_is_sold_out":false,"expected_updated_at":"2026-01-01T00:00:00Z"},"base_version":1},{"id":"foreign-id","entity_id":"foreign","entity_type":"product","action_type":"ToggleSoldOut","payload":{"is_sold_out":true,"expected_is_sold_out":false,"expected_updated_at":"2026-01-01T00:00:00Z"},"base_version":1}]});
    let response = request(&app, "/api/v1/sync/events", Some(&token), body).await;
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(value["applied_count"], 1);
    assert_eq!(value["failed_count"], 1);
    assert_eq!(value["outcomes"][0]["id"], "accepted-id");
    assert_eq!(value["outcomes"][0]["route"], "/api/v1/sync/events");
    assert_eq!(value["outcomes"][0]["status"], "acknowledged");
    assert_eq!(value["outcomes"][1]["status"], "blocked");
    let sold: bool = sqlx::query_scalar("SELECT is_sold_out FROM products WHERE id='foreign'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(!sold);
}
#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn missing_invalid_revoked_unscoped_auth_never_acknowledges() {
    let (pool, store) = fixture().await;
    let app = router(pool.clone());
    let token = store.issue_token(&user(Some("tenant-a"))).unwrap();
    let claims = store.validate_token(&token).await.unwrap();
    store
        .revoke_token(
            claims.jti,
            chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
            "tenant-a",
        )
        .await
        .unwrap();
    let unscoped = store.issue_token(&user(None)).unwrap();
    for token in [
        None,
        Some("invalid"),
        Some(token.as_str()),
        Some(unscoped.as_str()),
    ] {
        for (route, payload) in [
            ("/api/v1/sync/events", serde_json::json!({"events":[]})),
            (
                "/api/v1/sync/operation-intents",
                serde_json::json!({"intents":[]}),
            ),
        ] {
            assert_eq!(
                request(&app, route, token, payload).await.status(),
                StatusCode::UNAUTHORIZED
            );
        }
    }
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM sync_events")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}
#[tokio::test]
async fn mounted_terminal_rejects_forged_headers_without_verified_extension() {
    let app = Router::new()
        .route(
            "/api/v1/payments/terminal/sync_offline",
            post(crate::terminal_api::sync_offline_transactions_handler),
        )
        .with_state(std::sync::Arc::new(crate::Hub));
    assert_eq!(
        request(
            &app,
            "/api/v1/payments/terminal/sync_offline",
            None,
            serde_json::json!({"transactions":[]})
        )
        .await
        .status(),
        StatusCode::UNAUTHORIZED
    );
}

#[tokio::test]
async fn pos_reads_reject_forged_tenant_headers_without_verified_claims() {
    let app = crate::pos_read::router(std::sync::Arc::new(crate::Hub));
    for route in ["/api/v1/pos/orders", "/api/v1/pos/inventory"] {
        let response = app
            .clone()
            .oneshot(
                Request::get(route)
                    .header("x-tenant-id", "tenant-a")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn authenticated_pos_reads_supply_exact_observed_state_for_durable_sync() {
    let _guard = crate::db::POS_READ_LOCK.lock().await;
    let (pool, store) = fixture().await;
    sqlx::raw_sql("CREATE TABLE users(id TEXT,tenant_id TEXT,username TEXT,email TEXT,password_hash TEXT,roles TEXT[],active BOOL,oidc_subject TEXT,created_at TIMESTAMPTZ,updated_at TIMESTAMPTZ);CREATE TABLE customers(id TEXT,tenant_id TEXT,name TEXT);
    ALTER TABLE products ADD COLUMN title TEXT,ADD COLUMN description TEXT,ADD COLUMN price_cents BIGINT DEFAULT 100,ADD COLUMN currency TEXT DEFAULT 'USD',ADD COLUMN is_subscribable BOOL DEFAULT false,ADD COLUMN subscription_discount_percent INTEGER DEFAULT 0,ADD COLUMN subscription_frequency TEXT;
    ALTER TABLE orders ADD COLUMN created_at TIMESTAMPTZ DEFAULT now(),ADD COLUMN translated_notes TEXT;
    INSERT INTO users VALUES('owner','tenant-a','owner','owner@example.test','',ARRAY['ADMIN'],true,NULL,now(),now());
    INSERT INTO customers VALUES('same','tenant-a','Owned customer'),('same','tenant-b','Private customer');
    INSERT INTO products(id,tenant_id,title,is_sold_out,updated_at) VALUES('p','tenant-a','Owned',true,'2026-10-01T01:02:03.123456Z'),('foreign','tenant-b','Private',false,'2026-10-01T01:02:03.987654Z'),('unknown','tenant-a','Unknown token',false,NULL);
    INSERT INTO orders(id,tenant_id,customer_id,total_amount,status,updated_at) VALUES('o','tenant-a','same',1,'pending','2026-10-01T01:02:03.654321Z'),('foreign-o','tenant-b','same',2,'pending',now());").execute(&pool).await.unwrap();
    crate::db::set_pool(pool.clone());
    let token = store.issue_token(&user(Some("tenant-a"))).unwrap();
    let store = std::sync::Arc::new(store);
    let app = crate::pos_read::router(std::sync::Arc::new(crate::Hub)).layer(
        axum::middleware::from_fn_with_state(store, server_auth::strict_bearer_auth_middleware),
    );
    async fn get(app: &Router, route: &str, token: &str) -> serde_json::Value {
        let response = app
            .clone()
            .oneshot(
                Request::get(route)
                    .header("authorization", format!("Bearer {token}"))
                    .header("x-tenant-id", "tenant-b")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        serde_json::from_slice(
            &axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap()
    }
    let inventory = get(&app, "/api/v1/pos/inventory", &token).await;
    let products = inventory["inventory"].as_array().unwrap();
    assert_eq!(products.len(), 2);
    let product = products.iter().find(|p| p["id"] == "p").unwrap();
    assert_eq!(
        product["is_sold_out"], true,
        "stored sold-out state must reach the queue producer"
    );
    assert_eq!(product["updated_at"], "2026-10-01T01:02:03.123456+00:00");
    let unknown = products.iter().find(|p| p["id"] == "unknown").unwrap();
    assert!(
        unknown["updated_at"].is_null(),
        "missing token cannot be manufactured"
    );
    let order_response = get(&app, "/api/v1/pos/orders", &token).await;
    let orders = order_response["orders"].as_array().unwrap();
    assert_eq!(orders.len(), 1);
    assert_eq!(orders[0]["customer_name"], "Owned customer");
    assert_eq!(orders[0]["status"], "pending");
    assert_eq!(orders[0]["updated_at"], "2026-10-01T01:02:03.654321+00:00");
    let events = serde_json::json!({"events":[
      {"id":"pos-product","entity_type":"product","entity_id":"p","action_type":"ToggleSoldOut","base_version":0,"payload":{"is_sold_out":false,"expected_is_sold_out":product["is_sold_out"],"expected_updated_at":product["updated_at"]}},
      {"id":"pos-order","entity_type":"order","entity_id":"o","action_type":"UpdateStatus","base_version":0,"payload":{"status":"ready","expected_status":orders[0]["status"],"expected_updated_at":orders[0]["updated_at"]}}
    ]});
    let response = request(
        &router(pool.clone()),
        "/api/v1/sync/events",
        Some(&token),
        events,
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let outcome: serde_json::Value = serde_json::from_slice(
        &axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        outcome["applied_count"], 2,
        "both actions must use the actual observed row tokens: {outcome}"
    );
    assert!(
        outcome["outcomes"]
            .as_array()
            .unwrap()
            .iter()
            .all(|row| row["status"] == "acknowledged")
    );
    let foreign:(bool,String)=sqlx::query_as("SELECT p.is_sold_out,o.status FROM products p,orders o WHERE p.id='foreign' AND o.id='foreign-o'").fetch_one(&pool).await.unwrap();
    assert_eq!(foreign, (false, "pending".to_owned()));
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn empty_inventory_get_is_read_only_and_does_not_invent_catalog_entries() {
    let _guard = crate::db::POS_READ_LOCK.lock().await;
    let (pool, _) = fixture().await;
    sqlx::raw_sql("ALTER TABLE products ADD COLUMN title TEXT,ADD COLUMN description TEXT,ADD COLUMN price_cents BIGINT DEFAULT 100,ADD COLUMN currency TEXT DEFAULT 'USD',ADD COLUMN is_subscribable BOOL DEFAULT false,ADD COLUMN subscription_discount_percent INTEGER DEFAULT 0,ADD COLUMN subscription_frequency TEXT;").execute(&pool).await.unwrap();
    crate::db::set_pool(pool.clone());
    let claims: server_common::Claims = serde_json::from_value(serde_json::json!({"sub":"owner","exp":4000000000u64,"organization_id":"empty-tenant","roles":["ADMIN"],"iat":0})).unwrap();
    let app =
        crate::pos_read::router(std::sync::Arc::new(crate::Hub)).layer(axum::Extension(claims));
    for _ in 0..2 {
        let response = app
            .clone()
            .oneshot(
                Request::get("/api/v1/pos/inventory")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(
            body,
            serde_json::json!({"inventory":[]}),
            "an empty catalog is an actual empty result"
        );
        let count: i64 = sqlx::query_scalar("SELECT count(*) FROM products")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 0, "a GET must never seed a hardcoded product");
    }
}
