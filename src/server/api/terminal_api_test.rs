use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use std::sync::Arc;
use crate::hub::Hub;
use ::server_auth::orchestration::AuthInfo;
use tower::ServiceExt;

#[tokio::test]
async fn test_get_terminal_connection_token_unauthenticated() {
    let (tx, _) = tokio::sync::mpsc::channel(1);
    let pool_options = sqlx::postgres::PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("postgres")
        .password("postgres")
        .database("ohc");
    let pool = sqlx::Pool::<sqlx::Postgres>::connect_lazy_with(pool_options);

    let hub = Arc::new(Hub::new(tx, pool));
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
    let (tx, _) = tokio::sync::mpsc::channel(1);
    let pool_options = sqlx::postgres::PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("postgres")
        .password("postgres")
        .database("ohc");
    let pool = sqlx::Pool::<sqlx::Postgres>::connect_lazy_with(pool_options);

    let hub = Arc::new(Hub::new(tx, pool));
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
    let (tx, _) = tokio::sync::mpsc::channel(1);
    let pool_options = sqlx::postgres::PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("postgres")
        .password("postgres")
        .database("ohc");
    let pool = sqlx::Pool::<sqlx::Postgres>::connect_lazy_with(pool_options);

    let hub = Arc::new(Hub::new(tx, pool));
    let mut app = axum::Router::new()
        .merge(crate::api::terminal_api::router(hub));

    let auth_info = AuthInfo {
        agent_id: "test_user".to_string(),
        org_id: "test_tenant".to_string(),
        spiffe_id: "spiffe://onehumancorp.io/test_tenant/test_user".to_string(),
    };

    app = app.layer(axum::middleware::from_fn(move |mut req: axum::extract::Request, next: axum::middleware::Next| {
        let auth_info_clone = auth_info.clone();
        async move {
            req.extensions_mut().insert(auth_info_clone);
            next.run(req).await
        }
    }));

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
    assert!(body_str.contains("Stripe API error") || body_str.contains("Unrecognized request URL") || body_str.contains("Invalid API Key provided: sk_test_123"));
}

#[tokio::test]
async fn test_create_payment_intent_authenticated_calls_stripe() {
    let (tx, _) = tokio::sync::mpsc::channel(1);
    let pool_options = sqlx::postgres::PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("postgres")
        .password("postgres")
        .database("ohc");
    let pool = sqlx::Pool::<sqlx::Postgres>::connect_lazy_with(pool_options);

    let hub = Arc::new(Hub::new(tx, pool));
    let mut app = axum::Router::new()
        .merge(crate::api::terminal_api::router(hub));

    let auth_info = AuthInfo {
        agent_id: "test_user".to_string(),
        org_id: "test_tenant".to_string(),
        spiffe_id: "spiffe://onehumancorp.io/test_tenant/test_user".to_string(),
    };

    app = app.layer(axum::middleware::from_fn(move |mut req: axum::extract::Request, next: axum::middleware::Next| {
        let auth_info_clone = auth_info.clone();
        async move {
            req.extensions_mut().insert(auth_info_clone);
            next.run(req).await
        }
    }));

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
    // It should hit Stripe and get an error since "sk_test_123" is a bad API key
    assert!(body_str.contains("Stripe API error") || body_str.contains("Unrecognized request URL") || body_str.contains("Invalid API Key provided: sk_test_123"));
}

#[tokio::test]
async fn test_get_terminal_connection_token_authenticated_default_org() {
    let (tx, _) = tokio::sync::mpsc::channel(1);
    let pool_options = sqlx::postgres::PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("postgres")
        .password("postgres")
        .database("ohc");
    let pool = sqlx::Pool::<sqlx::Postgres>::connect_lazy_with(pool_options);

    let hub = Arc::new(Hub::new(tx, pool));
    let mut app = axum::Router::new()
        .merge(crate::api::terminal_api::router(hub));

    let auth_info = AuthInfo {
        agent_id: "test_user".to_string(),
        org_id: "".to_string(),
        spiffe_id: "spiffe://onehumancorp.io/test_tenant/test_user".to_string(),
    };

    app = app.layer(axum::middleware::from_fn(move |mut req: axum::extract::Request, next: axum::middleware::Next| {
        let auth_info_clone = auth_info.clone();
        async move {
            req.extensions_mut().insert(auth_info_clone);
            next.run(req).await
        }
    }));

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
    assert!(body_str.contains("Stripe API error") || body_str.contains("Unrecognized request URL") || body_str.contains("Invalid API Key provided: sk_test_123"));
}

#[tokio::test]
async fn test_create_payment_intent_authenticated_default_org() {
    let (tx, _) = tokio::sync::mpsc::channel(1);
    let pool_options = sqlx::postgres::PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("postgres")
        .password("postgres")
        .database("ohc");
    let pool = sqlx::Pool::<sqlx::Postgres>::connect_lazy_with(pool_options);

    let hub = Arc::new(Hub::new(tx, pool));
    let mut app = axum::Router::new()
        .merge(crate::api::terminal_api::router(hub));

    let auth_info = AuthInfo {
        agent_id: "test_user".to_string(),
        org_id: "".to_string(),
        spiffe_id: "spiffe://onehumancorp.io/test_tenant/test_user".to_string(),
    };

    app = app.layer(axum::middleware::from_fn(move |mut req: axum::extract::Request, next: axum::middleware::Next| {
        let auth_info_clone = auth_info.clone();
        async move {
            req.extensions_mut().insert(auth_info_clone);
            next.run(req).await
        }
    }));

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
    // It should hit Stripe and get an error since "sk_test_123" is a bad API key
    assert!(body_str.contains("Stripe API error") || body_str.contains("Unrecognized request URL") || body_str.contains("Invalid API Key provided: sk_test_123"));
}
