use sqlx::PgPool;
use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};

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
    ) -> Result<ChatMessage, sqlx::Error> {
        let msg = sqlx::query_as::<_, ChatMessage>(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type.clone())
        .bind(sender_id)
        .bind(content.clone())
        .fetch_one(&self.pool)
        .await?;

        if sender_type == "contact" {
            let job_id = Uuid::new_v4().to_string();
            let payload = serde_json::json!({
                "message_id": msg.id.to_string(),
                "conversation_id": conversation_id.to_string(),
                "content": content
            });
            sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'draft_omnichannel_reply', $3, 'PENDING')"
            )
            .bind(job_id)
            .bind(tenant_id.to_string())
            .bind(payload)
            .execute(&self.pool).await?;
        }

        // Broadcast to WebSocket
        let ws_data = serde_json::json!({
            "action": "new_message",
            "message": msg
        });
        let envelope = serde_json::json!({
            "channel": "omnichannel",
            "topic": format!("chat:{}", tenant_id),
            "seq": 0,
            "ts": chrono::Utc::now().timestamp_millis(),
            "data": ws_data
        });

        let _ = crate::api::unified_ws::get_broadcast_tx().send(envelope.to_string());

        Ok(msg)
    }

    pub async fn list_inboxes(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<ChatInbox>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, name, created_at, updated_at
            FROM chat_inboxes
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn list_conversations(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
    ) -> Result<Vec<ChatConversation>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            FROM chat_conversations
            WHERE tenant_id = $1 AND inbox_id = $2
            ORDER BY updated_at DESC
            "#
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn list_messages(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<Vec<ChatMessage>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at
            FROM chat_messages
            WHERE tenant_id = $1 AND conversation_id = $2
            ORDER BY created_at ASC
            "#
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .fetch_all(&self.pool)
        .await
    }
}
