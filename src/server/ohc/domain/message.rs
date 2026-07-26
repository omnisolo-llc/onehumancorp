use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageType {
    Incoming,
    Outgoing,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: String,
    pub conversation_id: Uuid,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub message_type: MessageType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Message {
    pub fn new(
        tenant_id: String,
        conversation_id: Uuid,
        sender_id: Option<Uuid>,
        content: String,
        message_type: MessageType,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id,
            sender_id,
            content,
            message_type,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_new() {
        let tenant_id = "tenant-123".to_string();
        let conversation_id = Uuid::new_v4();
        let sender_id = Some(Uuid::new_v4());
        let content = "Hello, how can I help you?".to_string();
        let message_type = MessageType::Outgoing;

        let message = Message::new(
            tenant_id.clone(),
            conversation_id,
            sender_id,
            content.clone(),
            message_type.clone(),
        );

        assert_eq!(message.tenant_id, tenant_id);
        assert_eq!(message.conversation_id, conversation_id);
        assert_eq!(message.sender_id, sender_id);
        assert_eq!(message.content, content);
        assert_eq!(message.message_type, message_type);
        assert!(!message.id.is_nil());
        assert!(message.created_at <= Utc::now());
        assert_eq!(message.created_at, message.updated_at);
    }

    #[test]
    fn test_message_new_system() {
        let tenant_id = "tenant-123".to_string();
        let conversation_id = Uuid::new_v4();
        let content = "Conversation assigned to agent.".to_string();
        let message_type = MessageType::System;

        let message = Message::new(
            tenant_id.clone(),
            conversation_id,
            None,
            content.clone(),
            message_type.clone(),
        );

        assert_eq!(message.tenant_id, tenant_id);
        assert_eq!(message.conversation_id, conversation_id);
        assert_eq!(message.sender_id, None);
        assert_eq!(message.content, content);
        assert_eq!(message.message_type, message_type);
    }

    #[test]
    fn test_message_type_serialization() {
        let msg_type = MessageType::Incoming;
        let json = serde_json::to_string(&msg_type).unwrap();
        assert_eq!(json, "\"Incoming\"");

        let deserialized: MessageType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, msg_type);
    }
}
