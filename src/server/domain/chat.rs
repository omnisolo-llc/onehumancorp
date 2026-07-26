use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub custom_attributes: Option<Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub channel: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub contact_id: Uuid,
    pub inbox_id: Uuid,
    pub status: String,
    pub assignee_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Contact {
    pub async fn create(
        pool: &PgPool,
        tenant_id: Uuid,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as!(
            Contact,
            r#"
            INSERT INTO contacts (id, tenant_id, name, email, phone)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, email, phone, custom_attributes, created_at, updated_at
            "#,
            id,
            tenant_id,
            name,
            email,
            phone
        )
        .fetch_one(pool)
        .await
    }
}

impl Inbox {
    pub async fn create(
        pool: &PgPool,
        tenant_id: Uuid,
        name: String,
        channel: String,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as!(
            Inbox,
            r#"
            INSERT INTO inboxes (id, tenant_id, name, channel)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, name, channel, created_at, updated_at
            "#,
            id,
            tenant_id,
            name,
            channel
        )
        .fetch_one(pool)
        .await
    }
}

impl Conversation {
    pub async fn create(
        pool: &PgPool,
        tenant_id: Uuid,
        contact_id: Uuid,
        inbox_id: Uuid,
        status: String,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as!(
            Conversation,
            r#"
            INSERT INTO conversations (id, tenant_id, contact_id, inbox_id, status)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, contact_id, inbox_id, status, assignee_id, created_at, updated_at
            "#,
            id,
            tenant_id,
            contact_id,
            inbox_id,
            status
        )
        .fetch_one(pool)
        .await
    }
}

impl Message {
    pub async fn create(
        pool: &PgPool,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        content: String,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as!(
            Message,
            r#"
            INSERT INTO messages (id, tenant_id, conversation_id, sender_type, content)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, conversation_id, sender_type, content, created_at, updated_at
            "#,
            id,
            tenant_id,
            conversation_id,
            sender_type,
            content
        )
        .fetch_one(pool)
        .await
    }
}
