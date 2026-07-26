pub mod models;
pub mod db;
pub mod service;
pub mod ws;

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use chrono::Utc;
    use models::{ChatInbox, ChatMessage};

    #[test]
    fn test_models() {
        let tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();
        let inbox = ChatInbox {
            id: inbox_id,
            tenant_id,
            name: "Test Inbox".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        assert_eq!(inbox.name, "Test Inbox");

        let msg = ChatMessage {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id: Uuid::new_v4(),
            sender_type: "agent".to_string(),
            sender_id: None,
            content: "Hello".to_string(),
            status: "sent".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        assert_eq!(msg.content, "Hello");
    }
}
