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
        } else if event.event_type == "project_milestone_completed" {
            let project_name = event.payload.get("project_name").and_then(|v| v.as_str()).unwrap_or("Unknown Project");
            let milestone_name = event.payload.get("milestone_name").and_then(|v| v.as_str()).unwrap_or("Milestone");
            let amount_cents = event.payload.get("amount_cents").and_then(|v| v.as_i64()).unwrap_or(0);

            let prompt = format!(
                "You are an AI financial assistant. A project milestone has been completed.
                Project: {}
                Milestone: {}
                Total Amount (cents): {}

                Please draft an invoice for this milestone.
                Return a JSON object with two keys:
                1. 'line_items': an array of objects, each with 'description' (string), 'quantity' (integer), 'unit_price' (number in dollars), and 'amount' (number in dollars). Ensure the total amount matches the amount given.
                2. 'generated_message': a short, polite email to the client attaching the invoice for the completed milestone.
                Do not include markdown blocks or any other text outside the JSON.",
                project_name, milestone_name, amount_cents
            );

            let llm_res = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                Ok("minimax") => {
                    if let Ok(api_key) = std::env::var("MINIMAX_API_KEY") {
                        crate::minimax::MinimaxClient::new(api_key).reason(&prompt).await
                    } else {
                        crate::minimax::LocalLLMClient::new().reason(&prompt).await
                    }
                },
                _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await,
            };

            let mut line_items = serde_json::json!([]);
            let mut generated_message = format!("Hi there, your invoice for {} - {} is attached.", project_name, milestone_name);

            if let Ok(res) = llm_res {
                let clean_json = res.trim().trim_start_matches("```json").trim_end_matches("```").trim();
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(clean_json) {
                    if let Some(items) = parsed.get("line_items") {
                        line_items = items.clone();
                    }
                    if let Some(msg) = parsed.get("generated_message").and_then(|v| v.as_str()) {
                        generated_message = msg.to_string();
                    }
                }
            }

            // Fallback line items if LLM fails
            if !line_items.is_array() || line_items.as_array().unwrap().is_empty() {
                line_items = serde_json::json!([{
                    "description": format!("{} - {}", project_name, milestone_name),
                    "quantity": 1,
                    "unit_price": amount_cents as f64 / 100.0,
                    "amount": amount_cents as f64 / 100.0
                }]);
            }

            payload = serde_json::json!({
                "feature_type": "invoice_draft",
                "project_name": project_name,
                "milestone_name": milestone_name,
                "amount_cents": amount_cents,
                "customer_id": event.payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or(""),
                "inbox_message_id": event.payload.get("inbox_message_id").and_then(|v| v.as_str()).unwrap_or(""),
                "line_items": line_items,
                "generated_message": generated_message,
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
