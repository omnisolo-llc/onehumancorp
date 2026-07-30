use crate::models::{ChatConversation, ChatInbox, ChatMessage, ChatContact};
use sqlx::PgPool;
use uuid::Uuid;

pub struct ChatService {
    pool: PgPool,
}

impl ChatService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, name: &str) -> Result<ChatInbox, sqlx::Error> {
        let id = Uuid::new_v4();
        let query = "INSERT INTO chat_inboxes (id, tenant_id, name) VALUES ($1, $2, $3) RETURNING id, tenant_id, name, created_at, updated_at";
        let mut tx = self.pool.begin().await?;
        sqlx::query(&format!("SET LOCAL app.current_tenant_id = '{}'", tenant_id)).execute(&mut *tx).await?;

        let inbox = sqlx::query_as::<_, ChatInbox>(query)
            .bind(id)
            .bind(tenant_id)
            .bind(name)
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(inbox)
    }

    pub async fn create_contact(&self, tenant_id: Uuid, name: Option<&str>, email: Option<&str>, phone: Option<&str>) -> Result<ChatContact, sqlx::Error> {
        let id = Uuid::new_v4();
        let query = "INSERT INTO chat_contacts (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, name, email, phone, created_at, updated_at";
        let mut tx = self.pool.begin().await?;
        sqlx::query(&format!("SET LOCAL app.current_tenant_id = '{}'", tenant_id)).execute(&mut *tx).await?;

        let contact = sqlx::query_as::<_, ChatContact>(query)
            .bind(id)
            .bind(tenant_id)
            .bind(name)
            .bind(email)
            .bind(phone)
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(contact)
    }

    pub async fn create_conversation(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid) -> Result<ChatConversation, sqlx::Error> {
        let id = Uuid::new_v4();
        let query = "INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, 'open') RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at";
        let mut tx = self.pool.begin().await?;
        sqlx::query(&format!("SET LOCAL app.current_tenant_id = '{}'", tenant_id)).execute(&mut *tx).await?;

        let conv = sqlx::query_as::<_, ChatConversation>(query)
            .bind(id)
            .bind(tenant_id)
            .bind(inbox_id)
            .bind(contact_id)
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(conv)
    }

    pub async fn add_message(&self, tenant_id: Uuid, conversation_id: Uuid, sender_type: &str, sender_id: Option<Uuid>, content: &str) -> Result<ChatMessage, sqlx::Error> {
        let id = Uuid::new_v4();
        let query = "INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at";
        let mut tx = self.pool.begin().await?;
        sqlx::query(&format!("SET LOCAL app.current_tenant_id = '{}'", tenant_id)).execute(&mut *tx).await?;

        let msg = sqlx::query_as::<_, ChatMessage>(query)
            .bind(id)
            .bind(tenant_id)
            .bind(conversation_id)
            .bind(sender_type)
            .bind(sender_id)
            .bind(content)
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;

        // AI Orchestration Hook simulation:
        // In a real scenario, this is where we would enqueue a job to the AI Job Queue
        // e.g. sqlx::query("INSERT INTO ai_jobs (tenant_id, task_type, payload) VALUES (...)")
        tracing::info!("AI Orchestration Hook: Enqueuing Work Triage agent for message: {}", id);

        Ok(msg)
    }
}
