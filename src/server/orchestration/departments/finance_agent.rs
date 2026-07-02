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
            "invoice.overdue".to_string()
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
            "Process multi-currency payment and generate localized invoice".to_string()
        } else if event.event_type == "charge.dispute.created" {
            "Draft dispute resolution for review".to_string()
        } else if event.event_type == "invoice.overdue" {
            "Draft personalized invoice follow-up for review".to_string()
        } else {
            "Record deposit and track payment".to_string()
        };

        let mut payload = event.payload.clone();
        if event.event_type == "payment.captured" {
            let tx_currency = event.payload.get("currency").and_then(|v| v.as_str()).unwrap_or("usd").to_uppercase();
            let base_currency = event.payload.get("base_currency").and_then(|v| v.as_str()).unwrap_or("USD").to_uppercase();
            let amount = event.payload.get("amount").and_then(|v| v.as_i64()).unwrap_or(0);
            let mut exchange_rate = 1.0;
            let mut localized_invoice_drafted = false;

            // Simple simulated check for Global Sales / multi-currency
            // A real implementation would fetch live rates based on tenant's base currency and transaction currency
            if tx_currency != base_currency {
                if tx_currency == "EUR" {
                    exchange_rate = 1.1; // Simulated exchange rate EUR -> USD
                } else if tx_currency == "GBP" {
                    exchange_rate = 1.25;
                } else if tx_currency == "CAD" {
                    exchange_rate = 0.74;
                }
                localized_invoice_drafted = true;
            }

            payload = serde_json::json!({
                "feature_type": "localized_invoicing",
                "transaction_currency": tx_currency,
                "base_currency": base_currency,
                "exchange_rate": exchange_rate,
                "original_amount": amount,
                "base_amount": (amount as f64 * exchange_rate).round() as i64,
                "localized_invoice_drafted": localized_invoice_drafted,
                "original_message": format!("Captured payment of {} {} (Base: {}).", amount, tx_currency, base_currency),
                "generated_response": if localized_invoice_drafted { "Autonomously drafted a localized tax-compliant invoice." } else { "Payment recorded." },
                "operational_action": "Record localized transaction in ledger",
            });
        } else if event.event_type == "charge.dispute.created" {
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
