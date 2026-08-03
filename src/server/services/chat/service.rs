use uuid::Uuid;
use sqlx::{Sqlite, Pool, Row};
use crate::services::chat::models::{ChatConversationWithContact, ChatMessage, ChatConversation, ChatContact, ChatInbox};

#[derive(Clone)]
pub struct ChatService {
    pool: Pool<Sqlite>,
}

impl ChatService {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn get_conversations(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<ChatConversationWithContact>, sqlx::Error> {
        let rows = sqlx::query_as!(
            ChatConversationWithContact,
            r#"
            SELECT
                c.id, c.tenant_id, c.inbox_id, c.contact_id, c.assignee_id, c.status, c.created_at, c.updated_at,
                ct.name as contact_name, ct.email as contact_email, ct.phone as contact_phone
            FROM chat_conversations c
            LEFT JOIN chat_contacts ct ON c.contact_id = ct.id
            WHERE c.tenant_id = $1
            ORDER BY c.updated_at DESC
            "#,
            tenant_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn get_messages(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<Vec<ChatMessage>, sqlx::Error> {
        let rows = sqlx::query_as!(
            ChatMessage,
            r#"
            SELECT id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at
            FROM chat_messages
            WHERE tenant_id = $1 AND conversation_id = $2
            ORDER BY created_at ASC
            "#,
            tenant_id,
            conversation_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn create_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        content: String,
    ) -> Result<ChatMessage, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();
        sqlx::query!(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, content, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            id, tenant_id, conversation_id, sender_type, content, now, now
        )
        .execute(&self.pool)
        .await?;

        let msg = ChatMessage {
            id,
            tenant_id,
            conversation_id,
            sender_type,
            sender_id: None,
            content,
            created_at: now,
            updated_at: now,
        };
        Ok(msg)
    }
}
