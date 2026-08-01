use sqlx::PgPool;
use uuid::Uuid;
<<<<<<< HEAD
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};
=======
use super::models::{ChatInbox, ChatContact, ChatContactInbox, ChatConversation, ChatMessage};
>>>>>>> 631d30d93 (security: synchronize npm dependency locks)

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
<<<<<<< HEAD
    ) -> Result<ChatInbox, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, name, created_at, updated_at
=======
        channel_type: String,
        settings: Option<serde_json::Value>,
    ) -> Result<ChatInbox, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name, channel_type, settings)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, channel_type, settings, created_at, updated_at
>>>>>>> 631d30d93 (security: synchronize npm dependency locks)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
<<<<<<< HEAD
=======
        .bind(channel_type)
        .bind(settings)
>>>>>>> 631d30d93 (security: synchronize npm dependency locks)
        .fetch_one(&self.pool)
        .await
    }

<<<<<<< HEAD
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
=======
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
>>>>>>> 631d30d93 (security: synchronize npm dependency locks)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_contact(
        &self,
        tenant_id: Uuid,
        name: Option<String>,
        email: Option<String>,
<<<<<<< HEAD
        phone: Option<String>,
    ) -> Result<ChatContact, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, email, phone)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, email, phone, created_at, updated_at
=======
        phone_number: Option<String>,
    ) -> Result<ChatContact, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, email, phone_number)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, email, phone_number, created_at, updated_at
>>>>>>> 631d30d93 (security: synchronize npm dependency locks)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(email)
<<<<<<< HEAD
        .bind(phone)
=======
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
>>>>>>> 631d30d93 (security: synchronize npm dependency locks)
        .fetch_one(&self.pool)
        .await
    }

<<<<<<< HEAD
    pub async fn start_conversation(
=======
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
>>>>>>> 631d30d93 (security: synchronize npm dependency locks)
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
<<<<<<< HEAD
=======
        status: String,
>>>>>>> 631d30d93 (security: synchronize npm dependency locks)
        assignee_id: Option<Uuid>,
    ) -> Result<ChatConversation, sqlx::Error> {
        sqlx::query_as(
            r#"
<<<<<<< HEAD
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status)
            VALUES ($1, $2, $3, $4, $5, 'open')
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
=======
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status, assignee_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, inbox_id, contact_id, status, assignee_id, created_at, updated_at
>>>>>>> 631d30d93 (security: synchronize npm dependency locks)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
<<<<<<< HEAD
=======
        .bind(status)
>>>>>>> 631d30d93 (security: synchronize npm dependency locks)
        .bind(assignee_id)
        .fetch_one(&self.pool)
        .await
    }

<<<<<<< HEAD
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
=======
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
>>>>>>> 631d30d93 (security: synchronize npm dependency locks)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(conversation_id)
<<<<<<< HEAD
        .bind(sender_type)
        .bind(sender_id)
        .bind(content)
        .fetch_one(&self.pool)
        .await
    }
=======
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
>>>>>>> 631d30d93 (security: synchronize npm dependency locks)
}
