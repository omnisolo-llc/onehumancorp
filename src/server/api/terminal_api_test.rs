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
                .uri("/connection_token")
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
                .uri("/create_payment_intent")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"amount": 1000, "currency": "usd"}"#))
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
    use crate::api::terminal_api::get_terminal_connection_token_handler;
    use axum::extract::State;
    use axum::http::HeaderMap;

    let hub = Arc::new(Hub::new());
    let auth_info = Some(axum::extract::Extension(::server_auth::orchestration::AuthInfo {
        org_id: "test_org".to_string(),
        user_id: "test_user".to_string(),
        email: "test@example.com".to_string(),
    }));

    let response = get_terminal_connection_token_handler(
        HeaderMap::new(),
        State(hub),
        auth_info,
    ).await;

    let result = response.0.unwrap();
    assert_eq!(result.secret, "tss_mock_token_for_test_org");
}

#[tokio::test]
async fn test_get_terminal_connection_token_authenticated_empty_org() {
    use crate::api::terminal_api::get_terminal_connection_token_handler;
    use axum::extract::State;
    use axum::http::HeaderMap;

    let hub = Arc::new(Hub::new());
    let auth_info = Some(axum::extract::Extension(::server_auth::orchestration::AuthInfo {
        org_id: "".to_string(),
        user_id: "test_user".to_string(),
        email: "test@example.com".to_string(),
    }));

    let response = get_terminal_connection_token_handler(
        HeaderMap::new(),
        State(hub),
        auth_info,
    ).await;

    let result = response.0.unwrap();
    assert_eq!(result.secret, "tss_mock_token_for_default");
}

#[tokio::test]
async fn test_create_payment_intent_authenticated() {
    use crate::api::terminal_api::{create_payment_intent_handler, PaymentIntentRequest};
    use axum::extract::State;
    use axum::http::HeaderMap;

    let hub = Arc::new(Hub::new());
    let auth_info = Some(axum::extract::Extension(::server_auth::orchestration::AuthInfo {
        org_id: "test_org".to_string(),
        user_id: "test_user".to_string(),
        email: "test@example.com".to_string(),
    }));

    let req_data = axum::extract::Json(PaymentIntentRequest {
        amount: 2500,
        currency: "usd".to_string(),
    });

    let response = create_payment_intent_handler(
        HeaderMap::new(),
        State(hub),
        auth_info,
        req_data,
    ).await;

    let result = response.0.unwrap();
    assert_eq!(result.client_secret, "pi_mock_intent_for_test_org_2500_usd");
}

#[tokio::test]
async fn test_create_payment_intent_authenticated_empty_org() {
    use crate::api::terminal_api::{create_payment_intent_handler, PaymentIntentRequest};
    use axum::extract::State;
    use axum::http::HeaderMap;

    let hub = Arc::new(Hub::new());
    let auth_info = Some(axum::extract::Extension(::server_auth::orchestration::AuthInfo {
        org_id: "".to_string(),
        user_id: "test_user".to_string(),
        email: "test@example.com".to_string(),
    }));

    let req_data = axum::extract::Json(PaymentIntentRequest {
        amount: 3000,
        currency: "usd".to_string(),
    });

    let response = create_payment_intent_handler(
        HeaderMap::new(),
        State(hub),
        auth_info,
        req_data,
    ).await;

    let result = response.0.unwrap();
    assert_eq!(result.client_secret, "pi_mock_intent_for_default_3000_usd");
}
