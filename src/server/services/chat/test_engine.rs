#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use sqlx::PgPool;
    use crate::services::chat::engine::ChatEngine;
    use tokio::sync::broadcast;

    // This is a comprehensive integration test for the service layer
    // utilizing sqlx::test for a real database connection in CI.

    #[sqlx::test(migrations = "src/server/db/migrations")]
    async fn test_chat_engine_crud_and_isolation(pool: PgPool) -> sqlx::Result<()> {
        let (tx, _) = broadcast::channel(10);
        let engine = ChatEngine::new(pool.clone(), tx, None);
        let tenant_id_1 = "tenant_1".to_string();
        let tenant_id_2 = "tenant_2".to_string();

        // 1. Create Inboxes
        let inbox_1 = engine.create_inbox(tenant_id_1.clone(), "Support".into(), "web".into()).await?;
        assert_eq!(inbox_1.tenant_id, tenant_id_1);

        let inbox_2 = engine.create_inbox(tenant_id_2.clone(), "Sales".into(), "ig".into()).await?;
        assert_eq!(inbox_2.tenant_id, tenant_id_2);

        // 2. Create Contacts
        let contact_1 = engine.create_contact(tenant_id_1.clone(), Some("Maya".into()), Some("maya@example.com".into()), None).await?;
        assert_eq!(contact_1.tenant_id, tenant_id_1);

        // 3. Start Conversations
        let conv_1 = engine.start_conversation(tenant_id_1.clone(), inbox_1.id, Some(contact_1.id), None).await?;
        assert_eq!(conv_1.tenant_id, tenant_id_1);

        // 4. Send Messages
        let msg_1 = engine.send_message(
            tenant_id_1.clone(),
            conv_1.id,
            "customer".into(),
            Some(contact_1.id.to_string()),
            "Do you do vegan cakes?".into(),
            "text".into()
        ).await?;
        assert_eq!(msg_1.tenant_id, tenant_id_1);

        // 5. Test AI Lock (Fallback since Redis is None)
        let lock_acquired = engine.acquire_ai_draft_lock(tenant_id_1.clone(), conv_1.id).await.unwrap();
        assert!(lock_acquired);

        let lock_released = engine.release_ai_draft_lock(tenant_id_1.clone(), conv_1.id).await;
        assert!(lock_released.is_ok());

        // Multi-tenant isolation: Attempt to read Tenant 1's inbox using Tenant 2's session
        let mut tx = pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&tenant_id_2)
            .execute(&mut *tx)
            .await?;

        let row: Result<(Uuid,), sqlx::Error> = sqlx::query_as("SELECT id FROM chat_inboxes WHERE id = $1")
            .bind(inbox_1.id)
            .fetch_one(&mut *tx)
            .await;

        assert!(matches!(row, Err(sqlx::Error::RowNotFound)), "RLS failed: Tenant 2 was able to read Tenant 1's inbox");

        tx.commit().await?;

        Ok(())
    }
}
