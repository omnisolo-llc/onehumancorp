use sqlx::{FromRow, PgPool};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Clone, Debug, FromRow)]
pub struct Inbox {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Channel {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub provider: String,
    pub config: sqlx::types::Json<Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Contact {
    pub id: String,
    pub tenant_id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Conversation {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Message {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender_type: String,
    pub content: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone)]
pub struct CoreInboxRepo {
    pool: Arc<PgPool>,
}

impl CoreInboxRepo {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, id: &str, tenant_id: &str, name: &str) -> Result<Inbox, sqlx::Error> {
        let record = sqlx::query_as::<_, Inbox>(
            "INSERT INTO inboxes (id, tenant_id, name) VALUES ($1, $2, $3) RETURNING id, tenant_id, name, created_at, updated_at"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&*self.pool)
        .await?;
        Ok(record)
    }

    pub async fn get_or_create_contact(&self, id: &str, tenant_id: &str, name: Option<&str>, email: Option<&str>, phone: Option<&str>) -> Result<Contact, sqlx::Error> {
        let record = sqlx::query_as::<_, Contact>(
            "INSERT INTO contacts (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, email = EXCLUDED.email, phone = EXCLUDED.phone RETURNING id, tenant_id, name, email, phone, created_at, updated_at"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .fetch_one(&*self.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_conversation(&self, id: &str, tenant_id: &str, inbox_id: &str, contact_id: &str) -> Result<Conversation, sqlx::Error> {
        let record = sqlx::query_as::<_, Conversation>(
            "INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, 'open') RETURNING id, tenant_id, inbox_id, contact_id, status, created_at, updated_at"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .fetch_one(&*self.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_message(&self, id: &str, tenant_id: &str, conversation_id: &str, sender_type: &str, content: &str) -> Result<Message, sqlx::Error> {
        let record = sqlx::query_as::<_, Message>(
            "INSERT INTO messages (id, tenant_id, conversation_id, sender_type, content, status) VALUES ($1, $2, $3, $4, $5, 'sent') RETURNING id, tenant_id, conversation_id, sender_type, content, status, created_at, updated_at"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(content)
        .fetch_one(&*self.pool)
        .await?;
        Ok(record)
    }
}
