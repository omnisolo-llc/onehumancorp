use super::chat_models::{Conversation, Message};
use sqlx::PgPool;

pub struct ChatRepository;

impl ChatRepository {
    pub async fn get_conversations(
        pool: &PgPool,
        tenant_id: &str,
    ) -> Result<Vec<Conversation>, sqlx::Error> {
        sqlx::query_as::<_, Conversation>(
            "SELECT * FROM omni_conversations WHERE tenant_id = $1 ORDER BY last_activity_at DESC",
        )
        .bind(tenant_id)
        .fetch_all(pool)
        .await
    }

    pub async fn get_messages(
        pool: &PgPool,
        tenant_id: &str,
        conversation_id: &str,
    ) -> Result<Vec<Message>, sqlx::Error> {
        sqlx::query_as::<_, Message>(
            "SELECT * FROM omni_chat_messages WHERE tenant_id = $1 AND conversation_id = $2 ORDER BY created_at ASC",
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .fetch_all(pool)
        .await
    }

    pub async fn insert_message(
        pool: &PgPool,
        tenant_id: &str,
        msg: Message,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO omni_chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content, message_type)
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(msg.id)
        .bind(tenant_id)
        .bind(msg.conversation_id)
        .bind(msg.sender_type)
        .bind(msg.sender_id)
        .bind(msg.content)
        .bind(msg.message_type)
        .execute(pool)
        .await?;

        Ok(())
    }
}
