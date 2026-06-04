use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use std::sync::Arc;
use crate::hub::Hub;

#[tokio::test]
async fn test_get_terminal_connection_token_unauthenticated() {
    let hub = Arc::new(Hub::new());
    let app = crate::api::terminal_api::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/token")
                .method("POST")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Unauthenticated"));
}

#[tokio::test]
async fn test_get_terminal_connection_token_authenticated_mock() {
    // Tests that an authenticated request invokes the stripe client
    // Since StripeClient makes a direct network call or uses env vars in the real setup,
    // we use a mocked headers/auth mechanism.

    let hub = Arc::new(Hub::new());
    // Direct handler call for testing authenticated state
    let auth_info = Some(axum::extract::Extension(::server_auth::orchestration::AuthInfo {
        org_id: "test_tenant".to_string(),
        user_id: "test_user".to_string(),
        session_id: "test_session".to_string(),
        roles: vec![],
    }));

    let response = crate::api::terminal_api::get_terminal_connection_token_handler(
        axum::http::HeaderMap::new(),
        axum::extract::State(hub),
        auth_info,
    ).await;

    // It should succeed with the mock token format `tss_mock_token_for_{tenant_id}`
    if let Ok(res) = response.0 {
        assert_eq!(res.token, "tss_mock_token_for_test_tenant");
    } else {
        panic!("Expected Ok response");
    }
}

#[tokio::test]
async fn test_create_payment_intent_unauthenticated() {
    let hub = Arc::new(Hub::new());
    let app = crate::api::terminal_api::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/intent")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"amount_cents": 1000, "currency": "usd"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Unauthenticated"));
}
