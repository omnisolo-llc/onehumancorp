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

    pub async fn create_inbox(
        &self,
        tenant_id: Uuid,
        name: String,
    ) -> Result<ChatInbox, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, name, working_hours_enabled, out_of_office_message, greeting_enabled, greeting_message, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_inbox(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        working_hours_enabled: Option<bool>,
        out_of_office_message: Option<String>,
        greeting_enabled: Option<bool>,
        greeting_message: Option<String>,
    ) -> Result<ChatInbox, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE chat_inboxes
            SET working_hours_enabled = COALESCE($3, working_hours_enabled),
                out_of_office_message = COALESCE($4, out_of_office_message),
                greeting_enabled = COALESCE($5, greeting_enabled),
                greeting_message = COALESCE($6, greeting_message),
                updated_at = NOW()
            WHERE id = $1 AND tenant_id = $2
            RETURNING id, tenant_id, name, working_hours_enabled, out_of_office_message, greeting_enabled, greeting_message, created_at, updated_at
            "#
        )
        .bind(inbox_id)
        .bind(tenant_id)
        .bind(working_hours_enabled)
        .bind(out_of_office_message)
        .bind(greeting_enabled)
        .bind(greeting_message)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_channel(
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

    pub async fn create_contact(
        &self,
        tenant_id: Uuid,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
    ) -> Result<ChatContact, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, email, phone)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, email, phone, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn start_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        assignee_id: Option<Uuid>,
        bot_assignee_id: Option<Uuid>,
    ) -> Result<ChatConversation, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, bot_assignee_id, status)
            VALUES ($1, $2, $3, $4, $5, $6, 'open')
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, bot_assignee_id, status, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(assignee_id)
        .bind(bot_assignee_id)
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
            SET status = $3, updated_at = NOW()
            WHERE id = $1 AND tenant_id = $2
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, bot_assignee_id, status, created_at, updated_at
            "#
        )
        .bind(conversation_id)
        .bind(tenant_id)
        .bind(status)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn assign_conversation(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        assignee_id: Option<Uuid>,
        bot_assignee_id: Option<Uuid>,
    ) -> Result<ChatConversation, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE chat_conversations
            SET assignee_id = COALESCE($3, assignee_id),
                bot_assignee_id = COALESCE($4, bot_assignee_id),
                updated_at = NOW()
            WHERE id = $1 AND tenant_id = $2
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, bot_assignee_id, status, created_at, updated_at
            "#
        )
        .bind(conversation_id)
        .bind(tenant_id)
        .bind(assignee_id)
        .bind(bot_assignee_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
    ) -> Result<ChatMessage, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(sender_id)
        .bind(content)
        .fetch_one(&self.pool)
        .await
    }
}
