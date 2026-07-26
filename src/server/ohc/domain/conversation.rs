use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConversationStatus {
    Open,
    Snoozed,
    Resolved,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Conversation {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub status: ConversationStatus,
    pub subject: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Conversation {
    pub fn new(
        id: String,
        tenant_id: String,
        inbox_id: String,
        contact_id: String,
        subject: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            tenant_id,
            inbox_id,
            contact_id,
            status: ConversationStatus::Open,
            subject,
            created_at: now,
            updated_at: now,
        }
    }
}

#[async_trait::async_trait]
pub trait ConversationService: Send + Sync {
    async fn create_conversation(
        &self,
        tenant_id: &str,
        inbox_id: &str,
        contact_id: &str,
        subject: Option<String>,
    ) -> Result<Conversation, String>;

    async fn get_conversation(&self, tenant_id: &str, id: &str) -> Result<Option<Conversation>, String>;

    async fn update_status(&self, tenant_id: &str, id: &str, status: ConversationStatus) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_creation_and_serde() {
        let conversation = Conversation::new(
            "conv-1".to_string(),
            "tenant-abc".to_string(),
            "inbox-1".to_string(),
            "contact-1".to_string(),
            Some("Need help with pricing".to_string()),
        );

        assert_eq!(conversation.id, "conv-1");
        assert_eq!(conversation.tenant_id, "tenant-abc");
        assert_eq!(conversation.inbox_id, "inbox-1");
        assert_eq!(conversation.contact_id, "contact-1");
        assert_eq!(conversation.status, ConversationStatus::Open);
        assert_eq!(conversation.subject, Some("Need help with pricing".to_string()));

        let serialized = serde_json::to_string(&conversation).unwrap();
        let deserialized: Conversation = serde_json::from_str(&serialized).unwrap();
        assert_eq!(conversation.id, deserialized.id);
        assert_eq!(conversation.tenant_id, deserialized.tenant_id);
        assert_eq!(conversation.inbox_id, deserialized.inbox_id);
        assert_eq!(conversation.contact_id, deserialized.contact_id);
        assert_eq!(conversation.status, deserialized.status);
        assert_eq!(conversation.subject, deserialized.subject);
    }

    #[test]
    fn test_conversation_statuses() {
        let statuses = vec![
            ConversationStatus::Open,
            ConversationStatus::Snoozed,
            ConversationStatus::Resolved,
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: ConversationStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(status, deserialized);
        }
    }
}
