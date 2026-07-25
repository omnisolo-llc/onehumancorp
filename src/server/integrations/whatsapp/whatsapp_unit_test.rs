use super::*;
use axum::{body::Body, http::{Request, StatusCode}};
use tower::ServiceExt;

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
async fn test_whatsapp_setup_service() {
    let service = WhatsAppSetupService::new("app123".to_string(), "token456".to_string());

    let result = service.register_webhook("https://example.com/webhook", "mytoken").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_webhook_parsing() {
    let payload = r#"{
        "object": "whatsapp_business_account",
        "entry": [
            {
                "id": "12345",
                "changes": [
                    {
                        "field": "messages",
                        "value": {
                            "messaging_product": "whatsapp",
                            "metadata": {
                                "display_phone_number": "15551234567",
                                "phone_number_id": "1234567890"
                            },
                            "messages": [
                                {
                                    "from": "15559876543",
                                    "id": "wamid.HBgLMTU1NTk4NzY1NDMW...",
                                    "timestamp": "1675123456",
                                    "type": "text",
                                    "text": {
                                        "body": "Hello OHC!"
                                    }
                                }
                            ]
                        }
                    }
                ]
            }
        ]
    }"#;

    let parsed: Result<WebhookPayload, _> = serde_json::from_str(payload);
    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();
    assert_eq!(parsed.entry[0].changes[0].value.messages.as_ref().unwrap()[0].text.as_ref().unwrap().body, "Hello OHC!");
}
