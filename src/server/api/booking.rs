use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;

#[derive(Deserialize)]
pub struct BookingRequestPayload {
    pub description: String,
    pub file_name: Option<String>,
    pub timestamp: String,
}

pub async fn booking_request_handler(
    State(db): State<Arc<DB>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<BookingRequestPayload>,
) -> impl IntoResponse {
    let tenant_id = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default");
    let user_id = headers.get("x-user-id").and_then(|v| v.to_str().ok()).unwrap_or("default");

    // Let's just create a dummy quote for this, or just return success
    // A proper implementation would invoke the operations agent or create a draft quote.

    // For now we just return a success payload as the frontend expects
    (StatusCode::OK, Json(serde_json::json!({ "status": "received", "tenant_id": tenant_id })))
}
