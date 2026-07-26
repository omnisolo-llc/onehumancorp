use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Conversation {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub status: String,
}

impl Conversation {
    pub fn new(
        id: String,
        tenant_id: String,
        inbox_id: String,
        contact_id: String,
        status: String,
    ) -> Self {
        Self {
            id,
            tenant_id,
            inbox_id,
            contact_id,
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_creation() {
        let conversation = Conversation::new(
            "conv_1".to_string(),
            "tenant_1".to_string(),
            "inbox_1".to_string(),
            "contact_1".to_string(),
            "open".to_string(),
        );

        assert_eq!(conversation.id, "conv_1");
        assert_eq!(conversation.tenant_id, "tenant_1");
        assert_eq!(conversation.inbox_id, "inbox_1");
        assert_eq!(conversation.contact_id, "contact_1");
        assert_eq!(conversation.status, "open");
    }
}
