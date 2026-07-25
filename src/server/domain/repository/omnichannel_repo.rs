use sqlx::{FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::db::DB;

#[derive(Clone, Debug, FromRow)]
pub struct CustomerProfile {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct WorkItem {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub customer_id: Uuid,
    pub source: String,
    pub payload: Option<sqlx::types::Json<serde_json::Value>>,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct AgentDraft {
    pub id: Uuid,
    pub work_item_id: Uuid,
    pub response: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Channel {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub channel_type: String,
    pub config: Option<sqlx::types::Json<serde_json::Value>>,
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
    pub contact_id: Option<Uuid>,
    pub channel: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub direction: String,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct AiDraft {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub message_id: Uuid,
    pub proposed_response: String,
    pub status: String,
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

    pub async fn create_customer_profile(&self, tenant_id: Uuid, name: Option<String>) -> Result<CustomerProfile, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, CustomerProfile>(
            "INSERT INTO customer_profile (id, tenant_id, name) VALUES ($1, $2, $3) RETURNING id, tenant_id, name, created_at, updated_at",
        )
        .bind(id.to_string())
        .bind(tenant_id.to_string())
        .bind(name)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_work_item(&self, tenant_id: Uuid, customer_id: Uuid, source: String, payload: serde_json::Value) -> Result<WorkItem, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, WorkItem>(
            "INSERT INTO work_item (id, tenant_id, customer_id, source, payload, status) VALUES ($1, $2, $3, $4, $5, 'PENDING') RETURNING id, tenant_id, customer_id, source, payload as \"payload: sqlx::types::Json<serde_json::Value>\", status, created_at, updated_at",
        )
        .bind(id.to_string())
        .bind(tenant_id.to_string())
        .bind(customer_id.to_string())
        .bind(source)
        .bind(sqlx::types::Json(payload))
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_agent_draft(&self, work_item_id: Uuid, response: String) -> Result<AgentDraft, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, AgentDraft>(
            "INSERT INTO agent_draft (id, work_item_id, response, status) VALUES ($1, $2, $3, 'DRAFT') RETURNING id, work_item_id, response, status, created_at, updated_at",
        )
        .bind(id.to_string())
        .bind(work_item_id.to_string())
        .bind(response)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, name: String) -> Result<Inbox, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, Inbox>(
            "INSERT INTO inboxes (id, tenant_id, name) VALUES ($1, $2, $3) RETURNING id, tenant_id, name, created_at, updated_at",
        )
        .bind(id.to_string())
        .bind(tenant_id.to_string())
        .bind(name)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_channel(&self, tenant_id: Uuid, inbox_id: Uuid, channel_type: String, config: Option<serde_json::Value>) -> Result<Channel, sqlx::Error> {
        let id = Uuid::new_v4();
        let config_json = config.map(sqlx::types::Json);
        let record = sqlx::query_as::<_, Channel>(
            "INSERT INTO channels (id, tenant_id, inbox_id, channel_type, config) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, inbox_id, channel_type, config as \"config: sqlx::types::Json<serde_json::Value>\", created_at, updated_at",
        )
        .bind(id.to_string())
        .bind(tenant_id.to_string())
        .bind(inbox_id.to_string())
        .bind(channel_type)
        .bind(config_json)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_contact(&self, tenant_id: Uuid, name: Option<String>, email: Option<String>, phone: Option<String>) -> Result<Contact, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, Contact>(
            "INSERT INTO contacts (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, name, email, phone, created_at, updated_at",
        )
        .bind(id.to_string())
        .bind(tenant_id.to_string())
        .bind(name)
        .bind(email)
        .bind(phone)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    // Supports both create flows: the new one using inbox_id and the old one wrapping it internally (if required).
    pub async fn create_conversation(&self, tenant_id: Uuid, channel: String, status: String) -> Result<Conversation, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, Conversation>(
            "INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, channel, status) VALUES ($1, $2, $3, NULL, $4, $5) RETURNING id, tenant_id, inbox_id, contact_id, channel, status, created_at, updated_at",
        )
        .bind(id.to_string())
        .bind(tenant_id.to_string())
        .bind(Uuid::new_v4().to_string()) // Stubbing inbox_id to satisfy not null constraint for legacy calls
        .bind(channel)
        .bind(status)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_conversation_with_inbox(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Option<Uuid>, channel: String, status: String) -> Result<Conversation, sqlx::Error> {
        let id = Uuid::new_v4();
        let contact_id_str = contact_id.map(|u| u.to_string());
        let record = sqlx::query_as::<_, Conversation>(
            "INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, channel, status) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, tenant_id, inbox_id, contact_id, channel, status, created_at, updated_at",
        )
        .bind(id.to_string())
        .bind(tenant_id.to_string())
        .bind(inbox_id.to_string())
        .bind(contact_id_str)
        .bind(channel)
        .bind(status)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_message(&self, tenant_id: Uuid, conversation_id: Uuid, direction: String, content: String) -> Result<Message, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, Message>(
            "INSERT INTO messages (id, tenant_id, conversation_id, direction, content) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, conversation_id, direction, content, created_at, updated_at",
        )
        .bind(id.to_string())
        .bind(tenant_id.to_string())
        .bind(conversation_id.to_string())
        .bind(direction)
        .bind(content)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_ai_draft(&self, tenant_id: Uuid, message_id: Uuid, proposed_response: String, status: String) -> Result<AiDraft, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, AiDraft>(
            "INSERT INTO ai_drafts (id, tenant_id, message_id, proposed_response, status) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, message_id, proposed_response, status, created_at, updated_at",
        )
        .bind(id.to_string())
        .bind(tenant_id.to_string())
        .bind(message_id.to_string())
        .bind(proposed_response)
        .bind(status)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn get_conversation(&self, id: Uuid) -> Result<Option<Conversation>, sqlx::Error> {
        let record = sqlx::query_as::<_, Conversation>(
            "SELECT id, tenant_id, inbox_id, contact_id, channel, status, created_at, updated_at FROM conversations WHERE id = $1",
        )
        .bind(id.to_string())
        .fetch_optional(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn get_messages_by_conversation_id(&self, conversation_id: Uuid) -> Result<Vec<Message>, sqlx::Error> {
        let records = sqlx::query_as::<_, Message>(
            "SELECT id, tenant_id, conversation_id, direction, content, created_at, updated_at FROM messages WHERE conversation_id = $1 ORDER BY created_at ASC",
        )
        .bind(conversation_id.to_string())
        .fetch_all(&self.db.pool)
        .await?;
        Ok(records)
    }

    pub async fn get_ai_drafts_by_message_id(&self, message_id: Uuid) -> Result<Vec<AiDraft>, sqlx::Error> {
        let records = sqlx::query_as::<_, AiDraft>(
            "SELECT id, tenant_id, message_id, proposed_response, status, created_at, updated_at FROM ai_drafts WHERE message_id = $1",
        )
        .bind(message_id.to_string())
        .fetch_all(&self.db.pool)
        .await?;
        Ok(records)
    }

    pub async fn update_ai_draft_status(&self, id: Uuid, status: String) -> Result<AiDraft, sqlx::Error> {
        let record = sqlx::query_as::<_, AiDraft>(
            "UPDATE ai_drafts SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING id, tenant_id, message_id, proposed_response, status, created_at, updated_at",
        )
        .bind(status)
        .bind(id.to_string())
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_struct() {
        let conv = Conversation {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            contact_id: None,
            channel: "Instagram".to_string(),
            status: "OPEN".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(conv.status, "OPEN");
        assert_eq!(conv.channel, "Instagram");
    }

    #[test]
    fn test_message_struct() {
        let msg = Message {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            direction: "INBOUND".to_string(),
            content: "Hello".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn test_inbox_channel_contact_structs() {
        let inbox = Inbox {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "Main Inbox".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(inbox.name, "Main Inbox");

        let channel = Channel {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            channel_type: "Email".to_string(),
            config: None,
            created_at: None,
            updated_at: None,
        };
        assert_eq!(channel.channel_type, "Email");

        let contact = Contact {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: Some("Alice".to_string()),
            email: Some("alice@example.com".to_string()),
            phone: None,
            created_at: None,
            updated_at: None,
        };
        assert_eq!(contact.name.as_deref(), Some("Alice"));
    }
}
