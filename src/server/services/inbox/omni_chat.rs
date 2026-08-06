use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChatInbox {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChatContact {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChatConversation {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub status: String,
    pub channel: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChatMessage {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender_type: String,
    pub sender_id: Option<String>,
    pub content: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone)]
pub struct OmniChatService {
    pool: PgPool,
}

impl OmniChatService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, tenant_id: &str, name: &str) -> Result<ChatInbox, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let id = Uuid::new_v4().to_string();
        let inbox = sqlx::query_as::<_, ChatInbox>(
            "INSERT INTO chat_inboxes (id, tenant_id, name) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        info!("Created chat inbox {} for tenant {}", id, tenant_id);
        Ok(inbox)
    }

    pub async fn get_inboxes(&self, tenant_id: &str) -> Result<Vec<ChatInbox>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let inboxes = sqlx::query_as::<_, ChatInbox>(
            "SELECT * FROM chat_inboxes WHERE tenant_id = $1 ORDER BY created_at DESC"
        )
        .bind(tenant_id)
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(inboxes)
    }

    pub async fn create_contact(&self, tenant_id: &str, name: &str, email: Option<&str>, phone: Option<&str>) -> Result<ChatContact, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let id = Uuid::new_v4().to_string();
        let contact = sqlx::query_as::<_, ChatContact>(
            "INSERT INTO chat_contacts (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5) RETURNING *"
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        info!("Created chat contact {} for tenant {}", id, tenant_id);
        Ok(contact)
    }

    pub async fn create_conversation(&self, tenant_id: &str, inbox_id: &str, contact_id: &str, channel: &str) -> Result<ChatConversation, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let id = Uuid::new_v4().to_string();
        let conversation = sqlx::query_as::<_, ChatConversation>(
            "INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, channel) VALUES ($1, $2, $3, $4, $5) RETURNING *"
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(channel)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        info!("Created chat conversation {} for tenant {}", id, tenant_id);
        Ok(conversation)
    }

    pub async fn get_conversations(&self, tenant_id: &str, inbox_id: &str) -> Result<Vec<ChatConversation>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let conversations = sqlx::query_as::<_, ChatConversation>(
            "SELECT * FROM chat_conversations WHERE tenant_id = $1 AND inbox_id = $2 ORDER BY created_at DESC"
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(conversations)
    }

    pub async fn create_message(&self, tenant_id: &str, conversation_id: &str, sender_type: &str, sender_id: Option<&str>, content: &str) -> Result<ChatMessage, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let id = Uuid::new_v4().to_string();
        let message = sqlx::query_as::<_, ChatMessage>(
            "INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(sender_id)
        .bind(content)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        info!("Created chat message {} for tenant {}", id, tenant_id);
        Ok(message)
    }

    pub async fn get_messages(&self, tenant_id: &str, conversation_id: &str) -> Result<Vec<ChatMessage>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let messages = sqlx::query_as::<_, ChatMessage>(
            "SELECT * FROM chat_messages WHERE tenant_id = $1 AND conversation_id = $2 ORDER BY created_at ASC"
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(messages)
    }
}
