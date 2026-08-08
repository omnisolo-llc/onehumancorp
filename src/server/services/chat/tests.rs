use super::service::ChatService;
use sqlx::PgPool;
use uuid::Uuid;
use std::env;

#[tokio::test]
async fn test_chat_service() {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
    let maybe_pool = PgPool::connect(&database_url).await;
    if maybe_pool.is_err() {
        return;
    }
    let pool = maybe_pool.unwrap();

    // Create tables if they don't exist
    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS chat_inboxes (
            id UUID PRIMARY KEY,
            tenant_id UUID NOT NULL,
            name TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        );
        CREATE TABLE IF NOT EXISTS chat_channels (
            id UUID PRIMARY KEY,
            tenant_id UUID NOT NULL,
            inbox_id UUID NOT NULL,
            channel_type TEXT NOT NULL,
            config JSONB NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        );
        CREATE TABLE IF NOT EXISTS chat_contacts (
            id UUID PRIMARY KEY,
            tenant_id UUID NOT NULL,
            name TEXT,
            email TEXT,
            phone TEXT,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        );
        CREATE TABLE IF NOT EXISTS chat_conversations (
            id UUID PRIMARY KEY,
            tenant_id UUID NOT NULL,
            inbox_id UUID NOT NULL,
            contact_id UUID NOT NULL,
            assignee_id UUID,
            status TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        );
        CREATE TABLE IF NOT EXISTS chat_messages (
            id UUID PRIMARY KEY,
            tenant_id UUID NOT NULL,
            conversation_id UUID NOT NULL,
            sender_type TEXT NOT NULL,
            sender_id UUID,
            content TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        );"
    ).execute(&pool).await.unwrap();

    let service = ChatService::new(pool);
    let tenant_id = Uuid::new_v4();

    let inbox = service.create_inbox(tenant_id, "Main Inbox".to_string()).await.unwrap();
    assert_eq!(inbox.name, "Main Inbox");

    let channel = service.create_channel(tenant_id, inbox.id, "whatsapp".to_string(), serde_json::json!({})).await.unwrap();
    assert_eq!(channel.channel_type, "whatsapp");

    let contact = service.create_contact(tenant_id, Some("John Doe".to_string()), None, None).await.unwrap();
    assert_eq!(contact.name, Some("John Doe".to_string()));

    let conversation = service.start_conversation(tenant_id, inbox.id, contact.id, None).await.unwrap();
    assert_eq!(conversation.status, "open");

    let message = service.send_message(tenant_id, conversation.id, "contact".to_string(), Some(contact.id), "Hello!".to_string()).await.unwrap();
    assert_eq!(message.content, "Hello!");
}
