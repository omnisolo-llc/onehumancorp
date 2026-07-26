use crate::models::*;
use sqlx::{PgPool, Error};
use uuid::Uuid;

pub async fn get_inboxes(pool: &PgPool, tenant_id: Uuid) -> Result<Vec<ChatInbox>, Error> {
    sqlx::query_as::<_, ChatInbox>(
        "SELECT id, tenant_id, name, created_at, updated_at FROM chat_inboxes WHERE tenant_id = $1"
    )
    .bind(tenant_id)
    .fetch_all(pool)
    .await
}

pub async fn get_conversations(pool: &PgPool, tenant_id: Uuid, inbox_id: Uuid) -> Result<Vec<ChatConversation>, Error> {
    sqlx::query_as::<_, ChatConversation>(
        "SELECT id, tenant_id, inbox_id, contact_id, status, created_at, updated_at FROM chat_conversations WHERE tenant_id = $1 AND inbox_id = $2 ORDER BY updated_at DESC"
    )
    .bind(tenant_id)
    .bind(inbox_id)
    .fetch_all(pool)
    .await
}

pub async fn get_messages(pool: &PgPool, tenant_id: Uuid, conversation_id: Uuid) -> Result<Vec<ChatMessage>, Error> {
    sqlx::query_as::<_, ChatMessage>(
        "SELECT id, tenant_id, conversation_id, sender_type, sender_id, content, status, created_at, updated_at FROM chat_messages WHERE tenant_id = $1 AND conversation_id = $2 ORDER BY created_at ASC"
    )
    .bind(tenant_id)
    .bind(conversation_id)
    .fetch_all(pool)
    .await
}

pub async fn create_message(
    pool: &PgPool,
    tenant_id: Uuid,
    conversation_id: Uuid,
    sender_type: String,
    sender_id: Option<Uuid>,
    content: String,
) -> Result<ChatMessage, Error> {
    let id = Uuid::new_v4();
    let status = "sent".to_string();

    sqlx::query_as::<_, ChatMessage>(
        r#"
        INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, status, created_at, updated_at
        "#
    )
    .bind(id)
    .bind(tenant_id)
    .bind(conversation_id)
    .bind(sender_type)
    .bind(sender_id)
    .bind(content)
    .bind(status)
    .fetch_one(pool)
    .await
}

pub async fn create_conversation(
    pool: &PgPool,
    tenant_id: Uuid,
    inbox_id: Uuid,
    contact_id: Uuid,
) -> Result<ChatConversation, Error> {
    let id = Uuid::new_v4();
    let status = "open".to_string();

    sqlx::query_as::<_, ChatConversation>(
        r#"
        INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, tenant_id, inbox_id, contact_id, status, created_at, updated_at
        "#
    )
    .bind(id)
    .bind(tenant_id)
    .bind(inbox_id)
    .bind(contact_id)
    .bind(status)
    .fetch_one(pool)
    .await
}
