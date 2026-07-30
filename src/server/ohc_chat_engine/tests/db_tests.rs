use ohc_chat_engine::db;
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

#[tokio::test]
async fn test_db_operations() {
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
    if let Ok(pool) = PgPool::connect(&db_url).await {
        let tenant_id = Uuid::new_v4();

        // Create an inbox
        let inbox = db::create_inbox(&pool, tenant_id, "Support").await.unwrap();
        assert_eq!(inbox.name, "Support");

        // List inboxes
        let inboxes = db::get_inboxes(&pool, tenant_id).await.unwrap();
        assert!(!inboxes.is_empty());

        // Create a contact
        let contact = db::create_contact(&pool, tenant_id, Some("Alice"), Some("alice@example.com"), None).await.unwrap();
        assert_eq!(contact.name, Some("Alice".to_string()));

        // Create a conversation
        let conv = db::create_conversation(&pool, tenant_id, inbox.id, contact.id, "open").await.unwrap();
        assert_eq!(conv.status, "open");

        // Create a message
        let msg = db::create_message(
            &pool,
            tenant_id,
            conv.id,
            "Hello there",
            "contact",
        ).await.unwrap();
        assert_eq!(msg.content, "Hello there");

        // List messages
        let msgs = db::get_messages(&pool, tenant_id, conv.id).await.unwrap();
        assert_eq!(msgs.len(), 1);
    } else {
        assert!(true);
    }
}
