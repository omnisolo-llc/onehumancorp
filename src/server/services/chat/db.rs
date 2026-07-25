#![allow(unused_imports)]
use crate::services::chat::models::{ChannelAdapter, Contact, Conversation, Inbox, Message};
use sqlx::{PgPool, Result};
use uuid::Uuid;

pub struct ChatDb {
    pool: PgPool,
}

impl ChatDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Inboxes
    pub async fn create_inbox(&self, tenant_id: Uuid, name: &str) -> Result<Inbox> {
        sqlx::query_as::<_, Inbox>(
            r#"
            INSERT INTO inboxes (tenant_id, name)
            VALUES ($1, $2)
            RETURNING id, tenant_id, name, created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_inbox(&self, tenant_id: Uuid, id: Uuid) -> Result<Inbox> {
        sqlx::query_as::<_, Inbox>(
            r#"
            SELECT id, tenant_id, name, created_at, updated_at
            FROM inboxes
            WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    // Channel Adapters
    pub async fn create_channel_adapter(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        provider_type: &str,
        credentials: serde_json::Value,
    ) -> Result<ChannelAdapter> {
        sqlx::query_as::<_, ChannelAdapter>(
            r#"
            INSERT INTO channel_adapters (tenant_id, inbox_id, provider_type, credentials)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, inbox_id, provider_type, credentials as "credentials: sqlx::types::Json<serde_json::Value>", created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(provider_type)
        .bind(sqlx::types::Json(credentials))
        .fetch_one(&self.pool)
        .await
    }

    // Contacts
    pub async fn create_contact(&self, tenant_id: Uuid, identifier: &str, name: Option<&str>) -> Result<Contact> {
        sqlx::query_as::<_, Contact>(
            r#"
            INSERT INTO contacts (tenant_id, identifier, name)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, identifier, name, created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(identifier)
        .bind(name)
        .fetch_one(&self.pool)
        .await
    }

    // Conversations
    pub async fn create_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        status: &str,
    ) -> Result<Conversation> {
        sqlx::query_as::<_, Conversation>(
            r#"
            INSERT INTO conversations (tenant_id, inbox_id, contact_id, status)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, inbox_id, contact_id, status, created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(status)
        .fetch_one(&self.pool)
        .await
    }

    // Messages
    pub async fn create_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        content: &str,
        message_type: &str,
    ) -> Result<Message> {
        sqlx::query_as::<_, Message>(
            r#"
            INSERT INTO messages (tenant_id, conversation_id, content, message_type)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, conversation_id, content, message_type, created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(content)
        .bind(message_type)
        .fetch_one(&self.pool)
        .await
    }
}
