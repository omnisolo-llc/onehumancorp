use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;

pub struct SalesAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl SalesAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait::async_trait]
impl Department for SalesAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Sales
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec!["tenant.quote.requested".to_string(), "tenant.message.received".to_string()]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        let risk = ActionRisk::DraftForReview;

        if event.event_type == "tenant.message.received" {
            let message = event.payload.get("message").and_then(|v| v.as_str()).unwrap_or("").to_lowercase();
            if message.contains("fix") || message.contains("tomorrow") || message.contains("plumbing") {
                let service = self.orchestrator.get_service_by_name_like(&event.tenant_id, "Plumbing Fix").await?;
                let (service_name, price) = service.unwrap_or(("Plumbing Fix".to_string(), 75.0));

                let drafted_message = format!("Hi! Yes, I have an opening tomorrow at 2 PM. The base callout fee is ${}. Would you like me to book this slot?", price);

                ::server_telemetry::record_business_event(&event.tenant_id, ::server_telemetry::get_deployment_mode(), "quote_generated_from_message");

                let action_payload = serde_json::json!({
                    "feature_type": "sales_quote",
                    "original_message": message,
                    "generated_quote": drafted_message,
                    "service": service_name,
                    "price": price,
                });

                self.orchestrator.execute_action(
                    DepartmentType::Sales,
                    format!("Draft quote for {}", service_name),
                    event.tenant_id.clone(),
                    risk,
                    action_payload,
                ).await.map(|_| ())?;
                return Ok(());
            }
        }

        // Query memory context
        let query_embedding = vec![0.5, 0.5, 0.5]; // Mock embedding
        let _context = self.orchestrator.query_long_term_memory(&event.tenant_id, &query_embedding, 5).await?;

        ::server_telemetry::record_business_event(&event.tenant_id, ::server_telemetry::get_deployment_mode(), "quote_generated");

        self.orchestrator.execute_action(
            DepartmentType::Sales,
            "Quote generated for review".to_string(),
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
        let embedding = vec![0.5, 0.5, 0.5];
        // Note: We need a tenant_id here, but the trait signature doesn't provide one.
        // We'll pass a dummy one or extract it if available.
        self.orchestrator.query_long_term_memory("default_tenant", &embedding, 5).await
    }

    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String> {
        self.orchestrator.execute_action(self.department_type(), description.clone(), tenant_id.clone(), risk, serde_json::json!({})).await
    }
}

#[async_trait::async_trait]
impl BaseAgent for SalesAgent {
    fn agent_id(&self) -> String {
        "sales_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }

    async fn execute(&self, _payload: Value) -> Result<(), String> {
        Ok(())
    }
}
