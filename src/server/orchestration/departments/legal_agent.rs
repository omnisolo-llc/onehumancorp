use crate::orchestration::departments::orchestrator::{Department, DepartmentOrchestrator, ActionRisk};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ApprovalStatus};
use uuid::Uuid;
use std::collections::HashMap;

pub struct LegalAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
    configs: HashMap<String, DepartmentConfig>,
}

impl LegalAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator, configs: HashMap::new() }
    }
}

#[async_trait::async_trait]
impl Department for LegalAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Legal
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec!["policy.update_needed".to_string(), "compliance.check".to_string()]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        let risk = ActionRisk::DraftForReview;

        let _ = self.request_approval(format!("Handled legal event: {}", event.event_type), event.tenant_id.clone(), risk).await;
        Ok(())
    }

    async fn query_memory(&self, query: &str) -> Result<Vec<String>, String> {
        Ok(vec![format!("Memory result for legal: {}", query)])
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
