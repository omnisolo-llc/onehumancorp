#[cfg(test)]
mod models_tests {
    use super::super::models::{ChatInbox, ChatContact, ChatConversation, ChatMessage};
    use uuid::Uuid;
    use chrono::Utc;

    #[test]
    fn test_chat_repository_models() {
        let tenant_id = Uuid::new_v4();

        let inbox = ChatInbox {
            id: Uuid::new_v4(),
            tenant_id,
            name: "Test Inbox".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        assert_eq!(inbox.name, "Test Inbox");

        let contact = ChatContact {
            id: Uuid::new_v4(),
            tenant_id,
            name: Some("Maya".to_string()),
            email: Some("maya@example.com".to_string()),
            phone: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        assert_eq!(contact.name, Some("Maya".to_string()));

        let conversation = ChatConversation {
            id: Uuid::new_v4(),
            tenant_id,
            inbox_id: inbox.id,
            contact_id: contact.id,
            assignee_id: None,
            status: "open".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        assert_eq!(conversation.status, "open");

        let message = ChatMessage {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id: conversation.id,
            sender_type: "contact".to_string(),
            sender_id: Some(contact.id),
            content: "Hello, I want to order a cake.".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        assert_eq!(message.content, "Hello, I want to order a cake.");
    }
}
