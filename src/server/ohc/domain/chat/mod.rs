use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "conversation_status", rename_all = "snake_case")]
pub enum ConversationStatus {
    Open,
    Snoozed,
    Resolved,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: ConversationStatus,
    pub assignee_id: Option<Uuid>,
    pub last_activity_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "message_type", rename_all = "snake_case")]
pub enum MessageType {
    Incoming,
    Outgoing,
    SystemAction,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub message_type: MessageType,
    pub content: String,
    pub sender_id: Option<Uuid>,
    pub sender_type: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub custom_attributes: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
