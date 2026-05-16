use crate::orchestration::departments::orchestrator::{DepartmentOrchestrator, ActionRisk, Department, BaseAgent, AgentTriggerType};
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
        vec![
            "tenant.invoice.overdue".to_string(),
            "tenant.payment.failed".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "tenant.invoice.overdue" {
            let invoice_id = event.payload.get("invoice_id").and_then(|v| v.as_str()).unwrap_or("unknown");
            let _req = self.request_approval(
                format!("Drafted payment reminder for overdue invoice #{}", invoice_id),
                event.tenant_id.clone(),
                ActionRisk::AutoExecute
            ).await?;
        }
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestration::departments::orchestrator::{DepartmentOrchestrator};
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::MemoryTransport;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_finance_agent_handle_event() {
        if std::env::var("DATABASE_URL").is_err() { return; }
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(MemoryTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = std::sync::Arc::new(DepartmentOrchestrator::new(db, mesh));
        let agent = FinanceAgent::new(orchestrator);

        let event = DepartmentEvent {
            id: "1".to_string(),
            tenant_id: "tenant1".to_string(),
            event_type: "tenant.invoice.overdue".to_string(),
            payload: serde_json::json!({"invoice_id": "INV-1001"}),
        };

        let result = agent.handle_event(&event).await;
        assert!(result.is_ok());
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
