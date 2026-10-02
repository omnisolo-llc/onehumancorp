use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub channel_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChannelAdapter {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub channel_type: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub identifier: Option<String>,
    pub custom_attributes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub status: String,
    pub custom_attributes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub message_type: i32,
    pub custom_attributes: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use uuid::Uuid;

    #[test]
    fn test_inbox_creation() {
        let tenant_id = Uuid::new_v4();
        let inbox = Inbox {
            id: Uuid::new_v4(),
            tenant_id,
            name: "Test Inbox".to_string(),
            channel_type: "web_widget".to_string(),
        };

        assert_eq!(inbox.tenant_id, tenant_id);
        assert_eq!(inbox.name, "Test Inbox");
        assert_eq!(inbox.channel_type, "web_widget");

        let serialized = serde_json::to_string(&inbox).unwrap();
        let deserialized: Inbox = serde_json::from_str(&serialized).unwrap();
        assert_eq!(inbox, deserialized);
    }

    #[test]
    fn test_channel_adapter_creation() {
        let tenant_id = Uuid::new_v4();
        let channel = ChannelAdapter {
            id: Uuid::new_v4(),
            tenant_id,
            inbox_id: Uuid::new_v4(),
            channel_type: "whatsapp".to_string(),
            config: json!({ "api_key": "123" }),
        };

        assert_eq!(channel.tenant_id, tenant_id);
        assert_eq!(channel.channel_type, "whatsapp");

        let serialized = serde_json::to_string(&channel).unwrap();
        let deserialized: ChannelAdapter = serde_json::from_str(&serialized).unwrap();
        assert_eq!(channel, deserialized);
    }

    #[test]
    fn test_contact_creation() {
        let tenant_id = Uuid::new_v4();
        let custom_attrs = json!({ "vip": true });
        let contact = Contact {
            id: Uuid::new_v4(),
            tenant_id,
            name: Some("John Doe".to_string()),
            email: Some("john@example.com".to_string()),
            phone: None,
            identifier: Some("ext-123".to_string()),
            custom_attributes: Some(custom_attrs.clone()),
        };

        assert_eq!(contact.tenant_id, tenant_id);
        assert_eq!(contact.name, Some("John Doe".to_string()));
        assert_eq!(contact.custom_attributes, Some(custom_attrs));

        let serialized = serde_json::to_string(&contact).unwrap();
        let deserialized: Contact = serde_json::from_str(&serialized).unwrap();
        assert_eq!(contact, deserialized);
    }

    #[test]
    fn test_conversation_creation() {
        let tenant_id = Uuid::new_v4();
        let conversation = Conversation {
            id: Uuid::new_v4(),
            tenant_id,
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            assignee_id: None,
            status: "open".to_string(),
            custom_attributes: None,
        };

        assert_eq!(conversation.tenant_id, tenant_id);
        assert_eq!(conversation.status, "open");

        let serialized = serde_json::to_string(&conversation).unwrap();
        let deserialized: Conversation = serde_json::from_str(&serialized).unwrap();
        assert_eq!(conversation, deserialized);
    }

    #[test]
    fn test_message_creation() {
        let tenant_id = Uuid::new_v4();
        let message = Message {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id: Uuid::new_v4(),
            sender_type: "agent".to_string(),
            sender_id: Some(Uuid::new_v4()),
            content: "Hello!".to_string(),
            message_type: 0,
            custom_attributes: None,
        };

        assert_eq!(message.tenant_id, tenant_id);
        assert_eq!(message.content, "Hello!");

        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&serialized).unwrap();
        assert_eq!(message, deserialized);
    }
}
