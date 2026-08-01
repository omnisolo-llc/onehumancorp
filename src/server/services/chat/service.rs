use sqlx::PgPool;
use uuid::Uuid;
use super::models::{ChatInbox, ChatContact, ChatContactInbox, ChatConversation, ChatMessage};

pub struct ChatService {
    pool: PgPool,
}

impl ChatService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(
        &self,
        tenant_id: &str,
        name: &str,
        channel_type: &str,
        config: Option<serde_json::Value>,
    ) -> Result<ChatInbox, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name, channel_type, config)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, channel_type, config, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(channel_type)
        .bind(config.unwrap_or(serde_json::json!({})))
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_contact(
        &self,
        tenant_id: &str,
        name: Option<&str>,
        email: Option<&str>,
        phone_number: Option<&str>,
        identifier: Option<&str>,
    ) -> Result<ChatContact, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, email, phone_number, identifier)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, name, email, phone_number, identifier, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone_number)
        .bind(identifier)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_contact_inbox(
        &self,
        tenant_id: &str,
        contact_id: Uuid,
        inbox_id: Uuid,
        source_id: Option<&str>,
    ) -> Result<ChatContactInbox, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_contact_inboxes (id, tenant_id, contact_id, inbox_id, source_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, contact_id, inbox_id, source_id, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(contact_id)
        .bind(inbox_id)
        .bind(source_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_conversation(
        &self,
        tenant_id: &str,
        inbox_id: Uuid,
        contact_id: Uuid,
        assignee_id: Option<Uuid>,
    ) -> Result<ChatConversation, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status)
            VALUES ($1, $2, $3, $4, $5, 'open')
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(assignee_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_conversation(
        &self,
        tenant_id: &str,
        conversation_id: Uuid,
    ) -> Result<ChatConversation, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            FROM chat_conversations
            WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_message(
        &self,
        tenant_id: &str,
        conversation_id: Uuid,
        message_type: &str,
        sender_id: Option<Uuid>,
        content: &str,
    ) -> Result<ChatMessage, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, message_type, sender_id, content)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, conversation_id, message_type, sender_id, status, content, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(message_type)
        .bind(sender_id)
        .bind(content)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_message(
        &self,
        tenant_id: &str,
        message_id: Uuid,
    ) -> Result<ChatMessage, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, conversation_id, message_type, sender_id, status, content, created_at, updated_at
            FROM chat_messages
            WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(message_id)
        .fetch_one(&self.pool)
        .await
    }
}
