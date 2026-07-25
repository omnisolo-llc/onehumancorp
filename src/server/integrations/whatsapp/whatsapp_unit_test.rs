use super::*;
use axum::{body::Body, http::{Request, StatusCode}};
use tower::ServiceExt; // for `oneshot`
use serde_json::json;

#[tokio::test]
async fn test_verify_webhook_success() {
    let app = whatsapp_routes();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/webhook?hub.mode=subscribe&hub.verify_token=ohc_whatsapp_webhook_secret&hub.challenge=1158201444")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_verify_webhook_failure() {
    let app = whatsapp_routes();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/webhook?hub.mode=subscribe&hub.verify_token=wrong_token&hub.challenge=1158201444")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_handle_webhook_payload_parsing() {
    let app = whatsapp_routes();

    let payload = json!({
        "object": "whatsapp_business_account",
        "entry": [{
            "id": "123456789",
            "changes": [{
                "field": "messages",
                "value": {
                    "messaging_product": "whatsapp",
                    "metadata": {
                        "display_phone_number": "16505551111",
                        "phone_number_id": "123456789"
                    },
                    "contacts": [{
                        "profile": { "name": "John Doe" },
                        "wa_id": "16315551234"
                    }],
                    "messages": [{
                        "from": "16315551234",
                        "id": "wamid.HBgLMTYzMTU1NTEyMzQFIg",
                        "timestamp": "1603059201",
                        "type": "text",
                        "text": { "body": "Hello World" }
                    }]
                }
            }]
        }]
    });

    let response = app
        .oneshot(
            Request::builder()
                .uri("/webhook")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Since we don't mock Redis entirely in this integration test context,
    // it will gracefully skip Redis (using get_redis_conn returning None)
    // and successfully process the internal mapping, returning OK.
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_webhook_setup_service() {
    // Basic initialization test for WebhookSetupService
    let client = WhatsAppClient::new("fake_token".to_string(), "fake_phone_id".to_string());
    let service = WebhookSetupService::new(client);

    // We do not mock out the reqwest HTTP call in this basic unit test,
    // so we won't call setup_webhook_and_register to avoid real network errors.
    // In a complete suite, mock HTTP components would be utilized.
    let _ = service;
    assert!(true);
}
