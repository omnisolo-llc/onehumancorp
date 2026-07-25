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
    pub image: Option<Media>,
    pub video: Option<Media>,
    pub audio: Option<Media>,
    pub document: Option<Media>,
    pub interactive: Option<Interactive>,
    pub location: Option<Location>,
    #[serde(rename = "type")]
    pub msg_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Text {
    pub body: String,
}

#[derive(Deserialize, Debug)]
pub struct Media {
    pub id: String,
    pub link: Option<String>,
    pub caption: Option<String>,
    pub filename: Option<String>,
    pub mime_type: Option<String>,
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
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub name: Option<String>,
    pub address: Option<String>,
    pub url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub id: String,
    pub status: String,
    pub timestamp: String,
    pub recipient_id: String,
    pub errors: Option<Vec<Error>>,
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









pub async fn handle_webhook(
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    // Process incoming webhook payload
    tracing::info!("Received WhatsApp webhook: {:?}", payload);

    for entry in payload.entry {
        for change in entry.changes {
            let val = change.value;

            // Handle Statuses
            if let Some(statuses) = val.statuses {
                for status in statuses {
                    tracing::info!("Processing status update for message id: {}, status: {}", status.id, status.status);
                    if status.status == "failed" {
                        if let Some(errors) = status.errors {
                            for error in errors {
                                if error.code == 131060 {
                                    tracing::warn!("Message id {} failed with unsupported message type (131060)", status.id);
                                } else {
                                    tracing::warn!("Message id {} failed with error code: {}", status.id, error.code);
                                }
                            }
                        }
                    }
                }
            }

            // Handle Messages
            if let Some(messages) = val.messages {
                for message in messages {
                    tracing::info!("Processing message id: {} of type {}", message.id, message.msg_type);

                    // Example mapping to internal types
                    match message.msg_type.as_str() {
                        "text" => tracing::info!("Text message: {:?}", message.text),
                        "image" | "video" | "audio" | "document" => tracing::info!("Media message received"),
                        "interactive" => tracing::info!("Interactive message: {:?}", message.interactive),
                        "location" => tracing::info!("Location message: {:?}", message.location),
                        _ => tracing::warn!("Unsupported message type: {}", message.msg_type),
                    }
                }
            }
        }
    }

    // Send a 200 OK response to acknowledge receipt
    StatusCode::OK
}
