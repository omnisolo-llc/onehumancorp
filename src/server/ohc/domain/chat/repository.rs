use sqlx::{PgPool, Result};
use crate::domain::chat::models::{Inbox, ChannelAdapter, Contact, Conversation, Message};

pub struct ChatRepository {
    pool: PgPool,
}

impl ChatRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, inbox: &Inbox) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO inboxes (id, tenant_id, name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            "#
        )
        .bind(inbox.id)
        .bind(inbox.tenant_id)
        .bind(&inbox.name)
        .bind(inbox.created_at)
        .bind(inbox.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_channel_adapter(&self, adapter: &ChannelAdapter) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO channel_adapters (id, tenant_id, inbox_id, type, config, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(adapter.id)
        .bind(adapter.tenant_id)
        .bind(adapter.inbox_id)
        .bind(&adapter.channel_type)
        .bind(&adapter.config)
        .bind(adapter.created_at)
        .bind(adapter.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_contact(&self, contact: &Contact) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO contacts (id, tenant_id, name, email, phone, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(contact.id)
        .bind(contact.tenant_id)
        .bind(&contact.name)
        .bind(&contact.email)
        .bind(&contact.phone)
        .bind(contact.created_at)
        .bind(contact.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_conversation(&self, conversation: &Conversation) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(conversation.id)
        .bind(conversation.tenant_id)
        .bind(conversation.inbox_id)
        .bind(conversation.contact_id)
        .bind(&conversation.status)
        .bind(conversation.created_at)
        .bind(conversation.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_message(&self, message: &Message) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO messages (id, tenant_id, conversation_id, sender_id, content, is_ai_draft, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(message.id)
        .bind(message.tenant_id)
        .bind(message.conversation_id)
        .bind(message.sender_id)
        .bind(&message.content)
        .bind(message.is_ai_draft)
        .bind(message.created_at)
        .bind(message.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
