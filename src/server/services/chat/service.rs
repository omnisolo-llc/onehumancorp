use sqlx::PgPool;
use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage, ChatWebhookIngress, ChatOutboxMessage};
use tracing::info;

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
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let inbox: ChatInbox = sqlx::query_as(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, name, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(inbox)
    }

    pub async fn create_channel(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        channel_type: String,
        config: serde_json::Value,
    ) -> Result<ChatChannel, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let channel: ChatChannel = sqlx::query_as(
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
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(channel)
    }

    pub async fn create_contact(
        &self,
        tenant_id: Uuid,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
    ) -> Result<ChatContact, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let contact: ChatContact = sqlx::query_as(
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
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(contact)
    }

    pub async fn start_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        assignee_id: Option<Uuid>,
    ) -> Result<ChatConversation, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let conv: ChatConversation = sqlx::query_as(
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
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(conv)
    }

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
    ) -> Result<ChatMessage, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let msg_id = Uuid::new_v4();
        let status = if sender_type == "agent" || sender_type == "bot" { "sent" } else { "unread" };

        let msg: ChatMessage = sqlx::query_as(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, status, created_at, updated_at
            "#
        )
        .bind(msg_id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(&sender_type)
        .bind(sender_id)
        .bind(&content)
        .bind(status)
        .fetch_one(&mut *tx)
        .await?;

        if sender_type == "agent" || sender_type == "bot" {
            let outbox_id = Uuid::new_v4();
            sqlx::query(
                r#"
                INSERT INTO chat_outbox_messages (id, tenant_id, message_id, status)
                VALUES ($1, $2, $3, 'pending')
                "#
            )
            .bind(outbox_id)
            .bind(tenant_id)
            .bind(msg_id)
            .execute(&mut *tx)
            .await?;
        }

        if sender_type == "contact" {
            self.trigger_ai_draft(&mut tx, &tenant_id, &conversation_id, &content).await?;
        }

        tx.commit().await?;
        Ok(msg)
    }

    async fn trigger_ai_draft(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        tenant_id: &Uuid,
        conversation_id: &Uuid,
        _content: &str,
    ) -> Result<(), sqlx::Error> {
        // Here we simulate the Promoter/Operations Agent picking up the message to draft a reply
        // Real logic would involve Redis lock (Redlock) checking if an agent is already drafting.
        // For Native Chat, we inject a bot drafted reply directly.
        let msg_id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content, status)
            VALUES ($1, $2, $3, 'bot', NULL, 'Thank you! We will get back to you shortly.', 'draft')
            "#
        )
        .bind(msg_id)
        .bind(tenant_id)
        .bind(conversation_id)
        .execute(&mut **tx)
        .await?;

        info!("Triggered AI draft for conversation {}", conversation_id);
        Ok(())
    }

    pub async fn ingest_webhook(
        &self,
        tenant_id: Uuid,
        payload: serde_json::Value,
    ) -> Result<ChatWebhookIngress, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let ingress: ChatWebhookIngress = sqlx::query_as(
            r#"
            INSERT INTO chat_webhook_ingress (id, tenant_id, payload, processed)
            VALUES ($1, $2, $3, FALSE)
            RETURNING id, tenant_id, payload, processed, created_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(payload)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(ingress)
    }

    pub async fn get_conversations(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<ChatConversation>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let convs = sqlx::query_as::<_, ChatConversation>(
            r#"
            SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            FROM chat_conversations
            WHERE tenant_id = $1
            ORDER BY updated_at DESC
            "#
        )
        .bind(tenant_id)
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(convs)
    }

    pub async fn get_messages(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<Vec<ChatMessage>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let msgs = sqlx::query_as::<_, ChatMessage>(
            r#"
            SELECT id, tenant_id, conversation_id, sender_type, sender_id, content, status, created_at, updated_at
            FROM chat_messages
            WHERE tenant_id = $1 AND conversation_id = $2
            ORDER BY created_at ASC
            "#
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(msgs)
    }
}
