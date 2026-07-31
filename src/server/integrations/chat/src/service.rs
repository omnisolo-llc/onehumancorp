use std::sync::Arc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use super::models::{ChatInbox, ChatConversation, ChatMessage};

pub struct ChatService {
    pool: Arc<Pool<Postgres>>,
}

impl ChatService {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, name: &str) -> Result<ChatInbox, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, ChatInbox>(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, name, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn create_conversation(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid) -> Result<ChatConversation, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, ChatConversation>(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn save_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: &str,
        sender_id: Option<Uuid>,
        content: &str,
    ) -> Result<ChatMessage, sqlx::Error> {
        let id = Uuid::new_v4();
        let message = sqlx::query_as::<_, ChatMessage>(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(sender_id)
        .bind(content)
        .fetch_one(&*self.pool)
        .await?;

        // Broadcast happens at a higher level or in the event layer.

        Ok(message)
    }
}
