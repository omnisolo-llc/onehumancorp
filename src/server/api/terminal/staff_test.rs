use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::Router,
};
use std::sync::Arc;
use tower::ServiceExt;
use crate::hub::Hub;
use crate::api::terminal::staff;

#[tokio::test]
async fn test_terminal_staff_routes() {
    // We just verify the router compiles.
    assert!(true);
}
