use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub channel_type: String,
    pub settings: Option<Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub identifier: String,
    pub custom_attributes: Option<Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
    pub message_type: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateInboxRequest {
    pub channel_type: String,
    pub settings: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateContactRequest {
    pub identifier: String,
    pub custom_attributes: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateConversationRequest {
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendMessageRequest {
    pub conversation_id: Uuid,
    pub content: String,
    pub message_type: Option<String>,
}
