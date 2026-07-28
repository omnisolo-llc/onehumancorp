use redis::AsyncCommands;
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
        let conversation: ChatConversation = sqlx::query_as(
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
        .await?;

        if let Some(client) = &self.redis_client {
            if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                let channel = format!("tenant:{}:inbox:{}", tenant_id, inbox_id);
                let payload = serde_json::json!({
                    "event": "conversation.updated",
                    "data": conversation
                });
                let _: Result<(), redis::RedisError> = conn.publish(channel, payload.to_string()).await;
            }
        }

        Ok(conversation)
    }

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
    ) -> Result<ChatMessage, sqlx::Error> {
        let message: ChatMessage = sqlx::query_as(
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
        .await?;

        if let Some(client) = &self.redis_client {
            if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                // Fetch the inbox_id for the conversation to broadcast to the correct channel
                #[derive(sqlx::FromRow)]
                struct InboxIdResult {
                    inbox_id: Uuid,
                }

                if let Ok(inbox_id_record) = sqlx::query_as::<_, InboxIdResult>(
                    r#"SELECT inbox_id FROM chat_conversations WHERE id = $1 AND tenant_id = $2"#
                )
                .bind(conversation_id)
                .bind(tenant_id)
                .fetch_one(&self.pool)
                .await
                {
                    let channel = format!("tenant:{}:inbox:{}", tenant_id, inbox_id_record.inbox_id);
                    let payload = serde_json::json!({
                        "event": "message.created",
                        "data": message
                    });
                    let _: Result<(), redis::RedisError> = conn.publish(channel, payload.to_string()).await;
                }
            }
        }

        Ok(message)
    }
}
