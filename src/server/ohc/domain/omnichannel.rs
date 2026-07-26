use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TenantId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Channel {
    Api,
    Email,
    TwilioSms,
    WebWidget,
    Instagram,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub name: String,
    pub channel: Channel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub status: ConversationStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConversationStatus {
    Open,
    Resolved,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SenderType {
    Contact,
    User,
    Bot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub conversation_id: Uuid,
    pub sender_type: SenderType,
    pub sender_id: Uuid,
    pub content: String,
    pub status: MessageStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageStatus {
    Sent,
    Delivered,
    Read,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub message_id: Uuid,
    pub file_url: String,
    pub file_type: String,
}
