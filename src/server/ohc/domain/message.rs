use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Channel {
    WebWidget,
    Email,
    WhatsApp,
    Instagram,
    FacebookPage,
    Sms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub tenant_id: String,
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
    pub channel: Channel,
    pub sender_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Message {
    pub fn new(
        tenant_id: String,
        conversation_id: Uuid,
        content: String,
        channel: Channel,
        sender_type: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            tenant_id,
            id: Uuid::new_v4(),
            conversation_id,
            content,
            channel,
            sender_type,
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
        let tenant_id = "tenant-123".to_string();
        let conversation_id = Uuid::new_v4();
        let content = "Hello, I need help with my order".to_string();
        let channel = Channel::WebWidget;
        let sender_type = "contact".to_string();

        let message = Message::new(
            tenant_id.clone(),
            conversation_id,
            content.clone(),
            channel.clone(),
            sender_type.clone(),
        );

        assert_eq!(message.tenant_id, tenant_id);
        assert_eq!(message.conversation_id, conversation_id);
        assert_eq!(message.content, content);
        assert_eq!(message.channel, channel);
        assert_eq!(message.sender_type, sender_type);
        assert!(!message.id.is_nil());
        assert!(message.created_at <= Utc::now());
        assert_eq!(message.created_at, message.updated_at);
    }
}
