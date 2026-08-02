use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, FromRow)]
pub struct ChatInbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct ChatChannel {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub channel_type: String,
    pub config: Option<sqlx::types::Json<serde_json::Value>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct ChatContact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct ChatConversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct ChatMessage {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_chat_inbox_creation() {
        let inbox = ChatInbox {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "Test Inbox".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(inbox.name, "Test Inbox");
    }

    #[test]
    fn test_chat_channel_creation() {
        let channel = ChatChannel {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            channel_type: "whatsapp".to_string(),
            config: None,
            created_at: None,
            updated_at: None,
        };
        assert_eq!(channel.channel_type, "whatsapp");
    }

    #[test]
    fn test_chat_contact_creation() {
        let contact = ChatContact {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: Some("John Doe".to_string()),
            email: Some("john@example.com".to_string()),
            phone: Some("+1234567890".to_string()),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(contact.name.unwrap(), "John Doe");
        assert_eq!(contact.email.unwrap(), "john@example.com");
    }

    #[test]
    fn test_chat_conversation_creation() {
        let conversation = ChatConversation {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            assignee_id: None,
            status: "open".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(conversation.status, "open");
    }

    #[test]
    fn test_chat_message_creation() {
        let message = ChatMessage {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            sender_type: "contact".to_string(),
            sender_id: None,
            content: "Hello!".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(message.content, "Hello!");
        assert_eq!(message.sender_type, "contact");
    }
}
