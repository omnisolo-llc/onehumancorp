use std::sync::Arc;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{DepartmentType, ActionRisk};
use serde_json::Value;

pub struct ActionDispatcher {
    orchestrator: Arc<DepartmentOrchestrator>,
}

impl ActionDispatcher {
    pub fn new(orchestrator: Arc<DepartmentOrchestrator>) -> Self {
        Self {
            orchestrator,
        }
    }

    pub async fn dispatch_action(
        &self,
        tenant_id: String,
        action_name: String,
        payload_json: String,
    ) -> Result<::server_ohc::interop::DispatchActionResponse, String> {
        // Parse the incoming JSON payload
        let payload: Value = match serde_json::from_str(&payload_json) {
            Ok(p) => p,
            Err(e) => return Err(format!("Invalid JSON payload: {}", e)),
        };

        // For now, map action_name loosely to departments. In a full implementation, we'd use SemanticRouter
        // or a strict capability map.
        let department = match action_name.as_str() {
            "create_product" | "update_inventory" => DepartmentType::Operations,
            "draft_email" | "draft_customer_message" => DepartmentType::CustomerSuccess,
            "issue_refund" | "request_payment" => DepartmentType::Finance,
            "update_booking" | "schedule_visit" => DepartmentType::Operations,
            "prepare_quote" | "quote_draft" => DepartmentType::Sales,
            _ => DepartmentType::Operations,
        };

        let description = format!("Action: {}", action_name);

        match self.orchestrator.execute_action(
            department.clone(),
            description.clone(),
            tenant_id,
            ActionRisk::DraftForReview,
            payload,
        ).await {
            Ok(approval_req) => Ok(::server_ohc::interop::DispatchActionResponse {
                approval_request_id: approval_req.id,
                status: "pending".to_string(),
                department_assigned: department.to_string(),
                description,
            }),
            Err(e) => Err(format!("Failed to execute action: {}", e)),
        }
    }
}
