use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, ActionRisk, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest};
use serde_json::{Value, json};

pub struct BusinessAdvisoryAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl BusinessAdvisoryAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }

    pub async fn generate_morning_briefing(&self, tenant_id: &str, metrics: Value) -> Result<String, String> {
        // In a real implementation, this would call an LLM.
        // For this mission, we'll implement a robust generator that follows the "Grandmother Test".

        let business_name = metrics.get("business_name").and_then(|v| v.as_str()).unwrap_or("your business");
        let orders_count = metrics.get("orders_count").and_then(|v| v.as_u64()).unwrap_or(0);
        let products_count = metrics.get("products_count").and_then(|v| v.as_u64()).unwrap_or(0);

        if orders_count == 0 && products_count == 0 {
            return Ok(format!("Good morning! Welcome to OneHumanCorp. Let's get {} started! Your first step is to add your first product or service so customers can find you.", business_name));
        }

        if orders_count == 0 && products_count > 0 {
            return Ok(format!("Good morning! You've got {} products ready to go. Now is a great time to share your link on Instagram to get your first sale!", products_count));
        }

        let briefing = format!(
            "Good morning! {} is looking great. You had {} orders recently. Trending: Customers are loving your new listings! Consider adding more varieties to keep the momentum going.",
            business_name, orders_count
        );

        Ok(briefing)
    }
}

#[async_trait::async_trait]
impl Department for BusinessAdvisoryAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::BusinessAdvisory
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec!["tenant.report.generated".to_string(), "tenant.insight.trending".to_string()]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        self.orchestrator.execute_action(
            DepartmentType::BusinessAdvisory,
            "Generate business health insight".to_string(),
            event.tenant_id.clone(),
            ActionRisk::DraftForReview,
            event.payload.clone(),
        ).await.map(|_| ())
    }

    fn get_config(&self, _tenant_id: &str) -> Option<DepartmentConfig> {
        Some(DepartmentConfig { tone_of_voice: "encouraging".to_string(), auto_approve_limits: 0.0 })
    }

    fn set_config(&mut self, _tenant_id: String, _config: DepartmentConfig) {
    }

    async fn query_memory(&self, _query: &str) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String> {
        self.orchestrator.execute_action(self.department_type(), description.clone(), tenant_id.clone(), risk, json!({}))
            .await
    }
}

#[async_trait::async_trait]
impl BaseAgent for BusinessAdvisoryAgent {
    fn agent_id(&self) -> String {
        "business_advisory_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }

    async fn execute(&self, _payload: Value) -> Result<(), String> {
        Ok(())
    }
}
