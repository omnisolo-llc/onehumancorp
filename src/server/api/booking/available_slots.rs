use axum::{
    extract::{State, Json, Path},
    response::IntoResponse,
    http::StatusCode,
    routing::get,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::db::Db;

#[derive(Serialize)]
pub struct AvailableSlotsResponse {
    pub slots: Vec<Slot>,
}

#[derive(Serialize)]
pub struct Slot {
    pub id: String,
    pub start_time: String,
    pub end_time: String,
}

pub fn router<S>(db: Arc<Db>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/:service_id", get(handle_get_available_slots))
        .with_state(db)
}

async fn handle_get_available_slots(
    State(db): State<Arc<Db>>,
    headers: axum::http::HeaderMap,
    Path(service_id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (axum::http::StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let slots = match db.store.query_available_slots(&tenant_id, &service_id).await {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Failed to query available slots: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to fetch available slots"})),
            )
                .into_response();
        }
    };

    let mut response_slots = Vec::new();
    for slot in slots {
        response_slots.push(Slot {
            id: slot.id,
            start_time: slot.start_time.to_rfc3339(),
            end_time: slot.end_time.to_rfc3339(),
        });
    }

    (
        StatusCode::OK,
        Json(AvailableSlotsResponse { slots: response_slots }),
    ).into_response()
}
