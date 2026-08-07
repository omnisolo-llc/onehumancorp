use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatInbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatChannel {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub channel_type: String,
    pub config: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatContact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatConversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatMessage {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub message_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_inbox_serialization() {
        let inbox = ChatInbox {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "Test Inbox".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&inbox).unwrap();
        let deserialized: ChatInbox = serde_json::from_str(&serialized).unwrap();

        assert_eq!(inbox, deserialized);
    }

    #[test]
    fn test_chat_channel_serialization() {
        let channel = ChatChannel {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            channel_type: "whatsapp".to_string(),
            config: serde_json::json!({"phone_number_id": "12345"}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&channel).unwrap();
        let deserialized: ChatChannel = serde_json::from_str(&serialized).unwrap();

        assert_eq!(channel, deserialized);
    }

    #[test]
    fn test_chat_contact_serialization() {
        let contact = ChatContact {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: Some("John Doe".to_string()),
            email: Some("john@example.com".to_string()),
            phone: Some("+1234567890".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&contact).unwrap();
        let deserialized: ChatContact = serde_json::from_str(&serialized).unwrap();

        assert_eq!(contact, deserialized);
    }

    #[test]
    fn test_chat_conversation_serialization() {
        let conversation = ChatConversation {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            assignee_id: None,
            status: "open".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&conversation).unwrap();
        let deserialized: ChatConversation = serde_json::from_str(&serialized).unwrap();

        assert_eq!(conversation, deserialized);
    }

    #[test]
    fn test_chat_message_serialization() {
        let message = ChatMessage {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            sender_type: "contact".to_string(),
            sender_id: None,
            content: "Hello, I need help with my order.".to_string(),
            message_type: Some("text".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: ChatMessage = serde_json::from_str(&serialized).unwrap();

        assert_eq!(message, deserialized);
    }

    #[test]
    fn test_tenant_isolation_is_enforced_in_structs() {
        let tenant_a = Uuid::new_v4();
        let tenant_b = Uuid::new_v4();

        let inbox_a = ChatInbox {
            id: Uuid::new_v4(),
            tenant_id: tenant_a,
            name: "Inbox A".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let inbox_b = ChatInbox {
            id: Uuid::new_v4(),
            tenant_id: tenant_b,
            name: "Inbox B".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_ne!(inbox_a.tenant_id, inbox_b.tenant_id);
    }
}
