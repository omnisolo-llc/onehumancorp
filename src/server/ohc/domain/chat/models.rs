use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConversationStatus {
    Open,
    Resolved,
    Pending,
    Snoozed,
}

impl Default for ConversationStatus {
    fn default() -> Self {
        ConversationStatus::Open
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub account_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub status: ConversationStatus,
    pub additional_attributes: Option<Value>,
    pub custom_attributes: Option<Value>,
    pub snoozed_until: Option<DateTime<Utc>>,
    pub last_activity_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Incoming,
    Outgoing,
    Activity,
    Template,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageStatus {
    Sent,
    Delivered,
    Read,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub account_id: Uuid,
    pub inbox_id: Uuid,
    pub conversation_id: Uuid,
    pub message_type: MessageType,
    pub content: Option<String>,
    pub status: MessageStatus,
    pub sender_id: Option<Uuid>,
    pub sender_type: Option<String>,
    pub source_id: Option<String>,
    pub additional_attributes: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub account_id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub avatar_url: Option<String>,
    pub identifier: Option<String>,
    pub additional_attributes: Option<Value>,
    pub custom_attributes: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_initialization() {
        let tenant_id = Uuid::new_v4();
        let conversation = Conversation {
            id: Uuid::new_v4(),
            tenant_id,
            account_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            assignee_id: None,
            status: ConversationStatus::Open,
            additional_attributes: None,
            custom_attributes: None,
            snoozed_until: None,
            last_activity_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(conversation.status, ConversationStatus::Open);
        assert_eq!(conversation.tenant_id, tenant_id);
    }

    #[test]
    fn test_message_initialization() {
        let tenant_id = Uuid::new_v4();
        let message = Message {
            id: Uuid::new_v4(),
            tenant_id,
            account_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            message_type: MessageType::Incoming,
            content: Some("Hello".to_string()),
            status: MessageStatus::Delivered,
            sender_id: None,
            sender_type: None,
            source_id: None,
            additional_attributes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(message.message_type, MessageType::Incoming);
        assert_eq!(message.status, MessageStatus::Delivered);
        assert_eq!(message.tenant_id, tenant_id);
    }

    #[test]
    fn test_contact_initialization() {
        let tenant_id = Uuid::new_v4();
        let contact = Contact {
            id: Uuid::new_v4(),
            tenant_id,
            account_id: Uuid::new_v4(),
            name: Some("John Doe".to_string()),
            email: Some("john@example.com".to_string()),
            phone_number: None,
            avatar_url: None,
            identifier: None,
            additional_attributes: None,
            custom_attributes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(contact.name.unwrap(), "John Doe");
        assert_eq!(contact.tenant_id, tenant_id);
    }
}
