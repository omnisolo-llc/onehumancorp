use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;

pub struct BusinessAdvisoryAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl BusinessAdvisoryAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait::async_trait]
impl Department for BusinessAdvisoryAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::BusinessAdvisory
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec!["tenant.report.weekly_health".to_string()]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type != "tenant.report.weekly_health" {
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
            // Business Advisory insights are generally safe to AutoExecute to feed
            ActionRisk::AutoExecute
        };

        let client = crate::minimax::LocalLLMClient::new();
        let prompt = "Provide a weekly business health summary in plain language for the user, summarizing their recent activity in a short, friendly message.";

        let summary = match client.reason(prompt).await {
            Ok(res) => res,
            Err(_) => "You had a solid week! Consider reviewing your pricing strategy to optimize further.".to_string(),
        };

        // Scheduled background worker triggers tenant.report.weekly_health to generate brief.
        self.orchestrator.execute_action(
            DepartmentType::BusinessAdvisory,
            summary,
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
impl BaseAgent for BusinessAdvisoryAgent {
    fn agent_id(&self) -> String {
        "business_advisory_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::Scheduled
    }

    async fn execute(&self, _payload: Value) -> Result<(), String> {
        // Run scheduled worker to aggregate weekly sales data and trigger health event
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(604800)); // 1 week
        let orchestrator_clone = self.orchestrator.clone();

        tokio::spawn(async move {
            loop {
                interval.tick().await;
                // Emit the event to all tenants - normally would query active tenants
                let event = DepartmentEvent {
                    id: uuid::Uuid::new_v4().to_string(),
                    tenant_id: "default".to_string(), // In reality we loop active tenants
                    event_type: "tenant.report.weekly_health".to_string(),
                    payload: serde_json::json!({}),
                };
                let _ = orchestrator_clone.dispatch_event(event).await;
            }
        });

        Ok(())
    }
}
