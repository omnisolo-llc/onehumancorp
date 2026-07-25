use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChannelAdapter {
    Sms,
    Instagram,
    WhatsApp,
    Email,
    WebWidget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub channel_adapter: ChannelAdapter,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub contact_id: Uuid,
    pub inbox_id: Uuid,
    pub source_id: String, // e.g., Instagram handle or WhatsApp number
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConversationStatus {
    Open,
    Resolved,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub contact_inbox_id: Uuid,
    pub status: ConversationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageType {
    Incoming,
    Outgoing,
    Template,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
    pub message_type: MessageType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
