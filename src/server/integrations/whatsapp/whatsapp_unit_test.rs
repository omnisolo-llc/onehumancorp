use super::*;
use axum::{body::Body, http::{Request, StatusCode}, Router};
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
async fn test_handle_webhook_text() {
    let app = whatsapp_routes();

    let payload = r#"{
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
                            "contacts": [{
                                "profile": {"name": "Kerry Fisher"},
                                "wa_id": "16315551234"
                            }],
                            "messages": [{
                                "from": "16315551234",
                                "id": "wamid.HBgLMTY...",
                                "timestamp": "1603059201",
                                "text": {"body": "Hello this is a test"},
                                "type": "text"
                            }]
                        },
                        "field": "messages"
                    }
                ]
            }
        ]
    }"#;

    unsafe { std::env::set_var("REDIS_URL", "redis://0.0.0.0:1"); }

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

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_handle_webhook_interactive() {
    let app = whatsapp_routes();

    let payload = r#"{
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
                            "messages": [{
                                "from": "16315551234",
                                "id": "wamid.HBgLMTY...",
                                "timestamp": "1603059201",
                                "type": "interactive",
                                "interactive": {
                                    "type": "button_reply",
                                    "button_reply": {
                                        "id": "unique-button-id",
                                        "title": "Yes"
                                    }
                                }
                            }]
                        },
                        "field": "messages"
                    }
                ]
            }
        ]
    }"#;

    unsafe { std::env::set_var("REDIS_URL", "redis://0.0.0.0:1"); }

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

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_handle_webhook_media() {
    let app = whatsapp_routes();

    let payload = r#"{
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
                            "messages": [{
                                "from": "16315551234",
                                "id": "wamid.HBgLMTY...",
                                "timestamp": "1603059201",
                                "type": "image",
                                "image": {
                                    "id": "image-id",
                                    "mime_type": "image/jpeg",
                                    "sha256": "hash",
                                    "caption": "Look at this!"
                                }
                            }]
                        },
                        "field": "messages"
                    }
                ]
            }
        ]
    }"#;

    unsafe { std::env::set_var("REDIS_URL", "redis://0.0.0.0:1"); }

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

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_handle_webhook_status() {
    let app = whatsapp_routes();

    let payload = r#"{
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
                            "statuses": [{
                                "id": "wamid.HBgLMTY...",
                                "status": "read",
                                "timestamp": "1603059201",
                                "recipient_id": "16315551234",
                                "conversation": {
                                    "id": "conv-123",
                                    "origin": {
                                        "type": "user_initiated"
                                    }
                                },
                                "pricing": {
                                    "billable": true,
                                    "pricing_model": "CBP",
                                    "category": "user_initiated"
                                }
                            }]
                        },
                        "field": "messages"
                    }
                ]
            }
        ]
    }"#;

    unsafe { std::env::set_var("REDIS_URL", "redis://0.0.0.0:1"); }

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

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
