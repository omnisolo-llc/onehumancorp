use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: String,
    pub channel_type: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub identifier: Option<String>,
    pub custom_attributes: Option<sqlx::types::Json<serde_json::Value>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: String,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
    pub assignee_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: String,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<String>,
    pub content: String,
    pub message_type: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct OmnichannelRepo {
    pool: PgPool,
}

impl OmnichannelRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, tenant_id: &str, name: &str, channel_type: &str) -> Result<Inbox, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, Inbox>(
            "INSERT INTO inboxes (id, tenant_id, name, channel_type) VALUES ($1, $2, $3, $4) RETURNING *"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(channel_type)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_contact(&self, tenant_id: &str, name: &str, email: Option<&str>, phone: Option<&str>) -> Result<Contact, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, Contact>(
            "INSERT INTO contacts (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5) RETURNING *"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_conversation(&self, tenant_id: &str, inbox_id: Uuid, contact_id: Uuid, status: &str) -> Result<Conversation, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, Conversation>(
            "INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, $5) RETURNING *"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(status)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn add_message(&self, tenant_id: &str, conversation_id: Uuid, sender_type: &str, content: &str, message_type: &str) -> Result<Message, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, Message>(
            "INSERT INTO messages (id, tenant_id, conversation_id, sender_type, content, message_type) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(content)
        .bind(message_type)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn list_conversations(&self, tenant_id: &str) -> Result<Vec<Conversation>, sqlx::Error> {
        sqlx::query_as::<_, Conversation>("SELECT * FROM conversations WHERE tenant_id = $1 ORDER BY created_at DESC")
            .bind(tenant_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn list_messages(&self, tenant_id: &str, conversation_id: Uuid) -> Result<Vec<Message>, sqlx::Error> {
        sqlx::query_as::<_, Message>("SELECT * FROM messages WHERE tenant_id = $1 AND conversation_id = $2 ORDER BY created_at ASC")
            .bind(tenant_id)
            .bind(conversation_id)
            .fetch_all(&self.pool)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_models_have_expected_fields() {
        let id = Uuid::new_v4();
        let contact = Contact {
            id,
            tenant_id: "t1".to_string(),
            name: "Test".to_string(),
            email: None,
            phone: None,
            identifier: None,
            custom_attributes: None,
            created_at: None,
            updated_at: None,
        };
        assert_eq!(contact.name, "Test");

        let inbox = Inbox {
            id,
            tenant_id: "t1".to_string(),
            name: "Test Inbox".to_string(),
            channel_type: "WebWidget".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(inbox.channel_type, "WebWidget");

        let conversation = Conversation {
            id,
            tenant_id: "t1".to_string(),
            inbox_id: id,
            contact_id: id,
            status: "Open".to_string(),
            assignee_id: None,
            created_at: None,
            updated_at: None,
        };
        assert_eq!(conversation.status, "Open");

        let message = Message {
            id,
            tenant_id: "t1".to_string(),
            conversation_id: id,
            sender_type: "Contact".to_string(),
            sender_id: None,
            content: "Hello".to_string(),
            message_type: "Incoming".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(message.content, "Hello");
    }
}
