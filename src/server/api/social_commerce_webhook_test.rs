use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::post,
    Router,
};
use tower::ServiceExt; // for `oneshot` and `ready`

use super::social_commerce_webhook::{handle_social_commerce_webhook, SocialCommerceState, SocialWebhookPayload, SocialWebhookResponse};

// Manually extract body bytes to avoid axum/hyper dependency issues in bazel
async fn get_body_bytes(body: axum::body::Body) -> Vec<u8> {
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    bytes.to_vec()
}

#[tokio::test]
async fn test_social_commerce_webhook_handler_quote() {
    let state = SocialCommerceState {};

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
    let state = SocialCommerceState {};

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
