use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;

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
        vec!["tenant.payment.received".to_string(), "invoice.payment_failed".to_string()]
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

        if event.event_type == "invoice.payment_failed" {
            // Trigger autonomous dunning workflow
            let description = "Draft dunning SMS/email for failed subscription payment".to_string();

            // In a full implementation, we'd query pgvector memory for the customer profile to set tone.
            // Here, we queue the action.
            self.orchestrator.execute_action(
                DepartmentType::Finance,
                description,
                event.tenant_id.clone(),
                ActionRisk::AutoExecute, // Let agent auto-dispatch friendly reminders
                serde_json::json!({
                    "action": "send_dunning_message",
                    "payload": event.payload.clone(),
                    "follow_up_in_hours": 48
                }),
            ).await.map(|_| ())?;

            // Push to job queue to follow up in 48 hours
            let pool = crate::db::get_pool();
            let _ = sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, next_retry_at)
                 VALUES ($1, $2, $3, $4, NOW() + INTERVAL '48 hours')"
            )
            .bind(uuid::Uuid::new_v4().to_string())
            .bind(&event.tenant_id)
            .bind("check_dunning_status")
            .bind(&event.payload)
            .execute(&pool)
            .await;

            return Ok(());
        }

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
