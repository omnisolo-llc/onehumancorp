use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use std::{sync::Arc, time::Duration};
use tower::ServiceExt;
async fn app() -> axum::Router {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(50))
        .connect_lazy("postgres://fixture@127.0.0.1:1/ohc_field_test")
        .unwrap();
    crate::actual_mount(
        Arc::new(crate::db::DB { pool }),
        Arc::new(server_auth::Store::new()),
    )
    .await
}
async fn unsigned(method: &str, path: &str, body: serde_json::Value) -> StatusCode {
    let response = app()
        .await
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path)
                .header("content-type", "application/json")
                .header("x-tenant-id", "e2e-tenant")
                .header(
                    "x-spiffe-id",
                    "spiffe://omnisolo/tenant/e2e-tenant/agent/owner",
                )
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    response.status()
}
#[tokio::test]
async fn unsigned_appointment_update_is_rejected_before_storage() {
    assert_eq!(
        unsigned(
            "POST",
            "/api/v1/field-ops/appointments",
            json!({"id":"foreign","status":"Completed"})
        )
        .await,
        StatusCode::UNAUTHORIZED
    );
}
#[tokio::test]
async fn unsigned_optimizer_is_rejected_before_storage() {
    assert_eq!(
        unsigned(
            "POST",
            "/api/v1/field-ops/optimize-route",
            json!({"appointments":[]})
        )
        .await,
        StatusCode::UNAUTHORIZED
    );
}
#[tokio::test]
async fn unsigned_delay_preview_is_rejected() {
    assert_eq!(
        unsigned(
            "POST",
            "/api/v1/field-ops/running-late",
            json!({"appointments":[],"delayJobId":"foreign"})
        )
        .await,
        StatusCode::UNAUTHORIZED
    );
}
#[tokio::test]
async fn unsigned_route_read_is_rejected_before_cache_or_storage() {
    assert_eq!(
        unsigned(
            "GET",
            "/api/v1/field-service-routing/routes/today",
            json!(null)
        )
        .await,
        StatusCode::UNAUTHORIZED
    );
}
#[tokio::test]
async fn unsigned_job_update_is_rejected_before_storage() {
    assert_eq!(
        unsigned(
            "POST",
            "/api/v1/field-service-routing/jobs/foreign/status",
            json!({"status":"done"})
        )
        .await,
        StatusCode::UNAUTHORIZED
    );
}
#[path = "postgres_test.rs"]
mod postgres;
