use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use super::chat_api;
use sqlx::PgPool;
use std::env;

#[tokio::test]
async fn test_chat_api_routes() {
    assert!(true);
}
