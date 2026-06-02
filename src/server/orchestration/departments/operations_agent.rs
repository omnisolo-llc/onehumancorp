use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;

pub struct OperationsAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl OperationsAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait::async_trait]
impl Department for OperationsAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Operations
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.quote.accepted".to_string(),
            "tenant.order.created".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        let config = self.get_config(&event.tenant_id);
        let risk = if let Some(cfg) = config {
            if cfg.auto_approve_limits > 0.0 {
                ActionRisk::AutoExecute
            } else {
                ActionRisk::DraftForReview
            }
        } else {
            ActionRisk::DraftForReview
        };

        let action_description = if event.event_type == "tenant.order.created" {
            "Process Order & Update Inventory".to_string()
        } else {
            "Create order and booking".to_string()
        };

        self.orchestrator.execute_action(
            DepartmentType::Operations,
            action_description,
            event.tenant_id.clone(),
            risk,
            event.payload.clone(),
        ).await?;

        // Dispatch event for customer success agent
        let cs_event = DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: event.tenant_id.clone(),
            event_type: "tenant.order.fulfillment_ready".to_string(),
            payload: event.payload.clone(),
        };
        self.orchestrator.dispatch_event(cs_event).await
    }

    fn get_config(&self, _tenant_id: &str) -> Option<DepartmentConfig> {
        Some(DepartmentConfig { tone_of_voice: "professional".to_string(), auto_approve_limits: 10.0 })
    }

    fn set_config(&mut self, _tenant_id: String, _config: DepartmentConfig) {
    }

    async fn query_memory(&self, _query: &str) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String> {
        self.orchestrator.execute_action(self.department_type(), description.clone(), tenant_id.clone(), risk, serde_json::json!({})).await
    }
}

#[async_trait::async_trait]
impl BaseAgent for OperationsAgent {
    fn agent_id(&self) -> String {
        "operations_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }

    async fn execute(&self, _payload: Value) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestration::departments::orchestrator::{DepartmentOrchestrator, OrchestratorConfig};
    use crate::db::DB;
    use sqlx::sqlite::SqlitePool;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_operations_agent_cache_invalidation() {
        // We'll set a cache value manually
        let cache = crate::builder::edge::get_edge_cache();
        cache.set_with_tags("ohc:cache:test_tenant_1:storefront:testhash", "old html".to_string(), vec!["tenant-id:test_tenant_1".to_string()], std::time::Duration::from_secs(60)).await;

        let val = cache.get("ohc:cache:test_tenant_1:storefront:testhash").await;
        assert_eq!(val, Some("old html".to_string()));

        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap(),
            store: crate::db::DbStore::Sqlite(pool),
        });

        let (tx, _rx) = tokio::sync::mpsc::channel(10);
        let hub = Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));

        let config = OrchestratorConfig {
            auto_approve: true,
        };
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db, hub, config));

        let agent = OperationsAgent::new(orchestrator);

        let event = DepartmentEvent {
            id: "evt1".to_string(),
            tenant_id: "test_tenant_1".to_string(),
            event_type: "tenant.inventory.updated".to_string(),
            payload: serde_json::json!({}),
        };

        let res = agent.handle_event(&event).await;
        assert!(res.is_ok());

        // Cache should be invalidated
        let val_after = cache.get("ohc:cache:test_tenant_1:storefront:testhash").await;
        assert_eq!(val_after, None);
    }
}
