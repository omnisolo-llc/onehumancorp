use super::service::ChatService;
use sqlx::PgPool;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_create_and_get_conversation() {
        let pool_str = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
        if !pool_str.starts_with("postgres") { return; } // skip if not postgres test env
        let pool = PgPoolOptions::new().connect(&pool_str).await.unwrap();
        let service = ChatService::new(pool.clone());
        let tenant_id = Uuid::new_v4().to_string();

        sqlx::query("SELECT set_config('app.current_tenant', $1, false)").bind(&tenant_id).execute(&pool).await.unwrap();

        let inbox = service.create_inbox(&tenant_id, "Test Inbox", "Widget", None).await.unwrap();
        let contact = service.create_contact(&tenant_id, Some("John"), None, None, None).await.unwrap();

        let conversation = service.create_conversation(&tenant_id, inbox.id, contact.id, None).await.unwrap();
        let fetched = service.get_conversation(&tenant_id, conversation.id).await.unwrap();
        assert_eq!(conversation.id, fetched.id);
        assert_eq!(conversation.status, "open");
        assert_eq!(fetched.status, "open");
    }

    #[tokio::test]
    async fn test_create_and_get_message() {
        let pool_str = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
        if !pool_str.starts_with("postgres") { return; } // skip if not postgres test env
        let pool = PgPoolOptions::new().connect(&pool_str).await.unwrap();
        let service = ChatService::new(pool.clone());
        let tenant_id = Uuid::new_v4().to_string();

        sqlx::query("SELECT set_config('app.current_tenant', $1, false)").bind(&tenant_id).execute(&pool).await.unwrap();

        let inbox = service.create_inbox(&tenant_id, "Test Inbox", "Widget", None).await.unwrap();
        let contact = service.create_contact(&tenant_id, Some("John"), None, None, None).await.unwrap();
        let conversation = service.create_conversation(&tenant_id, inbox.id, contact.id, None).await.unwrap();

        let message = service.create_message(&tenant_id, conversation.id, "incoming", None, "hello world").await.unwrap();
        let fetched = service.get_message(&tenant_id, message.id).await.unwrap();
        assert_eq!(message.id, fetched.id);
        assert_eq!(message.content, "hello world");
        assert_eq!(fetched.content, "hello world");
    }
}
