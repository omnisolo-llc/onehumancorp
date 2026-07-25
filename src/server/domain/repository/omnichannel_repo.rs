use sqlx::{FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::db::DB;

#[derive(Clone, Debug, FromRow)]
pub struct OmnichannelContact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct OmnichannelInbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub channel_type: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct OmnichannelConversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Option<Uuid>,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct OmnichannelMessage {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub content: String,
    pub is_private: bool,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Keep old structs for backward compatibility for now
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


pub struct OmniChannelRepo {
    db: Arc<DB>,
}

impl OmniChannelRepo {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_contact(&self, tenant_id: Uuid, name: Option<String>, email: Option<String>, phone: Option<String>) -> Result<OmnichannelContact, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, OmnichannelContact>(
            "INSERT INTO omnichannel_contact (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5) RETURNING *"
        )
        .bind(id).bind(tenant_id).bind(name).bind(email).bind(phone)
        .fetch_one(&self.db.pool).await
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, name: String, channel_type: String) -> Result<OmnichannelInbox, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, OmnichannelInbox>(
            "INSERT INTO omnichannel_inbox (id, tenant_id, name, channel_type) VALUES ($1, $2, $3, $4) RETURNING *"
        )
        .bind(id).bind(tenant_id).bind(name).bind(channel_type)
        .fetch_one(&self.db.pool).await
    }

    pub async fn create_conversation(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Option<Uuid>, status: String) -> Result<OmnichannelConversation, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, OmnichannelConversation>(
            "INSERT INTO omnichannel_conversation (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, $5) RETURNING *"
        )
        .bind(id).bind(tenant_id).bind(inbox_id).bind(contact_id).bind(status)
        .fetch_one(&self.db.pool).await
    }

    pub async fn create_message(&self, tenant_id: Uuid, conversation_id: Uuid, sender_type: String, content: String, is_private: bool, status: String) -> Result<OmnichannelMessage, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, OmnichannelMessage>(
            "INSERT INTO omnichannel_message (id, tenant_id, conversation_id, sender_type, content, is_private, status) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
        )
        .bind(id).bind(tenant_id).bind(conversation_id).bind(sender_type).bind(content).bind(is_private).bind(status)
        .fetch_one(&self.db.pool).await
    }

    pub async fn get_conversation(&self, id: Uuid) -> Result<Option<OmnichannelConversation>, sqlx::Error> {
        sqlx::query_as::<_, OmnichannelConversation>("SELECT * FROM omnichannel_conversation WHERE id = $1")
        .bind(id).fetch_optional(&self.db.pool).await
    }

    pub async fn update_conversation_status(&self, id: Uuid, status: String) -> Result<OmnichannelConversation, sqlx::Error> {
        sqlx::query_as::<_, OmnichannelConversation>("UPDATE omnichannel_conversation SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING *")
        .bind(status).bind(id)
        .fetch_one(&self.db.pool).await
    }

    // Keep old ones for backward compat
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

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structs() {
        let msg = OmnichannelMessage {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            sender_type: "customer".to_string(),
            content: "hello".to_string(),
            is_private: false,
            status: "delivered".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(msg.content, "hello");
    }
}
