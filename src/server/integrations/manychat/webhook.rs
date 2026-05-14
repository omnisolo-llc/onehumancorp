use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ManyChatWebhookPayload {
    pub message: String,
    pub recipient_id: String,
}

#[derive(Debug, Serialize)]
pub struct ManyChatWebhookResponse {
    pub status: String,
}

pub async fn handle_manychat_webhook(
    Json(payload): Json<ManyChatWebhookPayload>,
) -> Json<ManyChatWebhookResponse> {
    tracing::info!("Received ManyChat webhook: {:?}", payload);

    // In a real implementation, we would route this message to the Unified Inbox
    // and trigger the "Customer Success" AI agent.

    Json(ManyChatWebhookResponse {
        status: "ok".to_string(),
    })
}
