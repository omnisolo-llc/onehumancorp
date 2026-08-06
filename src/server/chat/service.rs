use sqlx::PgPool;
use uuid::Uuid;
use std::sync::Arc;

use super::db::ChatDb;
use super::models::{ChatMessage, Conversation, Inbox};

#[derive(Clone)]
pub struct ChatService {
    pool: Arc<PgPool>,
}

impl ChatService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn get_inboxes(&self, tenant_id: &str) -> Result<Vec<Inbox>, sqlx::Error> {
        ChatDb::get_inboxes(&self.pool, tenant_id).await
    }

    pub async fn get_conversations(
        &self,
        tenant_id: &str,
        inbox_id: Uuid,
    ) -> Result<Vec<Conversation>, sqlx::Error> {
        ChatDb::get_conversations(&self.pool, tenant_id, inbox_id).await
    }

    pub async fn get_messages(
        &self,
        tenant_id: &str,
        conversation_id: Uuid,
    ) -> Result<Vec<ChatMessage>, sqlx::Error> {
        ChatDb::get_messages(&self.pool, tenant_id, conversation_id).await
    }

    pub async fn send_message(
        &self,
        tenant_id: &str,
        conversation_id: Uuid,
        sender_id: Uuid,
        content: &str,
    ) -> Result<ChatMessage, sqlx::Error> {
        // Broadcast WebSocket message here in the future
        ChatDb::insert_message(
            &self.pool,
            tenant_id,
            conversation_id,
            "agent",
            Some(sender_id),
            content,
            false,
        )
        .await
    }

    pub async fn draft_ai_message(
        &self,
        tenant_id: &str,
        conversation_id: Uuid,
        content: &str,
    ) -> Result<ChatMessage, sqlx::Error> {
        // Enqueue background job or broadcast websocket draft created
        ChatDb::insert_message(
            &self.pool,
            tenant_id,
            conversation_id,
            "agent", // Typically draft appears as from agent/bot
            None,
            content,
            true, // is_ai_draft = true
        )
        .await
    }

    pub async fn approve_draft(
        &self,
        tenant_id: &str,
        message_id: Uuid,
    ) -> Result<ChatMessage, sqlx::Error> {
        let msg = ChatDb::mark_draft_as_sent(&self.pool, tenant_id, message_id).await?;
        // Broadcast WebSocket message that draft was sent
        Ok(msg)
    }

    // Helper for E2E tests to seed a conversation
    pub async fn create_test_conversation(
        &self,
        tenant_id: &str,
    ) -> Result<Conversation, sqlx::Error> {
        let inbox = ChatDb::create_inbox(&self.pool, tenant_id, "Test Inbox").await?;
        let contact = ChatDb::create_contact(&self.pool, tenant_id, Some("Maya Test"), None, None, None).await?;
        ChatDb::create_conversation(&self.pool, tenant_id, inbox.id, contact.id).await
    }

    pub async fn receive_customer_message(
        &self,
        tenant_id: &str,
        conversation_id: Uuid,
        contact_id: Uuid,
        content: &str,
    ) -> Result<ChatMessage, sqlx::Error> {
        ChatDb::insert_message(
            &self.pool,
            tenant_id,
            conversation_id,
            "contact",
            Some(contact_id),
            content,
            false,
        )
        .await
    }
}
