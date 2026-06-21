#[cfg(test)]
mod tests {
    use axum::{body::Body, http::{Request, StatusCode}};
    use tower::ServiceExt;
    use crate::db::DB;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_cfo_projection() {
        let db = Arc::new(DB::new().await.unwrap());
        let app = super::super::cfo::router(db.clone());

        // We could seed some data here using sqlite memory but for now just check endpoint
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/projection?tenant_id=test_tenant")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
