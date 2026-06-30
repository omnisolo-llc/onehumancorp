use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use std::sync::Arc;

use tokio::sync::mpsc;
use sqlx::postgres::PgPoolOptions;

async fn create_mock_hub() -> Arc<crate::hub::Hub> {
    let (tx, _) = mpsc::channel(100);
    // Dummy pool, won't actually connect unless needed by test body (which my_plan_unauthenticated shouldn't)
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect_lazy("postgres://postgres:postgres@localhost:5432/postgres")
        .expect("Failed to create dummy pool");
    Arc::new(crate::hub::Hub::new(tx, pool))
}

#[tokio::test]
async fn test_my_plan_unauthenticated() {
    let hub = create_mock_hub().await;
    let app = crate::api::billing_api::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/my-plan")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_create_billing_portal_session_unauthenticated() {
    let hub = create_mock_hub().await;
    let app = crate::api::billing_api::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/create-billing-portal-session")
                .method("POST")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_download_invoice_unauthenticated() {
    let hub = create_mock_hub().await;
    let app = crate::api::billing_api::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/download-invoice")
                .method("POST")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_download_invoice_authenticated_success() {
    let hub = create_mock_hub().await;
    let router = crate::api::billing_api::router(hub);
    let app = axum::Router::new()
        .merge(router)
        .route_layer(axum::middleware::from_fn(|mut req: Request<Body>, next: axum::middleware::Next| async move {
            req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
                spiffe_id: "test-spiffe".to_string(),
                org_id: "test-tenant".to_string(),
                agent_id: "test-agent".to_string(),
            });
            Ok::<axum::http::Response<axum::body::Body>, StatusCode>(next.run(req).await)
        }));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/download-invoice")
                .method("POST")
                .header("Authorization", "Bearer my-test-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Invoice download is ready"));
}

#[tokio::test]
async fn test_my_plan_authenticated_success() {
    let hub = create_mock_hub().await;
    let router = crate::api::billing_api::router(hub);
    let app = axum::Router::new()
        .merge(router)
        .route_layer(axum::middleware::from_fn(|mut req: Request<Body>, next: axum::middleware::Next| async move {
            req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
                spiffe_id: "test-spiffe".to_string(),
                org_id: "test-tenant".to_string(),
                agent_id: "test-agent".to_string(),
            });
            Ok::<axum::http::Response<axum::body::Body>, StatusCode>(next.run(req).await)
        }));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/my-plan")
                .method("GET")
                .header("Authorization", "Bearer test-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();

    // Attempt to deserialize it into `MyPlanResponse` shape to assert it successfully parses
    let parsed: serde_json::Value = serde_json::from_slice(&body).expect("Failed to parse JSON");
    assert!(parsed.get("current_plan").is_some(), "missing current_plan");
    assert!(parsed.get("ai_actions_used").is_some(), "missing ai_actions_used");
    assert!(parsed.get("storage_used_bytes").is_some(), "missing storage_used_bytes");
    assert!(parsed.get("next_bill_estimated").is_some(), "missing next_bill_estimated");
}
