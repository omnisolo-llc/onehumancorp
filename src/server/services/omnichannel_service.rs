use crate::domain::repository::omnichannel_repo::{OmniChannelRepo, ChatConversation, ChatMessage};
use std::sync::Arc;
use uuid::Uuid;
use crate::db::DB;

pub struct OmniChannelService {
    repo: OmniChannelRepo,
}

impl OmniChannelService {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            repo: OmniChannelRepo::new(db),
        }
    }

    pub async fn ingest_signal(&self, tenant_id_str: &str, customer_name: Option<String>, source: String, payload: serde_json::Value) -> Result<ChatConversation, String> {
        let tenant_id = Uuid::parse_str(tenant_id_str).map_err(|e| e.to_string())?;

        let inbox = self.repo.create_inbox(tenant_id, "Default Inbox".to_string())
            .await
            .map_err(|e| e.to_string())?;

        let _channel = self.repo.create_channel(tenant_id, inbox.id, source.clone(), serde_json::json!({}))
            .await
            .map_err(|e| e.to_string())?;

        let contact = self.repo.create_contact(tenant_id, customer_name, None, None)
            .await
            .map_err(|e| e.to_string())?;

        let conversation = self.repo.create_conversation(tenant_id, inbox.id, contact.id, None, "open".to_string())
            .await
            .map_err(|e| e.to_string())?;

        let _message = self.repo.create_message(tenant_id, conversation.id, "contact".to_string(), None, payload.to_string())
            .await
            .map_err(|e| e.to_string())?;

        Ok(conversation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_ingest_signal() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

        let maybe_pool = PgPool::connect(&database_url).await;
        if maybe_pool.is_err() {
            return;
        }
        let pool = maybe_pool.unwrap();
        let db = Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });

        let service = OmniChannelService::new(db.clone());
        let tenant_id = Uuid::new_v4().to_string();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS chat_inboxes (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW());
            CREATE TABLE IF NOT EXISTS chat_channels (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, inbox_id UUID NOT NULL, channel_type TEXT NOT NULL, config JSONB, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW());
            CREATE TABLE IF NOT EXISTS chat_contacts (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT, email TEXT, phone TEXT, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW());
            CREATE TABLE IF NOT EXISTS chat_conversations (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, inbox_id UUID NOT NULL, contact_id UUID NOT NULL, assignee_id UUID, status TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW());
            CREATE TABLE IF NOT EXISTS chat_messages (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, conversation_id UUID NOT NULL, sender_type TEXT NOT NULL, sender_id UUID, content TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW());
        ").execute(&db.pool).await;

        let result = service.ingest_signal(&tenant_id, Some("Test User".to_string()), "instagram".to_string(), serde_json::json!({"msg": "hello"})).await;

        assert!(result.is_ok());
        let conv = result.unwrap();
        assert_eq!(conv.status, "open");
    }
}
