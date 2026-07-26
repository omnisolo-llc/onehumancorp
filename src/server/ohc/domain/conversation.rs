use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConversationStatus {
    Open,
    Pending, // Waiting for customer reply, etc.
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Conversation {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub status: ConversationStatus,
    pub created_at_utc: i64,
    pub updated_at_utc: i64,
}

impl Conversation {
    pub fn new(
        id: String,
        tenant_id: String,
        inbox_id: String,
        contact_id: String,
        created_at_utc: i64,
    ) -> Self {
        Self {
            id,
            tenant_id,
            inbox_id,
            contact_id,
            status: ConversationStatus::Open,
            created_at_utc,
            updated_at_utc: created_at_utc,
        }
    }

    pub fn with_status(mut self, status: ConversationStatus, updated_at_utc: i64) -> Self {
        self.status = status;
        self.updated_at_utc = updated_at_utc;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_new() {
        let conversation = Conversation::new(
            "conv_1".to_string(),
            "tenant_1".to_string(),
            "inbox_1".to_string(),
            "contact_1".to_string(),
            1672531200,
        );

        assert_eq!(conversation.id, "conv_1");
        assert_eq!(conversation.tenant_id, "tenant_1");
        assert_eq!(conversation.inbox_id, "inbox_1");
        assert_eq!(conversation.contact_id, "contact_1");
        assert_eq!(conversation.status, ConversationStatus::Open);
        assert_eq!(conversation.created_at_utc, 1672531200);
        assert_eq!(conversation.updated_at_utc, 1672531200);
    }

    #[test]
    fn test_conversation_status_update() {
        let conversation = Conversation::new(
            "conv_1".to_string(),
            "tenant_1".to_string(),
            "inbox_1".to_string(),
            "contact_1".to_string(),
            1672531200,
        )
        .with_status(ConversationStatus::Resolved, 1672534800);

        assert_eq!(conversation.status, ConversationStatus::Resolved);
        assert_eq!(conversation.updated_at_utc, 1672534800);
    }

    #[test]
    fn test_conversation_serialization() {
        let conversation = Conversation::new(
            "conv_1".to_string(),
            "tenant_1".to_string(),
            "inbox_1".to_string(),
            "contact_1".to_string(),
            1672531200,
        )
        .with_status(ConversationStatus::Pending, 1672534800);

        let serialized = serde_json::to_string(&conversation).unwrap();
        let deserialized: Conversation = serde_json::from_str(&serialized).unwrap();

        assert_eq!(conversation, deserialized);
    }
}
