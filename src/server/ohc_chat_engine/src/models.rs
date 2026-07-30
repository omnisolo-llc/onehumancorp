use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: String,
    pub channel_type: String, // e.g., "web", "whatsapp", "instagram"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: String,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String, // e.g., "open", "resolved", "snoozed"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: String,
    pub conversation_id: Uuid,
    pub content: String,
    pub message_type: String, // e.g., "incoming", "outgoing", "system"
    pub content_attributes: Option<serde_json::Value>,
    pub external_source_ids: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
