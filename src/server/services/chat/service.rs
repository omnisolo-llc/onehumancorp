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
        tenant_id: Uuid,
        name: String,
        channel_type: String,
        settings: Option<serde_json::Value>,
    ) -> Result<ChatInbox, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name, channel_type, settings)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, channel_type, settings, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(channel_type)
        .bind(settings)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_inbox(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
    ) -> Result<ChatInbox, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, name, channel_type, settings, created_at, updated_at
            FROM chat_inboxes
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(inbox_id)
        .bind(tenant_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_contact(
        &self,
        tenant_id: Uuid,
        name: Option<String>,
        email: Option<String>,
        phone_number: Option<String>,
    ) -> Result<ChatContact, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, email, phone_number)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, email, phone_number, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone_number)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_contact(
        &self,
        tenant_id: Uuid,
        contact_id: Uuid,
    ) -> Result<ChatContact, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, name, email, phone_number, created_at, updated_at
            FROM chat_contacts
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(contact_id)
        .bind(tenant_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_contact_inbox(
        &self,
        tenant_id: Uuid,
        contact_id: Uuid,
        inbox_id: Uuid,
        source_id: Option<String>,
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
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        status: String,
        assignee_id: Option<Uuid>,
    ) -> Result<ChatConversation, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status, assignee_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, inbox_id, contact_id, status, assignee_id, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(status)
        .bind(assignee_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        content: String,
        sender_type: String,
        sender_id: Option<Uuid>,
    ) -> Result<ChatMessage, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, content, sender_type, sender_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, conversation_id, content, sender_type, sender_id, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(content)
        .bind(sender_type)
        .bind(sender_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn list_messages_for_conversation(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<Vec<ChatMessage>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, conversation_id, content, sender_type, sender_id, created_at, updated_at
            FROM chat_messages
            WHERE conversation_id = $1 AND tenant_id = $2
            ORDER BY created_at ASC
            "#
        )
        .bind(conversation_id)
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
    }
}
