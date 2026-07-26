use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageSender {
    Contact,
    Agent,
    Bot,
    System,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender: MessageSender,
    pub content: String,
}

impl Message {
    pub fn new(id: impl Into<String>, tenant_id: impl Into<String>, conversation_id: impl Into<String>, sender: MessageSender, content: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            conversation_id: conversation_id.into(),
            sender,
            content: content.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let message = Message::new("m1", "t1", "conv1", MessageSender::Contact, "Hello there!");
        assert_eq!(message.id, "m1");
        assert_eq!(message.tenant_id, "t1");
        assert_eq!(message.conversation_id, "conv1");
        assert_eq!(message.sender, MessageSender::Contact);
        assert_eq!(message.content, "Hello there!");
    }

    #[test]
    fn test_message_serialization() {
        let message = Message::new("m1", "t1", "conv1", MessageSender::Bot, "I am a bot.");
        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&serialized).unwrap();
        assert_eq!(message, deserialized);
    }
}
