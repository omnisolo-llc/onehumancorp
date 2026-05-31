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

        let is_local_delivery = event.payload.get("delivery_address").is_some() && event.payload.get("delivery_address").unwrap().as_str().unwrap_or_default() != "";

        let action_description = if event.event_type == "tenant.order.created" {
            if is_local_delivery {
                "Process Local Delivery Order & Group into Dispatch Routes".to_string()
            } else {
                "Process Order & Update Inventory".to_string()
            }
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

        if is_local_delivery {
            let pool = self.orchestrator.db_pool();
            let drivers: Vec<(String,)> = sqlx::query_as("SELECT id FROM drivers WHERE tenant_id = $1 AND is_active = true")
                .bind(&event.tenant_id)
                .fetch_all(&pool)
                .await
                .unwrap_or_default();

            let order_id = event.payload.get("order_id").and_then(|v| v.as_str()).unwrap_or("unknown_order");
            let task_id = format!("task_{}", uuid::Uuid::new_v4().to_string());

            if !drivers.is_empty() {
                let _ = sqlx::query("INSERT INTO delivery_tasks (task_id, tenant_id, order_id, driver_id, status) VALUES ($1, $2, $3, $4, 'assigned')")
                    .bind(&task_id)
                    .bind(&event.tenant_id)
                    .bind(order_id)
                    .bind(&drivers[0].0)
                    .execute(&pool)
                    .await;
            } else {
                let _ = sqlx::query("INSERT INTO delivery_tasks (task_id, tenant_id, order_id, status) VALUES ($1, $2, $3, 'pending')")
                    .bind(&task_id)
                    .bind(&event.tenant_id)
                    .bind(order_id)
                    .execute(&pool)
                    .await;
            }
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
