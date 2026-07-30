use sqlx::{FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::db::DB;

#[derive(Clone, Debug, FromRow)]
pub struct ChatInbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct ChatChannel {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub channel_type: String,
    pub config: Option<sqlx::types::Json<serde_json::Value>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct ChatContact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
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

#[derive(Clone, Debug, FromRow)]
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

pub struct OmniChannelRepo {
    db: Arc<DB>,
}

impl OmniChannelRepo {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, name: String) -> Result<ChatInbox, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, ChatInbox>(
            "INSERT INTO chat_inboxes (id, tenant_id, name) VALUES ($1, $2, $3) RETURNING id, tenant_id, name, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_channel(&self, tenant_id: Uuid, inbox_id: Uuid, channel_type: String, config: serde_json::Value) -> Result<ChatChannel, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, ChatChannel>(
            "INSERT INTO chat_channels (id, tenant_id, inbox_id, channel_type, config) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, inbox_id, channel_type, config as \"config: sqlx::types::Json<serde_json::Value>\", created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(channel_type)
        .bind(sqlx::types::Json(config))
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_contact(&self, tenant_id: Uuid, name: Option<String>, email: Option<String>, phone: Option<String>) -> Result<ChatContact, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, ChatContact>(
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

    pub async fn create_conversation(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid, assignee_id: Option<Uuid>, status: String) -> Result<ChatConversation, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, ChatConversation>(
            "INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(assignee_id)
        .bind(status)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_message(&self, tenant_id: Uuid, conversation_id: Uuid, sender_type: String, sender_id: Option<Uuid>, content: String) -> Result<ChatMessage, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, ChatMessage>(
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

    pub async fn get_conversation(&self, id: Uuid) -> Result<Option<ChatConversation>, sqlx::Error> {
        let record = sqlx::query_as::<_, ChatConversation>(
            "SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at FROM chat_conversations WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn get_messages_by_conversation_id(&self, conversation_id: Uuid) -> Result<Vec<ChatMessage>, sqlx::Error> {
        let records = sqlx::query_as::<_, ChatMessage>(
            "SELECT id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at FROM chat_messages WHERE conversation_id = $1",
        )
        .bind(conversation_id)
        .fetch_all(&self.db.pool)
        .await?;
        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_inbox_struct() {
        let inbox = ChatInbox {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "Main Inbox".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(inbox.name, "Main Inbox");
    }

    #[test]
    fn test_channel_struct() {
        let channel = ChatChannel {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            channel_type: "web".to_string(),
            config: None,
            created_at: None,
            updated_at: None,
        };
        assert_eq!(channel.channel_type, "web");
    }

    #[test]
    fn test_contact_struct() {
        let contact = ChatContact {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: Some("John Doe".to_string()),
            email: None,
            phone: None,
            created_at: None,
            updated_at: None,
        };
        assert_eq!(contact.name, Some("John Doe".to_string()));
    }

    #[test]
    fn test_conversation_struct() {
        let conv = ChatConversation {
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
    fn test_message_struct() {
        let msg = ChatMessage {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            sender_type: "contact".to_string(),
            sender_id: None,
            content: "Hello".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(msg.content, "Hello");
    }
}
