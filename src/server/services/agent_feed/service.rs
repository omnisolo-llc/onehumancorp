use crate::domain::repository::agent_feed_repo::{AgentFeedRepository, AgentFeedItem};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use serde_json::Value;

pub struct AgentFeedService {
    #[allow(dead_code)]
    pool: PgPool,
    repo: AgentFeedRepository,
}

impl AgentFeedService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: AgentFeedRepository::new(std::sync::Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres })),
            pool,
        }
    }

    pub async fn process_event(&self, tenant_id: &str, event_source: &str, payload: &Value) -> Result<AgentFeedItem, String> {
        // Build context via LLM/Minimax
        let prompt = format!(
            "Analyze the following event and provide a concise JSON object with a 'draft_action' containing a suggested response or action, and 'intent' summarizing the reason. Tenant: {}. Source: {}. Payload: {}",
            tenant_id, event_source, payload
        );
        let prompt = crate::pricing::compression::reduce_tokens(&prompt);

        let llm_res = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
            Ok("gemini") => {
                crate::minimax::LocalLLMClient::new().reason(&prompt).await
            }
            Ok("minimax") => {
                let api_key = std::env::var("MINIMAX_API_KEY")
                    .map_err(|_| "MINIMAX_API_KEY is required for minimax".to_string())?;
                crate::minimax::MinimaxClient::new(api_key).reason(&prompt).await
            }
            _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await,
        }?;

        let proposed_action = if let Ok(parsed) = serde_json::from_str::<Value>(&llm_res) {
             parsed
        } else {
             serde_json::json!({"draft_action": llm_res, "intent": "unknown"})
        };

        let item = AgentFeedItem {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            event_source: event_source.to_string(),
            context_payload: Some(sqlx::types::Json(payload.clone())),
            proposed_action: Some(sqlx::types::Json(proposed_action)),
            lifecycle_state: "PENDING_APPROVAL".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };

        self.repo.create(item.clone()).await.map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    #[ignore]
    async fn test_process_event() {
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = PgPool::connect(&database_url).await.unwrap();
        let service = AgentFeedService::new(pool);

        let payload = serde_json::json!({
            "message": "Do you have vegan cakes?",
            "customer": "Maya"
        });

        let result = service.process_event("test-tenant", "instagram_dm", &payload).await;
        assert!(result.is_ok());
        let item = result.unwrap();
        assert_eq!(item.tenant_id, "test-tenant");
        assert_eq!(item.event_source, "instagram_dm");
        assert_eq!(item.lifecycle_state, "PENDING_APPROVAL");
    }
}
