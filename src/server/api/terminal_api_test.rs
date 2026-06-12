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
    let app = crate::api::terminal_api::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/sync_offline")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"tenant_id": "test_tenant", "client_id": "client_1", "transactions": []}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Unauthenticated"));
}

#[tokio::test]
async fn test_sync_offline_transactions_authenticated() {
    let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
    if !database_url.contains("test") {
        return;
    }

    let pool = sqlx::postgres::PgPoolOptions::new().connect(&database_url).await.unwrap();

    let hub = Arc::new(Hub::new());
    let mut app = crate::api::terminal_api::router(hub);

    let tenant_id = "test_tenant_sync";
    let _ = sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'Test Tenant') ON CONFLICT DO NOTHING")
        .bind(tenant_id)
        .execute(&pool)
        .await;

    let mut req = Request::builder()
        .uri("/sync_offline")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"tenant_id": "test_tenant_sync", "client_id": "client_1", "transactions": [{"id": "tx_123", "tenant_id": "test_tenant_sync", "client_id": "client_1", "amount_cents": 1000, "currency": "usd", "payload": "{}", "status": "PENDING", "created_at_unix": 1234567890}]}"#))
        .unwrap();

    req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "spiffe://test".to_string(),
        agent_id: "agent_1".to_string(),
        org_id: tenant_id.to_string(),
    });

    let response = app
        .oneshot(req)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains(r#""success":true"#));

    let tx_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM pos_offline_transactions WHERE id = 'tx_123'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(tx_count.0, 1);

    let job_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ohc_job_queue WHERE job_type = 'offline_pos_sync' AND payload->>'pos_transaction_id' = 'tx_123'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(job_count.0, 1);
}
