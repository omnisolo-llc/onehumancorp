use std::sync::Arc;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::post,
    Router,
};
use tower::ServiceExt;

use crate::api::terminal::create_connection_token_handler;

#[tokio::test]
async fn test_create_connection_token() {
    let app = Router::new()
        .route("/api/v1/terminal/connection_token", post(create_connection_token_handler));

    let mut req = Request::builder()
        .method("POST")
        .uri("/api/v1/terminal/connection_token")
        .body(Body::empty())
        .unwrap();

    req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "spiffe://test.local/tenant/tenant_123".to_string(),
        org_id: "tenant_123".to_string(),
        agent_id: "agent_456".to_string(),
    });

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
