use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Native Rust Omnichannel Chat System
/// Replicates core Chatwoot data models and features without relying on the external service.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub custom_attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageType {
    Incoming,
    Outgoing,
    Template,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub content: String,
    pub message_type: MessageType,
    pub created_at: DateTime<Utc>,
    pub sender_id: Option<String>,
    pub is_private: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConversationStatus {
    Open,
    Resolved,
    Pending,
    Snoozed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub contact_id: String,
    pub status: ConversationStatus,
    pub messages: Vec<Message>,
    pub assignee_id: Option<String>,
    pub custom_attributes: HashMap<String, String>,
}

impl Conversation {
    pub fn new(id: String, contact_id: String) -> Self {
        Self {
            id,
            contact_id,
            status: ConversationStatus::Open,
            messages: Vec::new(),
            assignee_id: None,
            custom_attributes: HashMap::new(),
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}
