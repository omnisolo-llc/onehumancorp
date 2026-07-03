use crate::domain::repository::omnichannel_repo::{OmniChannelRepo, WorkItem};
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
                let api_key = std::env::var("MINIMAX_API_KEY").map_err(|_| "MINIMAX_API_KEY required".to_string())?;
                crate::minimax::MinimaxClient::new(api_key).reason(&prompt).await
            }
            _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await,
        };

        if let Ok(draft_text) = llm_res {
            let _ = self.repo.create_agent_draft(work_item.id, draft_text).await;
        }

        Ok(work_item)
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
        ").execute(&db.pool).await;

        let result = service.ingest_signal(&tenant_id, Some("Test User".to_string()), "instagram".to_string(), serde_json::json!({"msg": "hello"})).await;

        assert!(result.is_ok());
        let item = result.unwrap();
        assert_eq!(item.source, "instagram");
        assert_eq!(item.status, "PENDING");
    }
}
