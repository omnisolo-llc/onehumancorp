use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConversationStatus {
    Open,
    Resolved,
    Pending,
    Snoozed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub channel: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: ConversationStatus,
    pub priority: Option<Priority>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
