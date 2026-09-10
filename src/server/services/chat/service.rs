use super::models::{ChatChannel, ChatContact, ChatConversation, ChatInbox, ChatMessage, SessionCapsule, EventDeliveryEnvelope};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

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
            RETURNING id, tenant_id, name, created_at, updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
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
            "#,
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
            "#,
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

    pub async fn get_or_create_session_capsule(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        customer_id: Option<Uuid>,
    ) -> Result<SessionCapsule, sqlx::Error> {
        let existing = sqlx::query_as::<_, SessionCapsule>(
            "SELECT id, tenant_id, conversation_id, customer_id, context, created_at, updated_at FROM session_capsules WHERE tenant_id = $1 AND conversation_id = $2"
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(capsule) = existing {
            return Ok(capsule);
        }

        sqlx::query_as(
            r#"
            INSERT INTO session_capsules (id, tenant_id, conversation_id, customer_id, context)
            VALUES ($1, $2, $3, $4, '{}'::jsonb)
            RETURNING id, tenant_id, conversation_id, customer_id, context, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(customer_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_session_capsule_context(
        &self,
        tenant_id: Uuid,
        capsule_id: Uuid,
        context: serde_json::Value,
    ) -> Result<SessionCapsule, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE session_capsules
            SET context = $1, updated_at = NOW()
            WHERE id = $2 AND tenant_id = $3
            RETURNING id, tenant_id, conversation_id, customer_id, context, created_at, updated_at
            "#
        )
        .bind(context)
        .bind(capsule_id)
        .bind(tenant_id)
        .fetch_one(&self.pool)
        .await
    }

    pub fn normalize_inbound_webhook(&self, payload: serde_json::Value, source: &str, tenant_id: Uuid) -> EventDeliveryEnvelope {
        EventDeliveryEnvelope {
            id: Uuid::new_v4(),
            tenant_id,
            source: source.to_string(),
            payload,
            timestamp: Utc::now(),
        }
    }
}
