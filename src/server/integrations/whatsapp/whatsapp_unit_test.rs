use super::*;
use axum::{body::Body, http::{Request, StatusCode}};
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn test_verify_webhook_success() {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let redis_client = redis::Client::open(redis_url).unwrap();
    let state = std::sync::Arc::new(WhatsAppState {
        redis_client,
        access_token: "test".to_string(),
        phone_number_id: "test".to_string(),
    });
    let app = whatsapp_routes(state);

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
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let redis_client = redis::Client::open(redis_url).unwrap();
    let state = std::sync::Arc::new(WhatsAppState {
        redis_client,
        access_token: "test".to_string(),
        phone_number_id: "test".to_string(),
    });
    let app = whatsapp_routes(state);

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
async fn test_parse_interactive_webhook() {
    let payload = r#"{
      "object": "whatsapp_business_account",
      "entry": [
        {
          "id": "123",
          "changes": [
            {
              "value": {
                "messaging_product": "whatsapp",
                "metadata": {
                  "display_phone_number": "123",
                  "phone_number_id": "123"
                },
                "messages": [
                  {
                    "from": "1",
                    "id": "wamid.123",
                    "timestamp": "123",
                    "type": "interactive",
                    "interactive": {
                      "type": "button_reply",
                      "button_reply": {
                        "id": "btn_1",
                        "title": "Yes"
                      }
                    }
                  }
                ]
              },
              "field": "messages"
            }
          ]
        }
      ]
    }"#;
    let webhook: WebhookPayload = serde_json::from_str(payload).unwrap();
    let msg = &webhook.entry[0].changes[0].value.messages.as_ref().unwrap()[0];
    assert_eq!(msg.msg_type, "interactive");
    assert_eq!(msg.interactive.as_ref().unwrap().interactive_type, "button_reply");
    assert_eq!(msg.interactive.as_ref().unwrap().button_reply.as_ref().unwrap().id, "btn_1");
}

#[tokio::test]
async fn test_parse_status_webhook() {
    let payload = r#"{
      "object": "whatsapp_business_account",
      "entry": [
        {
          "id": "123",
          "changes": [
            {
              "value": {
                "messaging_product": "whatsapp",
                "metadata": {
                  "display_phone_number": "123",
                  "phone_number_id": "123"
                },
                "statuses": [
                  {
                    "id": "wamid.123",
                    "status": "delivered",
                    "timestamp": "123",
                    "recipient_id": "1"
                  }
                ]
              },
              "field": "messages"
            }
          ]
        }
      ]
    }"#;
    let webhook: WebhookPayload = serde_json::from_str(payload).unwrap();
    let status = &webhook.entry[0].changes[0].value.statuses.as_ref().unwrap()[0];
    assert_eq!(status.status, "delivered");
    assert_eq!(status.id, "wamid.123");
}
