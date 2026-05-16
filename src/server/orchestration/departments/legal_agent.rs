use crate::orchestration::departments::orchestrator::{DepartmentOrchestrator, ActionRisk, Department, BaseAgent, AgentTriggerType};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest};

use serde_json::Value;

pub struct LegalAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl LegalAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait::async_trait]
impl Department for LegalAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Legal
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.contract.requested".to_string(),
            "tenant.policy.update_needed".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "tenant.contract.requested" {
            let contract_type = event.payload.get("contract_type").and_then(|v| v.as_str()).unwrap_or("General Service");
            let _req = self.request_approval(
                format!("Drafted new {} contract.", contract_type),
                event.tenant_id.clone(),
                ActionRisk::DraftForReview
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
    async fn test_legal_agent_handle_event() {
        if std::env::var("DATABASE_URL").is_err() { return; }
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(MemoryTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = std::sync::Arc::new(DepartmentOrchestrator::new(db, mesh));
        let agent = LegalAgent::new(orchestrator);

        let event = DepartmentEvent {
            id: "1".to_string(),
            tenant_id: "tenant1".to_string(),
            event_type: "tenant.contract.requested".to_string(),
            payload: serde_json::json!({"contract_type": "NDA"}),
        };

        let result = agent.handle_event(&event).await;
        assert!(result.is_ok());
    }
}

#[async_trait::async_trait]
impl BaseAgent for LegalAgent {
    fn agent_id(&self) -> String {
        "legal_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }

    async fn execute(&self, _payload: Value) -> Result<(), String> {
        Ok(())
    }
}
