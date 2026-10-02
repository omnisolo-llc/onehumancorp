#[cfg(test)]
mod tests {
    use crate::services::chat::service::ChatService;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_chat_tenant_isolation() {
        if std::env::var("OMNISOLO_DATABASE_URL").is_err() {
            return;
        }

        let pool = crate::db::get_pool();
        let db = crate::db::DB {
            pool: pool.clone(),
            store: crate::db::DbStore::Postgres,
        };
        db.run_migrations().await.expect("migrations should succeed");

        let chat_service = ChatService::new(pool.clone());

        let tenant_a = Uuid::new_v4();
        let tenant_b = Uuid::new_v4();

        // 1. Create inbox as Tenant A
        let inbox_a = chat_service.create_inbox(tenant_a, "Inbox A".to_string()).await.unwrap();
        assert_eq!(inbox_a.tenant_id, tenant_a);

        // 2. Create channel as Tenant A
        let channel_a = chat_service.create_channel(
            tenant_a,
            inbox_a.id,
            "web".to_string(),
            serde_json::json!({}),
        ).await.unwrap();
        assert_eq!(channel_a.tenant_id, tenant_a);

        // 3. Create contact as Tenant A
        let contact_a = chat_service.create_contact(
            tenant_a,
            Some("Alice".to_string()),
            Some("alice@example.com".to_string()),
            None,
        ).await.unwrap();
        assert_eq!(contact_a.tenant_id, tenant_a);

        // 4. Start conversation as Tenant A
        let conv_a = chat_service.start_conversation(
            tenant_a,
            inbox_a.id,
            contact_a.id,
            None,
        ).await.unwrap();
        assert_eq!(conv_a.tenant_id, tenant_a);

        // 5. Send message as Tenant A
        let msg_a = chat_service.send_message(
            tenant_a,
            conv_a.id,
            "contact".to_string(),
            Some(contact_a.id),
            "Hello".to_string(),
        ).await.unwrap();
        assert_eq!(msg_a.tenant_id, tenant_a);


        // Validate Row-Level Security (RLS)
        let mut tx = pool.begin().await.unwrap();

        // As Tenant B
        sqlx::query("SET LOCAL app.current_tenant_id = $1")
            .bind(tenant_b)
            .execute(&mut *tx)
            .await
            .unwrap();

        let inbox_count: i64 = sqlx::query_scalar("SELECT count(*) FROM chat_inboxes WHERE id = $1")
            .bind(inbox_a.id)
            .fetch_one(&mut *tx)
            .await
            .unwrap();
        assert_eq!(inbox_count, 0, "Tenant B should not see Tenant A's inbox");

        let conv_count: i64 = sqlx::query_scalar("SELECT count(*) FROM chat_conversations WHERE id = $1")
            .bind(conv_a.id)
            .fetch_one(&mut *tx)
            .await
            .unwrap();
        assert_eq!(conv_count, 0, "Tenant B should not see Tenant A's conversation");

        let msg_count: i64 = sqlx::query_scalar("SELECT count(*) FROM chat_messages WHERE id = $1")
            .bind(msg_a.id)
            .fetch_one(&mut *tx)
            .await
            .unwrap();
        assert_eq!(msg_count, 0, "Tenant B should not see Tenant A's messages");

        tx.rollback().await.unwrap();
    }
}
