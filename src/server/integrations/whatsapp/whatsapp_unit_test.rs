use super::*;
use axum::{body::Body, http::{Request, StatusCode}, Router};
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
async fn test_webhook_parsing_text() {
let app = whatsapp_routes();

    let payload = json!({
        "object": "whatsapp_business_account",
        "entry": [
            {
                "id": "88592102",
                "changes": [
                    {
                        "value": {
                            "messaging_product": "whatsapp",
                            "metadata": {
                                "display_phone_number": "16505551111",
                                "phone_number_id": "123456123"
                            },
                            "contacts": [
                                {
                                    "profile": {
                                        "name": "Kerry Fisher"
                                    },
                                    "wa_id": "16315551234"
                                }
                            ],
                            "messages": [
                                {
                                    "from": "16315551234",
                                    "id": "wamid.HBgLMTYzMTU1NTEyMzQVAgASGCJ",
                                    "timestamp": "1603059201",
                                    "text": {
                                        "body": "Hello this is a test"
                                    },
                                    "type": "text"
                                }
                            ]
                        },
                        "field": "messages"
                    }
                ]
            }
        ]
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

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_webhook_parsing_status() {
let app = whatsapp_routes();

    let payload = json!({
        "object": "whatsapp_business_account",
        "entry": [
            {
                "id": "88592102",
                "changes": [
                    {
                        "value": {
                            "messaging_product": "whatsapp",
                            "metadata": {
                                "display_phone_number": "16505551111",
                                "phone_number_id": "123456123"
                            },
                            "statuses": [
                                {
                                    "id": "wamid.HBgLMTYzMTU1NTEyMzQVAgASGCJ",
                                    "status": "failed",
                                    "timestamp": "1603059201",
                                    "recipient_id": "16315551234",
                                    "errors": [{
                                        "code": 131060,
                                        "title": "Unsupported message type"
                                    }]
                                }
                            ]
                        },
                        "field": "messages"
                    }
                ]
            }
        ]
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

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_webhook_parsing_interactive() {
let app = whatsapp_routes();

    let payload = json!({
        "object": "whatsapp_business_account",
        "entry": [
            {
                "id": "88592102",
                "changes": [
                    {
                        "value": {
                            "messaging_product": "whatsapp",
                            "metadata": {
                                "display_phone_number": "16505551111",
                                "phone_number_id": "123456123"
                            },
                            "messages": [
                                {
                                    "from": "16315551234",
                                    "id": "wamid.HBgLMTYzMTU1NTEyMzQVAgASGCJ",
                                    "timestamp": "1603059201",
                                    "type": "interactive",
                                    "interactive": {
                                        "type": "button_reply",
                                        "button_reply": {
                                            "id": "unique-id",
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

    assert_eq!(response.status(), StatusCode::OK);
}
