use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub status: String,
}

impl Conversation {
    pub fn new(id: String, tenant_id: String, inbox_id: String, contact_id: String, status: String) -> Self {
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
            "conv-1".to_string(),
            "tenant-1".to_string(),
            "inbox-1".to_string(),
            "contact-1".to_string(),
            "open".to_string(),
        );
        assert_eq!(conversation.id, "conv-1");
        assert_eq!(conversation.tenant_id, "tenant-1");
        assert_eq!(conversation.inbox_id, "inbox-1");
        assert_eq!(conversation.contact_id, "contact-1");
        assert_eq!(conversation.status, "open");
    }
}
