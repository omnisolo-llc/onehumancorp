use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatContact {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub custom_attributes: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub id: Uuid,
    pub content: String,
    pub sender_type: SenderType,
    pub sender_id: Uuid,
    pub conversation_id: Uuid,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SenderType {
    Contact,
    Agent,
    Bot,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub id: Uuid,
    pub contact_id: Uuid,
    pub status: ConversationStatus,
    pub assignee_id: Option<Uuid>,
    pub custom_attributes: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ConversationStatus {
    Open,
    Resolved,
    Pending,
    Snoozed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChannelType {
    WebWidget,
    WhatsApp,
    Api,
    Email,
    Sms,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    pub channel_type: ChannelType,
    pub config: serde_json::Value,
}
