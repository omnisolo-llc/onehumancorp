use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CalComSyncPayload {
    pub user_id: String,
    pub availability_hours: String,
}

#[derive(Debug, Serialize)]
pub struct CalComSyncResponse {
    pub status: String,
}

pub async fn handle_calcom_sync(
    Json(payload): Json<CalComSyncPayload>,
) -> Json<CalComSyncResponse> {
    tracing::info!("Received Cal.com sync request: {:?}", payload);

    // In a real implementation, we would update the user's availability schedule
    // and sync it with connected external calendars.

    Json(CalComSyncResponse {
        status: "ok".to_string(),
    })
}
