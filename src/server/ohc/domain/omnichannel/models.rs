use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum ChannelType {
    Email,
    Sms,
    WebWidget,
    WhatsApp,
    Instagram,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub channel_type: ChannelType,
    pub auto_assignment_config: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub identifier: Option<String>, // External identifier if any
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum ConversationStatus {
    Open,
    Resolved,
    Pending,
    Snoozed,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: ConversationStatus,
    pub assignee_id: Option<Uuid>,
    pub unread_count: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum SenderType {
    Contact,
    Agent,
    Bot,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub inbox_id: Option<Uuid>,
    pub sender_type: SenderType,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub content_type: Option<String>, // e.g., 'text', 'html', 'markdown'
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inbox_creation() {
        let inbox = Inbox {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "Support".to_string(),
            channel_type: ChannelType::Email,
            auto_assignment_config: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        assert_eq!(inbox.name, "Support");
        assert_eq!(inbox.channel_type, ChannelType::Email);
    }

    #[test]
    fn test_contact_creation() {
        let contact = Contact {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: Some("John Doe".to_string()),
            email: Some("john@example.com".to_string()),
            phone_number: None,
            identifier: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        assert_eq!(contact.name.unwrap(), "John Doe");
        assert_eq!(contact.email.unwrap(), "john@example.com");
    }

    #[test]
    fn test_conversation_creation() {
        let conversation = Conversation {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            status: ConversationStatus::Open,
            assignee_id: None,
            unread_count: Some(0),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        assert_eq!(conversation.status, ConversationStatus::Open);
        assert_eq!(conversation.unread_count, Some(0));
    }

    #[test]
    fn test_message_creation() {
        let message = Message {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            inbox_id: Some(Uuid::new_v4()),
            sender_type: SenderType::Contact,
            sender_id: None,
            content: "Hello!".to_string(),
            content_type: Some("text".to_string()),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        assert_eq!(message.sender_type, SenderType::Contact);
        assert_eq!(message.content, "Hello!");
    }
}
