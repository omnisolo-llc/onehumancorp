use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub channel_type: String,
    pub custom_attributes: serde_json::Value,
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
    pub identifier: Option<String>,
    pub custom_attributes: serde_json::Value,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: ConversationStatus,
    pub custom_attributes: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageType {
    Incoming,
    Outgoing,
    Activity,
    Template,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
    pub message_type: MessageType,
    pub custom_attributes: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_inbox_serialization() {
        let inbox = Inbox {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "Main Inbox".to_string(),
            channel_type: "Email".to_string(),
            custom_attributes: json!({"theme": "dark"}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&inbox).unwrap();
        let deserialized: Inbox = serde_json::from_str(&serialized).unwrap();

        assert_eq!(inbox.id, deserialized.id);
        assert_eq!(inbox.tenant_id, deserialized.tenant_id);
        assert_eq!(inbox.name, deserialized.name);
        assert_eq!(inbox.channel_type, deserialized.channel_type);
        assert_eq!(inbox.custom_attributes, deserialized.custom_attributes);
    }

    #[test]
    fn test_contact_serialization() {
        let contact = Contact {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "John Doe".to_string(),
            email: Some("john@example.com".to_string()),
            phone_number: Some("+1234567890".to_string()),
            identifier: None,
            custom_attributes: json!({"vip": true}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&contact).unwrap();
        let deserialized: Contact = serde_json::from_str(&serialized).unwrap();

        assert_eq!(contact.id, deserialized.id);
        assert_eq!(contact.tenant_id, deserialized.tenant_id);
        assert_eq!(contact.name, deserialized.name);
        assert_eq!(contact.email, deserialized.email);
        assert_eq!(contact.phone_number, deserialized.phone_number);
        assert_eq!(contact.custom_attributes, deserialized.custom_attributes);
    }

    #[test]
    fn test_conversation_serialization() {
        let conversation = Conversation {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            status: ConversationStatus::Open,
            custom_attributes: json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&conversation).unwrap();
        let deserialized: Conversation = serde_json::from_str(&serialized).unwrap();

        assert_eq!(conversation.id, deserialized.id);
        assert_eq!(conversation.tenant_id, deserialized.tenant_id);
        assert_eq!(conversation.inbox_id, deserialized.inbox_id);
        assert_eq!(conversation.contact_id, deserialized.contact_id);
        assert_eq!(conversation.status, deserialized.status);
    }

    #[test]
    fn test_message_serialization() {
        let message = Message {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            content: "Hello there!".to_string(),
            message_type: MessageType::Incoming,
            custom_attributes: json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&serialized).unwrap();

        assert_eq!(message.id, deserialized.id);
        assert_eq!(message.tenant_id, deserialized.tenant_id);
        assert_eq!(message.conversation_id, deserialized.conversation_id);
        assert_eq!(message.content, deserialized.content);
        assert_eq!(message.message_type, deserialized.message_type);
    }
}
