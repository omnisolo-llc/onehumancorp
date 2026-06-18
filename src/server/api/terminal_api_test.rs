use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use std::sync::Arc;
use crate::hub::Hub;

#[tokio::test]
async fn test_get_terminal_connection_token_unauthenticated() {
    let hub = Arc::new(Hub::new());
    let app = crate::api::terminal_api::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/token")
                .method("POST")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Unauthenticated"));
}

#[tokio::test]
async fn test_get_terminal_connection_token_authenticated() {
    let hub = Arc::new(Hub::new());

    let app_with_auth = axum::Router::new()
        .route("/token", axum::routing::get(crate::api::terminal_api::get_terminal_connection_token_handler))
        .with_state(hub)
        .layer(axum::extract::Extension(::server_auth::orchestration::AuthInfo {
            spiffe_id: "spiffe://test".to_string(),
            agent_id: "agent_1".to_string(),
            org_id: "test_tenant".to_string(),
        }));

    let response = app_with_auth
        .oneshot(
            Request::builder()
                .uri("/token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Stripe API key is required") || body_str.contains("Stripe API error") || body_str.contains("Stripe Terminal connection token request failed"));
}

#[tokio::test]
async fn test_create_payment_intent_authenticated() {
    let hub = Arc::new(Hub::new());

    let app_with_auth = axum::Router::new()
        .route("/intent", axum::routing::post(crate::api::terminal_api::create_payment_intent_handler))
        .with_state(hub)
        .layer(axum::extract::Extension(::server_auth::orchestration::AuthInfo {
            spiffe_id: "spiffe://test".to_string(),
            agent_id: "agent_1".to_string(),
            org_id: "test_tenant".to_string(),
        }));

    let response = app_with_auth
        .oneshot(
            Request::builder()
                .uri("/intent")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"amount_cents": 1500, "currency": "usd", "product_id": "prod_1", "quantity": 1, "order_id": "ord_1"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Stripe API key is required") || body_str.contains("Stripe API error") || body_str.contains("Stripe Terminal connection token request failed"));
}

#[tokio::test]
async fn test_create_payment_intent_unauthenticated() {
    let hub = Arc::new(Hub::new());
    let app = crate::api::terminal_api::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/intent")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"amount_cents": 1000, "currency": "usd", "product_id": null, "quantity": null, "order_id": null}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Unauthenticated"));
}

#[tokio::test]
async fn test_get_terminal_connection_token_authenticated_via_router() {
    let hub = Arc::new(Hub::new());
    let mut app = crate::api::terminal_api::router(hub);

    let mut req = Request::builder()
        .uri("/token")
        .method("POST")
        .body(Body::empty())
        .unwrap();

    req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "spiffe://test".to_string(),
        agent_id: "agent_1".to_string(),
        org_id: "test_tenant".to_string(),
    });

    let response = app
        .oneshot(req)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Stripe API key is required") || body_str.contains("Stripe API error") || body_str.contains("Stripe Terminal connection token request failed"));
}

#[tokio::test]
async fn test_create_payment_intent_authenticated_via_router() {
    let hub = Arc::new(Hub::new());
    let mut app = crate::api::terminal_api::router(hub);

    let mut req = Request::builder()
        .uri("/intent")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"amount_cents": 1500, "currency": "usd", "product_id": "prod_2", "quantity": 2, "order_id": "ord_2"}"#))
        .unwrap();

    req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "spiffe://test".to_string(),
        agent_id: "agent_1".to_string(),
        org_id: "test_tenant".to_string(),
    });

    let response = app
        .oneshot(req)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Stripe API key is required") || body_str.contains("Stripe API error") || body_str.contains("Stripe Terminal connection token request failed"));
}

#[tokio::test]
async fn test_sync_offline_transactions_unauthenticated() {
    let hub = Arc::new(Hub::new());
    let mut app = crate::api::terminal_api::router(hub);

    let req = Request::builder()
        .uri("/sync_offline")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"session_id": "sess_1", "transactions": []}"#))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Unauthenticated"));
}

#[tokio::test]
async fn test_sync_offline_transactions_authenticated_empty() {
    let hub = Arc::new(Hub::new());
    let mut app = crate::api::terminal_api::router(hub);

    let mut req = Request::builder()
        .uri("/sync_offline")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"session_id": "sess_1", "transactions": []}"#))
        .unwrap();

    req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "spiffe://test".to_string(),
        agent_id: "agent_1".to_string(),
        org_id: "test_tenant".to_string(),
    });

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    let json: serde_json::Value = serde_json::from_str(&body_str).unwrap();
    assert_eq!(json["success"], true);
    assert_eq!(json["synced_count"], 0);
}

#[tokio::test]
async fn test_sync_offline_transactions_authenticated_with_data() {
    let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
    if !database_url.contains("test") {
        return; // skip if not test db
    }

    let pool = sqlx::postgres::PgPoolOptions::new().connect(&database_url).await.unwrap();
    let tenant_id = "tenant_test_terminal_sync";

    // Setup test tenant
    sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'Sync Test Tenant') ON CONFLICT DO NOTHING")
        .bind(tenant_id).execute(&pool).await.unwrap();

    let (tx, _rx) = tokio::sync::mpsc::channel(100);
    let hub = Arc::new(Hub::new(tx, pool.clone()));
    let mut app = crate::api::terminal_api::router(hub);

    let payload = r#"{
        "session_id": "sess_123",
        "transactions": [
            {
                "id": "tx_abc",
                "client_id": "client_abc",
                "amount_cents": 2500,
                "currency": "usd",
                "payload": "{\"product_id\": \"prod_x\", \"quantity\": 1}"
            }
        ]
    }"#;

    let mut req = Request::builder()
        .uri("/sync_offline")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(payload))
        .unwrap();

    req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "spiffe://test".to_string(),
        agent_id: "agent_1".to_string(),
        org_id: tenant_id.to_string(),
    });

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    let json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

    assert_eq!(json["success"], true);
    assert_eq!(json["synced_count"], 1);

    // Verify it was inserted into pos_offline_transactions
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM pos_offline_transactions WHERE id = 'tx_abc' AND tenant_id = $1")
        .bind(tenant_id)
        .fetch_one(&pool).await.unwrap();
    assert_eq!(count.0, 1);

    // Verify session was created
    let session_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM pos_terminal_sessions WHERE id = 'sess_123' AND tenant_id = $1")
        .bind(tenant_id)
        .fetch_one(&pool).await.unwrap();
    assert_eq!(session_count.0, 1);

    // Verify job was enqueued
    let job_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ohc_job_queue WHERE tenant_id = $1 AND job_type = 'offline_pos_sync' AND payload->>'pos_transaction_id' = 'tx_abc'")
        .bind(tenant_id)
        .fetch_one(&pool).await.unwrap();
    assert_eq!(job_count.0, 1);
}
