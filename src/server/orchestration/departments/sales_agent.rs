use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;
use crate::services::booking::BookingService;
use uuid::Uuid;

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
        vec!["tenant.quote.requested".to_string()]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        // Query memory context
        let query_embedding = vec![0.5, 0.5, 0.5]; // Mock embedding
        let _context = self.orchestrator.query_long_term_memory(&event.tenant_id, &query_embedding, 5).await?;

        let risk = ActionRisk::DraftForReview;
        let mut action_payload = event.payload.clone();

        // Process autonomous quote generation if requested
        let is_quote_request = event.event_type == "tenant.quote.requested"
            || action_payload.get("action").and_then(|v| v.as_str()) == Some("draft_quote");

        if is_quote_request {
            let tenant_uuid = Uuid::parse_str(&event.tenant_id).unwrap_or_else(|_| Uuid::new_v4());
            let customer_uuid = Uuid::new_v4(); // Generate dummy customer UUID for now
            let amount_cents = 15000; // $150.00 default estimate for a service

            // Create the draft quote in the system using BookingService
            let quote = BookingService::create_draft_quote(tenant_uuid, customer_uuid, amount_cents);

            // Enrich the payload with the quote details for the UI
            if let Some(obj) = action_payload.as_object_mut() {
                obj.insert("quote_id".to_string(), Value::String(quote.id.to_string()));
                obj.insert("amount".to_string(), Value::String(format!("${:.2}", amount_cents as f64 / 100.0)));
                obj.insert("description".to_string(), Value::String("Service Quote Estimate".to_string()));
            }
        }

        ::server_telemetry::record_business_event(&event.tenant_id, ::server_telemetry::get_deployment_mode(), "quote_generated");

        self.orchestrator.execute_action(
            DepartmentType::Sales,
            "Quote generated for review".to_string(),
            event.tenant_id.clone(),
            risk,
            action_payload,
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
