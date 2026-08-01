use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::db::DB;

#[derive(Clone, Debug, FromRow)]
pub struct OmniInbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub channel_type: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct OmniConversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
    pub snoozed_until: Option<DateTime<Utc>>,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct OmniMessage {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
    pub message_type: String,
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

    pub async fn create_inbox(&self, tenant_id: Uuid, name: String, channel_type: String) -> Result<OmniInbox, sqlx::Error> {
        let mut tx = self.db.pool.begin().await?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string()).await?;
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, OmniInbox>(
            "INSERT INTO omni_inboxes (id, tenant_id, name, channel_type) VALUES ($1, $2, $3, $4) RETURNING id, tenant_id, name, channel_type, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(channel_type)
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(record)
    }

    pub async fn create_conversation(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid) -> Result<OmniConversation, sqlx::Error> {
        let mut tx = self.db.pool.begin().await?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string()).await?;
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, OmniConversation>(
            "INSERT INTO omni_conversations (id, tenant_id, inbox_id, contact_id) VALUES ($1, $2, $3, $4) RETURNING id, tenant_id, inbox_id, contact_id, status, snoozed_until, last_activity_at, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(record)
    }

    pub async fn create_message(&self, tenant_id: Uuid, conversation_id: Uuid, content: String, message_type: String) -> Result<OmniMessage, sqlx::Error> {
        let mut tx = self.db.pool.begin().await?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string()).await?;
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, OmniMessage>(
            "INSERT INTO omni_messages (id, tenant_id, conversation_id, content, message_type) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, conversation_id, content, message_type, status, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(content)
        .bind(message_type)
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(record)
    }

    pub async fn get_conversations_for_tenant(&self, tenant_id: Uuid) -> Result<Vec<OmniConversation>, sqlx::Error> {
        let mut tx = self.db.pool.begin().await?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string()).await?;
        let records = sqlx::query_as::<_, OmniConversation>(
            "SELECT id, tenant_id, inbox_id, contact_id, status, snoozed_until, last_activity_at, created_at, updated_at FROM omni_conversations WHERE tenant_id = $1",
        )
        .bind(tenant_id)
        .fetch_all(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_omni_inbox_struct() {
        let inbox = OmniInbox {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "Main IG".to_string(),
            channel_type: "instagram".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(inbox.channel_type, "instagram");
    }

    #[test]
    fn test_omni_conversation_struct() {
        let conv = OmniConversation {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            status: "open".to_string(),
            snoozed_until: None,
            last_activity_at: None,
            created_at: None,
            updated_at: None,
        };
        assert_eq!(conv.status, "open");
    }

    #[test]
    fn test_omni_message_struct() {
        let msg = OmniMessage {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            content: "Hello".to_string(),
            message_type: "incoming".to_string(),
            status: "sent".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(msg.content, "Hello");
    }
}
