use crate::domain::{Inbox, Contact, Conversation, Message};
use uuid::Uuid;
use sqlx::PgPool;

pub struct ChatRepository {
    pool: PgPool,
}

impl ChatRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Sets the RLS tenant context for a transaction.
    async fn set_tenant_context(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, tenant_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, inbox: &Inbox) -> Result<Inbox, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        Self::set_tenant_context(&mut tx, tenant_id).await?;

        let result = sqlx::query_as(
            r#"
            INSERT INTO inboxes (id, tenant_id, name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, created_at, updated_at
            "#)
            .bind(inbox.id)
            .bind(inbox.tenant_id)
            .bind(&inbox.name)
            .bind(inbox.created_at)
            .bind(inbox.updated_at)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(result)
    }

    pub async fn get_inbox(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Inbox>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        Self::set_tenant_context(&mut tx, tenant_id).await?;

        let result = sqlx::query_as(
            "SELECT id, tenant_id, name, created_at, updated_at FROM inboxes WHERE id = $1")
            .bind(id)
        .fetch_optional(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(result)
    }

    pub async fn create_contact(&self, tenant_id: Uuid, contact: &Contact) -> Result<Contact, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        Self::set_tenant_context(&mut tx, tenant_id).await?;

        let result = sqlx::query_as(
            r#"
            INSERT INTO contacts (id, tenant_id, name, phone_number, email, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, tenant_id, name, phone_number, email, created_at, updated_at
            "#)
            .bind(contact.id)
            .bind(contact.tenant_id)
            .bind(&contact.name)
            .bind(&contact.phone_number)
            .bind(&contact.email)
            .bind(contact.created_at)
            .bind(contact.updated_at)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(result)
    }

    pub async fn create_conversation(&self, tenant_id: Uuid, conversation: &Conversation) -> Result<Conversation, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        Self::set_tenant_context(&mut tx, tenant_id).await?;

        let result = sqlx::query_as(
            r#"
            INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status, assignee_id, snoozed_until, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, tenant_id, inbox_id, contact_id, status, assignee_id, snoozed_until, created_at, updated_at
            "#)
            .bind(conversation.id)
            .bind(conversation.tenant_id)
            .bind(conversation.inbox_id)
            .bind(conversation.contact_id)
            .bind(&conversation.status)
            .bind(conversation.assignee_id)
            .bind(conversation.snoozed_until)
            .bind(conversation.created_at)
            .bind(conversation.updated_at)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(result)
    }

    pub async fn create_message(&self, tenant_id: Uuid, message: &Message) -> Result<Message, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        Self::set_tenant_context(&mut tx, tenant_id).await?;

        let result = sqlx::query_as(
            r#"
            INSERT INTO messages (id, tenant_id, conversation_id, channel_id, content, message_type, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, tenant_id, conversation_id, channel_id, content, message_type, status, created_at, updated_at
            "#)
            .bind(message.id)
            .bind(message.tenant_id)
            .bind(message.conversation_id)
            .bind(message.channel_id)
            .bind(&message.content)
            .bind(&message.message_type)
            .bind(&message.status)
            .bind(message.created_at)
            .bind(message.updated_at)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(result)
    }
}
