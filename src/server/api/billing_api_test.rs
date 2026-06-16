use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use std::sync::Arc;

use tokio::sync::mpsc;
use sqlx::postgres::PgPoolOptions;

async fn create_mock_hub() -> Arc<crate::hub::Hub> {
    let (tx, _) = mpsc::channel(100);
    // Dummy pool, won't actually connect unless needed by test body (which my_plan_unauthenticated shouldn't)
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect_lazy("postgres://postgres:postgres@localhost:5432/postgres")
        .expect("Failed to create dummy pool");
    Arc::new(crate::hub::Hub::new(tx, pool))
}

#[tokio::test]
async fn test_my_plan_unauthenticated() {
    let hub = create_mock_hub().await;
    let app = crate::api::billing_api::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/my-plan")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_download_invoice_unauthenticated() {
    let hub = create_mock_hub().await;
    let app = crate::api::billing_api::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/download-invoice")
                .method("POST")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_download_invoice_authenticated_success() {
    let hub = create_mock_hub().await;
    let app = crate::api::billing_api::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/download-invoice")
                .method("POST")
                .header("Authorization", "Bearer my-test-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Invoice download is ready"));
}
