#[cfg(test)]
mod tests {
    use crate::services::chat::service::ChatService;
    use sqlx::PgPool;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_chat_service_create_inbox() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://ohc:ohc@localhost:5432/ohc".to_string());
        let pool = match PgPool::connect(&database_url).await {
            Ok(p) => p,
            Err(_) => return, // Skip test if no db
        };

        let service = ChatService::new(pool.clone());
        let tenant_id = Uuid::new_v4().to_string();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS chat_inboxes (
                id UUID PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            );
        ").execute(&pool).await;

        let inbox = service.create_inbox(tenant_id.clone(), "Test Inbox".to_string()).await;

        assert!(inbox.is_ok());
        let inbox = inbox.unwrap();
        assert_eq!(inbox.name, "Test Inbox");
        assert_eq!(inbox.tenant_id, tenant_id);
    }
}
