use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::chat_engine::{ChatInbox, ChatContact, ChatConversation, ChatMessage};
use tokio::sync::broadcast;
use redis::AsyncCommands;

pub struct ChatEngine {
    pool: PgPool,
    tx: broadcast::Sender<(String, String)>,
    redis_client: Option<redis::Client>,
}

impl ChatEngine {
    pub fn new(pool: PgPool, tx: broadcast::Sender<(String, String)>, redis_client: Option<redis::Client>) -> Self {
        Self { pool, tx, redis_client }
    }

    /// Acquires a distributed lock for the conversation to prevent race conditions during AI drafting.
    pub async fn acquire_ai_draft_lock(&self, tenant_id: String, conversation_id: Uuid) -> Result<bool, String> {
        let lock_key = format!("ohc:lock:{}:conversation:{}", tenant_id, conversation_id);

        if let Some(client) = &self.redis_client {
            if let Ok(mut conn) = client.get_multiplexed_tokio_connection().await {
                // EX 30 ensures the lock expires after 30 seconds if not released
                let acquired: Option<String> = redis::cmd("SET")
                    .arg(&lock_key)
                    .arg("LOCKED")
                    .arg("NX")
                    .arg("EX")
                    .arg(30)
                    .query_async(&mut conn)
                    .await
                    .map_err(|e| e.to_string())?;

                return Ok(acquired.is_some());
            }
        }

        // Fallback or if redis is not configured
        Ok(true)
    }

    /// Releases the distributed lock.
    pub async fn release_ai_draft_lock(&self, tenant_id: String, conversation_id: Uuid) -> Result<(), String> {
        let lock_key = format!("ohc:lock:{}:conversation:{}", tenant_id, conversation_id);

        if let Some(client) = &self.redis_client {
            if let Ok(mut conn) = client.get_multiplexed_tokio_connection().await {
                let _: () = redis::cmd("DEL").arg(&lock_key).query_async(&mut conn).await.unwrap_or(());
            }
        }

        Ok(())
    }

    pub async fn create_inbox(
        &self,
        tenant_id: String,
        name: String,
        channel_type: String,
    ) -> Result<ChatInbox, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await?;

        let result = sqlx::query_as(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name, channel_type)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, name, channel_type, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(&tenant_id)
        .bind(name)
        .bind(channel_type)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(result)
    }

    pub async fn create_contact(
        &self,
        tenant_id: String,
        name: Option<String>,
        email: Option<String>,
        phone_number: Option<String>,
    ) -> Result<ChatContact, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await?;

        let result = sqlx::query_as(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, email, phone_number)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, email, phone_number, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(&tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone_number)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(result)
    }

    pub async fn start_conversation(
        &self,
        tenant_id: String,
        inbox_id: Uuid,
        contact_id: Option<Uuid>,
        assignee_id: Option<String>,
    ) -> Result<ChatConversation, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await?;

        let result = sqlx::query_as(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status)
            VALUES ($1, $2, $3, $4, $5, 'open')
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, last_activity_at, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(&tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(assignee_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(result)
    }

    pub async fn send_message(
        &self,
        tenant_id: String,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<String>,
        content: String,
        message_type: String,
    ) -> Result<ChatMessage, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await?;

        let result: ChatMessage = sqlx::query_as(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, content, message_type, sender_type, sender_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, tenant_id, conversation_id, content, message_type, sender_type, sender_id, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(&tenant_id)
        .bind(conversation_id)
        .bind(content)
        .bind(message_type)
        .bind(sender_type)
        .bind(sender_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        // Emit message.created event via WebSocket
        let payload = serde_json::json!({
            "event": "message.created",
            "data": result
        }).to_string();

        let _ = self.tx.send((tenant_id, payload));

        Ok(result)
    }
}
