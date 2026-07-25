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
async fn test_parse_text_message_webhook() {
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
                                "display_phone_number": "16505551111",
                                "phone_number_id": "123456789"
                            },
                            "contacts": [
                                {
                                    "profile": {
                                        "name": "Jane Doe"
                                    },
                                    "wa_id": "16505551234"
                                }
                            ],
                            "messages": [
                                {
                                    "from": "16505551234",
                                    "id": "wamid.HBgLMTY1MDU1NTEyMzQVEQA=",
                                    "timestamp": "1603059201",
                                    "type": "text",
                                    "text": {
                                        "body": "Hello!"
                                    }
                                }
                            ]
                        }
                    }
                ]
            }
        ]
    }"#;

    let webhook_payload: WebhookPayload = serde_json::from_str(payload).unwrap();
    assert_eq!(webhook_payload.entry[0].changes[0].value.messages.as_ref().unwrap()[0].msg_type, "text");
    assert_eq!(webhook_payload.entry[0].changes[0].value.messages.as_ref().unwrap()[0].text.as_ref().unwrap().body, "Hello!");
}

#[tokio::test]
async fn test_parse_status_webhook() {
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
                                "display_phone_number": "16505551111",
                                "phone_number_id": "123456789"
                            },
                            "statuses": [
                                {
                                    "id": "wamid.HBgLMTY1MDU1NTEyMzQVEQA=",
                                    "status": "read",
                                    "timestamp": "1603059201",
                                    "recipient_id": "16505551234"
                                }
                            ]
                        }
                    }
                ]
            }
        ]
    }"#;

    let webhook_payload: WebhookPayload = serde_json::from_str(payload).unwrap();
    assert_eq!(webhook_payload.entry[0].changes[0].value.statuses.as_ref().unwrap()[0].status, "read");
    assert_eq!(webhook_payload.entry[0].changes[0].value.statuses.as_ref().unwrap()[0].id, "wamid.HBgLMTY1MDU1NTEyMzQVEQA=");
}
