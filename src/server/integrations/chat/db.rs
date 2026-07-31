use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChatInbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChatChannel {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub channel_type: String,
    pub config: sqlx::types::Json<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChatContact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChatConversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChatMessage {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct ChatDb {
    pool: PgPool,
}

impl ChatDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_message(&self, message: ChatMessage) -> Result<ChatMessage, sqlx::Error> {
        let rec = sqlx::query_as::<_, ChatMessage>(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(message.id)
        .bind(message.tenant_id)
        .bind(message.conversation_id)
        .bind(message.sender_type)
        .bind(message.sender_id)
        .bind(message.content)
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn get_conversation_by_contact_and_inbox(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid) -> Result<Option<ChatConversation>, sqlx::Error> {
        let rec = sqlx::query_as::<_, ChatConversation>(
            r#"
            SELECT * FROM chat_conversations
            WHERE tenant_id = $1 AND inbox_id = $2 AND contact_id = $3
            "#
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn create_conversation(&self, conversation: ChatConversation) -> Result<ChatConversation, sqlx::Error> {
        let rec = sqlx::query_as::<_, ChatConversation>(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
        )
        .bind(conversation.id)
        .bind(conversation.tenant_id)
        .bind(conversation.inbox_id)
        .bind(conversation.contact_id)
        .bind(conversation.status)
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn get_or_create_contact_by_phone(&self, tenant_id: Uuid, phone: &str) -> Result<ChatContact, sqlx::Error> {
        let existing = sqlx::query_as::<_, ChatContact>(
            r#"
            SELECT * FROM chat_contacts
            WHERE tenant_id = $1 AND phone = $2
            "#
        )
        .bind(tenant_id)
        .bind(phone)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(contact) = existing {
            return Ok(contact);
        }

        let rec = sqlx::query_as::<_, ChatContact>(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, phone)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(phone)
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }
}
