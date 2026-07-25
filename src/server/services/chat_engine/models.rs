// models
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Inbox {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: uuid::Uuid,
    pub inbox_id: uuid::Uuid,
    pub provider: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Conversation {
    pub id: uuid::Uuid,
    pub inbox_id: uuid::Uuid,
    pub contact_id: uuid::Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: uuid::Uuid,
    pub conversation_id: uuid::Uuid,
    pub content: String,
}
