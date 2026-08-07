#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use uuid::Uuid;
    use crate::domain::omnichat::repository::OmnichatRepository;

    #[sqlx::test]
    async fn test_omnichat_repository_flow(pool: PgPool) -> Result<(), sqlx::Error> {
        let repo = OmnichatRepository::new(pool.clone());
        let tenant_id = Uuid::new_v4();

        // Bypass RLS for test setup if necessary, or execute within a tenant context
        sqlx::query(&format!("SET app.current_tenant_id = '{}'", tenant_id))
            .execute(&pool)
            .await?;

        // 1. Create Inbox
        let inbox = repo.create_inbox(tenant_id, "Main Support").await?;
        assert_eq!(inbox.name, "Main Support");

        // 2. Link Adapter
        let config = serde_json::json!({"token": "fake-token"});
        let adapter = repo.link_channel_adapter(tenant_id, inbox.id, "whatsapp", config).await?;
        assert_eq!(adapter.channel_type, "whatsapp");

        // 3. Create Contact
        let contact = repo.create_contact(tenant_id, "Carlos", Some("carlos@example.com"), None).await?;
        assert_eq!(contact.name, "Carlos");

        // 4. Create Conversation
        let conv = repo.get_or_create_conversation(tenant_id, inbox.id, contact.id).await?;
        assert_eq!(conv.status, "open");

        // 5. Ingest Message
        let msg = repo.ingest_message(tenant_id, conv.id, Some(contact.id), "Hello, do you fix sinks?").await?;
        assert_eq!(msg.content, "Hello, do you fix sinks?");

        Ok(())
    }
}
