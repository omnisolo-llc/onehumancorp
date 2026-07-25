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

#[derive(Deserialize, Debug)]
pub struct ChangeValue {
    pub messaging_product: String,
    pub metadata: Metadata,
    pub contacts: Option<Vec<Contact>>,
    pub messages: Option<Vec<Message>>,
    pub statuses: Option<Vec<Status>>,
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
    pub document: Option<Document>,
    pub location: Option<Location>,
    pub interactive: Option<InteractiveWebhook>,
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
}

#[derive(Deserialize, Debug)]
pub struct Document {
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
pub struct InteractiveWebhook {
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
    pub from: Option<String>,
    pub id: Option<String>,
    pub forwarded: Option<bool>,
    pub frequently_forwarded: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub id: String,
    pub status: String,
    pub timestamp: String,
    pub recipient_id: String,
    pub conversation: Option<Conversation>,
    pub pricing: Option<Pricing>,
    pub errors: Option<Vec<Error>>,
}

#[derive(Deserialize, Debug)]
pub struct Conversation {
    pub id: String,
    pub origin: Option<Origin>,
}

#[derive(Deserialize, Debug)]
pub struct Origin {
    #[serde(rename = "type")]
    pub origin_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Pricing {
    pub pricing_model: String,
    pub billable: bool,
    pub category: String,
}

#[derive(Deserialize, Debug)]
pub struct Error {
    pub code: i32,
    pub title: String,
    pub message: Option<String>,
    pub error_data: Option<ErrorData>,
}

#[derive(Deserialize, Debug)]
pub struct ErrorData {
    pub details: String,
}

use std::sync::OnceLock;

static REDIS_CLIENT: OnceLock<Option<redis::Client>> = OnceLock::new();

fn get_redis_client() -> Option<redis::Client> {
    REDIS_CLIENT.get_or_init(|| {
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        redis::Client::open(redis_url).ok()
    }).clone()
}

pub async fn handle_webhook(
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    tracing::info!("Received WhatsApp webhook: {:?}", payload);

    let client_opt = get_redis_client();
    if client_opt.is_none() {
        tracing::error!("Redis client not configured. Cannot process webhook idempotently.");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Redis not configured").into_response();
    }

    let client = client_opt.unwrap();

    let mut conn = match client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to connect to Redis: {}", e);
            // If redis is down, we must return 500 to Meta so it retries, preventing data loss
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    for entry in &payload.entry {
        for change in &entry.changes {
            if let Some(messages) = &change.value.messages {
                for message in messages {
                    let redis_key = format!("whatsapp:webhook:message:{}", message.id);

                    // SET NX to prevent double processing
                    let locked: redis::RedisResult<bool> = redis::cmd("SET")
                        .arg(&redis_key)
                        .arg("1")
                        .arg("NX")
                        .arg("EX")
                        .arg(3600)
                        .query_async(&mut conn).await;

                    match locked {
                        Ok(true) => {
                            tracing::info!("Processing new message {}", message.id);

                            // Map Meta's webhook format to internal OHC message structures
                            // For this task, we emulate processing by logging.
                            // In the actual app, this is where we'd dispatch to our internal AI job queue or chat engine.
                            let _sender = &message.from;
                            let _body = message.text.as_ref().map(|t| t.body.as_str()).unwrap_or("");

                            // Emulate processing
                        }
                        _ => {
                            tracing::info!("Skipping duplicate message {}", message.id);
                        }
                    }
                }
            }
            if let Some(statuses) = &change.value.statuses {
                for status in statuses {
                    let redis_key = format!("whatsapp:webhook:status:{}:{}", status.id, status.status);
                    let locked: redis::RedisResult<bool> = redis::cmd("SET")
                        .arg(&redis_key)
                        .arg("1")
                        .arg("NX")
                        .arg("EX")
                        .arg(3600)
                        .query_async(&mut conn).await;

                    match locked {
                        Ok(true) => {
                            tracing::info!("Processing new status {}", status.id);
                        }
                        _ => {
                            tracing::info!("Skipping duplicate status {}", status.id);
                        }
                    }
                }
            }
        }
    }

    // Send a 200 OK response to acknowledge receipt
    StatusCode::OK.into_response()
}
