use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::msgbus::DistributedLock;
use ohc_builtin_agent::llm::gemini::GeminiClient;
use ohc_builtin_agent::llm::LlmClient;
use ohc_builtin_agent_core::types::{ChatRequest, Message, Role};
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAction {
    pub id: String,
    pub tenant_id: String,
    pub agent_id: String,
    pub interaction_id: Option<String>,
    pub action_type: String,
    pub payload: serde_json::Value,
    pub status: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct AgentFeedService {
    db: Arc<DB>,
    lock_service: Arc<dyn DistributedLock>,
    gemini_client: GeminiClient,
}

impl AgentFeedService {
    pub fn new(db: Arc<DB>, lock_service: Arc<dyn DistributedLock>) -> Self {
        let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
        Self {
            db,
            lock_service,
            gemini_client: GeminiClient::new(api_key),
        }
    }

    pub async fn process_event(&self, tenant_id: &str, event_type: &str, event_payload: serde_json::Value) -> Result<(), String> {
        let event_id = event_payload.get("id").and_then(|v| v.as_str()).unwrap_or("unknown");
        let lock_key = format!("ohc:lock:{}:agent_action:{}", tenant_id, event_id);

        if !self.lock_service.acquire_lock(&lock_key, "agent_feed_service", 60).await? {
            return Ok(()); // Already being processed
        }

        let result = self.handle_event_internal(tenant_id, event_type, event_payload).await;

        let _ = self.lock_service.release_lock(&lock_key, "agent_feed_service").await;

        result
    }

    async fn handle_event_internal(&self, tenant_id: &str, event_type: &str, event_payload: serde_json::Value) -> Result<(), String> {
        let system_prompt = "You are an AI business assistant for OneHumanCorp.
Identify if this event requires a proactive action card for the business owner.
If yes, draft the action.
Output format: JSON with 'should_act' (bool), 'action_type' (string), 'title' (string), 'description' (string), 'draft_content' (string).";

        let user_prompt = format!("Event Type: {}\nPayload: {}", event_type, event_payload);

        let req = ChatRequest {
            model: "gemini-pro".to_string(),
            system: system_prompt.to_string(),
            messages: vec![Message {
                role: Role::User,
                content: user_prompt,
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            }],
            tools: vec![],
            max_tokens: 1024,
            temperature: 0.1,
        };

        let response = self.gemini_client.chat(req).await.map_err(|e| e.to_string())?;
        let content = response.message.content;

        let decision: serde_json::Value = serde_json::from_str(&content).map_err(|e| format!("Failed to parse LLM response: {}", e))?;

        if decision.get("should_act").and_then(|v| v.as_bool()).unwrap_or(false) {
            let action_id = Uuid::new_v4().to_string();
            let action_type = decision.get("action_type").and_then(|v| v.as_str()).unwrap_or("generic").to_string();

            let payload = serde_json::json!({
                "title": decision.get("title"),
                "description": decision.get("description"),
                "draft_content": decision.get("draft_content"),
                "original_event": event_payload,
            });

            self.persist_action(tenant_id, &action_id, &action_type, payload).await?;
        }

        Ok(())
    }

    pub async fn persist_action(&self, tenant_id: &str, id: &str, action_type: &str, payload: serde_json::Value) -> Result<(), String> {
        let now = Utc::now();
        let payload_json = serde_json::to_string(&payload).unwrap_or_default();

        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                crate::common::auth_utils::set_org_context(&mut *tx, tenant_id).await?;
                sqlx::query("INSERT INTO agent_actions (id, tenant_id, agent_id, action_type, payload, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
                    .bind(id)
                    .bind(tenant_id)
                    .bind("agent-feed-system")
                    .bind(action_type)
                    .bind(serde_json::from_str::<serde_json::Value>(&payload_json).unwrap())
                    .bind("PENDING")
                    .bind(now)
                    .bind(now)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query("INSERT INTO agent_actions (id, tenant_id, agent_id, action_type, payload, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
                    .bind(id)
                    .bind(tenant_id)
                    .bind("agent-feed-system")
                    .bind(action_type)
                    .bind(&payload_json)
                    .bind("PENDING")
                    .bind(now)
                    .bind(now)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn get_pending_actions(&self, tenant_id: &str) -> Result<Vec<AgentAction>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                crate::common::auth_utils::set_org_context(&mut *tx, tenant_id).await?;
                let rows = sqlx::query_as!(
                    AgentAction,
                    r#"SELECT id, tenant_id, agent_id, interaction_id, action_type, payload, status, created_at, updated_at FROM agent_actions WHERE tenant_id = $1 AND status = 'PENDING' ORDER BY created_at DESC"#,
                    tenant_id
                )
                .fetch_all(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())?;
                Ok(rows)
            }
            DbStore::Sqlite(pool) => {
                let rows = sqlx::query("SELECT id, tenant_id, agent_id, interaction_id, action_type, payload, status, created_at, updated_at FROM agent_actions WHERE tenant_id = ? AND status = 'PENDING' ORDER BY created_at DESC")
                    .bind(tenant_id)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;

                let mut actions = Vec::new();
                for row in rows {
                    use sqlx::Row;
                    let payload_str: String = row.get("payload");
                    actions.push(AgentAction {
                        id: row.get("id"),
                        tenant_id: row.get("tenant_id"),
                        agent_id: row.get("agent_id"),
                        interaction_id: row.get("interaction_id"),
                        action_type: row.get("action_type"),
                        payload: serde_json::from_str(&payload_str).unwrap_or(serde_json::json!({})),
                        status: row.get("status"),
                        created_at: row.try_get("created_at").ok(),
                        updated_at: row.try_get("updated_at").ok(),
                    });
                }
                Ok(actions)
            }
        }
    }

    pub async fn update_action_status(&self, tenant_id: &str, action_id: &str, status: &str) -> Result<(), String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                crate::common::auth_utils::set_org_context(&mut *tx, tenant_id).await?;
                sqlx::query("UPDATE agent_actions SET status = $1, updated_at = $2 WHERE id = $3 AND tenant_id = $4")
                    .bind(status)
                    .bind(now)
                    .bind(action_id)
                    .bind(tenant_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query("UPDATE agent_actions SET status = ?, updated_at = ? WHERE id = ? AND tenant_id = ?")
                    .bind(status)
                    .bind(now)
                    .bind(action_id)
                    .bind(tenant_id)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msgbus::MemoryBus;
    use crate::db::{DB, DbStore};
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_test_db() -> Arc<DB> {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap(),
            store: DbStore::Sqlite(pool),
        });
        db.run_migrations().await.unwrap();
        db
    }

    #[tokio::test]
    async fn test_persist_and_get_actions() {
        let db = setup_test_db().await;
        let bus = Arc::new(MemoryBus::new());
        let service = AgentFeedService::new(db.clone(), bus.clone());
        let tenant_id = "test-tenant";

        let payload = serde_json::json!({
            "title": "Test Action",
            "description": "This is a test",
            "draft_content": "Drafted content"
        });

        service.persist_action(tenant_id, "action-1", "test-type", payload.clone()).await.unwrap();

        let actions = service.get_pending_actions(tenant_id).await.unwrap();
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].id, "action-1");
        assert_eq!(actions[0].action_type, "test-type");
        assert_eq!(actions[0].status, "PENDING");
        assert_eq!(actions[0].payload["title"], "Test Action");
    }

    #[tokio::test]
    async fn test_update_action_status() {
        let db = setup_test_db().await;
        let bus = Arc::new(MemoryBus::new());
        let service = AgentFeedService::new(db.clone(), bus.clone());
        let tenant_id = "test-tenant";

        service.persist_action(tenant_id, "action-1", "test-type", serde_json::json!({})).await.unwrap();
        service.update_action_status(tenant_id, "action-1", "APPROVED").await.unwrap();

        let actions = service.get_pending_actions(tenant_id).await.unwrap();
        assert_eq!(actions.len(), 0);
    }
}
