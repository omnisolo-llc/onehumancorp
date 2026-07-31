use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the status of a conversation, mirroring Chatwoot's conversation.status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConversationStatus {
    Open,
    Resolved,
    Pending,
    Snoozed,
}

/// Represents the contact (visitor, lead, etc.), mirroring Chatwoot's contact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: i64,
    pub account_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub identifier: Option<String>,
    pub custom_attributes: serde_json::Value,
    pub additional_attributes: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents an inbox (channel), mirroring Chatwoot's inbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub id: i64,
    pub account_id: i64,
    pub name: String,
    pub channel_type: String,
    pub enable_auto_assignment: bool,
    pub greeting_enabled: bool,
    pub greeting_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents a conversation, mirroring Chatwoot's conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: i64,
    pub account_id: i64,
    pub inbox_id: i64,
    pub contact_id: i64,
    pub assignee_id: Option<i64>,
    pub status: ConversationStatus,
    pub custom_attributes: serde_json::Value,
    pub additional_attributes: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents the type of a message, mirroring Chatwoot's message.message_type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageType {
    Incoming = 0,
    Outgoing = 1,
    Activity = 2,
    Template = 3,
}

/// Represents a message, mirroring Chatwoot's message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub account_id: i64,
    pub inbox_id: i64,
    pub conversation_id: i64,
    pub message_type: MessageType,
    pub content: Option<String>,
    pub private: bool,
    pub sender_id: Option<i64>,
    pub sender_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ConversationStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, ConversationStatus::Open | ConversationStatus::Pending)
    }
}

impl MessageType {
    pub fn is_customer(&self) -> bool {
        matches!(self, MessageType::Incoming)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_status_is_active() {
        assert!(ConversationStatus::Open.is_active());
        assert!(ConversationStatus::Pending.is_active());
        assert!(!ConversationStatus::Resolved.is_active());
        assert!(!ConversationStatus::Snoozed.is_active());
    }

    #[test]
    fn test_message_type_is_customer() {
        assert!(MessageType::Incoming.is_customer());
        assert!(!MessageType::Outgoing.is_customer());
        assert!(!MessageType::Activity.is_customer());
        assert!(!MessageType::Template.is_customer());
    }

    #[test]
    fn test_contact_serialization() {
        let contact = Contact {
            id: 1,
            account_id: 1,
            name: "John Doe".to_string(),
            email: Some("john@example.com".to_string()),
            phone_number: None,
            identifier: None,
            custom_attributes: serde_json::json!({}),
            additional_attributes: serde_json::json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&contact).unwrap();
        let deserialized: Contact = serde_json::from_str(&json).unwrap();
        assert_eq!(contact.id, deserialized.id);
        assert_eq!(contact.name, deserialized.name);
    }

    #[test]
    fn test_inbox_serialization() {
        let inbox = Inbox {
            id: 1,
            account_id: 1,
            name: "Support Channel".to_string(),
            channel_type: "Channel::WebWidget".to_string(),
            enable_auto_assignment: true,
            greeting_enabled: false,
            greeting_message: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&inbox).unwrap();
        let deserialized: Inbox = serde_json::from_str(&json).unwrap();
        assert_eq!(inbox.id, deserialized.id);
        assert_eq!(inbox.name, deserialized.name);
    }

    #[test]
    fn test_conversation_serialization() {
        let conv = Conversation {
            id: 1,
            account_id: 1,
            inbox_id: 1,
            contact_id: 1,
            assignee_id: None,
            status: ConversationStatus::Open,
            custom_attributes: serde_json::json!({}),
            additional_attributes: serde_json::json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&conv).unwrap();
        let deserialized: Conversation = serde_json::from_str(&json).unwrap();
        assert_eq!(conv.id, deserialized.id);
        assert_eq!(conv.status, deserialized.status);
    }

    #[test]
    fn test_message_serialization() {
        let msg = Message {
            id: 1,
            account_id: 1,
            inbox_id: 1,
            conversation_id: 1,
            message_type: MessageType::Incoming,
            content: Some("Hello".to_string()),
            private: false,
            sender_id: None,
            sender_type: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(msg.id, deserialized.id);
        assert_eq!(msg.content, deserialized.content);
        assert_eq!(msg.message_type, deserialized.message_type);
    }
}
