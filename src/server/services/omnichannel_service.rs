use crate::domain::repository::omnichannel_repo::{OmniChannelRepo, WorkItem, Conversation, Message, CustomerProfile, AiDraft};
use serde_json::Value;
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

    pub async fn get_conversation(&self, id: Uuid) -> Result<Option<Conversation>, String> {
        self.repo.get_conversation(id).await.map_err(|e| e.to_string())
    }

    pub async fn ingest_signal(&self, tenant_id_str: &str, customer_name: Option<String>, source: String, payload: Value) -> Result<WorkItem, String> {
        let tenant_id = Uuid::parse_str(tenant_id_str).map_err(|e| e.to_string())?;

        let profile = self.repo.create_customer_profile(tenant_id, customer_name.clone())
            .await
            .map_err(|e| e.to_string())?;

        let work_item = self.repo.create_work_item(tenant_id, profile.id, source.clone(), payload.clone())
            .await
            .map_err(|e| e.to_string())?;

        // 1. Also create a conversation for the signal.
        let channel = source.clone();
        let conversation = self.repo.create_conversation(tenant_id, channel, "OPEN".to_string())
            .await
            .map_err(|e| e.to_string())?;

        // 2. Create the first message if payload has 'msg'
        let content = payload.get("msg").and_then(|m| m.as_str()).unwrap_or("").to_string();
        let message = self.repo.create_message(tenant_id, conversation.id, "INBOUND".to_string(), content.clone())
            .await
            .map_err(|e| e.to_string())?;

        // 3. Draft a response
        let prompt = format!(
            "Analyze the following event and provide a concise draft response. Tenant: {}. Source: {}. Payload: {}",
            tenant_id, source, payload
        );

        let prompt = crate::pricing::compression::reduce_tokens(&prompt);

        let llm_res = match std::env::var("OMNISOLO_LLM_PROVIDER").as_deref() {
            Ok("gemini") => crate::minimax::LocalLLMClient::new().reason(&prompt).await,
            Ok("minimax") => {
                let api_key = std::env::var("MINIMAX_API_KEY").map_err(|_| "MINIMAX_API_KEY required".to_string())?;
                crate::minimax::MinimaxClient::new(api_key).reason(&prompt).await
            }
            _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await,
        };

        if let Ok(draft_text) = llm_res {
            let _ = self.repo.create_agent_draft(work_item.id, draft_text.clone()).await;
            // Also store it as an AI draft against the message
            let _ = self.repo.create_ai_draft(tenant_id, message.id, draft_text, "PENDING".to_string()).await;
        }

        Ok(work_item)
    }

    // Web Widget Adapter
    pub async fn receive_web_widget_message(&self, tenant_id: Uuid, session_id: String, content: String) -> Result<Message, String> {
        let payload = serde_json::json!({"session_id": session_id, "msg": content});
        // We can reuse ingest_signal for consistency, or write specific logic.
        // For now, let's create conversation/message directly for the specific adapters if they are more specialized.
        let channel = "web_widget".to_string();

        // See if conversation exists for this session... (Simplified for now: create new)
        let conversation = self.repo.create_conversation(tenant_id, channel, "OPEN".to_string())
            .await
            .map_err(|e| e.to_string())?;

        let message = self.repo.create_message(tenant_id, conversation.id, "INBOUND".to_string(), content.clone())
            .await
            .map_err(|e| e.to_string())?;

        // internal event bus notify (simulated by calling LLM draft here directly for now,
        // normally we would publish to a Redis or in-memory channel)
        self.trigger_auto_draft(tenant_id, message.clone()).await;

        Ok(message)
    }

    // SMS Adapter
    pub async fn receive_sms_message(&self, tenant_id: Uuid, phone_number: String, content: String) -> Result<Message, String> {
        let channel = "sms".to_string();

        let conversation = self.repo.create_conversation(tenant_id, channel, "OPEN".to_string())
            .await
            .map_err(|e| e.to_string())?;

        let message = self.repo.create_message(tenant_id, conversation.id, "INBOUND".to_string(), content.clone())
            .await
            .map_err(|e| e.to_string())?;

        self.trigger_auto_draft(tenant_id, message.clone()).await;

        Ok(message)
    }

    async fn trigger_auto_draft(&self, tenant_id: Uuid, message: Message) {
        let prompt = format!(
            "Analyze the following message and provide a concise draft response. Content: {}",
            message.content
        );

        let llm_res = match std::env::var("OMNISOLO_LLM_PROVIDER").as_deref() {
            Ok("gemini") => crate::minimax::LocalLLMClient::new().reason(&prompt).await,
            Ok("minimax") => {
                let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                if api_key.is_empty() {
                    crate::minimax::LocalLLMClient::new().reason(&prompt).await
                } else {
                    crate::minimax::MinimaxClient::new(api_key).reason(&prompt).await
                }
            }
            _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await,
        };

        if let Ok(draft_text) = llm_res {
            let _ = self.repo.create_ai_draft(tenant_id, message.id, draft_text, "PENDING".to_string()).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_ingest_signal_with_conversation() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

        let maybe_pool = PgPool::connect(&database_url).await;
        if maybe_pool.is_err() {
            return;
        }
        let pool = maybe_pool.unwrap();
        let db = Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });

        let service = OmniChannelService::new(db.clone());
        let tenant_id = Uuid::new_v4().to_string();

        // Ensure tables exist for test
        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS customer_profile (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            CREATE TABLE IF NOT EXISTS work_item (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, customer_id UUID NOT NULL, source TEXT NOT NULL, payload JSONB, status TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            CREATE TABLE IF NOT EXISTS agent_draft (
                id UUID PRIMARY KEY, work_item_id UUID NOT NULL, response TEXT NOT NULL, status TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            CREATE TABLE IF NOT EXISTS conversations (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, channel TEXT NOT NULL, status TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            CREATE TABLE IF NOT EXISTS messages (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, conversation_id UUID NOT NULL, direction TEXT NOT NULL, content TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            CREATE TABLE IF NOT EXISTS ai_drafts (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, message_id UUID NOT NULL, proposed_response TEXT NOT NULL, status TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
        ").execute(&db.pool).await;

        let result = service.ingest_signal(&tenant_id, Some("Test User".to_string()), "instagram".to_string(), serde_json::json!({"msg": "hello"})).await;

        assert!(result.is_ok());
        let item = result.unwrap();
        assert_eq!(item.source, "instagram");
        assert_eq!(item.status, "PENDING");
    }

    #[tokio::test]
    async fn test_receive_web_widget_message() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

        let maybe_pool = PgPool::connect(&database_url).await;
        if maybe_pool.is_err() {
            return;
        }
        let pool = maybe_pool.unwrap();
        let db = Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });

        let service = OmniChannelService::new(db.clone());
        let tenant_id = Uuid::new_v4();

        // Tables are assumed to be created by the previous test or migrations
        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS conversations (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, channel TEXT NOT NULL, status TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            CREATE TABLE IF NOT EXISTS messages (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, conversation_id UUID NOT NULL, direction TEXT NOT NULL, content TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            CREATE TABLE IF NOT EXISTS ai_drafts (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, message_id UUID NOT NULL, proposed_response TEXT NOT NULL, status TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
        ").execute(&db.pool).await;

        let result = service.receive_web_widget_message(tenant_id, "session123".to_string(), "I need help with pricing".to_string()).await;

        assert!(result.is_ok());
        let msg = result.unwrap();
        assert_eq!(msg.content, "I need help with pricing");
        assert_eq!(msg.direction, "INBOUND");
    }

    #[tokio::test]
    async fn test_receive_sms_message() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

        let maybe_pool = PgPool::connect(&database_url).await;
        if maybe_pool.is_err() {
            return;
        }
        let pool = maybe_pool.unwrap();
        let db = Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });

        let service = OmniChannelService::new(db.clone());
        let tenant_id = Uuid::new_v4();

        let result = service.receive_sms_message(tenant_id, "+15551234567".to_string(), "Book a cake for tomorrow".to_string()).await;

        assert!(result.is_ok());
        let msg = result.unwrap();
        assert_eq!(msg.content, "Book a cake for tomorrow");
        assert_eq!(msg.direction, "INBOUND");
    }
}
