use super::*;
use axum::{body::Body, http::{Request, StatusCode}, Router};
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn test_verify_webhook_success() {
    unsafe { std::env::set_var("WHATSAPP_WEBHOOK_VERIFY_TOKEN", "ohc_whatsapp_webhook_secret"); }
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
    unsafe { std::env::remove_var("WHATSAPP_WEBHOOK_VERIFY_TOKEN"); }
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
async fn test_webhook_payload_parsing() {
    let payload_json = r#"{
        "object": "whatsapp_business_account",
        "entry": [{
            "id": "123456",
            "changes": [{
                "value": {
                    "messaging_product": "whatsapp",
                    "metadata": {
                        "display_phone_number": "1234567890",
                        "phone_number_id": "0987654321"
                    },
                    "messages": [{
                        "from": "1234567890",
                        "id": "wamid.123",
                        "timestamp": "1632345",
                        "type": "text",
                        "text": {
                            "body": "Hello World"
                        }
                    }]
                },
                "field": "messages"
            }]
        }]
    }"#;

    let payload: WebhookPayload = serde_json::from_str(payload_json).unwrap();
    assert_eq!(payload.object, "whatsapp_business_account");
    assert_eq!(payload.entry.len(), 1);
    let message = payload.entry[0].changes[0].value.messages.as_ref().unwrap().first().unwrap();
    assert_eq!(message.msg_type, "text");
    assert_eq!(message.text.as_ref().unwrap().body, "Hello World");
}
