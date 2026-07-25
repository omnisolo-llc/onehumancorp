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
) -> impl IntoResponse {
    let expected_token = "ohc_whatsapp_webhook_secret";

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

#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    pub from: String,
    pub id: String,
    pub timestamp: String,
    pub text: Option<Text>,
    pub image: Option<Media>,
    pub video: Option<Media>,
    pub audio: Option<Media>,
    pub document: Option<Media>,
    pub location: Option<Location>,
    pub contacts: Option<Vec<WebhookContact>>,
    pub button: Option<ButtonReply>,
    pub interactive: Option<InteractiveReply>,
    pub context: Option<Context>,
    #[serde(rename = "type")]
    pub msg_type: String,
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
    pub filename: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub name: Option<String>,
    pub address: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WebhookContact {
    pub name: Option<ContactName>,
    pub phones: Option<Vec<ContactPhone>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContactName {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub formatted_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContactPhone {
    pub phone: Option<String>,
    #[serde(rename = "type")]
    pub phone_type: Option<String>,
    pub wa_id: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ButtonReply {
    pub text: String,
    pub payload: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InteractiveReply {
    #[serde(rename = "type")]
    pub interactive_type: String,
    pub button_reply: Option<ButtonReply>,
    pub list_reply: Option<ListReply>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListReply {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Context {
    pub from: Option<String>,
    pub id: Option<String>,
    pub forwarded: Option<bool>,
    pub frequently_forwarded: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Status {
    pub id: String,
    pub status: String,
    pub timestamp: String,
    pub recipient_id: String,
    pub conversation: Option<Conversation>,
    pub pricing: Option<Pricing>,
    pub errors: Option<Vec<ErrorObj>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Conversation {
    pub id: String,
    pub origin: Origin,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Origin {
    #[serde(rename = "type")]
    pub origin_type: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Pricing {
    pub billable: Option<bool>,
    pub pricing_model: String,
    pub category: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ErrorObj {
    pub code: i32,
    pub title: String,
    pub message: Option<String>,
    pub error_data: Option<ErrorData>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ErrorData {
    pub details: String,
}

static PROCESSED_MESSAGES: std::sync::OnceLock<std::sync::Mutex<std::collections::HashSet<String>>> = std::sync::OnceLock::new();

pub async fn handle_webhook(
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    tracing::info!("Received WhatsApp webhook: {:?}", payload);

    for entry in &payload.entry {
        for change in &entry.changes {
            if let Some(messages) = &change.value.messages {
                for message in messages {
                    let mut processed = PROCESSED_MESSAGES.get_or_init(|| std::sync::Mutex::new(std::collections::HashSet::new())).lock().unwrap();
                    if processed.contains(&message.id) {
                        tracing::info!("Duplicate message detected, skipping: {}", message.id);
                        continue;
                    }
                    processed.insert(message.id.clone());

                    if processed.len() > 10000 {
                        processed.clear();
                        processed.insert(message.id.clone());
                    }
                    tracing::info!("Processing new message: {}", message.id);
                }
            }
            if let Some(statuses) = &change.value.statuses {
                for status in statuses {
                    tracing::info!("Processing status update for message {}: {}", status.id, status.status);
                }
            }
        }
    }

    StatusCode::OK
}
