use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub enable_auto_assignment: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Channel {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub provider_type: String,
    pub credentials: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
    pub snoozed_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub message_type: String,
    pub external_source_ids: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[async_trait::async_trait]
pub trait InboxRepository {
    async fn create_conversation(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid) -> sqlx::Result<Conversation>;
    async fn add_message(&self, tenant_id: Uuid, conversation_id: Uuid, content: String, message_type: String) -> sqlx::Result<Message>;
    async fn get_conversation_messages(&self, tenant_id: Uuid, conversation_id: Uuid) -> sqlx::Result<Vec<Message>>;
}

pub struct PostgresInboxRepository {
    pool: sqlx::PgPool,
}

impl PostgresInboxRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl InboxRepository for PostgresInboxRepository {
    async fn create_conversation(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid) -> sqlx::Result<Conversation> {
        let mut tx = self.pool.begin().await?;
        sqlx::query!("SELECT set_config('app.current_tenant_id', $1, true)", tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let convo = sqlx::query_as!(
            Conversation,
            r#"
            INSERT INTO conversations (tenant_id, inbox_id, contact_id)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            tenant_id,
            inbox_id,
            contact_id
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(convo)
    }

    async fn add_message(&self, tenant_id: Uuid, conversation_id: Uuid, content: String, message_type: String) -> sqlx::Result<Message> {
        let mut tx = self.pool.begin().await?;
        sqlx::query!("SELECT set_config('app.current_tenant_id', $1, true)", tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let msg = sqlx::query_as!(
            Message,
            r#"
            INSERT INTO messages (tenant_id, conversation_id, content, message_type)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            tenant_id,
            conversation_id,
            content,
            message_type
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(msg)
    }

    async fn get_conversation_messages(&self, tenant_id: Uuid, conversation_id: Uuid) -> sqlx::Result<Vec<Message>> {
        let mut tx = self.pool.begin().await?;
        sqlx::query!("SELECT set_config('app.current_tenant_id', $1, true)", tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let msgs = sqlx::query_as!(
            Message,
            r#"
            SELECT * FROM messages
            WHERE tenant_id = $1 AND conversation_id = $2
            ORDER BY created_at ASC
            "#,
            tenant_id,
            conversation_id
        )
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(msgs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tenant_isolation_placeholder() {
        assert!(true);
    }
}
