use serde::{Deserialize, Serialize};
use axum::{Json, routing::post, Router};
use std::sync::Arc;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::db::DB;

#[derive(Debug, Deserialize, Serialize)]
pub struct InstagramWebhookPayload {
    pub object: String,
    pub entry: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub id: String,
    pub time: i64,
    pub messaging: Vec<MessagingEvent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessagingEvent {
    pub sender: Sender,
    pub recipient: Recipient,
    pub timestamp: i64,
    pub message: Message,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sender {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipient {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub mid: String,
    pub text: String,
}

#[derive(Clone)]
pub struct OmnichannelWebhookState {
    pub orchestrator: Arc<DepartmentOrchestrator>,
    pub db: Arc<DB>,
}

pub fn router(state: OmnichannelWebhookState) -> Router {
    Router::new()
        .route("/instagram", post(handle_instagram_webhook))
        .with_state(state)
}

async fn handle_instagram_webhook(
    axum::extract::State(_state): axum::extract::State<OmnichannelWebhookState>,
    Json(payload): Json<InstagramWebhookPayload>,
) -> String {
    for entry in &payload.entry {
        for messaging_event in &entry.messaging {
            println!("Received message from {}: {}", messaging_event.sender.id, messaging_event.message.text);
            normalize_and_triage(&messaging_event.sender.id, "instagram", &messaging_event.message.text);
        }
    }
    "OK".to_string()
}

pub fn normalize_and_triage(_sender_id: &str, _channel: &str, _text: &str) {
    println!("Standardized message to OmniMessage format for routing.");
    // trigger AI agent
}
