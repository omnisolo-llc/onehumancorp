use sqlx::PgPool;
use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage, ChatEvent};
use redis::AsyncCommands;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ChatService {
    pool: PgPool,
    redis_client: Option<redis::Client>,
    redis_conn: Option<Arc<Mutex<redis::aio::MultiplexedConnection>>>,
}

impl ChatService {
    pub async fn new(pool: PgPool, redis_url: Option<String>) -> Self {
        let (redis_client, redis_conn) = if let Some(url) = redis_url {
            if let Ok(client) = redis::Client::open(url) {
                if let Ok(conn) = client.get_multiplexed_tokio_connection().await {
                    (Some(client), Some(Arc::new(Mutex::new(conn))))
                } else {
                    (Some(client), None)
                }
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        Self { pool, redis_client, redis_conn }
    }

    pub async fn get_redis_conn(&self) -> Option<redis::aio::MultiplexedConnection> {
        if let Some(conn_mutex) = &self.redis_conn {
            Some(conn_mutex.lock().await.clone())
        } else {
            None
        }
    }

    async fn publish_event(&self, event: ChatEvent) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(mut conn) = self.get_redis_conn().await {
            let channel = format!("tenant:{}:inbox:{}", event.tenant_id, event.inbox_id);
            let payload = serde_json::to_string(&event)?;
            let _: () = conn.publish(channel, payload).await?;
        }
        Ok(())
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
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, last_activity_at, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(assignee_id)
        .fetch_one(&self.pool)
        .await?;

        let event = ChatEvent {
            event_type: "conversation.created".to_string(),
            tenant_id,
            inbox_id,
            payload: serde_json::to_value(&conversation).unwrap_or_default(),
        };
        let _ = self.publish_event(event).await;

        Ok(conversation)
    }

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
        message_type: Option<String>,
        is_private: Option<bool>,
    ) -> Result<ChatMessage, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let message: ChatMessage = sqlx::query_as(
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
        .bind(message_type.unwrap_or_else(|| "incoming".to_string()))
        .bind(is_private.unwrap_or(false))
        .fetch_one(&mut *tx)
        .await?;

        let conversation: ChatConversation = sqlx::query_as(
            r#"
            UPDATE chat_conversations
            SET last_activity_at = NOW(), updated_at = NOW()
            WHERE id = $1 AND tenant_id = $2
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, last_activity_at, created_at, updated_at
            "#
        )
        .bind(conversation_id)
        .bind(tenant_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        let event = ChatEvent {
            event_type: "message.created".to_string(),
            tenant_id,
            inbox_id: conversation.inbox_id,
            payload: serde_json::to_value(&message).unwrap_or_default(),
        };
        let _ = self.publish_event(event).await;

        Ok(message)
    }
}
