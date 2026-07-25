use crate::domain::repository::omnichannel_repo::{OmniChannelRepo, WorkItem, OmnichannelContact, OmnichannelInbox, OmnichannelConversation, OmnichannelMessage};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;
use crate::db::DB;

pub trait ChannelAdapter {
    fn normalize_payload(&self, payload: &Value) -> Result<(String, String, String), String>; // (sender_name, phone_or_email, content)
}

pub struct TwilioAdapter;
impl ChannelAdapter for TwilioAdapter {
    fn normalize_payload(&self, payload: &Value) -> Result<(String, String, String), String> {
        let from = payload.get("From").and_then(|v| v.as_str()).unwrap_or("Unknown");
        let body = payload.get("Body").and_then(|v| v.as_str()).unwrap_or("");
        Ok(("Twilio User".to_string(), from.to_string(), body.to_string()))
    }
}

pub struct MetaWebhookAdapter;
impl ChannelAdapter for MetaWebhookAdapter {
    fn normalize_payload(&self, payload: &Value) -> Result<(String, String, String), String> {
        let from = payload.get("entry").and_then(|v| v.get(0)).and_then(|v| v.get("messaging")).and_then(|v| v.get(0)).and_then(|v| v.get("sender")).and_then(|v| v.get("id")).and_then(|v| v.as_str()).unwrap_or("Unknown");
        let body = payload.get("entry").and_then(|v| v.get(0)).and_then(|v| v.get("messaging")).and_then(|v| v.get(0)).and_then(|v| v.get("message")).and_then(|v| v.get("text")).and_then(|v| v.as_str()).unwrap_or("");
        Ok(("Meta User".to_string(), from.to_string(), body.to_string()))
    }
}

pub struct OmniChannelService {
    pub repo: OmniChannelRepo,
}

impl OmniChannelService {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            repo: OmniChannelRepo::new(db),
        }
    }

    pub async fn ingest_signal(&self, tenant_id_str: &str, customer_name: Option<String>, source: String, payload: Value) -> Result<WorkItem, String> {
        let tenant_id = Uuid::parse_str(tenant_id_str).map_err(|e| e.to_string())?;

        let profile = self.repo.create_customer_profile(tenant_id, customer_name)
            .await
            .map_err(|e| e.to_string())?;

        let work_item = self.repo.create_work_item(tenant_id, profile.id, source.clone(), payload.clone())
            .await
            .map_err(|e| e.to_string())?;

        let prompt = format!(
            "Analyze the following event and provide a concise draft response. Tenant: {}. Source: {}. Payload: {}",
            tenant_id, source, payload
        );

        let prompt = crate::pricing::compression::reduce_tokens(&prompt);

        let llm_res = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
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
            let _ = self.repo.create_agent_draft(work_item.id, draft_text).await;
        }

        Ok(work_item)
    }

    pub async fn ingest_webhook(&self, tenant_id_str: &str, source: &str, payload: &Value) -> Result<OmnichannelConversation, String> {
        let tenant_id = Uuid::parse_str(tenant_id_str).map_err(|e| e.to_string())?;

        let (name, contact_info, content) = match source {
            "twilio" => TwilioAdapter.normalize_payload(payload)?,
            "meta" => MetaWebhookAdapter.normalize_payload(payload)?,
            _ => ("Unknown".to_string(), "Unknown".to_string(), payload.to_string()),
        };

        // For simplicity, always create new contact and inbox in this flow, or you would look them up.
        let contact = self.repo.create_contact(tenant_id, Some(name), None, Some(contact_info)).await.map_err(|e| e.to_string())?;

        let inbox_name = format!("{} Inbox", source);
        let inbox = self.repo.create_inbox(tenant_id, inbox_name, source.to_string()).await.map_err(|e| e.to_string())?;

        let conversation = self.repo.create_conversation(tenant_id, inbox.id, Some(contact.id), "pending_ai".to_string()).await.map_err(|e| e.to_string())?;

        let _message = self.repo.create_message(tenant_id, conversation.id, "customer".to_string(), content.clone(), false, "delivered".to_string()).await.map_err(|e| e.to_string())?;

        // Simulate AI Draft
        let prompt = format!("Draft a reply to: {}", content);
        let llm_res = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
            Ok("gemini") => crate::minimax::LocalLLMClient::new().reason(&prompt).await,
            _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await,
        };

        if let Ok(draft_text) = llm_res {
            let _ = self.repo.create_message(tenant_id, conversation.id, "agent".to_string(), draft_text, true, "draft".to_string()).await.map_err(|e| e.to_string())?;
            let _ = self.repo.update_conversation_status(conversation.id, "needs_owner".to_string()).await;
        }

        Ok(conversation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_twilio_adapter() {
        let payload = serde_json::json!({
            "From": "+1234567890",
            "Body": "Hello there"
        });
        let adapter = TwilioAdapter;
        let (name, phone, body) = adapter.normalize_payload(&payload).unwrap();
        assert_eq!(name, "Twilio User");
        assert_eq!(phone, "+1234567890");
        assert_eq!(body, "Hello there");
    }

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
            CREATE TABLE IF NOT EXISTS customer_profile (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            CREATE TABLE IF NOT EXISTS work_item (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, customer_id UUID NOT NULL, source TEXT NOT NULL, payload JSONB, status TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            CREATE TABLE IF NOT EXISTS agent_draft (
                id UUID PRIMARY KEY, work_item_id UUID NOT NULL, response TEXT NOT NULL, status TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
        ").execute(&db.pool).await;

        let result = service.ingest_signal(&tenant_id, Some("Test User".to_string()), "instagram".to_string(), serde_json::json!({"msg": "hello"})).await;

        assert!(result.is_ok());
        let item = result.unwrap();
        assert_eq!(item.source, "instagram");
        assert_eq!(item.status, "PENDING");
    }

    #[tokio::test]
    async fn test_ingest_webhook() {
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
            CREATE TABLE IF NOT EXISTS omnichannel_contact (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT, email TEXT, phone TEXT, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW());
            CREATE TABLE IF NOT EXISTS omnichannel_inbox (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT NOT NULL, channel_type TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW());
            CREATE TABLE IF NOT EXISTS omnichannel_conversation (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, inbox_id UUID NOT NULL, contact_id UUID, status TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW());
            CREATE TABLE IF NOT EXISTS omnichannel_message (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, conversation_id UUID NOT NULL, sender_type TEXT NOT NULL, content TEXT NOT NULL, is_private BOOLEAN DEFAULT FALSE, status TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW());
        ").execute(&db.pool).await;

        let payload = serde_json::json!({
            "From": "+1234567890",
            "Body": "Hello there"
        });

        let result = service.ingest_webhook(&tenant_id, "twilio", &payload).await;
        assert!(result.is_ok());
    }
}
