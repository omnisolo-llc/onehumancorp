use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use super::models::{ChatInbox, ChatContact, ChatConversation, ChatMessage};

pub struct ChatService {
    pool: PgPool,
}

impl ChatService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, tenant_id: &str, name: &str, channel_type: &str, channel_config: serde_json::Value) -> Result<ChatInbox, sqlx::Error> {
        let id = Uuid::new_v4().to_string();

        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let res = sqlx::query_as::<_, ChatInbox>(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name, channel_type, channel_config)
            VALUES ($1, $2, $3, $4, $5::jsonb)
            RETURNING id, tenant_id, name, channel_type, channel_config, created_at, updated_at
            "#)
            .bind(id)
            .bind(tenant_id)
            .bind(name)
            .bind(channel_type)
            .bind(channel_config)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(res)
    }

    pub async fn create_contact(&self, tenant_id: &str, name: Option<&str>, identifier: &str, attributes: serde_json::Value) -> Result<ChatContact, sqlx::Error> {
        let id = Uuid::new_v4().to_string();

        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let res = sqlx::query_as::<_, ChatContact>(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, identifier, attributes)
            VALUES ($1, $2, $3, $4, $5::jsonb)
            RETURNING id, tenant_id, name, identifier, attributes, created_at, updated_at
            "#)
            .bind(id)
            .bind(tenant_id)
            .bind(name)
            .bind(identifier)
            .bind(attributes)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(res)
    }

    pub async fn create_conversation(&self, tenant_id: &str, inbox_id: &str, contact_id: &str) -> Result<ChatConversation, sqlx::Error> {
        let id = Uuid::new_v4().to_string();

        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let res = sqlx::query_as::<_, ChatConversation>(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status)
            VALUES ($1, $2, $3, $4, 'open')
            RETURNING id, tenant_id, inbox_id, contact_id, status, created_at, updated_at
            "#)
            .bind(id)
            .bind(tenant_id)
            .bind(inbox_id)
            .bind(contact_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(res)
    }

    pub async fn create_message(&self, tenant_id: &str, conversation_id: &str, sender_id: Option<&str>, sender_type: &str, content: Option<&str>, message_type: &str, attributes: serde_json::Value) -> Result<ChatMessage, sqlx::Error> {
        let id = Uuid::new_v4().to_string();

        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let res = sqlx::query_as::<_, ChatMessage>(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_id, sender_type, content, message_type, additional_attributes)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8::jsonb)
            RETURNING id, tenant_id, conversation_id, sender_id, sender_type, content, message_type, additional_attributes, created_at, updated_at
            "#)
            .bind(id)
            .bind(tenant_id)
            .bind(conversation_id)
            .bind(sender_id)
            .bind(sender_type)
            .bind(content)
            .bind(message_type)
            .bind(attributes)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(res)
    }

    pub async fn get_conversations_for_inbox(&self, tenant_id: &str, inbox_id: &str) -> Result<Vec<ChatConversation>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let res = sqlx::query_as::<_, ChatConversation>(
            r#"
            SELECT id, tenant_id, inbox_id, contact_id, status, created_at, updated_at
            FROM chat_conversations
            WHERE tenant_id = $1 AND inbox_id = $2
            ORDER BY created_at DESC
            "#)
            .bind(tenant_id)
            .bind(inbox_id)
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(res)
    }

    pub async fn get_messages_for_conversation(&self, tenant_id: &str, conversation_id: &str) -> Result<Vec<ChatMessage>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let res = sqlx::query_as::<_, ChatMessage>(
            r#"
            SELECT id, tenant_id, conversation_id, sender_id, sender_type, content, message_type, additional_attributes, created_at, updated_at
            FROM chat_messages
            WHERE tenant_id = $1 AND conversation_id = $2
            ORDER BY created_at ASC
            "#)
            .bind(tenant_id)
            .bind(conversation_id)
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(res)
    }
}
