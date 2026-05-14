use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, ActionRisk, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest};
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
        vec!["tenant.quote.accepted".to_string(), "OrderPlaced".to_string()]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "OrderPlaced" {
            let db = self.orchestrator.db();
            let mut processed = false;
            if let Some(items) = event.payload.get("items").and_then(|i| i.as_array()) {
                for item in items {
                    if let (Some(product_id), Some(quantity)) = (item.get("product_id").and_then(|p| p.as_str()), item.get("quantity").and_then(|q| q.as_i64())) {
                        if quantity > 0 {
                            processed = true;
                            match &db.store {
                                crate::db::DbStore::Postgres => {
                                    sqlx::query("UPDATE products SET inventory_count = inventory_count - $1 WHERE id = $2 AND organization_id = $3")
                                        .bind(quantity as i32)
                                        .bind(product_id)
                                        .bind(&event.tenant_id)
                                        .execute(&db.pool)
                                        .await.map_err(|e| e.to_string())?;
                                }
                                crate::db::DbStore::Sqlite(pool) => {
                                    sqlx::query("UPDATE products SET inventory_count = inventory_count - ? WHERE id = ? AND organization_id = ?")
                                        .bind(quantity as i32)
                                        .bind(product_id)
                                        .bind(&event.tenant_id)
                                        .execute(pool)
                                        .await.map_err(|e| e.to_string())?;
                                }
                            }
                        }
                    } else {
                        opentelemetry::global::meter("ohc.orchestrator").u64_counter("agent.error.malformed_payload").build().add(1, &[opentelemetry::KeyValue::new("tenant_id", event.tenant_id.clone())]);
                    }
                }
            } else {
                opentelemetry::global::meter("ohc.orchestrator").u64_counter("agent.error.missing_items").build().add(1, &[opentelemetry::KeyValue::new("tenant_id", event.tenant_id.clone())]);
            }

            if processed {
                let follow_up = DepartmentEvent {
                    id: uuid::Uuid::new_v4().to_string(),
                    tenant_id: event.tenant_id.clone(),
                    event_type: "tenant.order.fulfillment_ready".to_string(),
                    payload: event.payload.clone(),
                };
                self.orchestrator.dispatch_event(follow_up).await?;
            }
            return Ok(());
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

        self.orchestrator.execute_action(
            DepartmentType::Operations,
            "Create order and booking".to_string(),
            event.tenant_id.clone(),
            risk,
            event.payload.clone(),
        ).await.map(|_| ())
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
