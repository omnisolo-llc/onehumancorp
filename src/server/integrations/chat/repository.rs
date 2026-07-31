use crate::integrations::chat::domain::{Inbox, Contact, Channel, Conversation, Message};
use uuid::Uuid;
use sqlx::{PgPool, postgres::PgQueryResult};

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

        let result = sqlx::query_as!(
            Inbox,
            r#"
            INSERT INTO inboxes (id, tenant_id, name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, created_at, updated_at
            "#,
            inbox.id,
            inbox.tenant_id,
            inbox.name,
            inbox.created_at,
            inbox.updated_at
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(result)
    }

    pub async fn get_inbox(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Inbox>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        Self::set_tenant_context(&mut tx, tenant_id).await?;

        let result = sqlx::query_as!(
            Inbox,
            "SELECT id, tenant_id, name, created_at, updated_at FROM inboxes WHERE id = $1",
            id
        )
        .fetch_optional(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(result)
    }

    pub async fn create_contact(&self, tenant_id: Uuid, contact: &Contact) -> Result<Contact, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        Self::set_tenant_context(&mut tx, tenant_id).await?;

        let result = sqlx::query_as!(
            Contact,
            r#"
            INSERT INTO contacts (id, tenant_id, name, phone_number, email, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, tenant_id, name, phone_number, email, created_at, updated_at
            "#,
            contact.id,
            contact.tenant_id,
            contact.name,
            contact.phone_number,
            contact.email,
            contact.created_at,
            contact.updated_at
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(result)
    }

    pub async fn create_conversation(&self, tenant_id: Uuid, conversation: &Conversation) -> Result<Conversation, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        Self::set_tenant_context(&mut tx, tenant_id).await?;

        let result = sqlx::query_as!(
            Conversation,
            r#"
            INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status, assignee_id, snoozed_until, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, tenant_id, inbox_id, contact_id, status, assignee_id, snoozed_until, created_at, updated_at
            "#,
            conversation.id,
            conversation.tenant_id,
            conversation.inbox_id,
            conversation.contact_id,
            conversation.status,
            conversation.assignee_id,
            conversation.snoozed_until,
            conversation.created_at,
            conversation.updated_at
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(result)
    }

    pub async fn create_message(&self, tenant_id: Uuid, message: &Message) -> Result<Message, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        Self::set_tenant_context(&mut tx, tenant_id).await?;

        let result = sqlx::query_as!(
            Message,
            r#"
            INSERT INTO messages (id, tenant_id, conversation_id, channel_id, content, message_type, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, tenant_id, conversation_id, channel_id, content, message_type, status, created_at, updated_at
            "#,
            message.id,
            message.tenant_id,
            message.conversation_id,
            message.channel_id,
            message.content,
            message.message_type,
            message.status,
            message.created_at,
            message.updated_at
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(result)
    }
}
