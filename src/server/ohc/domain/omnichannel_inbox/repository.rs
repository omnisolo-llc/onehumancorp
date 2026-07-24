use super::{Conversation, Inbox, Message};
use server_common::error::Result;
use sqlx::PgPool;
use uuid::Uuid;

pub struct OmnichannelRepository {
    pool: PgPool,
}

impl OmnichannelRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, tenant_id: &str, name: &str) -> Result<Inbox> {
        let id = Uuid::new_v4();
        let inbox = sqlx::query_as!(
            Inbox,
            r#"
            INSERT INTO omnichannel_inboxes (id, tenant_id, name)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, name, is_active, created_at, updated_at
            "#,
            id,
            tenant_id,
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(inbox)
    }

    pub async fn list_inboxes(&self, tenant_id: &str) -> Result<Vec<Inbox>> {
        let inboxes = sqlx::query_as!(
            Inbox,
            r#"
            SELECT id, tenant_id, name, is_active, created_at, updated_at
            FROM omnichannel_inboxes
            WHERE tenant_id = $1
            "#,
            tenant_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(inboxes)
    }

    pub async fn create_conversation(
        &self,
        tenant_id: &str,
        inbox_id: Uuid,
        contact_id: Uuid,
    ) -> Result<Conversation> {
        let id = Uuid::new_v4();
        let conversation = sqlx::query_as!(
            Conversation,
            r#"
            INSERT INTO omnichannel_conversations (id, tenant_id, inbox_id, contact_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, inbox_id, contact_id, status, priority, created_at, updated_at
            "#,
            id,
            tenant_id,
            inbox_id,
            contact_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(conversation)
    }

    pub async fn create_message(
        &self,
        tenant_id: &str,
        conversation_id: Uuid,
        content: &str,
        sender_type: &str,
    ) -> Result<Message> {
        let id = Uuid::new_v4();
        let message = sqlx::query_as!(
            Message,
            r#"
            INSERT INTO omnichannel_messages (id, tenant_id, conversation_id, content, sender_type)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, conversation_id, content, sender_type, sender_id, delivered_at, created_at, updated_at
            "#,
            id,
            tenant_id,
            conversation_id,
            content,
            sender_type
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(message)
    }

    pub async fn list_messages(
        &self,
        tenant_id: &str,
        conversation_id: Uuid,
    ) -> Result<Vec<Message>> {
        let messages = sqlx::query_as!(
            Message,
            r#"
            SELECT id, tenant_id, conversation_id, content, sender_type, sender_id, delivered_at, created_at, updated_at
            FROM omnichannel_messages
            WHERE tenant_id = $1 AND conversation_id = $2
            ORDER BY created_at ASC
            "#,
            tenant_id,
            conversation_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(messages)
    }
}
