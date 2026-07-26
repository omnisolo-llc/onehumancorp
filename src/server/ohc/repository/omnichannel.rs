use crate::domain::chat::{Contact, Conversation, ConversationStatus, Message, MessageType};
use crate::domain::inbox::{ChannelType, Inbox};
use sqlx::{PgPool, Result};
use uuid::Uuid;

pub struct OmnichannelRepository {
    pool: PgPool,
}

impl OmnichannelRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(
        &self,
        tenant_id: Uuid,
        name: &str,
        channel_type: ChannelType,
        channel_config: Option<serde_json::Value>,
    ) -> Result<Inbox> {
        sqlx::query_as(
            r#"
            INSERT INTO inboxes (tenant_id, name, channel_type, channel_config)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, name, channel_type as "channel_type: _", channel_config, is_active, created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(name)
        .bind(channel_type as ChannelType)
        .bind(channel_config)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn list_conversations_for_inbox(&self, tenant_id: Uuid, inbox_id: Uuid) -> Result<Vec<Conversation>> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, inbox_id, contact_id, status as "status: _", assignee_id, last_activity_at, created_at, updated_at
            FROM conversations
            WHERE tenant_id = $1 AND inbox_id = $2
            ORDER BY last_activity_at DESC
            "#
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn add_message_to_conversation(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        message_type: MessageType,
        content: &str,
        sender_id: Option<Uuid>,
        sender_type: Option<&str>,
    ) -> Result<Message> {
        let mut tx = self.pool.begin().await?;

        let message = sqlx::query_as(
            r#"
            INSERT INTO messages (tenant_id, conversation_id, message_type, content, sender_id, sender_type)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, conversation_id, message_type as "message_type: _", content, sender_id, sender_type, created_at
            "#
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(message_type as MessageType)
        .bind(content)
        .bind(sender_id)
        .bind(sender_type)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query(
            r#"
            UPDATE conversations
            SET last_activity_at = NOW()
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(conversation_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(message)
    }
}
