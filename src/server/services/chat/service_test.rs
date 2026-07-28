#[cfg(test)]
mod tests {
    use crate::services::chat::service::ChatService;
    use crate::services::chat::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};
    use uuid::Uuid;
    use sqlx::PgPool;

    // A mock pool setup might be needed if they have a standard one, otherwise this is a skeleton test.
    // Real tests would spin up a test db via something like sqlx::test

    #[sqlx::test]
    async fn test_create_inbox(pool: PgPool) {
        let service = ChatService::new(pool);
        let tenant_id = Uuid::new_v4();

        let inbox = service.create_inbox(tenant_id, "Main Inbox".to_string()).await.unwrap();
        assert_eq!(inbox.tenant_id, tenant_id);
        assert_eq!(inbox.name, "Main Inbox");
    }

    #[sqlx::test]
    async fn test_create_channel(pool: PgPool) {
        let service = ChatService::new(pool);
        let tenant_id = Uuid::new_v4();
        let inbox = service.create_inbox(tenant_id, "Main Inbox".to_string()).await.unwrap();

        let config = serde_json::json!({"token": "123"});
        let channel = service.create_channel(tenant_id, inbox.id, "whatsapp".to_string(), config.clone()).await.unwrap();

        assert_eq!(channel.tenant_id, tenant_id);
        assert_eq!(channel.inbox_id, inbox.id);
        assert_eq!(channel.channel_type, "whatsapp");
    }

    #[sqlx::test]
    async fn test_create_contact(pool: PgPool) {
        let service = ChatService::new(pool);
        let tenant_id = Uuid::new_v4();

        let contact = service.create_contact(tenant_id, Some("John Doe".to_string()), Some("john@example.com".to_string()), None).await.unwrap();

        assert_eq!(contact.tenant_id, tenant_id);
        assert_eq!(contact.name, Some("John Doe".to_string()));
    }

    #[sqlx::test]
    async fn test_start_conversation_and_send_message(pool: PgPool) {
        let service = ChatService::new(pool);
        let tenant_id = Uuid::new_v4();
        let inbox = service.create_inbox(tenant_id, "Main Inbox".to_string()).await.unwrap();
        let contact = service.create_contact(tenant_id, Some("John Doe".to_string()), None, None).await.unwrap();

        let conversation = service.start_conversation(tenant_id, inbox.id, contact.id, None).await.unwrap();
        assert_eq!(conversation.tenant_id, tenant_id);
        assert_eq!(conversation.inbox_id, inbox.id);
        assert_eq!(conversation.contact_id, contact.id);
        assert_eq!(conversation.status, "open");

        let message = service.send_message(tenant_id, conversation.id, "contact".to_string(), Some(contact.id), "Hello!".to_string()).await.unwrap();
        assert_eq!(message.tenant_id, tenant_id);
        assert_eq!(message.conversation_id, conversation.id);
        assert_eq!(message.content, "Hello!");
    }
}
