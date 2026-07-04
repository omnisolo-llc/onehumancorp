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
        } else if event.event_type == "project_milestone_completed" {
            "Draft Invoice ready for Nora's Design Project".to_string()
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
            let invoice_id = uuid::Uuid::new_v4().to_string();
            let project_id = event.payload.get("project_id").and_then(|v| v.as_str()).unwrap_or("proj_1");

            let pool = self.orchestrator.db().pool.clone();
            match &self.orchestrator.db().store {
                crate::db::DbStore::Postgres => {
                    let _ = sqlx::query(
                        "INSERT INTO invoices (id, tenant_id, client_id, client_name, status, due_date, currency, total_amount)
                         VALUES ($1, $2, 'new-client', 'New Client', 'draft', $3, 'USD', 100.0) ON CONFLICT DO NOTHING"
                    )
                    .bind(&invoice_id)
                    .bind(&event.tenant_id)
                    .bind(chrono::Utc::now().timestamp() + 86400 * 30)
                    .execute(&pool)
                    .await;

                    let line_item_id = uuid::Uuid::new_v4().to_string();
                    let _ = sqlx::query(
                        "INSERT INTO invoice_line_items (id, tenant_id, invoice_id, description, quantity, unit_price, amount)
                         VALUES ($1, $2, $3, 'Consulting Services', 1, 100.0, 100.0) ON CONFLICT DO NOTHING"
                    )
                    .bind(&line_item_id)
                    .bind(&event.tenant_id)
                    .bind(&invoice_id)
                    .execute(&pool)
                    .await;
                }
                _ => {}
            }

            payload = serde_json::json!({
                "feature_type": "draft_invoice",
                "invoice_id": invoice_id,
                "project_id": project_id,
                "operational_action": "Draft Invoice"
            });
        } else if event.event_type == "invoice.overdue" {
            let invoice_id = event.payload.get("invoice_id").and_then(|v| v.as_str()).unwrap_or("unknown");

            let default_original = format!("Invoice {} is overdue.", invoice_id);
            let default_generated = format!("Hi there, just checking in to see if you received invoice {}. Let us know if you have any questions!", invoice_id);

            let original_message = event.payload.get("original_message").and_then(|v| v.as_str()).unwrap_or(&default_original);
            let generated_response = event.payload.get("generated_response").and_then(|v| v.as_str()).unwrap_or(&default_generated);

            payload = serde_json::json!({
                "feature_type": "invoice_followup",
                "invoice_id": invoice_id,
                "original_message": original_message,
                "generated_response": generated_response,
                "operational_action": "Draft personalized reminder",
                "customer_id": event.payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or(""),
            });
        } else if event.event_type == "project_milestone_completed" {
            let project_name = event.payload.get("project_name").and_then(|v| v.as_str()).unwrap_or("Unknown Project");
            let milestone_name = event.payload.get("milestone_name").and_then(|v| v.as_str()).unwrap_or("Milestone");
            let amount_cents = event.payload.get("amount_cents").and_then(|v| v.as_i64()).unwrap_or(0);
            payload = serde_json::json!({
                "feature_type": "invoice_draft",
                "project_name": project_name,
                "milestone_name": milestone_name,
                "amount_cents": amount_cents,
                "customer_id": event.payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or(""),
                "inbox_message_id": event.payload.get("inbox_message_id").and_then(|v| v.as_str()).unwrap_or(""),
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
