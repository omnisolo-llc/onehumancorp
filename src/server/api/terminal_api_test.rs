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

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

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

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Unauthenticated"));
}

#[tokio::test]
async fn test_get_terminal_connection_token_authenticated() {
    let hub = Arc::new(Hub::new());
    let app_with_auth = axum::Router::new()
        .route("/token", axum::routing::post(crate::api::terminal_api::get_terminal_connection_token_handler))
        .with_state(hub)
        .layer(axum::extract::Extension(::server_auth::orchestration::AuthInfo {
            org_id: "test_tenant".to_string(),
            ..Default::default()
        }));

    let req = Request::builder()
        .uri("/token")
        .method("POST")
        .body(Body::empty())
        .unwrap();

    let response = app_with_auth
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
    let app_with_auth = axum::Router::new()
        .route("/intent", axum::routing::post(crate::api::terminal_api::create_payment_intent_handler))
        .with_state(hub)
        .layer(axum::extract::Extension(::server_auth::orchestration::AuthInfo {
            org_id: "test_tenant".to_string(),
            ..Default::default()
        }));

    let req = Request::builder()
        .uri("/intent")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"amount_cents": 1500, "currency": "usd"}"#))
        .unwrap();

    let response = app_with_auth
        .oneshot(req)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("pi_mock_intent_for_test_tenant_1500_usd"));
}

#[tokio::test]
async fn test_create_payment_intent_with_product_ids_acquires_lock() {
    // Tests that a RedisLock is acquired when product_ids are present
    let hub = Arc::new(Hub::new());
    let app_with_auth = axum::Router::new()
        .route("/intent", axum::routing::post(crate::api::terminal_api::create_payment_intent_handler))
        .with_state(hub)
        .layer(axum::extract::Extension(::server_auth::orchestration::AuthInfo {
            org_id: "test_tenant_lock".to_string(),
            ..Default::default()
        }));

    // Generate a unique product ID for this test
    let product_id = format!("prod_{}", uuid::Uuid::new_v4());

    // First, let's create a payment intent which should lock the product
    let req = Request::builder()
        .uri("/intent")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(format!(r#"{{"amount_cents": 1500, "currency": "usd", "product_ids": ["{}"]}}"#, product_id)))
        .unwrap();

    let response = app_with_auth.clone()
        .oneshot(req)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    // Redis logic will lock it successfully. Wait, since RedisLock uses NX,
    // the endpoint itself acquires it.
    assert!(body_str.contains("pi_mock_intent_for_test_tenant_lock_1500_usd"));

    // If we try again while locked, it should return Out of stock.
    let req2 = Request::builder()
        .uri("/intent")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(format!(r#"{{"amount_cents": 1500, "currency": "usd", "product_ids": ["{}"]}}"#, product_id)))
        .unwrap();

    let response2 = app_with_auth
        .oneshot(req2)
        .await
        .unwrap();

    // Check if we handle missing or failing redis properly when no REDIS_URL is present, or if it succeeds.
    // If redis is running, it will return "Out of stock". If not, it falls back to success and warns.
    if std::env::var("REDIS_URL").is_ok() {
         assert_eq!(response2.status(), StatusCode::BAD_REQUEST);
         let body2 = axum::body::to_bytes(response2.into_body(), usize::MAX).await.unwrap();
         let body_str2 = String::from_utf8(body2.to_vec()).unwrap();
         assert!(body_str2.contains("Out of stock"));
    }
}
