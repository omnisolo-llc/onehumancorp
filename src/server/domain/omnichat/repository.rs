use sqlx::{PgPool, Error};
use uuid::Uuid;
use super::models::{Inbox, ChannelAdapter, Contact, Conversation, Message};

pub struct OmnichatRepository {
    pool: PgPool,
}

impl OmnichatRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, name: &str) -> Result<Inbox, Error> {
        sqlx::query_as::<_, Inbox>(
            r#"
            INSERT INTO omnichat_inboxes (tenant_id, name)
            VALUES ($1, $2)
            RETURNING *
            "#
        )
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn link_channel_adapter(&self, tenant_id: Uuid, inbox_id: Uuid, channel_type: &str, config: serde_json::Value) -> Result<ChannelAdapter, Error> {
        sqlx::query_as::<_, ChannelAdapter>(
            r#"
            INSERT INTO omnichat_channel_adapters (tenant_id, inbox_id, channel_type, config)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(channel_type)
        .bind(config)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_contact(&self, tenant_id: Uuid, name: &str, email: Option<&str>, phone: Option<&str>) -> Result<Contact, Error> {
        sqlx::query_as::<_, Contact>(
            r#"
            INSERT INTO omnichat_contacts (tenant_id, name, email, phone)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#
        )
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_or_create_conversation(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid) -> Result<Conversation, Error> {
        let existing = sqlx::query_as::<_, Conversation>(
            r#"
            SELECT * FROM omnichat_conversations
            WHERE tenant_id = $1 AND inbox_id = $2 AND contact_id = $3
            LIMIT 1
            "#
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(conv) = existing {
            return Ok(conv);
        }

        sqlx::query_as::<_, Conversation>(
            r#"
            INSERT INTO omnichat_conversations (tenant_id, inbox_id, contact_id)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn ingest_message(&self, tenant_id: Uuid, conversation_id: Uuid, contact_id: Option<Uuid>, content: &str) -> Result<Message, Error> {
        sqlx::query_as::<_, Message>(
            r#"
            INSERT INTO omnichat_messages (tenant_id, conversation_id, contact_id, content)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(contact_id)
        .bind(content)
        .fetch_one(&self.pool)
        .await
    }
}
