use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct Channel {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub provider_type: String,
    pub provider_config: sqlx::types::Json<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub channel_id: Uuid,
    pub content: String,
    pub sender_type: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use chrono::Utc;
    use serde_json::json;

    #[test]
    fn test_inbox_creation() {
        let inbox = Inbox {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "Main Inbox".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        assert_eq!(inbox.name, "Main Inbox");
    }

    #[test]
    fn test_channel_creation() {
        let channel = Channel {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            provider_type: "whatsapp".to_string(),
            provider_config: sqlx::types::Json(json!({"phone_number_id": "123"})),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        assert_eq!(channel.provider_type, "whatsapp");
    }

    #[test]
    fn test_contact_creation() {
        let contact = Contact {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "John Doe".to_string(),
            phone_number: Some("+1234567890".to_string()),
            email: Some("john@example.com".to_string()),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        assert_eq!(contact.name, "John Doe");
    }

    #[test]
    fn test_conversation_creation() {
        let conv = Conversation {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            status: "open".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        assert_eq!(conv.status, "open");
    }

    #[test]
    fn test_message_creation() {
        let msg = Message {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            channel_id: Uuid::new_v4(),
            content: "Hello!".to_string(),
            sender_type: "customer".to_string(),
            created_at: Some(Utc::now()),
        };
        assert_eq!(msg.content, "Hello!");
    }
}
