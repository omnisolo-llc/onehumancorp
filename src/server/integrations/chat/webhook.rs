use axum::{
    extract::{State, Json},
    routing::post,
    Router,
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use super::service::ChatService;

#[derive(Deserialize, Debug)]
pub struct IncomingWebhookPayload {
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_phone: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub status: String,
    pub message_id: Option<Uuid>,
}

pub async fn handle_webhook(
    State(service): State<Arc<ChatService>>,
    Json(payload): Json<IncomingWebhookPayload>,
) -> impl IntoResponse {
    match service.process_incoming_message(
        payload.tenant_id,
        payload.inbox_id,
        &payload.contact_phone,
        &payload.content,
    ).await {
        Ok(message) => {
            (StatusCode::OK, Json(WebhookResponse {
                status: "success".to_string(),
                message_id: Some(message.id),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to process webhook: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse {
                status: "error".to_string(),
                message_id: None,
            }))
        }
    }
}

pub fn webhook_routes(service: Arc<ChatService>) -> Router {
    Router::new()
        .route("/webhook", post(handle_webhook))
        .with_state(service)
}
