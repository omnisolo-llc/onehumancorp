use super::models::{ChatInbox, ChatChannel, ChatContact, ChatContactInbox, ChatConversation, ChatMessage};
use super::service::ChatService;
use sqlx::PgPool;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chat_service_instantiation() {
        // Without a database, we can only verify it compiles and instantiates
        // the structs correctly. Full CRUD requires a DB instance.
        let tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();
        let contact_id = Uuid::new_v4();

        let inbox = ChatInbox {
            id: inbox_id,
            tenant_id,
            name: "Support".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        assert_eq!(inbox.name, "Support");

        let contact_inbox = ChatContactInbox {
            id: Uuid::new_v4(),
            tenant_id,
            contact_id,
            inbox_id,
            source_id: "insta-123".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        assert_eq!(contact_inbox.source_id, "insta-123");
    }
}
