use axum::{body::Body, http::{Request, StatusCode}};
use tower::ServiceExt;
use sqlx::PgPool;
use super::sync_crdt_handler::{handle_crdt_sync, CrdtSyncRequest, CrdtDelta};
use serde_json::json;

#[tokio::test]
async fn test_handle_crdt_sync() {
    let pool_opts = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1);
    let pool = match pool_opts.connect("postgres://postgres:postgres@localhost:5432/test").await {
        Ok(pool) => pool,
        Err(_) => return, // Skip if db not available
    };

    // Ensure table exists for test
    sqlx::query("CREATE TABLE IF NOT EXISTS crdt_deltas (id TEXT PRIMARY KEY, entity_id TEXT NOT NULL, data TEXT NOT NULL, updated_at TIMESTAMPTZ NOT NULL)")
        .execute(&pool)
        .await
        .unwrap();

    let app = axum::Router::new()
        .route("/api/v1/sync/mcp-deltas", axum::routing::post(handle_crdt_sync))
        .with_state(pool);

    let req_body = json!({
        "deltas": [
            {
                "id": "delta_123",
                "entity_id": "task_abc",
                "data": "{\"status\": \"completed\"}",
                "updated_at": "2026-04-17T12:00:00Z"
            }
        ]
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/sync/mcp-deltas")
        .header("content-type", "application/json")
        .body(Body::from(req_body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
