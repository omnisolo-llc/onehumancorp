use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageType {
    Incoming,
    Outgoing,
    InternalNote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: String,
    pub conversation_id: Uuid,
    pub sender_id: Option<Uuid>, // Nullable for system messages or AI
    pub content: String,
    pub message_type: MessageType,
    pub is_read: bool,
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
            is_read: false,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_message() {
        let tenant_id = "tenant_1".to_string();
        let conversation_id = Uuid::new_v4();
        let sender_id = Some(Uuid::new_v4());
        let content = "Hello world".to_string();
        let msg_type = MessageType::Incoming;

        let msg = Message::new(
            tenant_id.clone(),
            conversation_id,
            sender_id,
            content.clone(),
            msg_type.clone(),
        );

        assert_eq!(msg.tenant_id, tenant_id);
        assert_eq!(msg.conversation_id, conversation_id);
        assert_eq!(msg.sender_id, sender_id);
        assert_eq!(msg.content, content);
        assert_eq!(msg.message_type, msg_type);
        assert!(!msg.is_read);
    }
}
