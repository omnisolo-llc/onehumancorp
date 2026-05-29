use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
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
            "tenant.insight.trending".to_string(),
            "tenant.job.completed_with_media".to_string(),
            "agent:marketing:approved".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        let risk = ActionRisk::DraftForReview;

        if event.event_type == "agent:marketing:approved" {
            // Actual logic to publish the case study to the storefront CDN when approved.
            // For now, we simulate invalidating cache and publishing.
            println!("Simulating edge cache invalidation and portfolio publish for tenant: {}", event.tenant_id);
            return Ok(());
        }

        if event.event_type == "tenant.job.completed_with_media" {
            let hero_image = event.payload.get("media_url").and_then(|v| v.as_str()).unwrap_or("");

            let action_payload = serde_json::json!({
                "feature_type": "case_study_generator",
                "title": "Cedar Fence Install",
                "generated_copy": "Beautiful new cedar privacy fence installed in downtown area. Completed on time and on budget.",
                "hero_image": hero_image
            });

            self.orchestrator.execute_action(
                DepartmentType::Marketing,
                "Draft new portfolio post for review".to_string(),
                event.tenant_id.clone(),
                risk,
                action_payload,
            ).await.map(|_| ())?;

            return Ok(());
        }

        self.orchestrator.execute_action(
            DepartmentType::Marketing,
            "Draft social media campaign for trending item".to_string(),
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
