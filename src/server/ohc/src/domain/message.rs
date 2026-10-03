use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender_id: String,
    pub content: String,
}

impl Message {
    pub fn new(id: String, tenant_id: String, conversation_id: String, sender_id: String, content: String) -> Self {
        Self {
            id,
            tenant_id,
            conversation_id,
            sender_id,
            content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let message = Message::new(
            "msg-1".to_string(),
            "tenant-1".to_string(),
            "conv-1".to_string(),
            "sender-1".to_string(),
            "Hello, world!".to_string(),
        );
        assert_eq!(message.id, "msg-1");
        assert_eq!(message.tenant_id, "tenant-1");
        assert_eq!(message.conversation_id, "conv-1");
        assert_eq!(message.sender_id, "sender-1");
        assert_eq!(message.content, "Hello, world!");
    }
}
