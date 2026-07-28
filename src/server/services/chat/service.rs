use sqlx::PgPool;
use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};

pub struct ChatService {
    pool: PgPool,
}

impl ChatService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox_record(
        &self,
        tenant_id: Uuid,
        name: String,
        enable_auto_assignment: Option<bool>,
        greeting_message: Option<String>,
        working_hours_enabled: Option<bool>,
    ) -> Result<ChatInbox, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name, enable_auto_assignment, greeting_message, working_hours_enabled)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, name, enable_auto_assignment, greeting_message, working_hours_enabled, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(enable_auto_assignment)
        .bind(greeting_message)
        .bind(working_hours_enabled)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_channel_record(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        channel_type: String,
        config: serde_json::Value,
    ) -> Result<ChatChannel, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_channels (id, tenant_id, inbox_id, channel_type, config)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, inbox_id, channel_type, config, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(channel_type)
        .bind(config)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_contact_record(
        &self,
        tenant_id: Uuid,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        custom_attributes: Option<serde_json::Value>,
    ) -> Result<ChatContact, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, email, phone, custom_attributes)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, name, email, phone, custom_attributes, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .bind(custom_attributes)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn start_chat_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        assignee_id: Option<Uuid>,
        priority: Option<i32>,
    ) -> Result<ChatConversation, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status, priority)
            VALUES ($1, $2, $3, $4, $5, 'open', $6)
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, priority, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(assignee_id)
        .bind(priority)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_conversation_status(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        status: String,
    ) -> Result<ChatConversation, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE chat_conversations
            SET status = $1, updated_at = NOW()
            WHERE id = $2 AND tenant_id = $3
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, priority, created_at, updated_at
            "#
        )
        .bind(status)
        .bind(conversation_id)
        .bind(tenant_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn send_chat_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
        content_type: Option<String>,
        additional_attributes: Option<serde_json::Value>,
    ) -> Result<ChatMessage, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content, content_type, additional_attributes)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, content_type, additional_attributes, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(sender_id)
        .bind(content)
        .bind(content_type)
        .bind(additional_attributes)
        .fetch_one(&self.pool)
        .await
    }
}
