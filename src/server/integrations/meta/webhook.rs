use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct MetaWebhookPayload {
    pub object: String,
    pub entry: Vec<MetaWebhookEntry>,
}

#[derive(Deserialize, Debug)]
pub struct MetaWebhookEntry {
    pub id: String,
    pub time: i64,
    pub messaging: Vec<MetaMessagingEvent>,
}

#[derive(Deserialize, Debug)]
pub struct MetaMessagingEvent {
    pub sender: MetaMessagingUser,
    pub recipient: MetaMessagingUser,
    pub message: Option<MetaMessageData>,
}

#[derive(Deserialize, Debug)]
pub struct MetaMessagingUser {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct MetaMessageData {
    pub mid: String,
    pub text: Option<String>,
}

pub async fn handle_meta_webhook(
    Path(tenant_id): Path<String>,
    Json(payload): Json<MetaWebhookPayload>,
) -> impl IntoResponse {
    if payload.object == "page" || payload.object == "instagram" || payload.object == "whatsapp_business_account" {
        for entry in payload.entry {
            for event in entry.messaging {
                if let Some(msg) = event.message {
                    if let Some(text) = msg.text {
                        tracing::info!("Received message on {}: {} from {}", payload.object, text, event.sender.id);
                        // Future implementation: route to unified inbox and LLM agent
                    }
                }
            }
        }
        (StatusCode::OK, "EVENT_RECEIVED").into_response()
    } else {
        (StatusCode::NOT_FOUND, "").into_response()
    }
}
