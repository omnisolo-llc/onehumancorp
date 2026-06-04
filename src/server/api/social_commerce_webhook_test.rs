use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::post,
    Router,
};
use std::sync::Arc;
use tower::ServiceExt; // for `oneshot` and `ready`
use ::server_tools::edgecommercemcp::server::EdgeCommerceMcpServer;

use super::social_commerce_webhook::{handle_social_commerce_webhook, SocialCommerceState, SocialWebhookPayload, SocialWebhookResponse};

// Manually extract body bytes to avoid axum/hyper dependency issues in bazel
async fn get_body_bytes(body: axum::body::Body) -> Vec<u8> {
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    bytes.to_vec()
}

#[tokio::test]
async fn test_social_commerce_webhook_handler_quote() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        return;
    }
    let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
    let pool = match sqlx::postgres::PgPoolOptions::new().connect(&db_url).await {
        Ok(p) => p,
        Err(_) => return,
    };

    let redis_client = match redis::Client::open("redis://localhost:6379/") {
        Ok(c) => c,
        Err(_) => return,
    };

    // ensure product exists
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS products (
            id TEXT,
            tenant_id TEXT,
            price_cents BIGINT,
            inventory_count BIGINT,
            PRIMARY KEY (id, tenant_id)
        )"
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO products (id, tenant_id, price_cents, inventory_count) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING"
    )
    .bind("prod-1")
    .bind("tenant-test")
    .bind(1000)
    .bind(10)
    .execute(&pool)
    .await
    .unwrap();

    let server = EdgeCommerceMcpServer::new(redis_client, pool);

    let state = SocialCommerceState {
        edge_commerce_server: Arc::new(server),
    };

    let app = Router::new()
        .route("/webhook", post(handle_social_commerce_webhook))
        .with_state(state);

    let payload = SocialWebhookPayload {
        channel: "instagram".to_string(),
        tenant_id: "tenant-test".to_string(),
        message: "I want to buy this".to_string(),
        customer_id: "cust-1".to_string(),
        product_id: Some("prod-1".to_string()),
        quantity: Some(2),
    };

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhook")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = get_body_bytes(response.into_body()).await;
    let resp_payload: SocialWebhookResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(resp_payload.status, "success");
    assert!(resp_payload.reply_message.unwrap().contains("20.00"));
    assert!(resp_payload.checkout_link.unwrap().contains("checkout.stripe.com"));
}

#[tokio::test]
async fn test_social_commerce_webhook_handler_generic() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        return;
    }
    let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
    let pool = match sqlx::postgres::PgPoolOptions::new().connect(&db_url).await {
        Ok(p) => p,
        Err(_) => return,
    };

    let redis_client = match redis::Client::open("redis://localhost:6379/") {
        Ok(c) => c,
        Err(_) => return,
    };
    let server = EdgeCommerceMcpServer::new(redis_client, pool);

    let state = SocialCommerceState {
        edge_commerce_server: Arc::new(server),
    };

    let app = Router::new()
        .route("/webhook", post(handle_social_commerce_webhook))
        .with_state(state);

    let payload = SocialWebhookPayload {
        channel: "whatsapp".to_string(),
        tenant_id: "tenant-test".to_string(),
        message: "Hello".to_string(),
        customer_id: "cust-1".to_string(),
        product_id: None,
        quantity: None,
    };

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhook")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = get_body_bytes(response.into_body()).await;
    let resp_payload: SocialWebhookResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(resp_payload.status, "success");
    assert!(resp_payload.reply_message.unwrap().contains("Message received"));
    assert!(resp_payload.checkout_link.is_none());
}
