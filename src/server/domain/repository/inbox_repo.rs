use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::db::DB;

#[derive(Clone, Debug, FromRow)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct InboxRepo {
    db: Arc<DB>,
}

impl InboxRepo {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, name: String) -> Result<Inbox, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, Inbox>(
            "INSERT INTO chat_inboxes (id, tenant_id, name) VALUES ($1, $2, $3) RETURNING id, tenant_id, name, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_contact(&self, tenant_id: Uuid, name: Option<String>, email: Option<String>, phone: Option<String>) -> Result<Contact, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, Contact>(
            "INSERT INTO chat_contacts (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, name, email, phone, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_conversation(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid, status: String) -> Result<Conversation, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, Conversation>(
            "INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(status)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_message(&self, tenant_id: Uuid, conversation_id: Uuid, sender_type: String, sender_id: Option<Uuid>, content: String) -> Result<Message, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, Message>(
            "INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(sender_id)
        .bind(content)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_inbox_struct_creation() {
        let inbox = Inbox {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "Main Support".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(inbox.name, "Main Support");
    }

    #[test]
    fn test_contact_struct_creation() {
        let contact = Contact {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: Some("Maya".to_string()),
            email: Some("maya@example.com".to_string()),
            phone: None,
            created_at: None,
            updated_at: None,
        };
        assert_eq!(contact.email.unwrap(), "maya@example.com");
    }

    #[test]
    fn test_conversation_struct_creation() {
        let conv = Conversation {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            assignee_id: None,
            status: "open".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(conv.status, "open");
    }

    #[test]
    fn test_message_struct_creation() {
        let msg = Message {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            sender_type: "customer".to_string(),
            sender_id: Some(Uuid::new_v4()),
            content: "Do you make vegan cakes?".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(msg.content, "Do you make vegan cakes?");
    }
}
