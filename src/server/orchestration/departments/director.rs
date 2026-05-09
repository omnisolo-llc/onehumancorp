use std::sync::Arc;
use tokio::sync::RwLock;

use crate::orchestration::departments::types::{DepartmentType, DepartmentConfig, DepartmentEvent};
use crate::orchestration::departments::orchestrator::{DepartmentOrchestrator, Department, DummyDepartment};

pub struct AiDirector {
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

impl AiDirector {
    pub fn new(orchestrator: Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }

    pub async fn submit_intent(&self, tenant_id: &str, intent: &str) -> Result<(), String> {
        let (dep_type, event_type, payload) = match intent.to_lowercase() {
            i if i.contains("post") || i.contains("campaign") || i.contains("promote") => (
                DepartmentType::Marketing,
                "MarketingIntent".to_string(),
                serde_json::json!({ "intent": intent }),
            ),
            i if i.contains("order") || i.contains("inventory") || i.contains("ship") => (
                DepartmentType::Operations,
                "OperationsIntent".to_string(),
                serde_json::json!({ "intent": intent }),
            ),
            i if i.contains("customer") || i.contains("support") || i.contains("reply") => (
                DepartmentType::CustomerSuccess,
                "CustomerSuccessIntent".to_string(),
                serde_json::json!({ "intent": intent }),
            ),
            _ => (
                DepartmentType::BusinessAdvisory,
                "AdvisoryIntent".to_string(),
                serde_json::json!({ "intent": intent }),
            ),
        };

        let event = DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            event_type,
            payload,
        };

        self.orchestrator.dispatch_event(event).await
    }
}

pub async fn setup_mock_departments(orchestrator: Arc<DepartmentOrchestrator>, tenant_id: &str) {
    let mut marketing = DummyDepartment::new(
        DepartmentType::Marketing,
        vec!["MarketingIntent".to_string()],
        orchestrator.clone(),
    );
    marketing.set_config(tenant_id.to_string(), DepartmentConfig { tone_of_voice: "Casual".to_string(), auto_approve_limits: 0.0, requires_review: true });

    let mut operations = DummyDepartment::new(
        DepartmentType::Operations,
        vec!["OperationsIntent".to_string()],
        orchestrator.clone(),
    );
    operations.set_config(tenant_id.to_string(), DepartmentConfig { tone_of_voice: "Professional".to_string(), auto_approve_limits: 0.0, requires_review: false });

    let mut support = DummyDepartment::new(
        DepartmentType::CustomerSuccess,
        vec!["CustomerSuccessIntent".to_string()],
        orchestrator.clone(),
    );
    support.set_config(tenant_id.to_string(), DepartmentConfig { tone_of_voice: "Empathetic".to_string(), auto_approve_limits: 0.0, requires_review: true });

    orchestrator.register_department(Arc::new(RwLock::new(marketing))).await;
    orchestrator.register_department(Arc::new(RwLock::new(operations))).await;
    orchestrator.register_department(Arc::new(RwLock::new(support))).await;
}
