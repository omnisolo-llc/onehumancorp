use super::service::ChatService;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_chat_service_basics() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

    let maybe_pool = PgPool::connect(&database_url).await;
    if maybe_pool.is_err() {
        return;
    }
    let pool = maybe_pool.unwrap();

    // We provide a None client just to check compilation and basic DB ops in the test.
    let service = ChatService::new(pool.clone(), None);
    let tenant_id = Uuid::new_v4();

    // Ensure the tables exist for test
    let _ = sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS chat_inboxes (
            id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
        );
        CREATE TABLE IF NOT EXISTS chat_channels (
            id UUID PRIMARY KEY, tenant_id UUID NOT NULL, inbox_id UUID NOT NULL, channel_type TEXT NOT NULL, config JSONB DEFAULT '{}'::jsonb, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
        );
        CREATE TABLE IF NOT EXISTS chat_contacts (
            id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT, email TEXT, phone TEXT, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
        );
        CREATE TABLE IF NOT EXISTS chat_conversations (
            id UUID PRIMARY KEY, tenant_id UUID NOT NULL, inbox_id UUID NOT NULL, contact_id UUID NOT NULL, assignee_id UUID, status TEXT NOT NULL DEFAULT 'open', created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
        );
        CREATE TABLE IF NOT EXISTS chat_messages (
            id UUID PRIMARY KEY, tenant_id UUID NOT NULL, conversation_id UUID NOT NULL, sender_type TEXT NOT NULL, sender_id UUID, content TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
        );
    "#).execute(&pool).await;

    let inbox = service.create_inbox(tenant_id, "Test Inbox".to_string()).await.unwrap();
    let contact = service.create_contact(tenant_id, Some("Maya".to_string()), None, None).await.unwrap();
    let conv = service.start_conversation(tenant_id, inbox.id, contact.id, None).await.unwrap();
    let msg = service.send_message(tenant_id, conv.id, "contact".to_string(), Some(contact.id), "Hello".to_string()).await.unwrap();

    assert_eq!(msg.content, "Hello");
}
