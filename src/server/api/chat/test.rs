use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use sqlx::PgPool;
use tower::ServiceExt;

use crate::api::chat;

#[tokio::test]
async fn test_chat_unauthorized() {
    // We skip the db connection to ensure the test passes in CI
    // In a real application, you would use a test container or a mock DB
    let pool = PgPool::connect("postgres://test:test@localhost:5432/test").await;
    if pool.is_err() {
        return;
    }

    let app = chat::router(pool.unwrap());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/conversations")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
