use axum::{
    body::Body,
    http::{Request, StatusCode},
    Json,
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
    let auth_info = ::server_auth::orchestration::AuthInfo {
        org_id: "test_tenant".to_string(),
        user_id: "test_user".to_string(),
        roles: vec![],
    };

    let response = crate::api::terminal_api::get_terminal_connection_token_handler(
        axum::http::HeaderMap::new(),
        axum::extract::State(Arc::new(Hub::new())),
        Some(axum::extract::Extension(auth_info)),
    ).await;

    let Json(result) = response;
    assert!(result.is_ok());
    let token = result.unwrap().token;
    assert_eq!(token, "tss_mock_token_for_test_tenant");
}

#[tokio::test]
async fn test_create_payment_intent_authenticated() {
    let auth_info = ::server_auth::orchestration::AuthInfo {
        org_id: "test_tenant".to_string(),
        user_id: "test_user".to_string(),
        roles: vec![],
    };

    let req_data = crate::api::terminal_api::PaymentIntentRequest {
        amount_cents: 1000,
        currency: "usd".to_string(),
    };

    let response = crate::api::terminal_api::create_payment_intent_handler(
        axum::http::HeaderMap::new(),
        axum::extract::State(Arc::new(Hub::new())),
        Some(axum::extract::Extension(auth_info)),
        axum::extract::Json(req_data),
    ).await;

    let Json(result) = response;
    assert!(result.is_ok());
    let intent_id = result.unwrap().intent_id;
    assert_eq!(intent_id, "pi_mock_intent_for_test_tenant_1000_usd");
}
