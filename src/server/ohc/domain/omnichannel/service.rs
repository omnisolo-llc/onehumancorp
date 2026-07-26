use super::models::{Inbox, Channel, Contact, Conversation, Message};
use sqlx::{PgPool, Error};
use uuid::Uuid;

pub struct OmnichannelService {
    pool: PgPool,
}

impl OmnichannelService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, name: String) -> Result<Inbox, Error> {
        let inbox = sqlx::query_as::<_, Inbox>(
            r#"
            INSERT INTO omnichannel_inboxes (tenant_id, name)
            VALUES ($1, $2)
            RETURNING id, tenant_id, name, created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&self.pool)
        .await?;
        Ok(inbox)
    }

    pub async fn list_conversations(&self, tenant_id: Uuid) -> Result<Vec<Conversation>, Error> {
        let conversations = sqlx::query_as::<_, Conversation>(
            r#"
            SELECT id, tenant_id, inbox_id, contact_id, status, created_at, updated_at
            FROM omnichannel_conversations
            WHERE tenant_id = $1
            ORDER BY updated_at DESC
            "#
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(conversations)
    }

    pub async fn send_message(&self, tenant_id: Uuid, conversation_id: Uuid, content: String, sender_type: String) -> Result<Message, Error> {
        let message = sqlx::query_as::<_, Message>(
            r#"
            INSERT INTO omnichannel_messages (tenant_id, conversation_id, content, sender_type)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, conversation_id, content, sender_type, created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(content)
        .bind(sender_type)
        .fetch_one(&self.pool)
        .await?;

        // Update conversation updated_at
        sqlx::query(
            r#"
            UPDATE omnichannel_conversations
            SET updated_at = NOW()
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(conversation_id)
        .bind(tenant_id)
        .execute(&self.pool)
        .await?;

        Ok(message)
    }
}
