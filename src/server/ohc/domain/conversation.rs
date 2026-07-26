use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub tenant_id: String,
    pub id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Conversation {
    pub fn new(tenant_id: String, inbox_id: Uuid, contact_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            tenant_id,
            id: Uuid::new_v4(),
            inbox_id,
            contact_id,
            status: "open".to_string(), // default status
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
        let tenant_id = "tenant-123".to_string();
        let inbox_id = Uuid::new_v4();
        let contact_id = Uuid::new_v4();

        let conversation = Conversation::new(tenant_id.clone(), inbox_id, contact_id);

        assert_eq!(conversation.tenant_id, tenant_id);
        assert_eq!(conversation.inbox_id, inbox_id);
        assert_eq!(conversation.contact_id, contact_id);
        assert_eq!(conversation.status, "open");
        assert!(!conversation.id.is_nil());
        assert!(conversation.created_at <= Utc::now());
        assert_eq!(conversation.created_at, conversation.updated_at);
    }
}
