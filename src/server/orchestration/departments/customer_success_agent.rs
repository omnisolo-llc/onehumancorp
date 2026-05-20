use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;

pub struct CustomerSuccessAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl CustomerSuccessAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait::async_trait]
impl Department for CustomerSuccessAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::CustomerSuccess
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.order.fulfillment_ready".to_string(),
            "tenant.message.received".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "tenant.message.received" {
            let record = ohc_builtin_agent::memory_store::EmbeddingRecord {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: event.tenant_id.clone(),
                agent_id: self.agent_id(),
                content: "Customer requested a quote for a vegan cake.".to_string(),
                embedding: vec![0.5, 0.5, 0.5],
                source_type: "SESSION_DATA".to_string(),
                created_at: chrono::Utc::now(),
                last_referenced_at: chrono::Utc::now(),
                reference_count: 1,
                reliability_score: 80,
                owner_override: false,
                metadata: None,
            };
            self.orchestrator.write_long_term_memory(record).await?;

            let follow_up = DepartmentEvent {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: event.tenant_id.clone(),
                event_type: "tenant.quote.requested".to_string(),
                payload: event.payload.clone(),
            };
            self.orchestrator.dispatch_event(follow_up).await?;
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

        self.orchestrator.execute_action(
            DepartmentType::CustomerSuccess,
            "Send personalized thank you & shipping ETA".to_string(),
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
impl BaseAgent for CustomerSuccessAgent {
    fn agent_id(&self) -> String {
        "customer_success_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }

    async fn execute(&self, _payload: Value) -> Result<(), String> {
        Ok(())
    }
}
