use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;
use std::collections::HashMap;

pub struct SalesAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
    configs: HashMap<String, DepartmentConfig>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuoteIntent {
    pub original_message: String,
    pub service_name: String,
}

impl SalesAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self {
            orchestrator,
            configs: HashMap::new(),
        }
    }
}

pub fn extract_quote_intent(payload: &Value) -> Option<QuoteIntent> {
    let original_message = payload.get("message").and_then(|v| v.as_str()).unwrap_or("").trim();

    if let Some(llm_intent) = payload.get("llm_intent") {
        let intent = llm_intent.get("intent").and_then(|v| v.as_str()).unwrap_or("");
        let confidence = llm_intent.get("confidence").and_then(|v| v.as_f64()).unwrap_or(0.0);
        if intent.eq_ignore_ascii_case("quote") && confidence >= 0.7 {
            let service_name = llm_intent
                .get("service_name")
                .and_then(|v| v.as_str())
                .filter(|name| !name.trim().is_empty())
                .unwrap_or("Plumbing Fix");
            return Some(QuoteIntent {
                original_message: original_message.to_string(),
                service_name: service_name.to_string(),
            });
        }
    }

    let message = original_message.to_lowercase();
    if message.contains("fix") || message.contains("tomorrow") || message.contains("plumbing") {
        return Some(QuoteIntent {
            original_message: original_message.to_string(),
            service_name: "Plumbing Fix".to_string(),
        });
    }

    None
}

pub fn risk_for_service_price(config: Option<&DepartmentConfig>, price: f64) -> ActionRisk {
    let Some(config) = config else {
        return ActionRisk::DraftForReview;
    };

    if config.auto_approve_limits > 0.0 && price <= config.auto_approve_limits {
        ActionRisk::AutoExecute
    } else {
        ActionRisk::DraftForReview
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
        if event.event_type == "tenant.message.received" {
            if let Some(intent) = extract_quote_intent(&event.payload) {
                let service = self.orchestrator.get_service_by_name_like(&event.tenant_id, &intent.service_name).await?;
                let (service_name, price) = service.unwrap_or((intent.service_name, 75.0));
                let risk = risk_for_service_price(self.get_config(&event.tenant_id).as_ref(), price);

                let drafted_message = format!("Hi! Yes, I have an opening tomorrow at 2 PM. The base callout fee is ${}. Would you like me to book this slot?", price);

                ::server_telemetry::record_business_event(&event.tenant_id, ::server_telemetry::get_deployment_mode(), "quote_generated_from_message");

                let action_payload = serde_json::json!({
                    "feature_type": "quote_draft",
                    "customer_inquiry": intent.original_message,
                    "suggested_price": price,
                    "scope": format!("{} including labor and standard materials.", service_name),
                    "suggested_time": "Tomorrow at 2 PM",
                    "generated_response": drafted_message,
                    "service": service_name.clone(),
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
            ActionRisk::DraftForReview,
            event.payload.clone(),
        ).await.map(|_| ())
    }

    fn get_config(&self, _tenant_id: &str) -> Option<DepartmentConfig> {
        self.configs.get(_tenant_id).cloned()
    }

    fn set_config(&mut self, _tenant_id: String, _config: DepartmentConfig) {
        self.configs.insert(_tenant_id, _config);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestration::departments::types::DepartmentConfig;

    #[test]
    fn llm_quote_intent_detects_service_without_keyword_fallback() {
        let payload = serde_json::json!({
            "message": "Can someone come by around 2 PM?",
            "llm_intent": {
                "intent": "quote",
                "service_name": "Plumbing Fix",
                "confidence": 0.91
            }
        });

        let intent = extract_quote_intent(&payload).expect("LLM intent should request a quote");

        assert_eq!(intent.service_name, "Plumbing Fix");
        assert_eq!(intent.original_message, "Can someone come by around 2 PM?");
    }

    #[test]
    fn dynamic_service_price_controls_action_risk() {
        let config = DepartmentConfig {
            tone_of_voice: "friendly".to_string(),
            auto_approve_limits: 100.0,
        };

        assert_eq!(
            risk_for_service_price(Some(&config), 75.0),
            ActionRisk::AutoExecute
        );
        assert_eq!(
            risk_for_service_price(Some(&config), 175.0),
            ActionRisk::DraftForReview
        );
        assert_eq!(
            risk_for_service_price(None, 1.0),
            ActionRisk::DraftForReview
        );
    }
}
