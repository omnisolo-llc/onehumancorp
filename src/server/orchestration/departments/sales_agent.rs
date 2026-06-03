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
        vec!["tenant.quote.requested".to_string()]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        // Query memory context
        let query_embedding = vec![0.5, 0.5, 0.5]; // Mock embedding
        let _context = self.orchestrator.query_long_term_memory(&event.tenant_id, &query_embedding, 5).await?;

        let risk = ActionRisk::DraftForReview;

        ::server_telemetry::record_business_event(&event.tenant_id, ::server_telemetry::get_deployment_mode(), "quote_generated");

        let inquiry = event.payload.get("message").and_then(|v| v.as_str()).unwrap_or("Unknown inquiry");

        let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
        let payload = if !api_key.is_empty() {
            let client = crate::minimax::MinimaxClient::new(api_key);
            let prompt = format!(
                "You are an AI sales agent. You need to analyze this inquiry and return a valid JSON object containing exactly three string fields: \"suggested_price\" (e.g. \"150\"), \"scope\" (a short description of the work needed), and \"suggested_time\" (a time to propose, e.g. \"Tue 2 PM\"). Inquiry: {}",
                inquiry
            );
            let compressed_prompt = ::server_pricing::compression::reduce_tokens(&prompt);
            match client.reason(&compressed_prompt).await {
                Ok(response) => {
                    let trimmed = response.trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(trimmed) {
                        let price = json.get("suggested_price").and_then(|v| v.as_str()).unwrap_or("150");
                        let scope = json.get("scope").and_then(|v| v.as_str()).unwrap_or("Fix leaky kitchen pipe including parts and labor.");
                        let time = json.get("suggested_time").and_then(|v| v.as_str()).unwrap_or("Tue 2 PM");
                        serde_json::json!({
                            "feature_type": "quote_draft",
                            "customer_inquiry": inquiry,
                            "suggested_price": price,
                            "scope": scope,
                            "suggested_time": time,
                        })
                    } else {
                        serde_json::json!({
                            "feature_type": "quote_draft",
                            "customer_inquiry": inquiry,
                            "suggested_price": "150",
                            "scope": "Fix leaky kitchen pipe including parts and labor.",
                            "suggested_time": "Tue 2 PM",
                        })
                    }
                }
                Err(_) => {
                    serde_json::json!({
                        "feature_type": "quote_draft",
                        "customer_inquiry": inquiry,
                        "suggested_price": "150",
                        "scope": "Fix leaky kitchen pipe including parts and labor.",
                        "suggested_time": "Tue 2 PM",
                    })
                }
            }
        } else {
            serde_json::json!({
                "feature_type": "quote_draft",
                "customer_inquiry": inquiry,
                "suggested_price": "150",
                "scope": "Fix leaky kitchen pipe including parts and labor.",
                "suggested_time": "Tue 2 PM",
            })
        };

        self.orchestrator.execute_action(
            DepartmentType::Sales,
            "Quote generated for review".to_string(),
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
        let embedding = vec![0.5, 0.5, 0.5];
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
