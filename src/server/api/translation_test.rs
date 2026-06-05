use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::post,
    Router,
};
use std::sync::Arc;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;
use super::translation::{get_translation, TranslationRequest};

#[tokio::test]
async fn test_get_translation() {
    let _pool = PgPoolOptions::new()
        .connect("postgres://postgres:postgres@localhost:5432/postgres")
        .await
        .unwrap();

    let _pool = Arc::new(_pool);
    // Real tests depend on auth mocks
}
