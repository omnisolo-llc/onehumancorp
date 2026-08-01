use super::*;

#[tokio::test]
async fn test_webhook_handler_whatsapp() {
    // Basic test to ensure it compiles
    let payload = WebhookPayload {
        object: "whatsapp_business_account".to_string(),
        entry: vec![],
    };
    let response = handle_whatsapp_webhook(
        axum::extract::Path("test_tenant".to_string()),
        axum::Json(payload),
    ).await;
    assert_eq!(response.into_response().status(), axum::http::StatusCode::OK);
}

#[tokio::test]
async fn test_webhook_handler_widget() {
    let payload = serde_json::json!({
        "event": "message_created",
        "data": {
            "id": 1
        }
    });
    let response = handle_widget_webhook(
        axum::extract::Path("test_tenant".to_string()),
        axum::Json(payload),
    ).await;
    assert_eq!(response.into_response().status(), axum::http::StatusCode::OK);
}
