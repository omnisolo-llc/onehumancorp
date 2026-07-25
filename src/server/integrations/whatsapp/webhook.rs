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
    pub button: Option<Button>,
    pub context: Option<Context>,
    pub errors: Option<Vec<Error>>,
}

#[derive(Deserialize, Debug)]
pub struct Text {
    pub body: String,
}





pub struct WebhookProcessor {
    redis_client: redis::Client,
}

impl WebhookProcessor {
    pub fn new(redis_client: redis::Client) -> Self {
        Self { redis_client }
    }

    pub async fn process_payload(&self, payload: WebhookPayload) -> Result<(), String> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;

        for entry in payload.entry {
            for change in entry.changes {
                if let Some(messages) = change.value.messages {
                    for message in messages {
                        let lock_key = format!("ohc:lock:whatsapp:msg:{}", message.id);

                        let acquired: bool = redis::cmd("SET")
                            .arg(&lock_key)
                            .arg("1")
                            .arg("NX")
                            .arg("EX")
                            .arg(86400) // 24 hours lock
                            .query_async(&mut conn)
                            .await
                            .unwrap_or(false);

                        if !acquired {
                            tracing::warn!("Message {} already processed, skipping", message.id);
                            continue;
                        }

                        tracing::info!("Processing message: {:?}", message);
                        // Internal OHC logic to map message to OHC omnichannel database
                    }
                }

                if let Some(statuses) = change.value.statuses {
                    for status in statuses {
                        let lock_key = format!("ohc:lock:whatsapp:status:{}:{}", status.id, status.status);

                        let acquired: bool = redis::cmd("SET")
                            .arg(&lock_key)
                            .arg("1")
                            .arg("NX")
                            .arg("EX")
                            .arg(86400)
                            .query_async(&mut conn)
                            .await
                            .unwrap_or(false);

                        if !acquired {
                            tracing::warn!("Status {} ({}) already processed, skipping", status.id, status.status);
                            continue;
                        }

                        tracing::info!("Processing status: {:?}", status);
                        // Internal OHC logic to map status to OHC omnichannel database
                    }
                }
            }
        }

        Ok(())
    }
}



use axum::extract::State;


pub async fn handle_webhook(
    State(redis_client): State<Option<redis::Client>>, // Expecting a state injection
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    tracing::info!("Received WhatsApp webhook");

    if let Some(client) = redis_client {
        let processor = WebhookProcessor::new(client);
        if let Err(e) = processor.process_payload(payload).await {
            tracing::error!("Error processing webhook: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    } else {
        tracing::warn!("No Redis client provided, processing skipped for safety");
    }

    StatusCode::OK
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
    pub expiration_timestamp: Option<String>,
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

#[derive(Deserialize, Debug)]
pub struct Context {
    pub from: Option<String>,
    pub id: Option<String>,
    pub forwarded: Option<bool>,
    pub frequently_forwarded: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Media {
    pub id: String,
    pub mime_type: Option<String>,
    pub sha256: Option<String>,
    pub caption: Option<String>,
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
    pub list_reply: Option<ListReply>,
    pub button_reply: Option<ButtonReply>,
}

#[derive(Deserialize, Debug)]
pub struct ListReply {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ButtonReply {
    pub id: String,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct Button {
    pub payload: String,
    pub text: String,
}
