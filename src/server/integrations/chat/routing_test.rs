use axum::{body::Body, http::Request, routing::get, Router};
use tower::ServiceExt;
use super::routing;

#[tokio::test]
async fn test_routing() {
    let state = routing::ChatState {};
    let app = routing::router(state);

    // Test that the endpoint is reachable (but returns NOT_IMPLEMENTED for now as we set up err stub)
    let response = app
        .oneshot(
            Request::builder()
                .uri("/inboxes/00000000-0000-0000-0000-000000000000")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::NOT_IMPLEMENTED);
}
