use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};

pub struct LegalAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl LegalAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait::async_trait]
impl Department for LegalAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Legal
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec!["tenant.compliance.check_needed".to_string(), "charge.dispute.created".to_string()]
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

        let action_description = if event.event_type == "charge.dispute.created" {
            "Draft dispute resolution response letter".to_string()
        } else {
            "Draft compliance terms and policy update".to_string()
        };

        let mut payload = event.payload.clone();
        if event.event_type == "charge.dispute.created" {
            // Reconstruct the simulated payload the UI expects for dispute resolution
            payload = serde_json::json!({
                "feature_type": "dispute_resolution",
                "dispute_id": event.payload.get("id").and_then(|v| v.as_str()).unwrap_or(""),
                "original_message": "Customer claimed charge was unauthorized.",
                "generated_response": "I've processed a refund for the disputed amount based on the bank's feedback. I've compiled delivery evidence and chat logs to submit to Stripe.",
                "refund_amount": event.payload.get("amount").and_then(|v| v.as_i64()).unwrap_or(0) / 100,
                "operational_action": "Mark transaction as disputed in ledger and submit evidence",
                "sender_id": "@customer",
                "customer_id": event.payload.get("customer").and_then(|v| v.as_str()).unwrap_or(""),
            });
        }

        self.orchestrator.execute_action(
            DepartmentType::Legal,
            action_description,
            event.tenant_id.clone(),
            risk,
            payload,
        ).await.map(|_| ())
    }

    fn get_config(&self, _tenant_id: &str) -> Option<DepartmentConfig> {
        None
    }


    async fn query_memory(&self, _query: &str) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String> {
        self.orchestrator.execute_action(self.department_type(), description.clone(), tenant_id.clone(), risk, serde_json::json!({})).await
    }
}

#[async_trait::async_trait]
impl BaseAgent for LegalAgent {
    fn agent_id(&self) -> String {
        "legal_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }

}
