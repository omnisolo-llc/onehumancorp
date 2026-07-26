use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SenderType {
    Contact,
    Agent,
    System,
    Ai,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender_id: String,
    pub sender_type: SenderType,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl Message {
    pub fn new(
        id: String,
        tenant_id: String,
        conversation_id: String,
        sender_id: String,
        sender_type: SenderType,
        content: String,
    ) -> Self {
        Self {
            id,
            tenant_id,
            conversation_id,
            sender_id,
            sender_type,
            content,
            created_at: Utc::now(),
        }
    }
}

#[async_trait::async_trait]
pub trait MessageService: Send + Sync {
    async fn send_message(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        sender_id: &str,
        sender_type: SenderType,
        content: String,
    ) -> Result<Message, String>;

    async fn list_messages(&self, tenant_id: &str, conversation_id: &str) -> Result<Vec<Message>, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation_and_serde() {
        let sender_types = vec![
            SenderType::Contact,
            SenderType::Agent,
            SenderType::System,
            SenderType::Ai,
        ];

        for (i, &sender_type) in sender_types.iter().enumerate() {
            let message = Message::new(
                format!("msg-{}", i),
                "tenant-abc".to_string(),
                "conv-1".to_string(),
                "sender-123".to_string(),
                sender_type,
                "Hello, how can I help you?".to_string(),
            );

            assert_eq!(message.id, format!("msg-{}", i));
            assert_eq!(message.tenant_id, "tenant-abc");
            assert_eq!(message.conversation_id, "conv-1");
            assert_eq!(message.sender_id, "sender-123");
            assert_eq!(message.sender_type, sender_type);
            assert_eq!(message.content, "Hello, how can I help you?");

            let serialized = serde_json::to_string(&message).unwrap();
            let deserialized: Message = serde_json::from_str(&serialized).unwrap();
            assert_eq!(message.id, deserialized.id);
            assert_eq!(message.tenant_id, deserialized.tenant_id);
            assert_eq!(message.conversation_id, deserialized.conversation_id);
            assert_eq!(message.sender_id, deserialized.sender_id);
            assert_eq!(message.sender_type, deserialized.sender_type);
            assert_eq!(message.content, deserialized.content);
        }
    }
}
