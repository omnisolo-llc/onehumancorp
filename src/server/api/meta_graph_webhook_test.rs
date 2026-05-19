#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::post,
        Router,
    };
    use tower::ServiceExt;
    use crate::api::meta_graph_webhook::{meta_graph_webhook_handler, MetaGraphWebhookState};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_meta_webhook_handler() {
        // Mock DB is complex due to the state, so we'll test the handler logic
        // in a simplified environment. This test ensures the route can accept the JSON payload.
        let state = MetaGraphWebhookState {
            db: Arc::new(crate::db::DB::new_in_memory().await.unwrap()),
        };

        let app = Router::new()
            .route("/webhook", post(meta_graph_webhook_handler))
            .with_state(state);

        let payload = serde_json::json!({
            "object": "page",
            "entry": [{
                "id": "PAGE_ID",
                "time": 1234567890,
                "messaging": [{
                    "sender": {"id": "SENDER_ID"},
                    "recipient": {"id": "PAGE_ID"},
                    "timestamp": 1234567890,
                    "message": {
                        "mid": "mid.1234",
                        "text": "Hello, do you have vegan cakes?"
                    }
                }]
            }]
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/webhook")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
