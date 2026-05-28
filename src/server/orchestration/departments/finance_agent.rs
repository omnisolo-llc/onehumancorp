use crate::orchestration::departments::orchestrator::{
    AgentTriggerType, BaseAgent, Department, DepartmentOrchestrator,
};
use crate::orchestration::departments::types::{
    ActionRisk, ApprovalRequest, DepartmentConfig, DepartmentEvent, DepartmentType,
};
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
        vec![
            "tenant.payment.received".to_string(),
            "tenant.expense.recorded".to_string(),
            "tenant.inventory.used".to_string(),
            "tenant.payment.fee_charged".to_string(),
            "tenant.refund.issued".to_string(),
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

        let (entry_type, description) = match event.event_type.as_str() {
            "tenant.payment.received" => ("revenue", "Add today's sale to the profit tracker"),
            "tenant.expense.recorded" => ("expense", "Match this bill to today's money kept"),
            "tenant.inventory.used" => ("cogs", "Count used supplies against today's sales"),
            "tenant.payment.fee_charged" => {
                ("fee", "Subtract the card processing fee from today's sales")
            }
            "tenant.refund.issued" => ("refund", "Subtract this refund from today's sales"),
            _ => (
                "expense",
                "Review this money movement for the profit tracker",
            ),
        };

        let mut payload = event.payload.clone();
        if let Some(obj) = payload.as_object_mut() {
            obj.insert(
                "profit_entry_type".to_string(),
                serde_json::json!(entry_type),
            );
            obj.insert(
                "plain_language_required".to_string(),
                serde_json::json!(true),
            );
        }

        self.orchestrator
            .execute_action(
                DepartmentType::Finance,
                description.to_string(),
                event.tenant_id.clone(),
                risk,
                payload,
            )
            .await
            .map(|_| ())
    }

    fn get_config(&self, _tenant_id: &str) -> Option<DepartmentConfig> {
        None
    }

    fn set_config(&mut self, _tenant_id: String, _config: DepartmentConfig) {}

    async fn query_memory(&self, _query: &str) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    async fn request_approval(
        &self,
        description: String,
        tenant_id: String,
        risk: ActionRisk,
    ) -> Result<ApprovalRequest, String> {
        self.orchestrator
            .execute_action(
                self.department_type(),
                description.clone(),
                tenant_id.clone(),
                risk,
                serde_json::json!({}),
            )
            .await
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
        // Scheduled finance runs reconcile profit ledger entries into owner-ready briefs.
        Ok(())
    }
}
