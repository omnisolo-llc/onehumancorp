use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::db::DB;

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
    pub provider_type: String,
    pub credentials: Option<sqlx::types::Json<serde_json::Value>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Contact {
    pub id: String,
    pub tenant_id: String,
    pub name: Option<String>,
    pub phone_number: Option<String>,
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
    pub content: String,
    pub sender_type: String,
    pub is_private_note: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct OmnichannelCoreRepo {
    db: Arc<DB>,
}

impl OmnichannelCoreRepo {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_inbox(&self, tenant_id: String, name: String) -> Result<Inbox, sqlx::Error> {
        let id = Uuid::new_v4().to_string();

        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let record = sqlx::query_as::<_, Inbox>(
                    "INSERT INTO inboxes (id, tenant_id, name) VALUES ($1, $2, $3) RETURNING id, tenant_id, name, created_at, updated_at",
                )
                .bind(&id)
                .bind(&tenant_id)
                .bind(&name)
                .fetch_one(&self.db.pool)
                .await?;
                Ok(record)
            },
            crate::db::DbStore::Sqlite(pool) => {
                let record = sqlx::query_as::<_, Inbox>(
                    "INSERT INTO inboxes (id, tenant_id, name) VALUES (?, ?, ?) RETURNING id, tenant_id, name, created_at, updated_at",
                )
                .bind(&id)
                .bind(&tenant_id)
                .bind(&name)
                .fetch_one(pool)
                .await?;
                Ok(record)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_inbox_struct() {
        let inbox = Inbox {
            id: Uuid::new_v4().to_string(),
            tenant_id: Uuid::new_v4().to_string(),
            name: "Test Inbox".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(inbox.name, "Test Inbox");
    }

    #[test]
    fn test_channel_struct() {
        let channel = Channel {
            id: Uuid::new_v4().to_string(),
            tenant_id: Uuid::new_v4().to_string(),
            inbox_id: Uuid::new_v4().to_string(),
            provider_type: "whatsapp".to_string(),
            credentials: None,
            created_at: None,
            updated_at: None,
        };
        assert_eq!(channel.provider_type, "whatsapp");
    }

    #[test]
    fn test_contact_struct() {
        let contact = Contact {
            id: Uuid::new_v4().to_string(),
            tenant_id: Uuid::new_v4().to_string(),
            name: Some("Test Name".to_string()),
            phone_number: Some("+123".to_string()),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(contact.phone_number, Some("+123".to_string()));
    }

    #[test]
    fn test_conversation_struct() {
        let conv = Conversation {
            id: Uuid::new_v4().to_string(),
            tenant_id: Uuid::new_v4().to_string(),
            inbox_id: Uuid::new_v4().to_string(),
            contact_id: Uuid::new_v4().to_string(),
            status: "open".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(conv.status, "open");
    }

    #[test]
    fn test_message_struct() {
        let msg = Message {
            id: Uuid::new_v4().to_string(),
            tenant_id: Uuid::new_v4().to_string(),
            conversation_id: Uuid::new_v4().to_string(),
            content: "Hello".to_string(),
            sender_type: "contact".to_string(),
            is_private_note: Some(false),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(msg.content, "Hello");
        assert_eq!(msg.sender_type, "contact");
    }
}
