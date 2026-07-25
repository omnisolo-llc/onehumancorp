use sqlx::{FromRow, PgPool};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChatChannel {
    pub id: String,
    pub tenant_id: String,
    pub provider: String,
    pub name: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChatConversation {
    pub id: String,
    pub tenant_id: String,
    pub channel_id: String,
    pub assignee_id: Option<String>,
    pub customer_id: Option<String>,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChatMessage {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender_type: String,
    pub content: String,
    pub ai_draft_status: String,
    pub draft_content: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

pub struct ChatEngineRepo {
    pool: PgPool,
}

impl ChatEngineRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_channel(&self, tenant_id: &str, provider: &str, name: &str) -> Result<ChatChannel, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query_as::<_, ChatChannel>(
            r#"INSERT INTO ohc_chat_channels (id, tenant_id, provider, name, status)
               VALUES ($1, $2, $3, $4, 'active')
               RETURNING id, tenant_id, provider, name, status, created_at, updated_at"#
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(provider)
        .bind(name)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_conversation(&self, tenant_id: &str, channel_id: &str, customer_id: Option<&str>) -> Result<ChatConversation, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query_as::<_, ChatConversation>(
            r#"INSERT INTO ohc_chat_conversations (id, tenant_id, channel_id, customer_id, status)
               VALUES ($1, $2, $3, $4, 'open')
               RETURNING id, tenant_id, channel_id, assignee_id, customer_id, status, created_at, updated_at"#
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(channel_id)
        .bind(customer_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_message(&self, tenant_id: &str, conversation_id: &str, sender_type: &str, content: &str) -> Result<ChatMessage, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query_as::<_, ChatMessage>(
            r#"INSERT INTO ohc_chat_messages (id, tenant_id, conversation_id, sender_type, content, ai_draft_status)
               VALUES ($1, $2, $3, $4, $5, 'none')
               RETURNING id, tenant_id, conversation_id, sender_type, content, ai_draft_status, draft_content, created_at"#
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(content)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_message_draft(&self, tenant_id: &str, message_id: &str, status: &str, draft_content: Option<&str>) -> Result<ChatMessage, sqlx::Error> {
        sqlx::query_as::<_, ChatMessage>(
            r#"UPDATE ohc_chat_messages
               SET ai_draft_status = $1, draft_content = $2
               WHERE id = $3 AND tenant_id = $4
               RETURNING id, tenant_id, conversation_id, sender_type, content, ai_draft_status, draft_content, created_at"#
        )
        .bind(status)
        .bind(draft_content)
        .bind(message_id)
        .bind(tenant_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn list_conversations(&self, tenant_id: &str) -> Result<Vec<ChatConversation>, sqlx::Error> {
        sqlx::query_as::<_, ChatConversation>(
            r#"SELECT id, tenant_id, channel_id, assignee_id, customer_id, status, created_at, updated_at
               FROM ohc_chat_conversations
               WHERE tenant_id = $1
               ORDER BY created_at DESC"#
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn list_messages(&self, tenant_id: &str, conversation_id: &str) -> Result<Vec<ChatMessage>, sqlx::Error> {
        sqlx::query_as::<_, ChatMessage>(
            r#"SELECT id, tenant_id, conversation_id, sender_type, content, ai_draft_status, draft_content, created_at
               FROM ohc_chat_messages
               WHERE tenant_id = $1 AND conversation_id = $2
               ORDER BY created_at ASC"#
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .fetch_all(&self.pool)
        .await
    }
}
