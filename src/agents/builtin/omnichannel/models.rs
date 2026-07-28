use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChannelType {
    WebWidget,
    Api,
    Email,
    Sms,
    Whatsapp,
    Facebook,
    Twitter,
    Instagram,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub id: i64,
    pub account_id: i64,
    pub name: String,
    pub channel_type: ChannelType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: i64,
    pub account_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConversationStatus {
    Open,
    Resolved,
    Pending,
    Snoozed,
    Bot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: i64,
    pub account_id: i64,
    pub inbox_id: i64,
    pub contact_id: i64,
    pub assignee_id: Option<i64>,
    pub status: ConversationStatus,
    pub created_at: DateTime<Utc>,
    pub custom_attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageType {
    Incoming,
    Outgoing,
    Template,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: i64,
    pub conversation_id: i64,
    pub account_id: i64,
    pub content: String,
    pub message_type: MessageType,
    pub created_at: DateTime<Utc>,
    pub sender_type: String, // "Contact", "User", "Bot"
    pub sender_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatwootWebhookPayload {
    pub event: String,
    pub conversation: Option<Conversation>,
    pub messages: Option<Vec<ChatMessage>>,
}
