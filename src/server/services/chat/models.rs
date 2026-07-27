use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tenant {
    pub tenant_id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub contact_id: String,
    pub tenant_id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Inbox {
    pub inbox_id: String,
    pub tenant_id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Conversation {
    pub conversation_id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message_id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub content: String,
}
