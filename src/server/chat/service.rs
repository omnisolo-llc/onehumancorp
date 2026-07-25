use std::sync::Arc;
use uuid::Uuid;
use sqlx::PgPool;
use crate::chat::models::*;

pub struct ChatService {
    pool: Arc<PgPool>,
}

impl ChatService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, req: CreateInboxRequest) -> Result<Inbox, sqlx::Error> {
        let inbox = sqlx::query_as(
            r#"
            INSERT INTO inboxes (tenant_id, channel_type, settings)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, channel_type, settings, created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(req.channel_type)
        .bind(req.settings)
        .fetch_one(&*self.pool)
        .await?;

        Ok(inbox)
    }

    pub async fn create_contact(&self, tenant_id: Uuid, req: CreateContactRequest) -> Result<Contact, sqlx::Error> {
        let contact = sqlx::query_as(
            r#"
            INSERT INTO contacts (tenant_id, identifier, custom_attributes)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, identifier, custom_attributes, created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(req.identifier)
        .bind(req.custom_attributes)
        .fetch_one(&*self.pool)
        .await?;

        Ok(contact)
    }

    pub async fn create_conversation(&self, tenant_id: Uuid, req: CreateConversationRequest) -> Result<Conversation, sqlx::Error> {
        let conv = sqlx::query_as(
            r#"
            INSERT INTO conversations (tenant_id, inbox_id, contact_id, status)
            VALUES ($1, $2, $3, 'open')
            RETURNING id, tenant_id, inbox_id, contact_id, status, created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(req.inbox_id)
        .bind(req.contact_id)
        .fetch_one(&*self.pool)
        .await?;

        Ok(conv)
    }

    pub async fn send_message(&self, tenant_id: Uuid, req: SendMessageRequest) -> Result<Message, sqlx::Error> {
        let msg_type = req.message_type.unwrap_or_else(|| "text".to_string());

        let msg = sqlx::query_as(
            r#"
            INSERT INTO messages (tenant_id, conversation_id, content, message_type)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, conversation_id, content, message_type, created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(req.conversation_id)
        .bind(req.content)
        .bind(msg_type)
        .fetch_one(&*self.pool)
        .await?;

        // TODO: publish event to message bus (e.g. msgbus.rs)

        Ok(msg)
    }
}
