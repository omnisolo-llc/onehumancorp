use axum::{body::Body, http::Request, routing::get, Router};
use std::sync::Arc;
use tower::ServiceExt;

use crate::hub::Hub;
use crate::api::health::health_handler;

#[tokio::test]
async fn test_health_handler_success() {
    let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
    if !db_url.starts_with("sqlite") && std::env::var("OHC_DATABASE_URL").is_err() {
        return;
    }

    let _pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect_lazy("sqlite::memory:")
        .unwrap();

    let pg_pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy("postgres://dummy")
        .unwrap();

    let (tx, _) = tokio::sync::mpsc::channel(100);
    let hub = Arc::new(Hub::new(tx, pg_pool));

    let app = Router::new()
        .route("/health", get(health_handler))
        .with_state(hub);

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(body.get("mode").is_some());
    assert!(body.get("status").is_some());
    assert_eq!(body.get("status").unwrap(), "degraded"); // Since db is dummy
    assert!(body.get("db_ping").is_some());
    assert!(body.get("hybrid_mode_ready").is_some());
    assert!(body.get("mesh_active").is_some());
}
