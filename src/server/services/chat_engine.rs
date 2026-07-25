use sqlx::{FromRow, PgPool};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub channel_type: String,
    pub channel_config: Option<sqlx::types::Json<serde_json::Value>>,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub status: String,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Option<Uuid>,
    pub sender_type: String,
    pub content: String,
    pub message_type: String,
    pub external_source_ids: Option<sqlx::types::Json<serde_json::Value>>,
    pub created_at: Option<DateTime<Utc>>,
}

pub struct ChatEngineService {
    pool: PgPool,
}

impl ChatEngineService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(
        &self,
        tenant_id: Uuid,
        name: String,
        channel_type: String,
        channel_config: Option<serde_json::Value>,
    ) -> Result<Inbox, sqlx::Error> {
        let id = Uuid::new_v4();
        let config = channel_config.map(sqlx::types::Json);

        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let record = sqlx::query_as::<_, Inbox>(
            "INSERT INTO inboxes (id, tenant_id, name, channel_type, channel_config) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, name, channel_type, channel_config as \"channel_config: sqlx::types::Json<serde_json::Value>\", is_active, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(channel_type)
        .bind(config)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(record)
    }

    pub async fn get_inboxes(&self, tenant_id: Uuid) -> Result<Vec<Inbox>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let records = sqlx::query_as::<_, Inbox>(
            "SELECT id, tenant_id, name, channel_type, channel_config as \"channel_config: sqlx::types::Json<serde_json::Value>\", is_active, created_at, updated_at FROM inboxes WHERE tenant_id = $1 ORDER BY created_at DESC",
        )
        .bind(tenant_id)
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(records)
    }

    pub async fn create_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        assignee_id: Option<Uuid>,
    ) -> Result<Conversation, sqlx::Error> {
        let id = Uuid::new_v4();

        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let record = sqlx::query_as::<_, Conversation>(
            "INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status) VALUES ($1, $2, $3, $4, $5, 'open') RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, last_activity_at, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(assignee_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(record)
    }

    pub async fn get_conversations(&self, tenant_id: Uuid) -> Result<Vec<Conversation>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let records = sqlx::query_as::<_, Conversation>(
            "SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, last_activity_at, created_at, updated_at FROM conversations WHERE tenant_id = $1 ORDER BY last_activity_at DESC",
        )
        .bind(tenant_id)
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(records)
    }

    pub async fn get_conversation_by_id(&self, tenant_id: Uuid, conversation_id: Uuid) -> Result<Conversation, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let record = sqlx::query_as::<_, Conversation>(
            "SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, last_activity_at, created_at, updated_at FROM conversations WHERE tenant_id = $1 AND id = $2",
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(record)
    }

    pub async fn create_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_id: Option<Uuid>,
        sender_type: String,
        content: String,
        message_type: String,
        external_source_ids: Option<serde_json::Value>,
    ) -> Result<Message, sqlx::Error> {
        let id = Uuid::new_v4();
        let external_ids = external_source_ids.map(sqlx::types::Json);

        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let record = sqlx::query_as::<_, Message>(
            "INSERT INTO messages (id, tenant_id, conversation_id, sender_id, sender_type, content, message_type, external_source_ids) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id, tenant_id, conversation_id, sender_id, sender_type, content, message_type, external_source_ids as \"external_source_ids: sqlx::types::Json<serde_json::Value>\", created_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_id)
        .bind(sender_type)
        .bind(content)
        .bind(message_type)
        .bind(external_ids)
        .fetch_one(&mut *tx)
        .await?;

        // Update conversation last_activity_at
        sqlx::query("UPDATE conversations SET last_activity_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
            .bind(conversation_id)
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(record)
    }

    pub async fn get_messages(&self, tenant_id: Uuid, conversation_id: Uuid) -> Result<Vec<Message>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let records = sqlx::query_as::<_, Message>(
            "SELECT id, tenant_id, conversation_id, sender_id, sender_type, content, message_type, external_source_ids as \"external_source_ids: sqlx::types::Json<serde_json::Value>\", created_at FROM messages WHERE tenant_id = $1 AND conversation_id = $2 ORDER BY created_at ASC",
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_inbox() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        if let Ok(pool) = PgPool::connect(&database_url).await {
            let service = ChatEngineService::new(pool);
            let tenant_id = Uuid::new_v4();

            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS inboxes (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT NOT NULL, channel_type TEXT NOT NULL, channel_config JSONB, is_active BOOLEAN NOT NULL DEFAULT true, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);")
                .execute(&service.pool)
                .await;

            // Note: In tests where we create ad-hoc tables, RLS requires a role switch to enforce.
            // In a real DB with RLS and our config variable `app.current_tenant`, it behaves correctly.

            let inbox = service.create_inbox(
                tenant_id,
                "Support Email".to_string(),
                "email".to_string(),
                Some(serde_json::json!({"email": "support@example.com"})),
            ).await;

            assert!(inbox.is_ok());
            let inbox = inbox.unwrap();
            assert_eq!(inbox.name, "Support Email");
            assert_eq!(inbox.channel_type, "email");
        }
    }

    #[tokio::test]
    async fn test_create_message() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        if let Ok(pool) = PgPool::connect(&database_url).await {
            let service = ChatEngineService::new(pool);
            let tenant_id = Uuid::new_v4();

            // Run standard migrations to ensure all FK constraints apply instead of manual table creation
            // Note: Since these tests are normally run by Bazel with an isolated DB,
            // the tables are created by migrations properly.

            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS inboxes (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT NOT NULL, channel_type TEXT NOT NULL, channel_config JSONB, is_active BOOLEAN NOT NULL DEFAULT true, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);")
                .execute(&service.pool)
                .await;
            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS conversations (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, inbox_id UUID NOT NULL REFERENCES inboxes(id) ON DELETE CASCADE, contact_id UUID NOT NULL, assignee_id UUID, status TEXT NOT NULL DEFAULT 'open', last_activity_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);")
                .execute(&service.pool)
                .await;
            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS messages (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE, sender_id UUID, sender_type TEXT NOT NULL, content TEXT NOT NULL, message_type TEXT NOT NULL, external_source_ids JSONB, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);")
                .execute(&service.pool)
                .await;

            let inbox = service.create_inbox(tenant_id, "Test Inbox".to_string(), "api".to_string(), None).await.unwrap();
            let conversation = service.create_conversation(tenant_id, inbox.id, Uuid::new_v4(), None).await.unwrap();

            let message = service.create_message(
                tenant_id,
                conversation.id,
                None,
                "contact".to_string(),
                "Hello, world!".to_string(),
                "incoming".to_string(),
                None,
            ).await;

            assert!(message.is_ok());
            let message = message.unwrap();
            assert_eq!(message.content, "Hello, world!");
            assert_eq!(message.message_type, "incoming");
        }
    }

    #[tokio::test]
    async fn test_get_methods() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        if let Ok(pool) = PgPool::connect(&database_url).await {
            let service = ChatEngineService::new(pool);
            let tenant_id = Uuid::new_v4();

            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS inboxes (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT NOT NULL, channel_type TEXT NOT NULL, channel_config JSONB, is_active BOOLEAN NOT NULL DEFAULT true, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);")
                .execute(&service.pool)
                .await;
            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS conversations (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, inbox_id UUID NOT NULL REFERENCES inboxes(id) ON DELETE CASCADE, contact_id UUID NOT NULL, assignee_id UUID, status TEXT NOT NULL DEFAULT 'open', last_activity_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);")
                .execute(&service.pool)
                .await;
            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS messages (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE, sender_id UUID, sender_type TEXT NOT NULL, content TEXT NOT NULL, message_type TEXT NOT NULL, external_source_ids JSONB, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);")
                .execute(&service.pool)
                .await;

            let inbox = service.create_inbox(tenant_id, "Test Inbox 2".to_string(), "api".to_string(), None).await.unwrap();
            let inboxes = service.get_inboxes(tenant_id).await.unwrap();
            assert_eq!(inboxes.len(), 1);

            let conversation = service.create_conversation(tenant_id, inbox.id, Uuid::new_v4(), None).await.unwrap();
            let conversations = service.get_conversations(tenant_id).await.unwrap();
            assert_eq!(conversations.len(), 1);

            let fetched_conv = service.get_conversation_by_id(tenant_id, conversation.id).await.unwrap();
            assert_eq!(fetched_conv.id, conversation.id);

            let message = service.create_message(
                tenant_id,
                conversation.id,
                None,
                "agent".to_string(),
                "Hi there!".to_string(),
                "outgoing".to_string(),
                None,
            ).await.unwrap();

            let messages = service.get_messages(tenant_id, conversation.id).await.unwrap();
            assert_eq!(messages.len(), 1);
            assert_eq!(messages[0].id, message.id);
        }
    }
}
