#[cfg(test)]
mod tests {
    use crate::api::chat::webhooks::{chat_routes, IncomingWebhookPayload};
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_webhook_ingestion() {
        let app = chat_routes();

        let payload = IncomingWebhookPayload {
            channel_identifier: "testuser".to_string(),
            content: "Do you have vegan options?".to_string(),
            customer_id: None,
        };

        let request = Request::builder()
            .method("POST")
            .uri("/webhooks/chat/org1/instagram")
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&payload).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
