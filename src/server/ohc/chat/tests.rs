#[cfg(test)]
mod tests {
    use crate::chat::domain::models::*;
    use uuid::Uuid;
    use chrono::Utc;

    #[test]
    fn test_inbox_creation() {
        let inbox = Inbox {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "WhatsApp Support".to_string(),
            channel_type: "whatsapp".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        assert_eq!(inbox.name, "WhatsApp Support");
        assert_eq!(inbox.channel_type, "whatsapp");
    }

    #[test]
    fn test_conversation_creation() {
        let conversation = Conversation {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            status: "open".to_string(),
            last_activity_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        assert_eq!(conversation.status, "open");
    }

    #[test]
    fn test_message_creation() {
        let message = Message {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            content: "Hello from OHC!".to_string(),
            message_type: "text".to_string(),
            sender_id: Uuid::new_v4(),
            sender_type: "human".to_string(),
            status: "sent".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        assert_eq!(message.content, "Hello from OHC!");
        assert_eq!(message.status, "sent");
    }
}
