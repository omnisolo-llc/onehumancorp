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
            "return.package.scanned".to_string()
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {

        if event.event_type == "return.package.scanned" {
            // Initiate refund since package was scanned
            let payment_intent_id = event.payload.get("payment_intent_id").and_then(|v| v.as_str()).unwrap_or("pi_test_123");

            #[cfg(ohc_bazel)]
            let stripe_client = crate::integrations::stripe::client::StripeClient::new(std::env::var("STRIPE_SECRET_KEY").unwrap_or_default());
            #[cfg(not(ohc_bazel))]
            let stripe_client = server_integrations_stripe::client::StripeClient::new(std::env::var("STRIPE_SECRET_KEY").unwrap_or_default());

            // Try refunding
            let _ = stripe_client.refund_payment(payment_intent_id, None).await;

            // Log to universal ledger
            let entry_id = uuid::Uuid::new_v4().to_string();
            match &self.orchestrator.db.store {
                crate::db::DbStore::Postgres => {
                    let _ = sqlx::query(
                        "INSERT INTO ohc_universal_ledger (id, tenant_id, event_type, department, payload, created_at) VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)"
                    )
                    .bind(&entry_id)
                    .bind(&event.tenant_id)
                    .bind("refund_processed")
                    .bind("finance")
                    .bind(serde_json::to_string(&event.payload).unwrap_or_default())
                    .execute(&self.orchestrator.db.pool)
                    .await;
                },
                crate::db::DbStore::Sqlite(pool) => {
                    let _ = sqlx::query(
                        "INSERT INTO ohc_universal_ledger (id, tenant_id, event_type, department, payload, created_at) VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP)"
                    )
                    .bind(&entry_id)
                    .bind(&event.tenant_id)
                    .bind("refund_processed")
                    .bind("finance")
                    .bind(serde_json::to_string(&event.payload).unwrap_or_default())
                    .execute(&pool.clone())
                    .await;
                }
            }

            return Ok(());
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

        let action_description = if event.event_type == "payment.captured" {
            "Analyze transaction for split tags and record ledger split".to_string()
        } else {
            "Record deposit and track payment".to_string()
        };

        self.orchestrator.execute_action(
            DepartmentType::Finance,
            action_description,
            event.tenant_id.clone(),
            risk,
            event.payload.clone(),
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
