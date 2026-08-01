use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct ChatProvider {
    pub tenant_id: String,
    pub channel_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub object: String,
    pub entry: Vec<serde_json::Value>,
}

pub async fn handle_whatsapp_webhook(
    Path(tenant_id): Path<String>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    tracing::info!("Received WhatsApp webhook for tenant {}: {:?}", tenant_id, payload);
    StatusCode::OK
}

pub async fn handle_widget_webhook(
    Path(tenant_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    tracing::info!("Received Web Widget webhook for tenant {}: {:?}", tenant_id, payload);
    StatusCode::OK
}

#[cfg(test)]
mod chat_unit_test;
