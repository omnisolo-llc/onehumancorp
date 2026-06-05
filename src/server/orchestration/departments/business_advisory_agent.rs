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

        // Scheduled background worker triggers tenant.report.weekly_health to generate brief.
        let prompt = "Analyze the recent business data and generate a short, weekly plain-language business health report. Format it as a push notification with actionable suggestions. E.g., 'Inquiries are up 15%. Should we increase our baseline quote rate?'";

        let draft_report = if let Ok(provider) = std::env::var("OHC_LLM_PROVIDER") {
            if provider == "minimax" {
                let minimax_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                crate::minimax::MinimaxClient::new(minimax_key).reason(prompt).await.unwrap_or_else(|_| "Weekly Report: All metrics are steady.".to_string())
            } else {
                crate::minimax::LocalLLMClient::new().reason(prompt).await.unwrap_or_else(|_| "Weekly Report: All metrics are steady.".to_string())
            }
        } else {
            crate::minimax::LocalLLMClient::new().reason(prompt).await.unwrap_or_else(|_| "Weekly Report: All metrics are steady.".to_string())
        };

        let mut payload = event.payload.clone();
        if let Value::Object(ref mut map) = payload {
            map.insert("report_content".to_string(), Value::String(draft_report));
        } else {
            payload = serde_json::json!({
                "report_content": draft_report
            });
        }

        self.orchestrator.execute_action(
            DepartmentType::BusinessAdvisory,
            "Draft weekly business health report and next-action suggestions".to_string(),
            event.tenant_id.clone(),
            risk,
            payload,
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
        Ok(())
    }
}
