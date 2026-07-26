use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: String,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Conversation {
    pub fn new(tenant_id: String, inbox_id: Uuid, contact_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            inbox_id,
            contact_id,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_new() {
        let tenant_id = "tenant-123".to_string();
        let inbox_id = Uuid::new_v4();
        let contact_id = Uuid::new_v4();

        let conversation = Conversation::new(tenant_id.clone(), inbox_id, contact_id);

        assert_eq!(conversation.tenant_id, tenant_id);
        assert_eq!(conversation.inbox_id, inbox_id);
        assert_eq!(conversation.contact_id, contact_id);
        assert!(!conversation.id.is_nil());
        assert!(conversation.created_at <= Utc::now());
        assert_eq!(conversation.created_at, conversation.updated_at);
    }

    #[test]
    fn test_conversation_serialization() {
        let conversation = Conversation::new(
            "tenant-123".to_string(),
            Uuid::new_v4(),
            Uuid::new_v4()
        );
        let serialized = serde_json::to_string(&conversation).unwrap();
        let deserialized: Conversation = serde_json::from_str(&serialized).unwrap();

        assert_eq!(conversation, deserialized);
    }
}
