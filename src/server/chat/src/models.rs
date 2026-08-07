use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub channel_type: String, // e.g., "web_widget", "whatsapp", "instagram"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String, // e.g., "open", "resolved", "snoozed"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
    pub message_type: String, // e.g., "incoming", "outgoing"
    pub sender_type: String,  // e.g., "contact", "user", "agent"
    pub created_at: DateTime<Utc>,
}
