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
async fn test_whatsapp_client_mock() {
    let client = WhatsAppClient::new("token".to_string(), "phone_id".to_string())
        .with_base_url("http://mock-url.test".to_string());

    let send_res = client.send_message("+12345", "Hello").await;
    assert!(send_res.is_ok());
    assert_eq!(send_res.unwrap(), "mock_message_id_123");

    let sync_res = client.sync_templates().await;
    assert!(sync_res.is_ok());
    let templates = sync_res.unwrap();
    assert_eq!(templates.len(), 1);
    assert_eq!(templates[0].name, "order_ready");

    let health_res = client.get_phone_number_health().await;
    assert!(health_res.is_ok());
    let health = health_res.unwrap();
    assert_eq!(health.verified_name, "Maya's Home Bakery");
    assert_eq!(health.quality_rating, "GREEN");

    let template_res = client.send_template_message("+12345", "order_ready", "en_US", vec![]).await;
    assert!(template_res.is_ok());
    assert_eq!(template_res.unwrap(), "mock_message_id_12345");
}

#[test]
fn test_webhook_media_and_statuses_deserialization() {
    let payload_json = json!({
        "object": "whatsapp_business_account",
        "entry": [
            {
                "id": "entry_id_123",
                "changes": [
                    {
                        "field": "messages",
                        "value": {
                            "messaging_product": "whatsapp",
                            "metadata": {
                                "display_phone_number": "+1234567890",
                                "phone_number_id": "phone_id_123"
                            },
                            "contacts": [
                                {
                                    "profile": {
                                        "name": "Maya"
                                    },
                                    "wa_id": "maya_wa_id"
                                }
                            ],
                            "messages": [
                                {
                                    "from": "0987654321",
                                    "id": "msg_id_123",
                                    "timestamp": "1625097600",
                                    "type": "image",
                                    "image": {
                                        "id": "img_id_999",
                                        "mime_type": "image/png",
                                        "sha256": "hash256",
                                        "caption": "Lovely Bakery"
                                    }
                                }
                            ],
                            "statuses": [
                                {
                                    "id": "msg_id_123",
                                    "status": "delivered",
                                    "timestamp": "1625097605",
                                    "recipient_id": "0987654321"
                                }
                            ]
                        }
                    }
                ]
            }
        ]
    });

    let payload: WebhookPayload = serde_json::from_value(payload_json).unwrap();
    assert_eq!(payload.object, "whatsapp_business_account");
    let entry = &payload.entry[0];
    assert_eq!(entry.id, "entry_id_123");
    let change = &entry.changes[0];
    assert_eq!(change.field, "messages");
    let val = &change.value;
    assert_eq!(val.messaging_product, "whatsapp");
    assert_eq!(val.metadata.display_phone_number, "+1234567890");

    let msg = &val.messages.as_ref().unwrap()[0];
    assert_eq!(msg.from, "0987654321");
    assert_eq!(msg.msg_type, "image");

    let img = msg.image.as_ref().unwrap();
    assert_eq!(img.id, "img_id_999");
    assert_eq!(img.mime_type.as_deref(), Some("image/png"));
    assert_eq!(img.caption.as_deref(), Some("Lovely Bakery"));

    let status = &val.statuses.as_ref().unwrap()[0];
    assert_eq!(status.id, "msg_id_123");
    assert_eq!(status.status, "delivered");
}
