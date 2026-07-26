use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Incoming,
    Outgoing,
    Template,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_type: MessageType,
    pub content: String,
    pub sender_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Message {
    pub fn new(
        tenant_id: String,
        conversation_id: String,
        message_type: MessageType,
        content: String,
        sender_id: Option<String>
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            tenant_id,
            conversation_id,
            message_type,
            content,
            sender_id,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let tenant_id = "tenant-000".to_string();
        let conversation_id = "conv-1".to_string();
        let content = "Hello there!".to_string();
        let sender_id = Some("contact-1".to_string());

        let message = Message::new(
            tenant_id.clone(),
            conversation_id.clone(),
            MessageType::Incoming,
            content.clone(),
            sender_id.clone()
        );

        assert!(!message.id.is_empty());
        assert_eq!(message.tenant_id, tenant_id);
        assert_eq!(message.conversation_id, conversation_id);
        assert_eq!(message.message_type, MessageType::Incoming);
        assert_eq!(message.content, content);
        assert_eq!(message.sender_id, sender_id);
        assert!(message.created_at <= Utc::now());
        assert_eq!(message.created_at, message.updated_at);
    }
}
