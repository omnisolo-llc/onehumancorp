use sqlx::PgPool;
use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};

pub struct ChatService {
    pool: PgPool,
    redis_client: Option<redis::Client>,
}

impl ChatService {
    pub fn new(pool: PgPool, redis_client: Option<redis::Client>) -> Self {
        Self { pool, redis_client }
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
            "#
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
        message_type: String,
        is_private: bool,
    ) -> Result<ChatMessage, sqlx::Error> {
        let msg: ChatMessage = sqlx::query_as(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content, message_type, is_private)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, message_type, is_private, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(sender_id)
        .bind(content)
        .bind(message_type)
        .bind(is_private)
        .fetch_one(&self.pool)
        .await?;

        // Update conversation last_activity_at or updated_at
        let _ = sqlx::query(
            r#"
            UPDATE chat_conversations
            SET updated_at = NOW()
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(msg.conversation_id)
        .bind(msg.tenant_id)
        .execute(&self.pool)
        .await;

        // Basic Redis pub/sub mechanism
        tracing::info!("Broadcasting message.created for tenant {}, conversation {}", tenant_id, conversation_id);
        if let Some(client) = &self.redis_client {
            if let Ok(mut con) = client.get_multiplexed_async_connection().await {
                let channel = format!("tenant:{}:inbox:{}", tenant_id, msg.conversation_id);
                let payload = serde_json::json!({
                    "event": "message.created",
                    "data": msg
                });
                let _: Result<(), redis::RedisError> = redis::cmd("PUBLISH")
                    .arg(channel)
                    .arg(payload.to_string())
                    .query_async(&mut con)
                    .await;
            }
        }

        Ok(msg)
    }

    pub async fn update_conversation_status(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        status: String,
    ) -> Result<ChatConversation, sqlx::Error> {
        let conv: ChatConversation = sqlx::query_as(
            r#"
            UPDATE chat_conversations
            SET status = $1, updated_at = NOW()
            WHERE id = $2 AND tenant_id = $3
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            "#
        )
        .bind(status)
        .bind(conversation_id)
        .bind(tenant_id)
        .fetch_one(&self.pool)
        .await?;

        tracing::info!("Broadcasting conversation.updated for tenant {}, conversation {}", tenant_id, conversation_id);
        if let Some(client) = &self.redis_client {
            if let Ok(mut con) = client.get_multiplexed_async_connection().await {
                let channel = format!("tenant:{}:inbox:{}", tenant_id, conv.inbox_id);
                let payload = serde_json::json!({
                    "event": "conversation.updated",
                    "data": conv
                });
                let _: Result<(), redis::RedisError> = redis::cmd("PUBLISH")
                    .arg(channel)
                    .arg(payload.to_string())
                    .query_async(&mut con)
                    .await;
            }
        }

        Ok(conv)
    }
}
