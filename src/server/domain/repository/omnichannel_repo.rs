use sqlx::{FromRow};
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
pub struct ChannelAdapter {
    pub id: Uuid,
    pub inbox_id: Uuid,
    pub tenant_id: Uuid,
    pub provider_type: String,
    pub config: Option<sqlx::types::Json<serde_json::Value>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Conversation {
    pub id: Uuid,
    pub inbox_id: Uuid,
    pub tenant_id: Uuid,
    pub contact_id: Option<Uuid>,
    pub status: String,
    pub channel: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
    pub sender_type: String,
    pub direction: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

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

    pub async fn create_inbox(&self, tenant_id: Uuid, name: String) -> Result<Inbox, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, Inbox>(
            "INSERT INTO omni_inboxes (id, tenant_id, name) VALUES ($1, $2, $3) RETURNING id, tenant_id, name, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_channel_adapter(&self, inbox_id: Uuid, tenant_id: Uuid, provider_type: String, config: serde_json::Value) -> Result<ChannelAdapter, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, ChannelAdapter>(
            "INSERT INTO omni_channel_adapters (id, inbox_id, tenant_id, provider_type, config) VALUES ($1, $2, $3, $4, $5) RETURNING id, inbox_id, tenant_id, provider_type, config as \"config: sqlx::types::Json<serde_json::Value>\", created_at, updated_at",
        )
        .bind(id)
        .bind(inbox_id)
        .bind(tenant_id)
        .bind(provider_type)
        .bind(sqlx::types::Json(config))
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_contact(&self, tenant_id: Uuid, name: String, phone: Option<String>) -> Result<Contact, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, Contact>(
            "INSERT INTO omni_contacts (id, tenant_id, name, phone) VALUES ($1, $2, $3, $4) RETURNING id, tenant_id, name, phone, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(phone)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_conversation(&self, inbox_id: Uuid, tenant_id: Uuid, contact_id: Option<Uuid>, status: String, channel: Option<String>) -> Result<Conversation, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, Conversation>(
            "INSERT INTO omni_conversations (id, inbox_id, tenant_id, contact_id, status, channel) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, inbox_id, tenant_id, contact_id, status, channel, created_at, updated_at",
        )
        .bind(id)
        .bind(inbox_id)
        .bind(tenant_id)
        .bind(contact_id)
        .bind(status)
        .bind(channel)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_message(&self, tenant_id: Uuid, conversation_id: Uuid, content: String, sender_type: String, direction: Option<String>) -> Result<Message, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, Message>(
            "INSERT INTO omni_messages (id, tenant_id, conversation_id, content, sender_type, direction) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, tenant_id, conversation_id, content, sender_type, direction, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(content)
        .bind(sender_type)
        .bind(direction)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }


    pub async fn create_customer_profile(&self, tenant_id: Uuid, name: Option<String>) -> Result<CustomerProfile, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, CustomerProfile>(
            "INSERT INTO customer_profile (id, tenant_id, name) VALUES ($1, $2, $3) RETURNING id, tenant_id, name, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
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
        .bind(id)
        .bind(tenant_id)
        .bind(customer_id)
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
        .bind(id)
        .bind(work_item_id)
        .bind(response)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_ai_draft(&self, tenant_id: Uuid, message_id: Uuid, proposed_response: String, status: String) -> Result<AiDraft, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, AiDraft>(
            "INSERT INTO ai_drafts (id, tenant_id, message_id, proposed_response, status) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, message_id, proposed_response, status, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(message_id)
        .bind(proposed_response)
        .bind(status)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn get_conversation(&self, id: Uuid) -> Result<Option<Conversation>, sqlx::Error> {
        let record = sqlx::query_as::<_, Conversation>(
            "SELECT id, inbox_id, tenant_id, contact_id, status, channel, created_at, updated_at FROM omni_conversations WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn get_messages_by_conversation_id(&self, conversation_id: Uuid) -> Result<Vec<Message>, sqlx::Error> {
        let records = sqlx::query_as::<_, Message>(
            "SELECT id, tenant_id, conversation_id, content, sender_type, direction, created_at, updated_at FROM omni_messages WHERE conversation_id = $1 ORDER BY created_at ASC",
        )
        .bind(conversation_id)
        .fetch_all(&self.db.pool)
        .await?;
        Ok(records)
    }

    pub async fn get_ai_drafts_by_message_id(&self, message_id: Uuid) -> Result<Vec<AiDraft>, sqlx::Error> {
        let records = sqlx::query_as::<_, AiDraft>(
            "SELECT id, tenant_id, message_id, proposed_response, status, created_at, updated_at FROM ai_drafts WHERE message_id = $1",
        )
        .bind(message_id)
        .fetch_all(&self.db.pool)
        .await?;
        Ok(records)
    }

    pub async fn update_ai_draft_status(&self, id: Uuid, status: String) -> Result<AiDraft, sqlx::Error> {
        let record = sqlx::query_as::<_, AiDraft>(
            "UPDATE ai_drafts SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING id, tenant_id, message_id, proposed_response, status, created_at, updated_at",
        )
        .bind(status)
        .bind(id)
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
    fn test_conversation_struct() {
        let conv = Conversation {
            id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            contact_id: None,
            status: "OPEN".to_string(),
            channel: Some("Instagram".to_string()),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(conv.status, "OPEN");
    }

    #[test]
    fn test_message_struct() {
        let msg = Message {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            content: "Hello".to_string(),
            sender_type: "customer".to_string(),
            direction: Some("INBOUND".to_string()),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn test_inbox_struct() {
        let inbox = Inbox {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "Main Inbox".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(inbox.name, "Main Inbox");
    }
}
