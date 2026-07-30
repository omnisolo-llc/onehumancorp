use sqlx::{PgPool, Error};
use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};

pub struct OmnichannelRepository {
    pool: PgPool,
}

impl OmnichannelRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, inbox: &ChatInbox) -> Result<ChatInbox, Error> {
        let row = sqlx::query_as!(
            ChatInbox,
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, name, created_at, updated_at
            "#,
            inbox.id,
            inbox.tenant_id,
            inbox.name
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn create_conversation(&self, conv: &ChatConversation) -> Result<ChatConversation, Error> {
        let row = sqlx::query_as!(
            ChatConversation,
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            "#,
            conv.id,
            conv.tenant_id,
            conv.inbox_id,
            conv.contact_id,
            conv.assignee_id,
            conv.status
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn create_message(&self, msg: &ChatMessage) -> Result<ChatMessage, Error> {
        let row = sqlx::query_as!(
            ChatMessage,
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at
            "#,
            msg.id,
            msg.tenant_id,
            msg.conversation_id,
            msg.sender_type,
            msg.sender_id,
            msg.content
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }
}
