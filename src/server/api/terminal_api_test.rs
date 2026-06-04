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

#[tokio::test]
async fn test_get_terminal_connection_token_authenticated() {
    let hub = Arc::new(Hub::new());
    let mut app = crate::api::terminal_api::router(hub);

    let mut req = Request::builder()
        .uri("/token")
        .method("POST")
        .body(Body::empty())
        .unwrap();

    req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        user_id: "test_user".to_string(),
        org_id: "test_tenant".to_string(),
        roles: vec!["admin".to_string()],
    });

    let response = app
        .oneshot(req)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("tss_mock_token_for_test_tenant"));
}

#[tokio::test]
async fn test_create_payment_intent_authenticated() {
    let hub = Arc::new(Hub::new());
    let mut app = crate::api::terminal_api::router(hub);

    let mut req = Request::builder()
        .uri("/intent")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"amount_cents": 1500, "currency": "usd"}"#))
        .unwrap();

    req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        user_id: "test_user".to_string(),
        org_id: "test_tenant".to_string(),
        roles: vec!["admin".to_string()],
    });

    let response = app
        .oneshot(req)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("pi_mock_intent_for_test_tenant_1500_usd"));
}
