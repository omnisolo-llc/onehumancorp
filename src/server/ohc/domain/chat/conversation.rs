use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConversationStatus {
    Open,
    Resolved,
    Snoozed,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conversation {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub status: ConversationStatus,
    pub assignee_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Conversation {
    pub fn new(tenant_id: String, inbox_id: String, contact_id: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            tenant_id,
            inbox_id,
            contact_id,
            status: ConversationStatus::Open,
            assignee_id: None,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_creation() {
        let tenant_id = "tenant-789".to_string();
        let inbox_id = "inbox-1".to_string();
        let contact_id = "contact-1".to_string();

        let conversation = Conversation::new(
            tenant_id.clone(),
            inbox_id.clone(),
            contact_id.clone()
        );

        assert!(!conversation.id.is_empty());
        assert_eq!(conversation.tenant_id, tenant_id);
        assert_eq!(conversation.inbox_id, inbox_id);
        assert_eq!(conversation.contact_id, contact_id);
        assert_eq!(conversation.status, ConversationStatus::Open);
        assert_eq!(conversation.assignee_id, None);
        assert!(conversation.created_at <= Utc::now());
        assert_eq!(conversation.created_at, conversation.updated_at);
    }
}
