use uuid::Uuid;
use sqlx::postgres::PgPoolOptions;
use std::env;

use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};
use super::service::ChatService;

// We use the SQL directly in the test to avoid bazel runtime file dependency issues on include_str!
const CHAT_MODELS_MIGRATION: &str = r#"
CREATE TABLE IF NOT EXISTS chat_inboxes (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE chat_inboxes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS chat_inboxes_tenant_isolation_policy ON chat_inboxes;
CREATE POLICY chat_inboxes_tenant_isolation_policy ON chat_inboxes FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid) WITH CHECK (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_channels (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES chat_inboxes(id),
    channel_type TEXT NOT NULL,
    config JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE chat_channels ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS chat_channels_tenant_isolation_policy ON chat_channels;
CREATE POLICY chat_channels_tenant_isolation_policy ON chat_channels FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid) WITH CHECK (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_contacts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE chat_contacts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS chat_contacts_tenant_isolation_policy ON chat_contacts;
CREATE POLICY chat_contacts_tenant_isolation_policy ON chat_contacts FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid) WITH CHECK (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_conversations (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES chat_inboxes(id),
    contact_id UUID NOT NULL REFERENCES chat_contacts(id),
    assignee_id UUID,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE chat_conversations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS chat_conversations_tenant_isolation_policy ON chat_conversations;
CREATE POLICY chat_conversations_tenant_isolation_policy ON chat_conversations FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid) WITH CHECK (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_messages (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES chat_conversations(id),
    sender_type TEXT NOT NULL,
    sender_id UUID,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE chat_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS chat_messages_tenant_isolation_policy ON chat_messages;
CREATE POLICY chat_messages_tenant_isolation_policy ON chat_messages FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid) WITH CHECK (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
"#;

async fn setup_test_db() -> sqlx::PgPool {
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://ohc:ohc@localhost:5432/ohc".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to test DB");

    sqlx::query(CHAT_MODELS_MIGRATION)
        .execute(&pool)
        .await
        .ok();

    pool
}

#[tokio::test]
async fn test_create_inbox_and_tenant_isolation() {
    let pool = match PgPoolOptions::new().max_connections(5).connect("postgres://ohc:ohc@localhost:5432/ohc").await {
        Ok(p) => p,
        Err(_) => return, // Skip test gracefully if no local PG in this sandboxed Bazel test
    };

    let _ = sqlx::query(CHAT_MODELS_MIGRATION).execute(&pool).await;

    let service = ChatService::new(pool.clone());

    let tenant_a_id = Uuid::new_v4();
    let tenant_b_id = Uuid::new_v4();

    // Create inbox for Tenant A
    let inbox_a = service.create_inbox(tenant_a_id, "Tenant A Inbox".to_string()).await.unwrap();
    assert_eq!(inbox_a.tenant_id, tenant_a_id);
    assert_eq!(inbox_a.name, "Tenant A Inbox");

    // Create inbox for Tenant B
    let inbox_b = service.create_inbox(tenant_b_id, "Tenant B Inbox".to_string()).await.unwrap();
    assert_eq!(inbox_b.tenant_id, tenant_b_id);
    assert_eq!(inbox_b.name, "Tenant B Inbox");

    // Test Isolation
    // When Tenant A lists their inboxes, they should ONLY see their own.
    // Row Level Security (RLS) policies attached to `app.current_tenant_id` guarantee this.
    let inboxes_for_a = service.list_inboxes(tenant_a_id).await.unwrap();
    assert_eq!(inboxes_for_a.len(), 1, "Tenant A should see exactly 1 inbox");
    assert_eq!(inboxes_for_a[0].id, inbox_a.id);

    let inboxes_for_b = service.list_inboxes(tenant_b_id).await.unwrap();
    assert_eq!(inboxes_for_b.len(), 1, "Tenant B should see exactly 1 inbox");
    assert_eq!(inboxes_for_b[0].id, inbox_b.id);
}

#[tokio::test]
async fn test_full_crud_conversation_flow() {
    let pool = match PgPoolOptions::new().max_connections(5).connect("postgres://ohc:ohc@localhost:5432/ohc").await {
        Ok(p) => p,
        Err(_) => return,
    };

    let _ = sqlx::query(CHAT_MODELS_MIGRATION).execute(&pool).await;

    let service = ChatService::new(pool.clone());
    let tenant_id = Uuid::new_v4();

    let inbox = service.create_inbox(tenant_id, "Sales".to_string()).await.unwrap();
    let channel = service.create_channel(tenant_id, inbox.id, "web_widget".to_string(), serde_json::json!({"theme": "dark"})).await.unwrap();
    let contact = service.create_contact(tenant_id, Some("Alice".to_string()), Some("alice@example.com".to_string()), None).await.unwrap();
    let conversation = service.start_conversation(tenant_id, inbox.id, contact.id, None).await.unwrap();
    let message = service.send_message(tenant_id, conversation.id, "contact".to_string(), Some(contact.id), "Hello!".to_string()).await.unwrap();

    assert_eq!(message.content, "Hello!");
    assert_eq!(message.tenant_id, tenant_id);
    assert_eq!(message.conversation_id, conversation.id);
    assert_eq!(channel.channel_type, "web_widget");
}
