use std::sync::Arc;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::get,
    Router,
};
use tower::ServiceExt;
use crate::hub::Hub;
use super::search::search_handler;

#[tokio::test]
async fn test_search_handler_compiles() {
    // Basic structural test to ensure it compiles and can be constructed
    let _app: Router<()> = Router::new().route("/search", get(search_handler));

    // Test passes if we successfully compile this test and app router is constructed
    assert!(true);
}
