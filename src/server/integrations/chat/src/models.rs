use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConversationStatus {
    Open,
    Resolved,
    Pending,
    Snoozed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: String,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub status: ConversationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Incoming,
    Outgoing,
    Template,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: String,
    pub conversation_id: Uuid,
    pub content: String,
    pub message_type: MessageType,
    pub sender_id: Option<Uuid>,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CannedResponse {
    pub id: Uuid,
    pub tenant_id: String,
    pub short_code: String,
    pub content: String,
}

impl Contact {
    pub fn new(tenant_id: &str, name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            tenant_id: tenant_id.to_string(),
            name: name.to_string(),
            email: None,
            phone_number: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl CannedResponse {
    pub fn new(tenant_id: &str, short_code: &str, content: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            tenant_id: tenant_id.to_string(),
            short_code: short_code.to_string(),
            content: content.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_new() {
        let contact = Contact::new("tenant1", "Alice");
        assert_eq!(contact.tenant_id, "tenant1");
        assert_eq!(contact.name, "Alice");
    }

    #[test]
    fn test_canned_response_new() {
        let canned = CannedResponse::new("tenant1", "greet", "Hello!");
        assert_eq!(canned.tenant_id, "tenant1");
        assert_eq!(canned.short_code, "greet");
        assert_eq!(canned.content, "Hello!");
    }
}
