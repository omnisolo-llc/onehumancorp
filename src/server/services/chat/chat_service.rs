use sqlx::PgPool;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatInbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatConversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

pub struct ChatService {
    pool: PgPool,
}

impl ChatService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, name: &str) -> Result<ChatInbox, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as!(
            ChatInbox,
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, name
            "#,
            id,
            tenant_id,
            name
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_inboxes(&self, tenant_id: Uuid) -> Result<Vec<ChatInbox>, sqlx::Error> {
        sqlx::query_as!(
            ChatInbox,
            r#"
            SELECT id, tenant_id, name FROM chat_inboxes WHERE tenant_id = $1
            "#,
            tenant_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn create_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
    ) -> Result<ChatConversation, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as!(
            ChatConversation,
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status)
            VALUES ($1, $2, $3, $4, 'open')
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status
            "#,
            id,
            tenant_id,
            inbox_id,
            contact_id
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: &str,
        content: &str,
    ) -> Result<ChatMessage, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as!(
            ChatMessage,
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, content)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content
            "#,
            id,
            tenant_id,
            conversation_id,
            sender_type,
            content
        )
        .fetch_one(&self.pool)
        .await
    }
}
