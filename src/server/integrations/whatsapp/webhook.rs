use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct VerifyQuery {
    #[serde(rename = "hub.mode")]
    pub mode: String,
    #[serde(rename = "hub.verify_token")]
    pub verify_token: String,
    #[serde(rename = "hub.challenge")]
    pub challenge: String,
}

pub async fn verify_webhook(
    Query(query): Query<VerifyQuery>,
    // In a real implementation we would inject the expected token from config
) -> impl IntoResponse {
    let expected_token = "ohc_whatsapp_webhook_secret"; // This should come from config

    if query.mode == "subscribe" && query.verify_token == expected_token {
        (StatusCode::OK, query.challenge)
    } else {
        (StatusCode::FORBIDDEN, "Forbidden".to_string())
    }
}

#[derive(Deserialize, Debug)]
pub struct WebhookPayload {
    pub object: String,
    pub entry: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
pub struct Entry {
    pub id: String,
    pub changes: Vec<Change>,
}

#[derive(Deserialize, Debug)]
pub struct Change {
    pub value: ChangeValue,
    pub field: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChangeValue {
    pub messaging_product: String,
    pub metadata: Metadata,
    pub contacts: Option<Vec<Contact>>,
    pub messages: Option<Vec<Message>>,
    pub statuses: Option<Vec<Status>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Metadata {
    pub display_phone_number: String,
    pub phone_number_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Contact {
    pub profile: Profile,
    pub wa_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    pub from: String,
    pub id: String,
    pub timestamp: String,
    pub text: Option<Text>,
    #[serde(rename = "type")]
    pub msg_type: String,

    // Attachments
    pub image: Option<Media>,
    pub video: Option<Media>,
    pub audio: Option<Media>,
    pub document: Option<Media>,
    pub sticker: Option<Media>,

    // Location
    pub location: Option<Location>,

    // Interactive
    pub interactive: Option<Interactive>,

    // Context (for replies/echoes)
    pub context: Option<Context>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Text {
    pub body: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Media {
    pub id: String,
    pub mime_type: Option<String>,
    pub sha256: Option<String>,
    pub caption: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub name: Option<String>,
    pub address: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Interactive {
    #[serde(rename = "type")]
    pub interactive_type: String,
    pub button_reply: Option<ButtonReply>,
    pub list_reply: Option<ListReply>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ButtonReply {
    pub id: String,
    pub title: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListReply {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Context {
    pub from: String,
    pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Status {
    pub id: String,
    pub status: String, // sent, delivered, read, failed
    pub timestamp: String,
    pub recipient_id: String,
    pub errors: Option<Vec<ErrorDetail>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ErrorDetail {
    pub code: i32,
    pub title: String,
    pub message: Option<String>,
    pub error_data: Option<serde_json::Value>,
}

use super::handler::WhatsAppState;
use std::sync::Arc;
use axum::extract::State;

pub async fn handle_webhook(
    State(state): State<Arc<WhatsAppState>>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    tracing::info!("Received WhatsApp webhook: {:?}", payload);

    let redis_client = &state.redis_client;

    match redis_client.get_multiplexed_async_connection().await {
        Ok(mut conn) => {
            for entry in &payload.entry {
                for change in &entry.changes {
                    // Process messages
                    if let Some(messages) = &change.value.messages {
                        for message in messages {
                            let lock_key = format!("ohc:whatsapp:webhook:msg:{}", message.id);
                            let cmd_result: redis::RedisResult<Option<String>> = redis::cmd("SET")
                                .arg(&lock_key)
                                .arg("1")
                                .arg("NX")
                                .arg("EX")
                                .arg(86400) // 1 day
                                .query_async(&mut conn)
                                .await;

                            match cmd_result {
                                Ok(Some(_)) => {
                                    tracing::info!("Processing message: {}", message.id);
                                    // TODO: Actually process the message natively
                                }
                                Ok(None) => {
                                    tracing::debug!("Skipping duplicate message webhook: {}", message.id);
                                }
                                Err(e) => {
                                    tracing::error!("Failed to acquire lock for message {}: {}", message.id, e);
                                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                                }
                            }
                        }
                    }

                    // Process statuses
                    if let Some(statuses) = &change.value.statuses {
                        for status in statuses {
                            let lock_key = format!("ohc:whatsapp:webhook:status:{}:{}", status.id, status.status);
                            let cmd_result: redis::RedisResult<Option<String>> = redis::cmd("SET")
                                .arg(&lock_key)
                                .arg("1")
                                .arg("NX")
                                .arg("EX")
                                .arg(86400) // 1 day
                                .query_async(&mut conn)
                                .await;

                            match cmd_result {
                                Ok(Some(_)) => {
                                    tracing::info!("Processing status: {} ({})", status.id, status.status);
                                    // TODO: Actually process the status natively
                                }
                                Ok(None) => {
                                    tracing::debug!("Skipping duplicate status webhook: {} ({})", status.id, status.status);
                                }
                                Err(e) => {
                                    tracing::error!("Failed to acquire lock for status {}: {}", status.id, e);
                                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            tracing::error!("Could not connect to Redis, failing webhook to allow retry: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    StatusCode::OK.into_response()
}
