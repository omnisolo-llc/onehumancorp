use sqlx::{PgPool, Error};
use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};

pub async fn create_inbox(pool: &PgPool, inbox: &ChatInbox) -> Result<(), Error> {
    sqlx::query("INSERT INTO chat_inboxes (id, tenant_id, name) VALUES ($1, $2, $3)")
        .bind(inbox.id)
        .bind(inbox.tenant_id)
        .bind(&inbox.name)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn create_channel(pool: &PgPool, channel: &ChatChannel) -> Result<(), Error> {
    sqlx::query("INSERT INTO chat_channels (id, tenant_id, inbox_id, channel_type) VALUES ($1, $2, $3, $4)")
        .bind(channel.id)
        .bind(channel.tenant_id)
        .bind(channel.inbox_id)
        .bind(&channel.channel_type)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_contact_by_phone(pool: &PgPool, tenant_id: Uuid, phone: &str) -> Result<Option<ChatContact>, Error> {
    let contact = sqlx::query_as::<_, ChatContact>(
        "SELECT id, tenant_id, name, email, phone FROM chat_contacts WHERE tenant_id = $1 AND phone = $2"
    )
    .bind(tenant_id)
    .bind(phone)
    .fetch_optional(pool)
    .await?;
    Ok(contact)
}

pub async fn create_contact(pool: &PgPool, contact: &ChatContact) -> Result<(), Error> {
    sqlx::query("INSERT INTO chat_contacts (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5)")
        .bind(contact.id)
        .bind(contact.tenant_id)
        .bind(&contact.name)
        .bind(&contact.email)
        .bind(&contact.phone)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_conversation(pool: &PgPool, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid) -> Result<Option<ChatConversation>, Error> {
    let conv = sqlx::query_as::<_, ChatConversation>(
        "SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status FROM chat_conversations WHERE tenant_id = $1 AND inbox_id = $2 AND contact_id = $3"
    )
    .bind(tenant_id)
    .bind(inbox_id)
    .bind(contact_id)
    .fetch_optional(pool)
    .await?;
    Ok(conv)
}

pub async fn create_conversation(pool: &PgPool, conv: &ChatConversation) -> Result<(), Error> {
    sqlx::query("INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(conv.id)
        .bind(conv.tenant_id)
        .bind(conv.inbox_id)
        .bind(conv.contact_id)
        .bind(conv.assignee_id)
        .bind(&conv.status)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn create_message(pool: &PgPool, msg: &ChatMessage) -> Result<(), Error> {
    sqlx::query("INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(msg.id)
        .bind(msg.tenant_id)
        .bind(msg.conversation_id)
        .bind(&msg.sender_type)
        .bind(msg.sender_id)
        .bind(&msg.content)
        .execute(pool)
        .await?;
    Ok(())
}
