use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: String,
    pub conversation_id: Uuid,
    pub content: String,
    pub sender_id: Option<Uuid>,
    pub is_from_contact: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Message {
    pub fn new(tenant_id: String, conversation_id: Uuid, content: String, sender_id: Option<Uuid>, is_from_contact: bool) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id,
            content,
            sender_id,
            is_from_contact,
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
        let content = "Hello, I need help".to_string();
        let sender_id = Some(Uuid::new_v4());
        let is_from_contact = true;

        let message = Message::new(tenant_id.clone(), conversation_id, content.clone(), sender_id, is_from_contact);

        assert_eq!(message.tenant_id, tenant_id);
        assert_eq!(message.conversation_id, conversation_id);
        assert_eq!(message.content, content);
        assert_eq!(message.sender_id, sender_id);
        assert_eq!(message.is_from_contact, is_from_contact);
        assert!(!message.id.is_nil());
        assert!(message.created_at <= Utc::now());
        assert_eq!(message.created_at, message.updated_at);
    }

    #[test]
    fn test_message_serialization() {
        let message = Message::new(
            "tenant-123".to_string(),
            Uuid::new_v4(),
            "Test message".to_string(),
            None,
            false
        );
        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&serialized).unwrap();

        assert_eq!(message, deserialized);
    }
}
