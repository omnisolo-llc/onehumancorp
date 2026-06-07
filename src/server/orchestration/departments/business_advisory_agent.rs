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
        vec!["tenant.report.weekly_health".to_string(), "tenant.inventory.analyze_stagnant".to_string()]
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

        if event.event_type == "tenant.inventory.analyze_stagnant" {
            // CRON-based inventory analysis pipeline to detect stagnant stock based on sales velocity
            // In a real scenario, this queries inventory and order_history tables.
            // For now, we simulate finding a stagnant product matching the smart pricing policy.

            let payload = serde_json::json!({
                "context": {
                    "smart_pricing": true,
                    "product_id": uuid::Uuid::new_v4().to_string(),
                    "product_name": "Winter Scarf",
                    "old_price": 50.0,
                    "new_price": 42.5,
                    "discount_amount": 7.5,
                    "sales_projection": "+$120",
                    "stagnant_days": 60,
                    "margin_percent": 40
                }
            });

            return self.orchestrator.execute_action(
                DepartmentType::BusinessAdvisory,
                "Smart Price Suggestion: Winter Scarf".to_string(),
                event.tenant_id.clone(),
                ActionRisk::DraftForReview, // Always draft for review for pricing changes initially
                payload
            ).await.map(|_| ());
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

    async fn execute(&self, payload: Value) -> Result<(), String> {
        // If triggered as a CRON job for inventory analysis
        if payload.get("action").and_then(|v| v.as_str()) == Some("analyze_stagnant_inventory") {
            let tenant_id = payload.get("tenant_id").and_then(|v| v.as_str()).unwrap_or("system");
            let event = DepartmentEvent {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: tenant_id.to_string(),
                event_type: "tenant.inventory.analyze_stagnant".to_string(),
                payload: payload.clone(),
            };
            return self.handle_event(&event).await;
        }
        Ok(())
    }
}
