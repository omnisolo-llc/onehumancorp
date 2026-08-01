use sqlx::PgPool;
use uuid::Uuid;
use super::service::ChatService;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_db() -> PgPool {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://ohc:ohc@localhost:5432/ohc".to_string());
        // For testing we just assume the pool connects and migrations are run,
        // consistent with other tests in this repository.
        PgPool::connect(&db_url).await.expect("Failed to connect to test db")
    }

    #[tokio::test]
    async fn test_create_and_get_inbox() {
        let pool = setup_db().await;
        let service = ChatService::new(pool.clone());
        let tenant_id = Uuid::new_v4();

        let inbox = service.create_inbox(
            tenant_id,
            "Support".to_string(),
            "web_widget".to_string(),
            Some(serde_json::json!({"color": "blue"}))
        ).await.expect("Failed to create inbox");

        assert_eq!(inbox.tenant_id, tenant_id);
        assert_eq!(inbox.name, "Support");
        assert_eq!(inbox.channel_type, "web_widget");

        let fetched = service.get_inbox(tenant_id, inbox.id).await.expect("Failed to get inbox");
        assert_eq!(fetched.id, inbox.id);
    }

    #[tokio::test]
    async fn test_rls_tenant_isolation() {
        let pool = setup_db().await;
        let service = ChatService::new(pool.clone());
        let tenant_a = Uuid::new_v4();
        let tenant_b = Uuid::new_v4();

        let inbox = service.create_inbox(
            tenant_a,
            "Tenant A Inbox".to_string(),
            "email".to_string(),
            None
        ).await.expect("Failed to create inbox");

        // Attempting to fetch Tenant A's inbox using Tenant B's context should fail (RowNotFound)
        let result = service.get_inbox(tenant_b, inbox.id).await;
        assert!(result.is_err(), "Should not be able to fetch another tenant's inbox");
    }

    #[tokio::test]
    async fn test_conversation_flow() {
        let pool = setup_db().await;
        let service = ChatService::new(pool.clone());
        let tenant_id = Uuid::new_v4();

        let inbox = service.create_inbox(tenant_id, "Sales".into(), "sms".into(), None).await.unwrap();
        let contact = service.create_contact(tenant_id, Some("John".into()), None, Some("1234567890".into())).await.unwrap();

        let contact_inbox = service.create_contact_inbox(tenant_id, contact.id, inbox.id, Some("sms_123".into())).await.unwrap();
        assert_eq!(contact_inbox.contact_id, contact.id);

        let conv = service.create_conversation(tenant_id, inbox.id, contact.id, "open".into(), None).await.unwrap();

        let msg = service.create_message(tenant_id, conv.id, "Hello".into(), "contact".into(), Some(contact.id)).await.unwrap();
        assert_eq!(msg.content, "Hello");

        let messages = service.list_messages_for_conversation(tenant_id, conv.id).await.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].id, msg.id);
    }
}
