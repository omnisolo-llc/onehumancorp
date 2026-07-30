use sqlx::{PgPool, Row, Executor};
use uuid::Uuid;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::*;

#[tokio::test]
async fn test_models_fields() {
    let tenant_id = Uuid::new_v4();
    let inbox_id = Uuid::new_v4();
    let inbox = ChatInbox {
        id: inbox_id, tenant_id, name: "Support".to_string(),
        created_at: chrono::Utc::now(), updated_at: chrono::Utc::now(),
    };
    assert_eq!(inbox.name, "Support");
}

#[sqlx::test]
async fn test_chat_service_crud_methods(pool: PgPool) {
    // Note: Due to limitations of using sqlx::test in this specific sandbox environment without actual migrations running beforehand,
    // we explicitly execute the table creation queries first so the tests pass hermetically.
    pool.execute(r#"
        CREATE TABLE IF NOT EXISTS chat_inboxes (
            id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
        );
        CREATE TABLE IF NOT EXISTS chat_channels (
            id UUID PRIMARY KEY, tenant_id UUID NOT NULL, inbox_id UUID NOT NULL, channel_type TEXT NOT NULL, config JSONB, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
        );
        CREATE TABLE IF NOT EXISTS chat_contacts (
            id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT, email TEXT, phone TEXT, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
        );
        CREATE TABLE IF NOT EXISTS chat_contact_inboxes (
            id UUID PRIMARY KEY, tenant_id UUID NOT NULL, contact_id UUID NOT NULL, inbox_id UUID NOT NULL, source_id TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
        );
        CREATE TABLE IF NOT EXISTS chat_conversations (
            id UUID PRIMARY KEY, tenant_id UUID NOT NULL, inbox_id UUID NOT NULL, contact_id UUID NOT NULL, assignee_id UUID, status TEXT NOT NULL DEFAULT 'open', created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
        );
        CREATE TABLE IF NOT EXISTS chat_messages (
            id UUID PRIMARY KEY, tenant_id UUID NOT NULL, conversation_id UUID NOT NULL, sender_type TEXT NOT NULL, sender_id UUID, content TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
        );
        CREATE TABLE IF NOT EXISTS chat_canned_responses (
            id UUID PRIMARY KEY, tenant_id UUID NOT NULL, short_code TEXT NOT NULL, content TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
        );
    "#).await.unwrap();

    let service = ChatService::new(pool);
    let tenant_id = Uuid::new_v4();

    let inbox = service.create_inbox(tenant_id, "Test Inbox".to_string()).await.unwrap();
    assert_eq!(inbox.name, "Test Inbox");

    let channel = service.create_channel(tenant_id, inbox.id, "email".to_string(), serde_json::json!({})).await.unwrap();
    assert_eq!(channel.channel_type, "email");

    let contact = service.create_contact(tenant_id, Some("Bob".to_string()), None, None).await.unwrap();
    assert_eq!(contact.name.unwrap(), "Bob");

    let conv = service.start_conversation(tenant_id, inbox.id, contact.id, None).await.unwrap();
    assert_eq!(conv.status, "open");

    let msg = service.send_message(tenant_id, conv.id, "agent".to_string(), None, "Hello".to_string()).await.unwrap();
    assert_eq!(msg.content, "Hello");

    let link = service.link_contact_to_inbox(tenant_id, contact.id, inbox.id, "test_source".to_string()).await.unwrap();
    assert_eq!(link.source_id, "test_source");

    let canned = service.create_canned_response(tenant_id, "hi".to_string(), "Hi there!".to_string()).await.unwrap();
    assert_eq!(canned.short_code, "hi");
}
