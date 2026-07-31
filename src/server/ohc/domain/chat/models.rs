use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "chat_inboxes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type Inbox = Model;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebWidgetConfig {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub widget_color: String,
    pub welcome_title: String,
    pub welcome_tagline: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub email_address: String,
    pub forward_to_email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String, // e.g., "Contact", "Agent", "Bot"
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub message_type: String, // e.g., "incoming", "outgoing"
    pub created_at: DateTime<Utc>,
}

#[async_trait::async_trait]
pub trait ChannelAdapter: Send + Sync {
    async fn send_message(&self, tenant_id: Uuid, conversation_id: Uuid, message: &Message) -> Result<(), String>;
    fn get_channel_type(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inbox_creation() {
        let tenant_id = Uuid::new_v4();
        let now = Utc::now();
        let inbox = Inbox {
            id: Uuid::new_v4(),
            tenant_id,
            name: "Main Inbox".to_string(),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(inbox.name, "Main Inbox");
        assert_eq!(inbox.tenant_id, tenant_id);
    }

    #[test]
    fn test_web_widget_config() {
        let tenant_id = Uuid::new_v4();
        let now = Utc::now();
        let config = WebWidgetConfig {
            id: Uuid::new_v4(),
            tenant_id,
            inbox_id: Uuid::new_v4(),
            widget_color: "#FFFFFF".to_string(),
            welcome_title: "Hello".to_string(),
            welcome_tagline: "How can we help?".to_string(),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(config.widget_color, "#FFFFFF");
    }

    #[test]
    fn test_email_config() {
        let tenant_id = Uuid::new_v4();
        let now = Utc::now();
        let config = EmailConfig {
            id: Uuid::new_v4(),
            tenant_id,
            inbox_id: Uuid::new_v4(),
            email_address: "support@example.com".to_string(),
            forward_to_email: "fwd@example.com".to_string(),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(config.email_address, "support@example.com");
    }

    #[test]
    fn test_contact_creation() {
        let tenant_id = Uuid::new_v4();
        let now = Utc::now();
        let contact = Contact {
            id: Uuid::new_v4(),
            tenant_id,
            name: "John Doe".to_string(),
            email: Some("john@example.com".to_string()),
            phone_number: None,
            avatar_url: None,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(contact.name, "John Doe");
        assert_eq!(contact.email.unwrap(), "john@example.com");
    }

    #[test]
    fn test_conversation_creation() {
        let tenant_id = Uuid::new_v4();
        let now = Utc::now();
        let conversation = Conversation {
            id: Uuid::new_v4(),
            tenant_id,
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            assignee_id: None,
            status: "open".to_string(),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(conversation.status, "open");
    }

    #[test]
    fn test_message_creation() {
        let tenant_id = Uuid::new_v4();
        let now = Utc::now();
        let message = Message {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id: Uuid::new_v4(),
            sender_type: "Contact".to_string(),
            sender_id: None,
            content: "Hello there".to_string(),
            message_type: "incoming".to_string(),
            created_at: now,
        };
        assert_eq!(message.content, "Hello there");
        assert_eq!(message.message_type, "incoming");
    }
}
