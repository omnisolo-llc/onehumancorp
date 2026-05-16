use crate::orchestration::departments::orchestrator::{DepartmentOrchestrator, ActionRisk, Department, BaseAgent, AgentTriggerType};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest};

use serde_json::Value;

pub struct MarketingAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl MarketingAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait::async_trait]
impl Department for MarketingAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Marketing
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.product.created".to_string(),
            "tenant.campaign.scheduled".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "tenant.product.created" {
            let product_name = event.payload.get("name").and_then(|v| v.as_str()).unwrap_or("New Product");
            let _req = self.request_approval(
                format!("Drafted social media post for '{}'.", product_name),
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
    async fn test_marketing_agent_handle_event() {
        if std::env::var("DATABASE_URL").is_err() { return; }
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(MemoryTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = std::sync::Arc::new(DepartmentOrchestrator::new(db, mesh));
        let agent = MarketingAgent::new(orchestrator);

        let event = DepartmentEvent {
            id: "1".to_string(),
            tenant_id: "tenant1".to_string(),
            event_type: "tenant.product.created".to_string(),
            payload: serde_json::json!({"name": "Summer Collection"}),
        };

        let result = agent.handle_event(&event).await;
        assert!(result.is_ok());
    }
}

#[async_trait::async_trait]
impl BaseAgent for MarketingAgent {
    fn agent_id(&self) -> String {
        "marketing_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }

    async fn execute(&self, _payload: Value) -> Result<(), String> {
        Ok(())
    }
}
