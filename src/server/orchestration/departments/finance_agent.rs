use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};

pub struct FinanceAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl FinanceAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait::async_trait]
impl Department for FinanceAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Finance
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.payment.received".to_string(),
            "payment.captured".to_string(),
            "charge.dispute.created".to_string(),
            "invoice.overdue".to_string(),
            "project_milestone_completed".to_string()
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

        let action_description = if event.event_type == "payment.captured" {
            "Analyze transaction for split tags and record ledger split".to_string()
        } else if event.event_type == "charge.dispute.created" {
            "Draft dispute resolution for review".to_string()
        } else if event.event_type == "invoice.overdue" {
            "Draft personalized invoice follow-up for review".to_string()
        } else if event.event_type == "project_milestone_completed" {
            "Draft invoice for completed project milestone".to_string()
        } else {
            "Record deposit and track payment".to_string()
        };

        let mut payload = event.payload.clone();
        if event.event_type == "charge.dispute.created" {
            // Reconstruct the simulated payload the UI expects for dispute resolution
            payload = serde_json::json!({
                "feature_type": "dispute_resolution",
                "dispute_id": event.payload.get("id").and_then(|v| v.as_str()).unwrap_or(""),
                "original_message": "Customer claimed charge was unauthorized.",
                "generated_response": "I've processed a refund for the disputed amount based on the bank's feedback.",
                "refund_amount": event.payload.get("amount").and_then(|v| v.as_i64()).unwrap_or(0) / 100,
                "operational_action": "Mark transaction as disputed in ledger",
                "sender_id": "@customer",
                "customer_id": event.payload.get("customer").and_then(|v| v.as_str()).unwrap_or(""),
            });
        } else if event.event_type == "invoice.overdue" {
            let invoice_id = event.payload.get("invoice_id").and_then(|v| v.as_str()).unwrap_or("unknown");
            payload = serde_json::json!({
                "feature_type": "invoice_followup",
                "invoice_id": invoice_id,
                "original_message": format!("Invoice {} is overdue.", invoice_id),
                "generated_response": format!("Hi there, just checking in to see if you received invoice {}. Let us know if you have any questions!", invoice_id),
                "operational_action": "Draft personalized reminder",
                "customer_id": event.payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or(""),
            });
        } else if event.event_type == "project_milestone_completed" {
            let project_id = event.payload.get("project_id").and_then(|v| v.as_str()).unwrap_or("unknown");
            let milestone_name = event.payload.get("milestone_name").and_then(|v| v.as_str()).unwrap_or("Milestone");
            let customer_id = event.payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("");
            let amount = event.payload.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let amount_cents = (amount * 100.0) as i64;

            #[cfg(not(ohc_bazel))]
            use server_integrations_core::client::StripeClient;
            #[cfg(ohc_bazel)]
            use crate::integrations::stripe::client::StripeClient;

            let api_key = std::env::var("STRIPE_API_KEY").unwrap_or_default();
            let stripe_client = StripeClient::new(api_key);

            let description = format!("Invoice for {} - {}", project_id, milestone_name);
            let invoice_id = stripe_client.create_draft_invoice(customer_id, amount_cents, &description).await.unwrap_or_else(|_| "mock_invoice_id".to_string());

            payload = serde_json::json!({
                "feature_type": "invoice_draft",
                "project_id": project_id,
                "milestone_name": milestone_name,
                "original_message": format!("Project milestone '{}' has been marked complete.", milestone_name),
                "generated_response": format!("I have drafted an invoice for the completed milestone '{}'. Please review and send.", milestone_name),
                "operational_action": "Draft milestone invoice",
                "customer_id": customer_id,
                "stripe_invoice_id": invoice_id,
            });
        }

        self.orchestrator.execute_action(
            DepartmentType::Finance,
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
impl BaseAgent for FinanceAgent {
    fn agent_id(&self) -> String {
        "finance_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::Scheduled
    }

}
