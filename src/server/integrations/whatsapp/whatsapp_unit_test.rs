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
async fn test_handle_webhook_payload() {
    let app = whatsapp_routes();

    let payload = r#"{
        "object": "whatsapp_business_account",
        "entry": [{
            "id": "123",
            "changes": [{
                "field": "messages",
                "value": {
                    "messaging_product": "whatsapp",
                    "metadata": {
                        "display_phone_number": "123",
                        "phone_number_id": "456"
                    },
                    "statuses": [{
                        "id": "msg_id_1",
                        "status": "delivered",
                        "timestamp": "123456",
                        "recipient_id": "987654"
                    }],
                    "messages": [{
                        "from": "user_1",
                        "id": "msg_id_2",
                        "timestamp": "123457",
                        "type": "text",
                        "text": {
                            "body": "Hello World"
                        }
                    }]
                }
            }]
        }]
    }"#;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/webhook")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(payload))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
