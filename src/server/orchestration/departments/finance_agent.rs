use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, ActionRisk, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest};
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
        vec!["tenant.payment.received".to_string(), "tenant.financial.end_of_month".to_string()]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "tenant.payment.received" {
            let risk = ActionRisk::AutoExecute;
            self.orchestrator.execute_action(
                DepartmentType::Finance,
                "Generate standard receipt".to_string(),
                event.tenant_id.clone(),
                risk,
                event.payload.clone(),
            ).await.map(|_| ())
        } else if event.event_type == "tenant.financial.end_of_month" {
            let risk = ActionRisk::DraftForReview;
            self.orchestrator.execute_action(
                DepartmentType::Finance,
                "Draft tax report for review".to_string(),
                event.tenant_id.clone(),
                risk,
                event.payload.clone(),
            ).await.map(|_| ())
        } else {
            Ok(())
        }
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
        AgentTriggerType::EventDriven
    }

    async fn execute(&self, _payload: Value) -> Result<(), String> {
        Ok(())
    }
}
