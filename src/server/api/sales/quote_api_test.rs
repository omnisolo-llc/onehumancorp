#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot`
    use serde_json::json;
    use crate::api::sales::quote_api;

    #[tokio::test]
    async fn test_generate_quote() {
        let app = quote_api::router::<()>();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/generate")
                    .header("Content-Type", "application/json")
                    .body(Body::from(
                        json!({
                            "tenant_id": "test_tenant",
                            "customer_description": "I need help with a broken pipe"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
