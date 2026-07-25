use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Debug)]
pub struct ChangeValue {
    pub messaging_product: String,
    pub metadata: Metadata,
    pub contacts: Option<Vec<Contact>>,
    pub messages: Option<Vec<Message>>,
    pub statuses: Option<Vec<Status>>,
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub id: String,
    pub status: String,
    pub timestamp: String,
    pub recipient_id: String,
    pub errors: Option<Vec<ErrorDetail>>,
}

#[derive(Deserialize, Debug)]
pub struct ErrorDetail {
    pub code: i32,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub display_phone_number: String,
    pub phone_number_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Contact {
    pub profile: Profile,
    pub wa_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub from: String,
    pub id: String,
    pub timestamp: String,
    pub text: Option<Text>,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub image: Option<Media>,
    pub video: Option<Media>,
    pub audio: Option<Media>,
    pub document: Option<Media>,
    pub location: Option<Location>,
    pub interactive: Option<Interactive>,
    pub context: Option<Context>,
}

#[derive(Deserialize, Debug)]
pub struct Text {
    pub body: String,
}

#[derive(Deserialize, Debug)]
pub struct Media {
    pub id: String,
    pub mime_type: Option<String>,
    pub sha256: Option<String>,
    pub caption: Option<String>,
    pub filename: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub name: Option<String>,
    pub address: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Interactive {
    #[serde(rename = "type")]
    pub interactive_type: String,
    pub button_reply: Option<ButtonReply>,
    pub list_reply: Option<ListReply>,
}

#[derive(Deserialize, Debug)]
pub struct ButtonReply {
    pub id: String,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct ListReply {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Context {
    pub from: String,
    pub id: String,
    pub forwarded: Option<bool>,
    pub frequently_forwarded: Option<bool>,
}

pub async fn handle_webhook(
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    tracing::info!("Received WhatsApp webhook: {:?}", payload);

    // In order to not depend on a specific crate which causes build errors, we'll try to get it directly from REDIS_URL
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let mut redis_conn = if let Ok(client) = redis::Client::open(redis_url) {
        match client.get_connection() {
            Ok(conn) => Some(conn),
            Err(e) => {
                tracing::error!("Failed to get redis connection: {}", e);
                None
            }
        }
    } else {
        None
    };

    for entry in &payload.entry {
        for change in &entry.changes {
            if change.field == "messages" {
                if let Some(messages) = &change.value.messages {
                    for message in messages {
                        let mut should_process = true;
                        if let Some(ref mut conn) = redis_conn {
                            let lock_key = format!("ohc:lock:whatsapp:message:{}", message.id);
                            let acquired: Result<bool, redis::RedisError> = redis::cmd("SET")
                                .arg(&lock_key)
                                .arg("1")
                                .arg("NX")
                                .arg("EX")
                                .arg(86400) // 1 day ttl
                                .query(conn);

                            if let Ok(acquired_lock) = acquired {
                                if !acquired_lock {
                                    tracing::info!("Skipping duplicated message: {}", message.id);
                                    should_process = false;
                                }
                            } else {
                                tracing::error!("Failed to acquire lock for message: {}", message.id);
                                // Fail open is bad for idempotency. Fail closed.
                                should_process = false;
                            }
                        } else {
                            // If redis is down, we must fail closed to guarantee idempotency.
                            tracing::error!("Redis is unavailable, failing closed for message: {}", message.id);
                            should_process = false;
                        }

                        if should_process {
                            // TODO: Dispatch to the job queue, persist to omnichannel database
                            tracing::info!("Processing message: {}", message.id);
                        }
                    }
                }

                if let Some(statuses) = &change.value.statuses {
                    for status in statuses {
                        let mut should_process = true;
                        if let Some(ref mut conn) = redis_conn {
                            let lock_key = format!("ohc:lock:whatsapp:status:{}:{}", status.id, status.status);
                            let acquired: Result<bool, redis::RedisError> = redis::cmd("SET")
                                .arg(&lock_key)
                                .arg("1")
                                .arg("NX")
                                .arg("EX")
                                .arg(86400) // 1 day ttl
                                .query(conn);

                            if let Ok(acquired_lock) = acquired {
                                if !acquired_lock {
                                    tracing::info!("Skipping duplicated status: {} ({})", status.id, status.status);
                                    should_process = false;
                                }
                            } else {
                                tracing::error!("Failed to acquire lock for status: {}", status.id);
                                should_process = false;
                            }
                        } else {
                             tracing::error!("Redis is unavailable, failing closed for status: {}", status.id);
                             should_process = false;
                        }

                        if should_process {
                            // TODO: Dispatch to job queue, persist status update
                            tracing::info!("Processing status: {} ({})", status.id, status.status);
                        }
                    }
                }
            }
        }
    }

    // Send a 200 OK response to acknowledge receipt
    StatusCode::OK
}
