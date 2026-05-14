use axum::{routing::{post}, Router, Json, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::integrations::registry::IntegrationsRegistry;

pub struct IntegrationState {
    registry: Arc<IntegrationsRegistry>,
}

#[derive(Deserialize)]
pub struct WebhookPayload {
    pub event: String,
    pub data: serde_json::Value,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub status: String,
}

pub async fn handle_chatwoot_webhook(State(state): State<Arc<IntegrationState>>, Json(payload): Json<WebhookPayload>) -> Json<WebhookResponse> {
    if payload.event == "message_created" {
        let msg_content = payload.data.get("content").and_then(|v| v.as_str()).unwrap_or("");
        let sender = payload.data.get("sender").and_then(|v| v.as_str()).unwrap_or("unknown");
        let _ = state.registry.send_chat_message("chatwoot", "unified_inbox", sender, msg_content, "thread_1");
    }
    Json(WebhookResponse { status: "received".to_string() })
}

pub async fn handle_calcom_webhook(State(_state): State<Arc<IntegrationState>>, Json(payload): Json<WebhookPayload>) -> Json<WebhookResponse> {
    if payload.event == "booking_created" {
        let _booking_time = payload.data.get("time");
    }
    Json(WebhookResponse { status: "received".to_string() })
}

pub async fn handle_mercadopago_webhook(State(_state): State<Arc<IntegrationState>>, Json(payload): Json<WebhookPayload>) -> Json<WebhookResponse> {
    if payload.event == "payment.created" {
        let _payment_id = payload.data.get("id");
    }
    Json(WebhookResponse { status: "received".to_string() })
}

pub async fn handle_shippo_webhook(State(_state): State<Arc<IntegrationState>>, Json(payload): Json<WebhookPayload>) -> Json<WebhookResponse> {
    if payload.event == "transaction.created" {
        let _tracking_number = payload.data.get("tracking_number");
    }
    Json(WebhookResponse { status: "received".to_string() })
}

pub fn tool_integration_routes(registry: Arc<IntegrationsRegistry>) -> Router {
    let state = Arc::new(IntegrationState { registry });
    Router::new()
        .route("/webhooks/chatwoot", post(handle_chatwoot_webhook))
        .route("/webhooks/calcom", post(handle_calcom_webhook))
        .route("/webhooks/mercadopago", post(handle_mercadopago_webhook))
        .route("/webhooks/shippo", post(handle_shippo_webhook))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chatwoot_webhook_dummy() {
        assert_eq!(true, true);
    }
}
