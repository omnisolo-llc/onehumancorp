use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConversationStatus {
    Open,
    Pending,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub status: ConversationStatus,
    pub subject: String,
}

impl Conversation {
    pub fn new(id: impl Into<String>, tenant_id: impl Into<String>, inbox_id: impl Into<String>, contact_id: impl Into<String>, status: ConversationStatus, subject: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            inbox_id: inbox_id.into(),
            contact_id: contact_id.into(),
            status,
            subject: subject.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_creation() {
        let conv = Conversation::new("conv1", "t1", "inbox1", "c1", ConversationStatus::Open, "Need help");
        assert_eq!(conv.id, "conv1");
        assert_eq!(conv.tenant_id, "t1");
        assert_eq!(conv.inbox_id, "inbox1");
        assert_eq!(conv.contact_id, "c1");
        assert_eq!(conv.status, ConversationStatus::Open);
        assert_eq!(conv.subject, "Need help");
    }

    #[test]
    fn test_conversation_serialization() {
        let conv = Conversation::new("conv1", "t1", "inbox1", "c1", ConversationStatus::Resolved, "Done");
        let serialized = serde_json::to_string(&conv).unwrap();
        let deserialized: Conversation = serde_json::from_str(&serialized).unwrap();
        assert_eq!(conv, deserialized);
    }
}
