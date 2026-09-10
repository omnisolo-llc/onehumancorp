use axum::{
    extract::{State, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use super::models::WhatsAppWebhookPayload;

#[derive(Deserialize)]
pub struct WebhookVerifyQuery {
    #[serde(rename = "hub.mode")]
    pub mode: Option<String>,
    #[serde(rename = "hub.verify_token")]
    pub verify_token: Option<String>,
    #[serde(rename = "hub.challenge")]
    pub challenge: Option<String>,
}

pub async fn verify_webhook(
    Query(query): Query<WebhookVerifyQuery>,
) -> impl IntoResponse {
    // In a real app, verify_token would be loaded from config/env
    let expected_token = "omnisolo_whatsapp_verify_token";

    if let (Some(mode), Some(token), Some(challenge)) = (query.mode, query.verify_token, query.challenge) {
        if mode == "subscribe" && token == expected_token {
            return (StatusCode::OK, challenge).into_response();
        }
    }
    StatusCode::FORBIDDEN.into_response()
}

pub async fn handle_webhook(
    Json(payload): Json<WhatsAppWebhookPayload>,
) -> impl IntoResponse {
    // Process incoming messages and statuses here
    // Example: route to SSE/WebSocket engine based on phone_number_id mapping to tenant_id
    StatusCode::OK.into_response()
}

pub fn webhook_router() -> Router {
    Router::new()
        .route("/api/webhooks/whatsapp", get(verify_webhook).post(handle_webhook))
}
