use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use std::sync::Arc;
use crate::hub::Hub;

#[tokio::test]
async fn test_create_cart_unauthenticated() {
    let hub = Arc::new(Hub::new());
    let app = crate::api::cart::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"channel": "in_store"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Unauthenticated"));
}

#[tokio::test]
async fn test_add_item_unauthenticated() {
    let hub = Arc::new(Hub::new());
    let app = crate::api::cart::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/cart_1/items")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"product_id": "prod_1", "quantity": 1}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Unauthenticated"));
}

#[tokio::test]
async fn test_checkout_cart_unauthenticated() {
    let hub = Arc::new(Hub::new());
    let app = crate::api::cart::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/cart_1/checkout")
                .method("POST")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Unauthenticated"));
}

#[tokio::test]
async fn test_create_cart_authenticated_missing_tenant() {
    let hub = Arc::new(Hub::new());

    let app_with_auth = axum::Router::new()
        .nest("/cart", crate::api::cart::router(hub.clone()))
        .layer(axum::extract::Extension(::server_auth::orchestration::AuthInfo {
            org_id: "".to_string(), // Missing tenant ID
            ..Default::default()
        }));

    let response = app_with_auth
        .oneshot(
            Request::builder()
                .uri("/cart/")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"channel": "in_store"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Missing tenant ID"));
}

#[tokio::test]
async fn test_process_payment_unauthenticated() {
    let hub = Arc::new(Hub::new());
    let app = crate::api::cart::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/cart_1/process_payment")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"client_secret": "pi_123_secret_abc", "amount_cents": 1000}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Unauthenticated"));
}

#[tokio::test]
async fn test_create_cart_authenticated() {
    let hub = Arc::new(Hub::new());

    let app_with_auth = axum::Router::new()
        .nest("/cart", crate::api::cart::router(hub.clone()))
        .layer(axum::extract::Extension(::server_auth::orchestration::AuthInfo {
            org_id: "test_tenant".to_string(), // Set tenant ID
            ..Default::default()
        }));

    let response = app_with_auth
        .oneshot(
            Request::builder()
                .uri("/cart/")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"channel": "in_store"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    // Since db::get_pool() is used inside the handler without an active postgres DB in unit tests,
    // we expect this to fail.
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Failed to start transaction")); // Mock PG not running
}
