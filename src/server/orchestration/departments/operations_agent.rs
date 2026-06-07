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
            "tenant.pricing.discount_approved".to_string(),
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

        if event.event_type == "tenant.pricing.discount_approved" {
            let product_id = event.payload.get("product_id").and_then(|v| v.as_str()).unwrap_or("");
            let new_price = event.payload.get("new_price").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let product_name = event.payload.get("product_name").and_then(|v| v.as_str()).unwrap_or("Product");

            // Normally we'd do an actual DB update here:
            // "UPDATE products SET price = $1 WHERE id = $2 AND tenant_id = $3"
            // And insert into active_discounts.

            // Dispatch event to clear Redis Cache implicitly by propagating state
            // and tell Marketing to broadcast the flash sale.
            let marketing_event = DepartmentEvent {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: event.tenant_id.clone(),
                event_type: "tenant.pricing.discount_applied".to_string(),
                payload: serde_json::json!({
                    "product_id": product_id,
                    "product_name": product_name,
                    "new_price": new_price
                }),
            };
            return self.orchestrator.dispatch_event(marketing_event).await;
        }

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
