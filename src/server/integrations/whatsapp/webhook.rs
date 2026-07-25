use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use redis::AsyncCommands;
use std::sync::OnceLock;

static REDIS_CLIENT: OnceLock<Option<redis::Client>> = OnceLock::new();

// In a real app we'd inject this via config or state, but for the integration
// we'll use a OnceLock to hold the Redis client and prevent connection exhaustion.
pub async fn get_redis_conn() -> Option<redis::aio::MultiplexedConnection> {
    let client_opt = REDIS_CLIENT.get_or_init(|| {
        if let Ok(url) = std::env::var("REDIS_URL") {
            redis::Client::open(url).ok()
        } else {
            None
        }
    });

    if let Some(client) = client_opt {
        client.get_multiplexed_tokio_connection().await.ok()
    } else {
        None
    }
}

#[derive(Debug, PartialEq)]
pub struct InternalMessage {
    pub provider: String,
    pub message_id: String,
    pub from: String,
    pub to: String,
    pub timestamp: String,
    pub content: String,
    pub media_id: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct InternalStatus {
    pub provider: String,
    pub message_id: String,
    pub status: String,
    pub timestamp: String,
    pub recipient_id: String,
}

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
) -> impl IntoResponse {
    let expected_token = "ohc_whatsapp_webhook_secret"; // This should come from config

    if query.mode == "subscribe" && query.verify_token == expected_token {
        (StatusCode::OK, query.challenge)
    } else {
        (StatusCode::FORBIDDEN, "Forbidden".to_string())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WebhookPayload {
    pub object: String,
    pub entry: Vec<Entry>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Entry {
    pub id: String,
    pub changes: Vec<Change>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Change {
    pub value: ChangeValue,
    pub field: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChangeValue {
    pub messaging_product: String,
    pub metadata: Metadata,
    pub contacts: Option<Vec<Contact>>,
    pub messages: Option<Vec<Message>>,
    pub statuses: Option<Vec<Status>>,
    pub errors: Option<Vec<Error>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Metadata {
    pub display_phone_number: String,
    pub phone_number_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Contact {
    pub profile: Profile,
    pub wa_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Profile {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub from: String,
    pub id: String,
    pub timestamp: String,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub text: Option<Text>,
    pub image: Option<Media>,
    pub video: Option<Media>,
    pub audio: Option<Media>,
    pub document: Option<Media>,
    pub sticker: Option<Media>,
    pub location: Option<Location>,
    pub interactive: Option<Interactive>,
    pub context: Option<Context>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Status {
    pub id: String,
    pub status: String,
    pub timestamp: String,
    pub recipient_id: String,
    pub errors: Option<Vec<Error>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Error {
    pub code: i32,
    pub title: String,
    pub message: Option<String>,
    pub error_data: Option<ErrorData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorData {
    pub details: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Text {
    pub body: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Media {
    pub id: String,
    pub mime_type: Option<String>,
    pub sha256: Option<String>,
    pub caption: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub name: Option<String>,
    pub address: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Interactive {
    #[serde(rename = "type")]
    pub interactive_type: String,
    pub button_reply: Option<ButtonReply>,
    pub list_reply: Option<ListReply>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ButtonReply {
    pub id: String,
    pub title: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListReply {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Context {
    pub from: Option<String>,
    pub id: Option<String>,
    pub forwarded: Option<bool>,
    pub frequently_forwarded: Option<bool>,
}

pub async fn handle_webhook(
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    tracing::info!("Received WhatsApp webhook: {:?}", payload);

    let mut redis_conn = get_redis_conn().await;
    let mut internal_messages = Vec::new();
    let mut internal_statuses = Vec::new();

    for entry in &payload.entry {
        for change in &entry.changes {
            let phone_number_id = &change.value.metadata.phone_number_id;

            if let Some(messages) = &change.value.messages {
                for message in messages {
                    if let Some(conn) = &mut redis_conn {
                        let lock_key = format!("whatsapp_msg_lock:{}", message.id);
                        // Try to set lock with EX 86400 (1 day) NX
                        let acquired: bool = redis::cmd("SET")
                            .arg(&lock_key)
                            .arg("1")
                            .arg("NX")
                            .arg("EX")
                            .arg(86400)
                            .query_async(conn)
                            .await
                            .unwrap_or(false);

                        if !acquired {
                            tracing::info!("Duplicate message detected (id: {}), skipping", message.id);
                            continue;
                        }
                    }

                    let content = if let Some(text) = &message.text {
                        text.body.clone()
                    } else if message.image.is_some() {
                        "[Image]".to_string()
                    } else if message.document.is_some() {
                        "[Document]".to_string()
                    } else if message.video.is_some() {
                        "[Video]".to_string()
                    } else if message.audio.is_some() {
                        "[Audio]".to_string()
                    } else if message.interactive.is_some() {
                        "[Interactive]".to_string()
                    } else {
                        "[Unsupported Message]".to_string()
                    };

                    let media_id = message.image.as_ref()
                        .or(message.document.as_ref())
                        .or(message.video.as_ref())
                        .or(message.audio.as_ref())
                        .map(|m| m.id.clone());

                    let internal_msg = InternalMessage {
                        provider: "whatsapp".to_string(),
                        message_id: message.id.clone(),
                        from: message.from.clone(),
                        to: phone_number_id.clone(),
                        timestamp: message.timestamp.clone(),
                        content,
                        media_id,
                    };

                    tracing::info!("Mapped internal message: {:?}", internal_msg);
                    internal_messages.push(internal_msg);
                }
            }
            if let Some(statuses) = &change.value.statuses {
                for status in statuses {
                    if let Some(conn) = &mut redis_conn {
                        // For statuses, the ID is the message ID, but we can receive multiple statuses for the same message
                        // so we lock on id + status
                        let lock_key = format!("whatsapp_status_lock:{}:{}", status.id, status.status);
                        let acquired: bool = redis::cmd("SET")
                            .arg(&lock_key)
                            .arg("1")
                            .arg("NX")
                            .arg("EX")
                            .arg(86400)
                            .query_async(conn)
                            .await
                            .unwrap_or(false);

                        if !acquired {
                            tracing::info!("Duplicate status detected (id: {}, status: {}), skipping", status.id, status.status);
                            continue;
                        }
                    }

                    let internal_status = InternalStatus {
                        provider: "whatsapp".to_string(),
                        message_id: status.id.clone(),
                        status: status.status.clone(),
                        timestamp: status.timestamp.clone(),
                        recipient_id: status.recipient_id.clone(),
                    };

                    tracing::info!("Mapped internal status: {:?}", internal_status);
                    internal_statuses.push(internal_status);
                }
            }
        }
    }

    // In a full application, internal_messages and internal_statuses would be published
    // to a message bus or processed by a dedicated service here.

    StatusCode::OK
}
