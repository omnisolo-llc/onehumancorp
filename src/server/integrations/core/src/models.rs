use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub external_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub channel_type: String, // e.g., "instagram", "whatsapp", "email"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub contact_id: Uuid,
    pub inbox_id: Uuid,
    pub status: String, // "open", "resolved", "snoozed"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String, // "contact", "agent", "bot"
    pub sender_id: Option<Uuid>, // Contact ID or Agent ID
    pub content: String,
    pub status: String, // "draft", "pending", "sent", "failed", "delivered"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutboxMessage {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub message_id: Uuid,
    pub channel_type: String,
    pub payload: serde_json::Value,
    pub status: String, // "pending", "processing", "completed", "failed"
    pub attempts: i32,
    pub last_attempt_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
