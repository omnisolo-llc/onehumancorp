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
    let expected_token = "ohc_whatsapp_webhook_secret"; // This should come from config

    if query.mode == "subscribe" && query.verify_token == expected_token {
        (StatusCode::OK, query.challenge)
    } else {
        (StatusCode::FORBIDDEN, "Forbidden".to_string())
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct WebhookPayload {
    pub object: String,
    pub entry: Vec<Entry>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Entry {
    pub id: String,
    pub changes: Vec<Change>,
}

#[derive(Deserialize, Debug, Clone)]
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
    pub statuses: Option<Vec<MessageStatus>>,
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
    pub image: Option<ImageMedia>,
    pub audio: Option<AudioMedia>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Text {
    pub body: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ImageMedia {
    pub id: String,
    pub mime_type: Option<String>,
    pub sha256: Option<String>,
    pub caption: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AudioMedia {
    pub id: String,
    pub mime_type: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MessageStatus {
    pub id: String,
    pub status: String, // "sent", "delivered", "read", "failed"
    pub timestamp: String,
    pub recipient_id: String,
}

pub async fn handle_webhook(
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    // Process incoming webhook payload
    tracing::info!("Received WhatsApp webhook: {:?}", payload);

    // Send a 200 OK response to acknowledge receipt
    StatusCode::OK
}
