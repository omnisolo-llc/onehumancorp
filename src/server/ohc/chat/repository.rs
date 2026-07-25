use uuid::Uuid;
use crate::chat::domain::models::{Inbox, Conversation, Message};
use sqlx::{PgPool, Error};

pub struct ChatRepository {
    pool: PgPool,
}

impl ChatRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, inbox: &Inbox) -> Result<Inbox, Error> {
        let rec = sqlx::query_as!(
            Inbox,
            r#"
            INSERT INTO omnichannel_inboxes (id, tenant_id, name, channel_type, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, name, channel_type, created_at, updated_at
            "#,
            inbox.id,
            inbox.tenant_id,
            inbox.name,
            inbox.channel_type,
            inbox.created_at,
            inbox.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn list_conversations(&self, tenant_id: Uuid, inbox_id: Uuid) -> Result<Vec<Conversation>, Error> {
        let recs = sqlx::query_as!(
            Conversation,
            r#"
            SELECT id, tenant_id, contact_id, inbox_id, status, last_activity_at, created_at, updated_at
            FROM omnichannel_conversations
            WHERE tenant_id = $1 AND inbox_id = $2
            ORDER BY last_activity_at DESC
            "#,
            tenant_id,
            inbox_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(recs)
    }

    pub async fn create_conversation(&self, conversation: &Conversation) -> Result<Conversation, Error> {
        let rec = sqlx::query_as!(
            Conversation,
            r#"
            INSERT INTO omnichannel_conversations (id, tenant_id, contact_id, inbox_id, status, last_activity_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, tenant_id, contact_id, inbox_id, status, last_activity_at, created_at, updated_at
            "#,
            conversation.id,
            conversation.tenant_id,
            conversation.contact_id,
            conversation.inbox_id,
            conversation.status,
            conversation.last_activity_at,
            conversation.created_at,
            conversation.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn list_messages(&self, tenant_id: Uuid, conversation_id: Uuid) -> Result<Vec<Message>, Error> {
        let recs = sqlx::query_as!(
            Message,
            r#"
            SELECT id, tenant_id, conversation_id, content, message_type, sender_id, sender_type, status, created_at, updated_at
            FROM omnichannel_messages
            WHERE tenant_id = $1 AND conversation_id = $2
            ORDER BY created_at ASC
            "#,
            tenant_id,
            conversation_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(recs)
    }

    pub async fn send_message(&self, message: &Message) -> Result<Message, Error> {
        let rec = sqlx::query_as!(
            Message,
            r#"
            INSERT INTO omnichannel_messages (id, tenant_id, conversation_id, content, message_type, sender_id, sender_type, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, tenant_id, conversation_id, content, message_type, sender_id, sender_type, status, created_at, updated_at
            "#,
            message.id,
            message.tenant_id,
            message.conversation_id,
            message.content,
            message.message_type,
            message.sender_id,
            message.sender_type,
            message.status,
            message.created_at,
            message.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }
}
