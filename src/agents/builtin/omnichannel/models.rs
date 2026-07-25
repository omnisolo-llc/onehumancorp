use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Inbox {
    pub id: String,
    pub name: String,
    pub channel_type: String, // e.g., "widget", "whatsapp", "email"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Contact {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConversationStatus {
    Open,
    Resolved,
    Pending,
    Snoozed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Conversation {
    pub id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub assignee_id: Option<String>,
    pub status: ConversationStatus,
    pub is_bot_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageType {
    Incoming,
    Outgoing,
    Template,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub content: String,
    pub message_type: MessageType,
    pub sender_id: Option<String>, // None if sent by the system/bot in some contexts
    pub private: bool, // true if it's an internal note
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub email: String,
}
