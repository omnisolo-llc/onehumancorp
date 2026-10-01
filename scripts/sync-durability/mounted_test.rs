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
    let app = Router::new()
        .route(
            "/api/v1/sync/events",
            post(crate::offline_sync::sync_events_handler),
        )
        .route(
            "/api/v1/sync/operation-intents",
            post(crate::offline_sync::operation_intents_handler),
        )
        .with_state(pool);
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
    Router::new()
        .route(
            "/api/v1/sync/events",
            post(crate::offline_sync::sync_events_handler),
        )
        .route(
            "/api/v1/sync/operation-intents",
            post(crate::offline_sync::operation_intents_handler),
        )
        .with_state(pool)
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
