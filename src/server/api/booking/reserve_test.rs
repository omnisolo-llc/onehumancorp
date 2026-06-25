use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use std::sync::Arc;
use server_lib::db::{DB, DbStore};
use server_lib::api::booking::reserve;

#[tokio::test]
async fn test_reserve_booking_missing_tenant() {
    let pool = sqlx::PgPool::connect_lazy("postgres://dummy").unwrap();
    let db = Arc::new(DB { pool, store: DbStore::Postgres });
    let app = reserve::router(db);

    let payload = serde_json::json!({
        "service_id": "service-1",
        "start_time": "2025-01-01T10:00:00Z",
        "end_time": "2025-01-01T11:00:00Z"
    });

    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .method("POST")
                .header("Content-Type", "application/json")
                // Missing x-tenant-id header
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
