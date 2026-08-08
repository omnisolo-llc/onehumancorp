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

    pub async fn create_inbox(
        &self,
        tenant_id: &str,
        name: &str,
        channel_type: &str,
        channel_config: serde_json::Value,
    ) -> Result<ChatInbox, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let row = sqlx::query_as(
            r#"
            INSERT INTO inboxes (id, tenant_id, name, channel_type, channel_config)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, channel_type, channel_config, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(channel_type)
        .bind(channel_config)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(row)
    }

    pub async fn create_contact(
        &self,
        tenant_id: &str,
        identifier: Option<&str>,
        name: &str,
        email: Option<&str>,
        phone_number: Option<&str>,
    ) -> Result<ChatContact, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let row = sqlx::query_as(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, identifier, name, email, phone_number)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, identifier, name, email, phone_number, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(identifier)
        .bind(name)
        .bind(email)
        .bind(phone_number)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(row)
    }

    pub async fn start_conversation(
        &self,
        tenant_id: &str,
        inbox_id: Uuid,
        contact_id: Uuid,
    ) -> Result<ChatConversation, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let row = sqlx::query_as(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status)
            VALUES ($1, $2, $3, $4, 'open')
            RETURNING id, tenant_id, contact_id, inbox_id, status, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(row)
    }

    pub async fn send_message(
        &self,
        tenant_id: &str,
        conversation_id: Uuid,
        sender_type: &str,
        sender_id: Option<&str>,
        content: &str,
        message_type: &str,
    ) -> Result<ChatMessage, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let row = sqlx::query_as(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content, message_type)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, message_type, created_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(sender_id)
        .bind(content)
        .bind(message_type)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(row)
    }

    pub async fn get_conversations(
        &self,
        tenant_id: &str,
        inbox_id: Uuid,
    ) -> Result<Vec<ChatConversation>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let rows = sqlx::query_as(
            r#"
            SELECT id, tenant_id, contact_id, inbox_id, status, created_at, updated_at
            FROM chat_conversations
            WHERE tenant_id = $1 AND inbox_id = $2
            ORDER BY updated_at DESC
            "#
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(rows)
    }
}
