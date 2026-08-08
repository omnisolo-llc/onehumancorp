use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub channel_type: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub enum MessageType {
    Incoming = 0,
    Outgoing = 1,
    Activity = 2,
    Template = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
    pub message_type: MessageType,
    pub custom_attributes: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_inbox_creation() {
        let tenant_id = Uuid::new_v4();
        let inbox = Inbox {
            id: Uuid::new_v4(),
            tenant_id,
            name: "WhatsApp Support".to_string(),
            channel_type: "whatsapp".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(inbox.tenant_id, tenant_id);
        assert_eq!(inbox.name, "WhatsApp Support");
    }

    #[test]
    fn test_contact_serialization() {
        let tenant_id = Uuid::new_v4();
        let custom_attrs = json!({"vip_level": "gold", "preferences": ["no_calls"]});
        let contact = Contact {
            id: Uuid::new_v4(),
            tenant_id,
            name: "Maya Baker".to_string(),
            email: Some("maya@example.com".to_string()),
            phone_number: None,
            identifier: Some("insta_maya".to_string()),
            custom_attributes: Some(custom_attrs.clone()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&contact).unwrap();
        let deserialized: Contact = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.name, contact.name);
        assert_eq!(deserialized.tenant_id, contact.tenant_id);
        assert_eq!(deserialized.custom_attributes, Some(custom_attrs));
    }

    #[test]
    fn test_conversation_status() {
        let tenant_id = Uuid::new_v4();
        let conv = Conversation {
            id: Uuid::new_v4(),
            tenant_id,
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            status: ConversationStatus::Open,
            custom_attributes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(conv.status, ConversationStatus::Open);
    }

    #[test]
    fn test_message_types() {
        let tenant_id = Uuid::new_v4();
        let conv_id = Uuid::new_v4();

        let msg = Message {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id: conv_id,
            content: "Hello!".to_string(),
            message_type: MessageType::Incoming,
            custom_attributes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(msg.message_type, MessageType::Incoming);
    }
}
