use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Channel {
    WebWidget,
    WhatsApp,
    Email,
    Instagram,
    Sms,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageType {
    Incoming,
    Outgoing,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: String,
    pub conversation_id: Uuid,
    pub channel: Channel,
    pub message_type: MessageType,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Message {
    pub fn new(
        tenant_id: impl Into<String>,
        conversation_id: Uuid,
        channel: Channel,
        message_type: MessageType,
        content: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id: tenant_id.into(),
            conversation_id,
            channel,
            message_type,
            content: content.into(),
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
        let tenant_id = "tenant-123";
        let conversation_id = Uuid::new_v4();
        let message = Message::new(
            tenant_id,
            conversation_id,
            Channel::WhatsApp,
            MessageType::Incoming,
            "Hello, world!",
        );

        assert_eq!(message.tenant_id, tenant_id);
        assert_eq!(message.conversation_id, conversation_id);
        assert_eq!(message.channel, Channel::WhatsApp);
        assert_eq!(message.message_type, MessageType::Incoming);
        assert_eq!(message.content, "Hello, world!");
    }

    #[test]
    fn test_message_serialization() {
        let message = Message::new(
            "tenant-123",
            Uuid::new_v4(),
            Channel::Instagram,
            MessageType::Outgoing,
            "How can I help you?",
        );

        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&serialized).unwrap();

        assert_eq!(message, deserialized);
    }
}
