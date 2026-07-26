use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub tenant_id: String,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub status: ConversationStatus,
    pub unread_count: u32,
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
            assignee_id: None,
            status: ConversationStatus::Open,
            unread_count: 0,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_conversation() {
        let tenant_id = "tenant_1".to_string();
        let inbox_id = Uuid::new_v4();
        let contact_id = Uuid::new_v4();

        let conv = Conversation::new(tenant_id.clone(), inbox_id, contact_id);

        assert_eq!(conv.tenant_id, tenant_id);
        assert_eq!(conv.inbox_id, inbox_id);
        assert_eq!(conv.contact_id, contact_id);
        assert_eq!(conv.status, ConversationStatus::Open);
        assert_eq!(conv.unread_count, 0);
        assert!(conv.assignee_id.is_none());
    }
}
