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
    let expected_token = std::env::var("WHATSAPP_WEBHOOK_VERIFY_TOKEN")
        .unwrap_or_else(|_| "ohc_whatsapp_webhook_secret".to_string());

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
    pub image: Option<MediaPayload>,
    pub video: Option<MediaPayload>,
    pub document: Option<MediaPayload>,
    pub audio: Option<MediaPayload>,
    pub location: Option<LocationPayload>,
    pub interactive: Option<InteractivePayload>,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub context: Option<ContextPayload>,
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub id: String,
    pub status: String,
    pub timestamp: String,
    pub recipient_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Text {
    pub body: String,
}

#[derive(Deserialize, Debug)]
pub struct MediaPayload {
    pub id: String,
    pub mime_type: Option<String>,
    pub sha256: Option<String>,
    pub caption: Option<String>,
    pub filename: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct LocationPayload {
    pub latitude: f64,
    pub longitude: f64,
    pub name: Option<String>,
    pub address: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct InteractivePayload {
    #[serde(rename = "type")]
    pub interactive_type: String,
    pub button_reply: Option<ButtonReplyPayload>,
    pub list_reply: Option<ListReplyPayload>,
}

#[derive(Deserialize, Debug)]
pub struct ButtonReplyPayload {
    pub id: String,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct ListReplyPayload {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ContextPayload {
    pub from: String,
    pub id: String,
}



pub async fn handle_webhook(
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    tracing::info!("Received WhatsApp webhook: {:?}", payload);

    for entry in payload.entry {
        for change in entry.changes {
            if change.field != "messages" {
                continue;
            }

            let value = change.value;
            let _phone_number_id = value.metadata.phone_number_id;

            if let Some(messages) = value.messages {
                for message in messages {
                    // Here we'd acquire a Redis lock to ensure idempotency based on `message.id`.
                    // Example (pseudo-code):
                    // let lock_key = format!("ohc:lock:whatsapp_message:{}", message.id);
                    // if !redis.set_nx(lock_key).await { continue; }

                    tracing::info!("Processing incoming WhatsApp message from {} (type: {})", message.from, message.msg_type);

                    // Map to internal OHC message structure and route to appropriate tenant inbox
                }
            }

            if let Some(statuses) = value.statuses {
                for status in statuses {
                    tracing::info!("Processing WhatsApp message status: {} (id: {})", status.status, status.id);
                    // Map status updates to internal OHC message delivery status
                }
            }
        }
    }

    StatusCode::OK
}
