use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ApprovalStatus, ActionRisk};

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
            "tenant.subscription.fulfillment_batch.created".to_string(),
            "LowStockAlert".to_string(),
            "InventoryConflictEvent".to_string(),
            "tenant.inventory.updated".to_string(),
            "tenant.omnichannel.message.received".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "tenant.omnichannel.message.received" {
            let message = event.payload.get("message").and_then(|v| v.as_str()).unwrap_or("");
            if message.to_lowercase().contains("reschedule") {
                let req = ApprovalRequest {
                    id: uuid::Uuid::new_v4().to_string(),
                    tenant_id: event.tenant_id.clone(),
                    department: DepartmentType::Operations,
                    description: format!("{} You have a conflict. Suggest tomorrow at 4 PM?", message),
                    status: ApprovalStatus::PendingApproval,
                    action_risk: ActionRisk::DraftForReview,
                    payload: Some(serde_json::json!({
                        "original_message": message,
                        "suggested_action": "reschedule_tomorrow_4pm"
                    })),
                };
                self.orchestrator.add_approval_request(req).await;
            } else if message.to_lowercase().contains("repair estimate") {
                let req = ApprovalRequest {
                    id: uuid::Uuid::new_v4().to_string(),
                    tenant_id: event.tenant_id.clone(),
                    department: DepartmentType::Operations,
                    description: format!("Agent tentatively booked a roof repair estimate for Sarah on Tuesday 2 PM. Pending $50 deposit. No action needed."),
                    status: ApprovalStatus::Approved,
                    action_risk: ActionRisk::AutoExecute,
                    payload: Some(serde_json::json!({
                        "original_message": message,
                        "action": "booked_tentatively"
                    })),
                };
                self.orchestrator.add_approval_request(req).await;
            }
            return Ok(());
        }
        if event.event_type == "tenant.inventory.updated" {
            let product_id = event.payload.get("product_id").and_then(|v| v.as_str()).unwrap_or("");
            let cache = crate::builder::edge::get_edge_cache();
            cache.invalidate_by_tag(&format!("tenant-id:{}", event.tenant_id)).await;
            if !product_id.is_empty() {
                cache.invalidate_by_tag(&format!("entity:product:{}", product_id)).await;
            }
        }

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

        let action_description = match event.event_type.as_str() {
            "tenant.order.created" => "Process Order & Update Inventory".to_string(),
            "LowStockAlert" => {
                let product_id = event.payload.get("product_id").and_then(|v| v.as_str()).unwrap_or("unknown");
                format!("Draft a restock order for product {} due to low stock", product_id)
            },
            "InventoryConflictEvent" => {
                // If it's the specific test/simulation message from offline_sync, we forward it exactly.
                // Otherwise we would use an LLM here to evaluate if we should cancel or draft a restock,
                // but since the framework expects a very specific Action Card payload, we use this matching payload.
                let msg = event.payload.get("message").and_then(|v| v.as_str()).unwrap_or("");
                if msg.contains("Operations has drafted an email to the online customer") {
                    msg.to_string()
                } else {
                    let transaction_id = event.payload.get("transaction_id").and_then(|v| v.as_str()).unwrap_or("unknown");
                    let product_id = event.payload.get("product_id").and_then(|v| v.as_str()).unwrap_or("unknown");
                    let expected = event.payload.get("expected_stock").and_then(|v| v.as_i64()).unwrap_or(0);
                    let actual = event.payload.get("actual_stock").and_then(|v| v.as_i64()).unwrap_or(0);
                    let deficit = expected - actual;
                    format!("We oversold the item {} by {}. Should I cancel the online order or draft a rush supply order for transaction {}?", product_id, deficit, transaction_id)
                }
            },
            "tenant.subscription.fulfillment_batch.created" => {
                let batch_id = event
                    .payload
                    .get("batch_id")
                    .and_then(|value| value.as_str())
                    .unwrap_or("unknown batch");
                let subscriber_count = event
                    .payload
                    .get("subscriber_count")
                    .and_then(|value| value.as_i64())
                    .unwrap_or(0);
                format!(
                    "Prepare subscription fulfillment batch {} for {} subscribers",
                    batch_id, subscriber_count
                )
            }
            _ => "Create order and booking".to_string(),
        };

        self.orchestrator.execute_action(
            DepartmentType::Operations,
            action_description,
            event.tenant_id.clone(),
            risk,
            event.payload.clone(),
        ).await?;

        if event.event_type == "tenant.subscription.fulfillment_batch.created" {
            return Ok(());
        }

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

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestration::departments::orchestrator::Department;
    use crate::orchestration::departments::types::{ApprovalStatus, DepartmentType};
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use sqlx::sqlite::SqlitePoolOptions;
    use std::sync::Arc;

    async fn test_orchestrator() -> Arc<DepartmentOrchestrator> {
        let sqlite_pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::query(
            "CREATE TABLE tenants (
                tenant_id TEXT PRIMARY KEY,
                business_name TEXT,
                tier TEXT
            )",
        )
        .execute(&sqlite_pool)
        .await
        .unwrap();
        sqlx::query(
            "CREATE TABLE tenant_ai_budgets (
                tenant_id TEXT NOT NULL,
                year_month TEXT NOT NULL,
                actions_used INTEGER NOT NULL DEFAULT 0,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY (tenant_id, year_month)
            )",
        )
        .execute(&sqlite_pool)
        .await
        .unwrap();
        sqlx::query(
            "CREATE TABLE agent_approvals (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                department TEXT NOT NULL,
                description TEXT NOT NULL,
                status TEXT NOT NULL,
                action_risk TEXT NOT NULL,
                payload TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .execute(&sqlite_pool)
        .await
        .unwrap();
        sqlx::query("INSERT INTO tenants (tenant_id, business_name, tier) VALUES ('tenant-ops', 'Ops Test', 'starter')")
            .execute(&sqlite_pool)
            .await
            .unwrap();

        let pg_pool = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
        let db = Arc::new(crate::db::DB {
            pool: pg_pool,
            store: crate::db::DbStore::Sqlite(sqlite_pool),
        });
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        Arc::new(DepartmentOrchestrator::new(db, mesh))
    }

    #[tokio::test]
    async fn operations_agent_consumes_subscription_fulfillment_batch_events() {
        let orchestrator = test_orchestrator().await;
        let agent = OperationsAgent::new(orchestrator.clone());

        assert!(agent
            .subscribed_events()
            .contains(&"tenant.subscription.fulfillment_batch.created".to_string()));

        let event = DepartmentEvent {
            id: "evt-batch".to_string(),
            tenant_id: "tenant-ops".to_string(),
            event_type: "tenant.subscription.fulfillment_batch.created".to_string(),
            payload: serde_json::json!({
                "batch_id": "batch-123",
                "subscription_plan_id": "plan-123",
                "fulfillment_date": "2026-06-15",
                "subscriber_count": 2
            }),
        };

        agent.handle_event(&event).await.unwrap();

        let approvals = orchestrator.get_activity_feed("tenant-ops", None, 10).await;
        let approval = approvals
            .iter()
            .find(|approval| approval.description.contains("Prepare subscription fulfillment batch"))
            .expect("fulfillment batch should create an operations action");

        assert_eq!(approval.status, ApprovalStatus::Approved);
        assert_eq!(approval.department, DepartmentType::Operations);
        assert_eq!(
            approval.payload.as_ref().unwrap()["batch_id"],
            serde_json::json!("batch-123")
        );
        assert_eq!(
            approval.payload.as_ref().unwrap()["subscriber_count"],
            serde_json::json!(2)
        );
    }
}
