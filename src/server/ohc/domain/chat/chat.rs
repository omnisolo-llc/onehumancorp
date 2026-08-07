use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use chrono::{DateTime, Utc};

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
    pub custom_attributes: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
    pub custom_attributes: Option<Value>,
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
    pub custom_attributes: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_inbox_instantiation() {
        let id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let now = Utc::now();

        let inbox = Inbox {
            id,
            tenant_id,
            name: "Main Support".to_string(),
            channel_type: "Email".to_string(),
            created_at: now,
            updated_at: now,
        };

        assert_eq!(inbox.id, id);
        assert_eq!(inbox.tenant_id, tenant_id);
        assert_eq!(inbox.name, "Main Support");
        assert_eq!(inbox.channel_type, "Email");
        assert_eq!(inbox.created_at, now);
        assert_eq!(inbox.updated_at, now);
    }

    #[test]
    fn test_inbox_serialization() {
        let id = Uuid::parse_str("d1a1b412-1f48-4228-98e3-5339f4082218").unwrap();
        let tenant_id = Uuid::parse_str("a2e7c3e1-7e81-4b13-9118-ef485c2921ef").unwrap();
        let now = DateTime::parse_from_rfc3339("2023-10-25T10:00:00Z").unwrap().with_timezone(&Utc);

        let inbox = Inbox {
            id,
            tenant_id,
            name: "Main Support".to_string(),
            channel_type: "Email".to_string(),
            created_at: now,
            updated_at: now,
        };

        let serialized = serde_json::to_string(&inbox).unwrap();
        let expected_json = r#"{"id":"d1a1b412-1f48-4228-98e3-5339f4082218","tenant_id":"a2e7c3e1-7e81-4b13-9118-ef485c2921ef","name":"Main Support","channel_type":"Email","created_at":"2023-10-25T10:00:00Z","updated_at":"2023-10-25T10:00:00Z"}"#;
        assert_eq!(serialized, expected_json);

        let deserialized: Inbox = serde_json::from_str(&serialized).unwrap();
        assert_eq!(inbox, deserialized);
    }

    #[test]
    fn test_contact_with_custom_attributes() {
        let id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let now = Utc::now();
        let custom_attr = json!({
            "vip_status": "gold",
            "last_purchase_date": "2023-10-20"
        });

        let contact = Contact {
            id,
            tenant_id,
            name: "John Doe".to_string(),
            email: Some("john@example.com".to_string()),
            phone_number: Some("+1234567890".to_string()),
            identifier: Some("ext_123".to_string()),
            custom_attributes: Some(custom_attr.clone()),
            created_at: now,
            updated_at: now,
        };

        assert_eq!(contact.custom_attributes, Some(custom_attr));

        let serialized = serde_json::to_string(&contact).unwrap();
        let deserialized: Contact = serde_json::from_str(&serialized).unwrap();

        assert_eq!(contact, deserialized);
    }

    #[test]
    fn test_conversation_instantiation() {
        let id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();
        let contact_id = Uuid::new_v4();
        let now = Utc::now();

        let conversation = Conversation {
            id,
            tenant_id,
            inbox_id,
            contact_id,
            status: "Open".to_string(),
            custom_attributes: None,
            created_at: now,
            updated_at: now,
        };

        assert_eq!(conversation.status, "Open");
        assert_eq!(conversation.custom_attributes, None);

        let serialized = serde_json::to_string(&conversation).unwrap();
        let deserialized: Conversation = serde_json::from_str(&serialized).unwrap();
        assert_eq!(conversation, deserialized);
    }

    #[test]
    fn test_message_instantiation() {
        let id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let conversation_id = Uuid::new_v4();
        let now = Utc::now();

        let message = Message {
            id,
            tenant_id,
            conversation_id,
            content: "Hello, how can I help?".to_string(),
            message_type: 1,
            custom_attributes: Some(json!({"source": "web"})),
            created_at: now,
            updated_at: now,
        };

        assert_eq!(message.content, "Hello, how can I help?");
        assert_eq!(message.message_type, 1);

        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&serialized).unwrap();
        assert_eq!(message, deserialized);
    }
}
