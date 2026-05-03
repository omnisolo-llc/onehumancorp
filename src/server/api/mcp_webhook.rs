use axum::{
    extract::{State, Path},
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
    Json,
    routing::post,
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use crate::integrations::mcp::async_task_tracker::AsyncTaskTracker;
use crate::db::DbStore;
use crate::db::DB;

#[derive(Deserialize)]
pub struct WebhookPayload {
    pub payload: String,
}

#[derive(Clone)]
pub struct WebhookState {
    pub db_pool: Arc<DB>,
    pub secret: String,
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

pub async fn mcp_webhook_handler(
    headers: HeaderMap,
    State(state): State<WebhookState>,
    Path(task_id): Path<String>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok());
    let expected = format!("Bearer {}", state.secret);

    if let Some(header_val) = auth_header {
        if !constant_time_eq(header_val.as_bytes(), expected.as_bytes()) {
            return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
        }
    } else {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    let tracker = match &state.db_pool.store {
        DbStore::Postgres => AsyncTaskTracker::new_postgres(state.db_pool.pool.clone()),
        DbStore::Sqlite(sqlite_pool) => AsyncTaskTracker::new_sqlite(sqlite_pool.clone()),
    };

    match tracker.complete_task(&task_id, &payload.payload).await {
        Ok(_) => (StatusCode::OK, "Task completed").into_response(),
        Err(e) => {
            eprintln!("Failed to complete task: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to complete task").into_response()
        }
    }
}

pub fn router<S: Clone + Send + Sync + 'static>(db_pool: Arc<DB>) -> Router<S> {
    let secret = std::env::var("MCP_WEBHOOK_SECRET").expect("MCP_WEBHOOK_SECRET must be set");
    let state = WebhookState { db_pool, secret };
    Router::new()
        .route("/:task_id", post(mcp_webhook_handler))
        .with_state(state)
}
