use crate::orchestration::departments::orchestrator::{Department, DepartmentOrchestrator, ActionRisk};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ApprovalStatus};
use uuid::Uuid;
use std::collections::HashMap;

pub struct MarketingAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
    configs: HashMap<String, DepartmentConfig>,
}

impl MarketingAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator, configs: HashMap::new() }
    }
}

#[async_trait::async_trait]
impl Department for MarketingAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Marketing
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec!["campaign.needed".to_string(), "social_media.mention".to_string(), "website.published".to_string()]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        let risk = if event.event_type == "website.published" {
            ActionRisk::AutoExecute
        } else {
            ActionRisk::DraftForReview
        };

        let _ = self.request_approval(format!("Handled marketing event: {}", event.event_type), event.tenant_id.clone(), risk).await;
        Ok(())
    }

    async fn query_memory(&self, query: &str) -> Result<Vec<String>, String> {
        Ok(vec![format!("Memory result for marketing: {}", query)])
    }

    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String> {
        let risk_str = match risk {
            ActionRisk::AutoExecute => "LOW",
            ActionRisk::DraftForReview => "HIGH",
        };
        let req = ApprovalRequest {
            id: Uuid::new_v4().to_string(),
            tenant_id,
            department: self.department_type(),
            description,
            status: match risk {
                ActionRisk::AutoExecute => ApprovalStatus::Approved,
                ActionRisk::DraftForReview => ApprovalStatus::Pending,
            },
            action_risk: risk_str.to_string(),
        };
        self.orchestrator.add_approval_request(req.clone()).await;
        Ok(req)
    }

    fn get_config(&self, tenant_id: &str) -> Option<DepartmentConfig> {
        self.configs.get(tenant_id).cloned()
    }

    fn set_config(&mut self, tenant_id: String, config: DepartmentConfig) {
        self.configs.insert(tenant_id, config);
    }
}
