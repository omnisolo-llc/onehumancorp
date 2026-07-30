use sqlx::{PgPool, Result};
use uuid::Uuid;
use crate::models::{Inbox, Conversation, Message, Contact};

pub async fn create_inbox(pool: &PgPool, tenant_id: &str, name: &str, channel_type: &str) -> Result<Inbox> {
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

    let inbox = sqlx::query_as::<_, Inbox>(
        r#"
        INSERT INTO chat_inboxes (id, tenant_id, name, channel_type)
        VALUES ($1, $2, $3, $4)
        RETURNING id, tenant_id, name, channel_type, created_at, updated_at
        "#
    )
    .bind(Uuid::new_v4())
    .bind(tenant_id)
    .bind(name)
    .bind(channel_type)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(inbox)
}

pub async fn get_inboxes(pool: &PgPool, tenant_id: &str) -> Result<Vec<Inbox>> {
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

    let inboxes = sqlx::query_as::<_, Inbox>(
        r#"
        SELECT id, tenant_id, name, channel_type, created_at, updated_at
        FROM chat_inboxes
        WHERE tenant_id = $1
        "#
    )
    .bind(tenant_id)
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(inboxes)
}

pub async fn create_contact(pool: &PgPool, tenant_id: &str, name: &str, email: Option<&str>, phone_number: Option<&str>) -> Result<Contact> {
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

    let contact = sqlx::query_as::<_, Contact>(
        r#"
        INSERT INTO chat_contacts (id, tenant_id, name, email, phone_number)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, tenant_id, name, email, phone_number, created_at, updated_at
        "#
    )
    .bind(Uuid::new_v4())
    .bind(tenant_id)
    .bind(name)
    .bind(email)
    .bind(phone_number)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(contact)
}

pub async fn create_conversation(pool: &PgPool, tenant_id: &str, inbox_id: Uuid, contact_id: Uuid, status: &str) -> Result<Conversation> {
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

    let conversation = sqlx::query_as::<_, Conversation>(
        r#"
        INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, tenant_id, inbox_id, contact_id, status, created_at, updated_at
        "#
    )
    .bind(Uuid::new_v4())
    .bind(tenant_id)
    .bind(inbox_id)
    .bind(contact_id)
    .bind(status)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(conversation)
}

pub async fn create_message(
    pool: &PgPool,
    tenant_id: &str,
    conversation_id: Uuid,
    content: &str,
    message_type: &str,
    content_attributes: Option<serde_json::Value>,
    external_source_ids: Option<serde_json::Value>
) -> Result<Message> {
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

    let message = sqlx::query_as::<_, Message>(
        r#"
        INSERT INTO chat_messages (id, tenant_id, conversation_id, content, message_type, content_attributes, external_source_ids)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, tenant_id, conversation_id, content, message_type, content_attributes, external_source_ids, created_at, updated_at
        "#
    )
    .bind(Uuid::new_v4())
    .bind(tenant_id)
    .bind(conversation_id)
    .bind(content)
    .bind(message_type)
    .bind(content_attributes)
    .bind(external_source_ids)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(message)
}

pub async fn get_messages(pool: &PgPool, tenant_id: &str, conversation_id: Uuid) -> Result<Vec<Message>> {
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

    let messages = sqlx::query_as::<_, Message>(
        r#"
        SELECT id, tenant_id, conversation_id, content, message_type, content_attributes, external_source_ids, created_at, updated_at
        FROM chat_messages
        WHERE tenant_id = $1 AND conversation_id = $2
        ORDER BY created_at ASC
        "#
    )
    .bind(tenant_id)
    .bind(conversation_id)
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(messages)
}
