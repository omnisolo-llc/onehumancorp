use sqlx::PgPool;
use uuid::Uuid;

use super::models::{ChatContact, ChatMessage, Conversation, Inbox};

pub struct ChatDb;

impl ChatDb {
    pub async fn get_inboxes(
        pool: &PgPool,
        tenant_id: &str,
    ) -> Result<Vec<Inbox>, sqlx::Error> {
        let inboxes = sqlx::query_as!(
            Inbox,
            "SELECT id, tenant_id, name, created_at, updated_at FROM inboxes WHERE tenant_id = $1",
            tenant_id
        )
        .fetch_all(pool)
        .await?;
        Ok(inboxes)
    }

    pub async fn get_conversations(
        pool: &PgPool,
        tenant_id: &str,
        inbox_id: Uuid,
    ) -> Result<Vec<Conversation>, sqlx::Error> {
        let conversations = sqlx::query_as!(
            Conversation,
            "SELECT id, tenant_id, inbox_id, contact_id, status, created_at, updated_at FROM conversations WHERE tenant_id = $1 AND inbox_id = $2",
            tenant_id,
            inbox_id
        )
        .fetch_all(pool)
        .await?;
        Ok(conversations)
    }

    pub async fn get_messages(
        pool: &PgPool,
        tenant_id: &str,
        conversation_id: Uuid,
    ) -> Result<Vec<ChatMessage>, sqlx::Error> {
        let messages = sqlx::query_as!(
            ChatMessage,
            "SELECT id, tenant_id, conversation_id, sender_type, sender_id, content, is_ai_draft, created_at, updated_at FROM chat_messages WHERE tenant_id = $1 AND conversation_id = $2 ORDER BY created_at ASC",
            tenant_id,
            conversation_id
        )
        .fetch_all(pool)
        .await?;
        Ok(messages)
    }

    pub async fn insert_message(
        pool: &PgPool,
        tenant_id: &str,
        conversation_id: Uuid,
        sender_type: &str,
        sender_id: Option<Uuid>,
        content: &str,
        is_ai_draft: bool,
    ) -> Result<ChatMessage, sqlx::Error> {
        let message = sqlx::query_as!(
            ChatMessage,
            "INSERT INTO chat_messages (tenant_id, conversation_id, sender_type, sender_id, content, is_ai_draft) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, is_ai_draft, created_at, updated_at",
            tenant_id,
            conversation_id,
            sender_type,
            sender_id,
            content,
            is_ai_draft
        )
        .fetch_one(pool)
        .await?;
        Ok(message)
    }

    pub async fn create_inbox(
        pool: &PgPool,
        tenant_id: &str,
        name: &str,
    ) -> Result<Inbox, sqlx::Error> {
        let inbox = sqlx::query_as!(
            Inbox,
            "INSERT INTO inboxes (tenant_id, name) VALUES ($1, $2) RETURNING id, tenant_id, name, created_at, updated_at",
            tenant_id,
            name
        )
        .fetch_one(pool)
        .await?;
        Ok(inbox)
    }

    pub async fn create_contact(
        pool: &PgPool,
        tenant_id: &str,
        name: Option<&str>,
        email: Option<&str>,
        phone: Option<&str>,
        external_id: Option<&str>,
    ) -> Result<ChatContact, sqlx::Error> {
        let contact = sqlx::query_as!(
            ChatContact,
            "INSERT INTO chat_contacts (tenant_id, name, email, phone, external_id) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, name, email, phone, external_id, created_at, updated_at",
            tenant_id,
            name,
            email,
            phone,
            external_id
        )
        .fetch_one(pool)
        .await?;
        Ok(contact)
    }

    pub async fn create_conversation(
        pool: &PgPool,
        tenant_id: &str,
        inbox_id: Uuid,
        contact_id: Uuid,
    ) -> Result<Conversation, sqlx::Error> {
        let conversation = sqlx::query_as!(
            Conversation,
            "INSERT INTO conversations (tenant_id, inbox_id, contact_id) VALUES ($1, $2, $3) RETURNING id, tenant_id, inbox_id, contact_id, status, created_at, updated_at",
            tenant_id,
            inbox_id,
            contact_id
        )
        .fetch_one(pool)
        .await?;
        Ok(conversation)
    }

    pub async fn mark_draft_as_sent(
        pool: &PgPool,
        tenant_id: &str,
        message_id: Uuid,
    ) -> Result<ChatMessage, sqlx::Error> {
        let message = sqlx::query_as!(
            ChatMessage,
            "UPDATE chat_messages SET is_ai_draft = false, updated_at = CURRENT_TIMESTAMP WHERE tenant_id = $1 AND id = $2 RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, is_ai_draft, created_at, updated_at",
            tenant_id,
            message_id
        )
        .fetch_one(pool)
        .await?;
        Ok(message)
    }
}
