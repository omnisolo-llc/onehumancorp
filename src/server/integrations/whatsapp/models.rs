use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppMessage {
    pub from: String,
    pub to: String,
    pub body: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppWebhookPayload {
    pub object: String,
    pub entry: Vec<WhatsAppWebhookEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppWebhookEntry {
    pub id: String,
    pub changes: Vec<WhatsAppWebhookChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppWebhookChange {
    pub value: WhatsAppWebhookValue,
    pub field: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppWebhookValue {
    pub messaging_product: String,
    pub metadata: WhatsAppWebhookMetadata,
    pub messages: Option<Vec<WhatsAppWebhookMessage>>,
    pub statuses: Option<Vec<WhatsAppWebhookStatus>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppWebhookMetadata {
    pub display_phone_number: String,
    pub phone_number_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppWebhookMessage {
    pub from: String,
    pub id: String,
    pub timestamp: String,
    pub text: Option<WhatsAppWebhookText>,
    pub image: Option<WhatsAppWebhookImage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppWebhookText {
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppWebhookImage {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppWebhookStatus {
    pub id: String,
    pub status: String,
    pub timestamp: String,
}
