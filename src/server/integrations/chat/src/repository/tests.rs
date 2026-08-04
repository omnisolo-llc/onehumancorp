#[cfg(test)]
mod tests {
    use sea_orm::*;
    use uuid::Uuid;
    use crate::models::{channel, contact, conversation, inbox, message};
    use crate::repository::{ChatRepository, ChatRepositoryImpl};

    async fn setup_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:").await.unwrap();

        db.execute(Statement::from_string(
            db.get_database_backend(),
            r#"
            CREATE TABLE chat_channels (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT NOT NULL,
                channel_type TEXT NOT NULL,
                provider_config TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE chat_inboxes (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                channel_id TEXT,
                name TEXT NOT NULL,
                is_default BOOLEAN DEFAULT false,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (channel_id) REFERENCES chat_channels(id) ON DELETE SET NULL
            );
            CREATE TABLE chat_contacts (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT,
                email TEXT,
                phone_number TEXT,
                identifier TEXT,
                custom_attributes TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE chat_conversations (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                inbox_id TEXT NOT NULL,
                contact_id TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'open',
                priority TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (inbox_id) REFERENCES chat_inboxes(id) ON DELETE CASCADE,
                FOREIGN KEY (contact_id) REFERENCES chat_contacts(id) ON DELETE CASCADE
            );
            CREATE TABLE chat_messages (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                conversation_id TEXT NOT NULL,
                content TEXT NOT NULL,
                message_type TEXT NOT NULL,
                sender_type TEXT NOT NULL,
                sender_id TEXT,
                is_draft BOOLEAN DEFAULT false,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (conversation_id) REFERENCES chat_conversations(id) ON DELETE CASCADE
            );
            "#.to_owned(),
        )).await.unwrap();

        db
    }

    #[tokio::test]
    async fn test_tenant_isolation_messages() {
        let db = setup_db().await;
        let repo = ChatRepositoryImpl;

        let tenant_a = Uuid::new_v4();
        let tenant_b = Uuid::new_v4();

        // Setup Channel
        let channel_a = repo.create_channel(&db, channel::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_a),
            name: Set("WhatsApp A".to_string()),
            channel_type: Set("whatsapp".to_string()),
            ..Default::default()
        }).await.unwrap();

        let channel_b = repo.create_channel(&db, channel::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_b),
            name: Set("WhatsApp B".to_string()),
            channel_type: Set("whatsapp".to_string()),
            ..Default::default()
        }).await.unwrap();

        // Setup Inbox
        let inbox_a = repo.create_inbox(&db, inbox::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_a),
            channel_id: Set(Some(channel_a.id)),
            name: Set("Inbox A".to_string()),
            ..Default::default()
        }).await.unwrap();

        let inbox_b = repo.create_inbox(&db, inbox::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_b),
            channel_id: Set(Some(channel_b.id)),
            name: Set("Inbox B".to_string()),
            ..Default::default()
        }).await.unwrap();

        // Setup Contact
        let contact_a = repo.create_contact(&db, contact::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_a),
            name: Set(Some("Alice".to_string())),
            ..Default::default()
        }).await.unwrap();

        let contact_b = repo.create_contact(&db, contact::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_b),
            name: Set(Some("Bob".to_string())),
            ..Default::default()
        }).await.unwrap();

        // Setup Conversation
        let conv_a = repo.create_conversation(&db, conversation::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_a),
            inbox_id: Set(inbox_a.id),
            contact_id: Set(contact_a.id),
            status: Set("open".to_string()),
            ..Default::default()
        }).await.unwrap();

        let conv_b = repo.create_conversation(&db, conversation::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_b),
            inbox_id: Set(inbox_b.id),
            contact_id: Set(contact_b.id),
            status: Set("open".to_string()),
            ..Default::default()
        }).await.unwrap();

        // Setup Message
        repo.create_message(&db, message::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_a),
            conversation_id: Set(conv_a.id),
            content: Set("Hello from A".to_string()),
            message_type: Set("incoming".to_string()),
            sender_type: Set("contact".to_string()),
            ..Default::default()
        }).await.unwrap();

        repo.create_message(&db, message::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_b),
            conversation_id: Set(conv_b.id),
            content: Set("Hello from B".to_string()),
            message_type: Set("incoming".to_string()),
            sender_type: Set("contact".to_string()),
            ..Default::default()
        }).await.unwrap();

        // Verify isolation
        let messages_a = repo.get_messages_by_conversation(&db, tenant_a, conv_a.id).await.unwrap();
        assert_eq!(messages_a.len(), 1);
        assert_eq!(messages_a[0].content, "Hello from A");

        let messages_b_for_a = repo.get_messages_by_conversation(&db, tenant_a, conv_b.id).await.unwrap();
        assert_eq!(messages_b_for_a.len(), 0); // Tenant A cannot see Tenant B's conversation messages

        let messages_b = repo.get_messages_by_conversation(&db, tenant_b, conv_b.id).await.unwrap();
        assert_eq!(messages_b.len(), 1);
        assert_eq!(messages_b[0].content, "Hello from B");

        let conv_b_for_a = repo.get_conversation_by_id(&db, tenant_a, conv_b.id).await.unwrap();
        assert!(conv_b_for_a.is_none());

        let inbox_b_for_a = repo.get_inbox_by_id(&db, tenant_a, inbox_b.id).await.unwrap();
        assert!(inbox_b_for_a.is_none());
    }
}
