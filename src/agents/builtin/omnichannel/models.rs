use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelType {
    WebWidget,
    Email,
    Api,
    Facebook,
    Twitter,
    Line,
    Whatsapp,
    Sms,
    Instagram,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub account_id: String,
    pub inbox_id: String,
    pub status: String,
    pub assignee_id: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub content: String,
    pub message_type: String, // 'incoming' or 'outgoing'
    pub sender_id: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub id: String,
    pub name: String,
    pub channel_type: ChannelType,
}
