use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender_id: String,
    pub content: String,
    pub is_read: bool,
}

impl Message {
    pub fn new(
        id: String,
        tenant_id: String,
        conversation_id: String,
        sender_id: String,
        content: String,
        is_read: bool,
    ) -> Self {
        Self {
            id,
            tenant_id,
            conversation_id,
            sender_id,
            content,
            is_read,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let message = Message::new(
            "msg_1".to_string(),
            "tenant_1".to_string(),
            "conv_1".to_string(),
            "sender_1".to_string(),
            "Hello, world!".to_string(),
            false,
        );

        assert_eq!(message.id, "msg_1");
        assert_eq!(message.tenant_id, "tenant_1");
        assert_eq!(message.conversation_id, "conv_1");
        assert_eq!(message.sender_id, "sender_1");
        assert_eq!(message.content, "Hello, world!");
        assert_eq!(message.is_read, false);
    }
}
