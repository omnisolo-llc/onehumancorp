use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;

pub struct ReceptionistAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl ReceptionistAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait::async_trait]
impl Department for ReceptionistAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Receptionist
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.message.received".to_string(), // For chat/text booking requests
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        let risk = ActionRisk::DraftForReview; // Booking usually needs approval, unless configured otherwise

        if event.event_type == "tenant.message.received" {
            let message = event.payload.get("message").and_then(|v| v.as_str()).unwrap_or("");

            // Check for booking/quote intent
            let is_booking_request = message.to_lowercase().contains("book") ||
                                     message.to_lowercase().contains("appointment") ||
                                     message.to_lowercase().contains("schedule") ||
                                     message.to_lowercase().contains("quote") ||
                                     message.to_lowercase().contains("price");

            if is_booking_request {
                let generated_response = "I can help you book that! I've prepared a quote and available times for your review.";

                let description = "Draft service quote and booking request for review".to_string();

                let action_payload = serde_json::json!({
                    "feature_type": "service_quote",
                    "original_message": message,
                    "generated_response": generated_response,
                    "estimated_price": "$100", // Placeholder for actual estimation logic
                    "proposed_time": "Tuesday 10 AM", // Placeholder
                });

                self.orchestrator.execute_action(
                    DepartmentType::Receptionist,
                    description,
                    event.tenant_id.clone(),
                    risk,
                    action_payload,
                ).await.map(|_| ())?;
            }
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

#[async_trait::async_trait]
impl BaseAgent for ReceptionistAgent {
    fn agent_id(&self) -> String {
        "receptionist_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }

    async fn execute(&self, _payload: Value) -> Result<(), String> {
        Ok(())
    }
}
