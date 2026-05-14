use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ResendBroadcastPayload {
    pub audience: String,
    pub prompt: String,
}

#[derive(Debug, Serialize)]
pub struct ResendBroadcastResponse {
    pub status: String,
}

pub async fn handle_resend_broadcast(
    Json(payload): Json<ResendBroadcastPayload>,
) -> Json<ResendBroadcastResponse> {
    tracing::info!("Received Resend broadcast request: {:?}", payload);

    // In a real implementation, we would use the "Marketing" AI agent to draft an
    // email based on the prompt and send it to the specified audience via Resend API.

    Json(ResendBroadcastResponse {
        status: "ok".to_string(),
    })
}
