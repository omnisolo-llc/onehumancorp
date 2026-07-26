use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageSender {
    Contact(String), // contact_id
    Agent(String),   // agent_id or user_id
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender: MessageSender,
    pub content: String,
    pub created_at_utc: i64,
}

impl Message {
    pub fn new(
        id: String,
        tenant_id: String,
        conversation_id: String,
        sender: MessageSender,
        content: String,
        created_at_utc: i64,
    ) -> Self {
        Self {
            id,
            tenant_id,
            conversation_id,
            sender,
            content,
            created_at_utc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_new() {
        let message = Message::new(
            "msg_1".to_string(),
            "tenant_1".to_string(),
            "conv_1".to_string(),
            MessageSender::Contact("contact_1".to_string()),
            "Hello, I need help with an order.".to_string(),
            1672531200,
        );

        assert_eq!(message.id, "msg_1");
        assert_eq!(message.tenant_id, "tenant_1");
        assert_eq!(message.conversation_id, "conv_1");
        assert_eq!(
            message.sender,
            MessageSender::Contact("contact_1".to_string())
        );
        assert_eq!(message.content, "Hello, I need help with an order.");
        assert_eq!(message.created_at_utc, 1672531200);
    }

    #[test]
    fn test_message_serialization() {
        let message = Message::new(
            "msg_1".to_string(),
            "tenant_1".to_string(),
            "conv_1".to_string(),
            MessageSender::Agent("agent_1".to_string()),
            "Sure, I can help with that.".to_string(),
            1672531260,
        );

        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&serialized).unwrap();

        assert_eq!(message, deserialized);
    }
}
