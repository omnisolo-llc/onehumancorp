use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;
use crate::db::get_pool;

pub struct FinanceAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl FinanceAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }

    async fn evaluate_capital_offer(&self, event: &DepartmentEvent) -> Result<(), String> {
        // Extract booking amount from payload (simplified for example)
        let amount = event.payload.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let merchant_id = event.payload.get("merchant_id").and_then(|v| v.as_str()).unwrap_or("");

        if amount > 500.0 && !merchant_id.is_empty() {
            // Generate a smart offer for 30% of the booking value
            let offer_amount = amount * 0.3;
            let flat_fee = offer_amount * 0.075; // 7.5% flat fee

            let pool = get_pool();
            let offer_id = Uuid::new_v4().to_string();

            sqlx::query(
                r#"
                INSERT INTO capital_offers (id, tenant_id, merchant_id, amount, flat_fee, repayment_percentage, status, expires_at)
                VALUES ($1, $2, $3, $4, $5, $6, 'active', NOW() + INTERVAL '7 days')
                "#
            )
            .bind(&offer_id)
            .bind(&event.tenant_id)
            .bind(merchant_id)
            .bind(offer_amount)
            .bind(flat_fee)
            .bind(0.10) // 10% daily repayment
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create capital offer: {}", e))?;

            // Notify via Operations/CS agent context (simulated here via orchestrator action)
            self.orchestrator.execute_action(
                DepartmentType::Finance,
                format!("Created smart capital offer for {}", merchant_id),
                event.tenant_id.clone(),
                ActionRisk::AutoExecute,
                serde_json::json!({
                    "offer_id": offer_id,
                    "amount": offer_amount,
                    "flat_fee": flat_fee
                }),
            ).await?;
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl Department for FinanceAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Finance
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec!["tenant.payment.received".to_string(), "tenant.booking.confirmed".to_string()]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "tenant.booking.confirmed" {
            self.evaluate_capital_offer(event).await?;
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
            DepartmentType::Finance,
            "Record deposit and track payment".to_string(),
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
impl BaseAgent for FinanceAgent {
    fn agent_id(&self) -> String {
        "finance_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::Scheduled
    }

    async fn execute(&self, _payload: Value) -> Result<(), String> {
        // Run scheduled worker to aggregate weekly sales data.
        Ok(())
    }
}
