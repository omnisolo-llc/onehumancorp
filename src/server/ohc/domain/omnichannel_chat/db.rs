use sqlx::{PgPool, Result as SqlxResult};
use uuid::Uuid;
use super::models::{ChatInbox, ChatConversation, ChatMessage};

pub struct ChatRepository {
    pool: PgPool,
}

impl ChatRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, tenant_id: &str, name: &str) -> SqlxResult<ChatInbox> {
        let inbox = sqlx::query_as(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, name
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&self.pool)
        .await?;

        Ok(inbox)
    }

    pub async fn get_inboxes(&self, tenant_id: &str) -> SqlxResult<Vec<ChatInbox>> {
        let inboxes = sqlx::query_as(
            r#"
            SELECT id, tenant_id, name
            FROM chat_inboxes
            WHERE tenant_id = $1
            "#,
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(inboxes)
    }

    pub async fn create_conversation(&self, tenant_id: &str, inbox_id: Uuid, contact_id: Uuid, status: &str) -> SqlxResult<ChatConversation> {
        let conv = sqlx::query_as(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(status)
        .fetch_one(&self.pool)
        .await?;

        Ok(conv)
    }

    pub async fn get_conversations(&self, tenant_id: &str, inbox_id: Uuid) -> SqlxResult<Vec<ChatConversation>> {
        let convs = sqlx::query_as(
            r#"
            SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status
            FROM chat_conversations
            WHERE tenant_id = $1 AND inbox_id = $2
            "#,
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(convs)
    }

    pub async fn add_message(&self, tenant_id: &str, conversation_id: Uuid, content: &str, sender_type: &str, sender_id: Option<String>) -> SqlxResult<ChatMessage> {
        let msg = sqlx::query_as(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, content, sender_type, sender_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, conversation_id, content, sender_type, sender_id
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(content)
        .bind(sender_type)
        .bind(sender_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
