use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::types::JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmnichannelInbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub enable_auto_assignment: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmnichannelChannel {
    pub id: Uuid,
    pub inbox_id: Uuid,
    pub provider_type: String, // e.g. "whatsapp", "web_widget"
    pub credentials: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmnichannelContact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmnichannelConversation {
    pub id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String, // e.g. "open", "resolved", "bot_handling"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmnichannelMessage {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
    pub sender_type: String, // e.g. "contact", "agent", "bot"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
