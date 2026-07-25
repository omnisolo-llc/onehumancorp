use sqlx::{FromRow};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::db::DB;

#[derive(Clone, Debug, FromRow)]
pub struct Inbox {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Channel {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub channel_type: String,
    pub credentials: sqlx::types::Json<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Contact {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub avatar_url: Option<String>,
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
    pub last_activity_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Message {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub content: String,
    pub message_type: String,
    pub sender_id: Option<String>,
    pub attachments: Option<sqlx::types::Json<serde_json::Value>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct OmniChannelRepo {
    db: Arc<DB>,
}

impl OmniChannelRepo {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_inbox(&self, id: String, tenant_id: String, name: String) -> Result<Inbox, sqlx::Error> {
        let record = sqlx::query_as::<_, Inbox>(
            "INSERT INTO inboxes (id, tenant_id, name) VALUES ($1, $2, $3) RETURNING id, tenant_id, name, is_active, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_channel(&self, id: String, tenant_id: String, inbox_id: String, channel_type: String, credentials: serde_json::Value) -> Result<Channel, sqlx::Error> {
        let record = sqlx::query_as::<_, Channel>(
            "INSERT INTO channels (id, tenant_id, inbox_id, channel_type, credentials) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, inbox_id, channel_type, credentials as \"credentials: sqlx::types::Json<serde_json::Value>\", created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(channel_type)
        .bind(sqlx::types::Json(credentials))
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_contact(&self, id: String, tenant_id: String, name: String, email: Option<String>, phone_number: Option<String>) -> Result<Contact, sqlx::Error> {
        let record = sqlx::query_as::<_, Contact>(
            "INSERT INTO contacts (id, tenant_id, name, email, phone_number) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, name, email, phone_number, avatar_url, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone_number)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_conversation(&self, id: String, tenant_id: String, inbox_id: String, contact_id: String) -> Result<Conversation, sqlx::Error> {
        let record = sqlx::query_as::<_, Conversation>(
            "INSERT INTO conversations (id, tenant_id, inbox_id, contact_id) VALUES ($1, $2, $3, $4) RETURNING id, tenant_id, inbox_id, contact_id, status, last_activity_at, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_message(&self, id: String, tenant_id: String, conversation_id: String, content: String, message_type: String) -> Result<Message, sqlx::Error> {
        let record = sqlx::query_as::<_, Message>(
            "INSERT INTO messages (id, tenant_id, conversation_id, content, message_type) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, conversation_id, content, message_type, sender_id, attachments as \"attachments: sqlx::types::Json<serde_json::Value>\", created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(content)
        .bind(message_type)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inbox_struct() {
        let inbox = Inbox {
            id: "1".to_string(),
            tenant_id: "t1".to_string(),
            name: "Main".to_string(),
            is_active: true,
            created_at: None,
            updated_at: None,
        };
        assert_eq!(inbox.name, "Main");
    }

    #[test]
    fn test_conversation_struct() {
        let conv = Conversation {
            id: "1".to_string(),
            tenant_id: "t1".to_string(),
            inbox_id: "inbox1".to_string(),
            contact_id: "contact1".to_string(),
            status: "open".to_string(),
            last_activity_at: None,
            created_at: None,
            updated_at: None,
        };
        assert_eq!(conv.status, "open");
    }
}
