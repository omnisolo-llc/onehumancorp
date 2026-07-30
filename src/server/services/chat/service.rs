use sqlx::PgPool;
use uuid::Uuid;
use super::models::{ChatInbox, ChatContact, ChatConversation, ChatMessage, ChatActionRequiredQueue};

pub struct ChatService {
    pool: PgPool,
}

impl ChatService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(
        &self,
        tenant_id: Uuid,
        channel_type: String,
        name: String,
    ) -> Result<ChatInbox, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, channel_type, name)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, channel_type, name, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(channel_type)
        .bind(name)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_or_create_contact(
        &self,
        tenant_id: Uuid,
        identifier: String,
        name: Option<String>,
    ) -> Result<ChatContact, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, identifier, name)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (tenant_id, identifier) DO UPDATE SET
                name = EXCLUDED.name,
                updated_at = NOW()
            RETURNING id, tenant_id, name, identifier, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(identifier)
        .bind(name)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn start_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
    ) -> Result<ChatConversation, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status)
            VALUES ($1, $2, $3, $4, 'open')
            RETURNING id, tenant_id, inbox_id, contact_id, status, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        content: String,
        is_draft: bool,
    ) -> Result<ChatMessage, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, content, is_draft)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, conversation_id, sender_type, is_draft, content, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(content)
        .bind(is_draft)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_draft_for_approval(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        content: String,
    ) -> Result<(ChatMessage, ChatActionRequiredQueue), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let message: ChatMessage = sqlx::query_as(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, content, is_draft)
            VALUES ($1, $2, $3, 'agent', $4, true)
            RETURNING id, tenant_id, conversation_id, sender_type, is_draft, content, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(&content)
        .fetch_one(&mut *tx)
        .await?;

        let action_queue: ChatActionRequiredQueue = sqlx::query_as(
            r#"
            INSERT INTO chat_action_required_queue (id, tenant_id, message_id, status)
            VALUES ($1, $2, $3, 'pending')
            RETURNING id, tenant_id, message_id, status, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(message.id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok((message, action_queue))
    }

    pub async fn approve_draft(
        &self,
        tenant_id: Uuid,
        action_queue_id: Uuid,
    ) -> Result<ChatMessage, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let action_queue: ChatActionRequiredQueue = sqlx::query_as(
            r#"
            UPDATE chat_action_required_queue
            SET status = 'approved', updated_at = NOW()
            WHERE id = $1 AND tenant_id = $2
            RETURNING id, tenant_id, message_id, status, created_at, updated_at
            "#
        )
        .bind(action_queue_id)
        .bind(tenant_id)
        .fetch_one(&mut *tx)
        .await?;

        let message: ChatMessage = sqlx::query_as(
            r#"
            UPDATE chat_messages
            SET is_draft = false, updated_at = NOW()
            WHERE id = $1 AND tenant_id = $2
            RETURNING id, tenant_id, conversation_id, sender_type, is_draft, content, created_at, updated_at
            "#
        )
        .bind(action_queue.message_id)
        .bind(tenant_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(message)
    }
}
