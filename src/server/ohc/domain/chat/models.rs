use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub channel_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub identifier: Option<String>,
    pub custom_attributes: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConversationStatus {
    Open,
    Resolved,
    Pending,
    Snoozed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: ConversationStatus,
    pub custom_attributes: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
    pub message_type: i32,
    pub custom_attributes: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::str::FromStr;

    #[test]
    fn test_inbox_serialization() {
        let inbox = Inbox {
            id: Uuid::from_str("123e4567-e89b-12d3-a456-426614174000").unwrap(),
            tenant_id: Uuid::from_str("123e4567-e89b-12d3-a456-426614174001").unwrap(),
            name: "Main Support".to_string(),
            channel_type: "Email".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&inbox).unwrap();
        let deserialized: Inbox = serde_json::from_str(&serialized).unwrap();

        assert_eq!(inbox, deserialized);
    }

    #[test]
    fn test_contact_serialization() {
        let contact = Contact {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "John Doe".to_string(),
            email: Some("john@example.com".to_string()),
            phone_number: Some("+1234567890".to_string()),
            identifier: Some("ext-123".to_string()),
            custom_attributes: Some(json!({"vip": true})),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&contact).unwrap();
        let deserialized: Contact = serde_json::from_str(&serialized).unwrap();

        assert_eq!(contact, deserialized);
    }

    #[test]
    fn test_conversation_serialization() {
        let conversation = Conversation {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            status: ConversationStatus::Open,
            custom_attributes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&conversation).unwrap();
        let deserialized: Conversation = serde_json::from_str(&serialized).unwrap();

        assert_eq!(conversation, deserialized);
    }

    #[test]
    fn test_message_serialization() {
        let message = Message {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            content: "Hello, I need help".to_string(),
            message_type: 1,
            custom_attributes: Some(json!({"source": "web"})),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&serialized).unwrap();

        assert_eq!(message, deserialized);
    }
}
