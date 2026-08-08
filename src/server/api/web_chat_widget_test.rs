use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::post,
    Router,
};
use std::sync::Arc;
use temp_env::with_vars;

// #[tokio::test]
// async fn test_web_chat_ingest() {
//    // Tests for Web Chat Widget API
// }
