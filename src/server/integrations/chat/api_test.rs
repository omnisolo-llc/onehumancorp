#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{CreateInboxReq, CreateContactReq, CreateConversationReq, CreateMessageReq};

    #[tokio::test]
    async fn test_create_inbox() {
        let req = CreateInboxReq {
            name: "Test Inbox".to_string(),
            channel_type: "email".to_string(),
            channel_config: None,
        };
        assert_eq!(req.name, "Test Inbox");
        assert_eq!(req.channel_type, "email");
    }

    #[tokio::test]
    async fn test_create_contact() {
        let req = CreateContactReq {
            name: Some("Test Contact".to_string()),
            identifier: "test@example.com".to_string(),
            attributes: None,
        };
        assert_eq!(req.identifier, "test@example.com");
        assert_eq!(req.name.unwrap(), "Test Contact");
    }

    #[tokio::test]
    async fn test_create_conversation() {
        let req = CreateConversationReq {
            inbox_id: "inbox-1".to_string(),
            contact_id: "contact-1".to_string(),
        };
        assert_eq!(req.inbox_id, "inbox-1");
        assert_eq!(req.contact_id, "contact-1");
    }

    #[tokio::test]
    async fn test_create_message() {
        let req = CreateMessageReq {
            sender_id: Some("agent-1".to_string()),
            sender_type: "agent".to_string(),
            content: Some("Hello".to_string()),
            message_type: "outgoing".to_string(),
            additional_attributes: None,
        };
        assert_eq!(req.sender_type, "agent");
        assert_eq!(req.message_type, "outgoing");
        assert_eq!(req.content.unwrap(), "Hello");
        assert_eq!(req.sender_id.unwrap(), "agent-1");
    }
}
