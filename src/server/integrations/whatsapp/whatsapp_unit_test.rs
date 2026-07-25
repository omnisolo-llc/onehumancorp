use super::*;
use axum::{body::Body, http::{Request, StatusCode}};
use tower::ServiceExt; // for `oneshot`

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
async fn test_webhook_parsing_message() {
    let payload = r#"{
        "object": "whatsapp_business_account",
        "entry": [{
            "id": "123",
            "changes": [{
                "value": {
                    "messaging_product": "whatsapp",
                    "metadata": {
                        "display_phone_number": "12345",
                        "phone_number_id": "12345"
                    },
                    "contacts": [{
                        "profile": {"name": "Test User"},
                        "wa_id": "12345"
                    }],
                    "messages": [{
                        "from": "12345",
                        "id": "wamid.123",
                        "timestamp": "12345",
                        "type": "text",
                        "text": {"body": "Hello"}
                    }]
                },
                "field": "messages"
            }]
        }]
    }"#;
    let parsed: crate::webhook::WebhookPayload = serde_json::from_str(payload).unwrap();
    assert_eq!(parsed.entry[0].changes[0].value.messages.as_ref().unwrap()[0].text.as_ref().unwrap().body, "Hello");
}

#[tokio::test]
async fn test_webhook_parsing_interactive() {
    let payload = r#"{
        "object": "whatsapp_business_account",
        "entry": [{
            "id": "123",
            "changes": [{
                "value": {
                    "messaging_product": "whatsapp",
                    "metadata": {
                        "display_phone_number": "12345",
                        "phone_number_id": "12345"
                    },
                    "messages": [{
                        "from": "12345",
                        "id": "wamid.123",
                        "timestamp": "12345",
                        "type": "interactive",
                        "interactive": {
                            "type": "button_reply",
                            "button_reply": {
                                "id": "btn-1",
                                "title": "Click me"
                            }
                        }
                    }]
                },
                "field": "messages"
            }]
        }]
    }"#;
    let parsed: crate::webhook::WebhookPayload = serde_json::from_str(payload).unwrap();
    assert_eq!(parsed.entry[0].changes[0].value.messages.as_ref().unwrap()[0].interactive.as_ref().unwrap().button_reply.as_ref().unwrap().title, "Click me");
}
