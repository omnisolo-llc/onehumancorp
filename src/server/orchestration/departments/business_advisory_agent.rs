use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;

pub struct BusinessAdvisoryAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl BusinessAdvisoryAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait::async_trait]
impl Department for BusinessAdvisoryAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::BusinessAdvisory
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.report.weekly_health".to_string(),
            "system.inventory.check_stagnant".to_string(),
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

        if event.event_type == "system.inventory.check_stagnant" {
            let product_name = event.payload.get("product_name").and_then(|v| v.as_str()).unwrap_or("Product");
            let product_id = event.payload.get("product_id").and_then(|v| v.as_str()).unwrap_or("");
            let suggested_discount = event.payload.get("suggested_discount").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let margin = event.payload.get("margin").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let original_price = event.payload.get("original_price").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let new_price = original_price * (1.0 - suggested_discount / 100.0);

            let action_desc = format!("Smart Price Suggestion: {}", product_name);
            let payload = serde_json::json!({
                "product_id": product_id,
                "product_name": product_name,
                "suggested_discount": suggested_discount,
                "margin": margin,
                "original_price": original_price,
                "new_price": new_price,
                "message": format!("Your '{}' hasn't sold recently. Would you like to apply a {}% discount this weekend to clear space? Your profit margin will remain safe at {}%.", product_name, suggested_discount, margin)
            });

            self.orchestrator.execute_action(
                DepartmentType::BusinessAdvisory,
                action_desc,
                event.tenant_id.clone(),
                ActionRisk::DraftForReview, // Always ask for approval for pricing
                payload,
            ).await?;

            return Ok(());
        }

        // Scheduled background worker triggers tenant.report.weekly_health to generate brief.
        self.orchestrator.execute_action(
            DepartmentType::BusinessAdvisory,
            "Draft weekly business health report and next-action suggestions".to_string(),
            event.tenant_id.clone(),
            risk,
            event.payload.clone(),
        ).await.map(|_| ())
    }

    fn get_config(&self, _tenant_id: &str) -> Option<DepartmentConfig> {
        None
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
impl BaseAgent for BusinessAdvisoryAgent {
    fn agent_id(&self) -> String {
        "business_advisory_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::Scheduled
    }

    async fn execute(&self, _payload: Value) -> Result<(), String> {
        Ok(())
    }
}
