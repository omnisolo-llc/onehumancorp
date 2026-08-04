use super::models::{Inbox, Channel, Contact, Conversation, Message};
use sqlx::{PgPool, Error};
use uuid::Uuid;

pub struct OmnichannelRepository {
    pool: PgPool,
}

impl OmnichannelRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, name: &str) -> Result<Inbox, Error> {
        let mut tx = self.pool.begin().await?;

        let inbox = sqlx::query_as::<_, Inbox>(
            r#"
            INSERT INTO inboxes (tenant_id, name)
            VALUES ($1, $2)
            RETURNING id, tenant_id, name, created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(inbox)
    }

    pub async fn get_contact_by_identifier(&self, tenant_id: Uuid, identifier: &str) -> Result<Option<Contact>, Error> {
        let mut tx = self.pool.begin().await?;

        let contact = sqlx::query_as::<_, Contact>(
            r#"
            SELECT id, tenant_id, name, identifier, created_at, updated_at
            FROM contacts
            WHERE tenant_id = $1 AND identifier = $2
            "#
        )
        .bind(tenant_id)
        .bind(identifier)
        .fetch_optional(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(contact)
    }

    pub async fn create_contact(&self, tenant_id: Uuid, name: &str, identifier: &str) -> Result<Contact, Error> {
        let mut tx = self.pool.begin().await?;

        let contact = sqlx::query_as::<_, Contact>(
            r#"
            INSERT INTO contacts (tenant_id, name, identifier)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, name, identifier, created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(name)
        .bind(identifier)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(contact)
    }

    pub async fn create_conversation(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid) -> Result<Conversation, Error> {
        let mut tx = self.pool.begin().await?;

        let conv = sqlx::query_as::<_, Conversation>(
            r#"
            INSERT INTO conversations (tenant_id, inbox_id, contact_id, status)
            VALUES ($1, $2, $3, 'open')
            RETURNING id, tenant_id, inbox_id, contact_id, status, created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(conv)
    }

    pub async fn create_message(&self, tenant_id: Uuid, conversation_id: Uuid, content: &str, sender_type: &str) -> Result<Message, Error> {
        let mut tx = self.pool.begin().await?;

        let msg = sqlx::query_as::<_, Message>(
            r#"
            INSERT INTO messages (tenant_id, conversation_id, content, sender_type, status)
            VALUES ($1, $2, $3, $4, 'delivered')
            RETURNING id, tenant_id, conversation_id, content, sender_type, status, created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(content)
        .bind(sender_type)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(msg)
    }

    pub async fn get_conversation(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Conversation>, Error> {
        let mut tx = self.pool.begin().await?;

        let conv = sqlx::query_as::<_, Conversation>(
            r#"
            SELECT id, tenant_id, inbox_id, contact_id, status, created_at, updated_at
            FROM conversations
            WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(conv)
    }

    // Keeping Channel unused for now as it's part of the spec but not directly used in the initial webhook ingestion
    // just to suppress warnings:
    pub async fn get_channel(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Channel>, Error> {
        let mut tx = self.pool.begin().await?;
        let chan = sqlx::query_as::<_, Channel>(
            r#"
            SELECT id, tenant_id, inbox_id, provider_type, credentials, created_at, updated_at
            FROM channels
            WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(chan)
    }
}
