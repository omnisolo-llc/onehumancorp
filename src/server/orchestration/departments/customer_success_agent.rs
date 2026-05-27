use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;
use std::collections::HashMap;

pub struct CustomerSuccessAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
    configs: HashMap<String, DepartmentConfig>,
}

impl CustomerSuccessAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self {
            orchestrator,
            configs: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl Department for CustomerSuccessAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::CustomerSuccess
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.order.fulfillment_ready".to_string(),
            "tenant.message.received".to_string(),
            "agent:customer_success:approved".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        let config = self.get_config(&event.tenant_id);
        let risk = ActionRisk::AutoExecute;

        if event.event_type == "agent:customer_success:approved" {
            // Actual logic to send the message when approved.
            // For now, we simulate sending the message.
            tracing::info!("Simulating sending approved message for tenant");
            return Ok(());
        }

        if event.event_type == "tenant.message.received" {
            let message = event.payload.get("message").and_then(|v| v.as_str()).unwrap_or("");

            // Dummy query embedding for simulation
            let query_embedding = vec![0.5; 1536];
            let memories = self.orchestrator.query_long_term_memory(&event.tenant_id, &query_embedding, 5).await.unwrap_or_default();

            let context_summary = if !memories.is_empty() {
                memories.join("\n")
            } else {
                "No relevant memory found.".to_string()
            };

            let generated_response = if message.to_lowercase().contains("vegan") && context_summary.to_lowercase().contains("vegan") {
                "Yes, we do vegan cakes!"
            } else {
                "Thank you for your message. We will get back to you shortly."
            };

            let description = if risk == ActionRisk::AutoExecute {
                format!("Auto-replied to message: '{}' with '{}'", message, generated_response)
            } else {
                "Draft email for review".to_string()
            };

            let action_payload = serde_json::json!({
                "feature_type": "ambassador_reply",
                "original_message": message,
                "generated_response": generated_response,
                "context_used": context_summary,
            });

            self.orchestrator.execute_action(
                DepartmentType::CustomerSuccess,
                description,
                event.tenant_id.clone(),
                risk,
                action_payload,
            ).await.map(|_| ())?;

            return Ok(());
        }

        let mut drafted_msg = "Hi! Your order has been processed and is being prepared for shipment. Thank you!".to_string();

        let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
        if !api_key.is_empty() {
            let prompt = format!(
                "You are the customer success ambassador. Draft a helpful and polite reply to confirm a new order. Order details: '{}'. Keep it concise and professional.",
                event.payload
            );
            let client = crate::minimax::MinimaxClient::new(api_key);
            if let Ok(content) = client.reason(&prompt).await {
                if !content.is_empty() {
                    drafted_msg = content;
                }
            }
        }

        let mut payload = event.payload.clone();
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("drafted_message".to_string(), serde_json::Value::String(drafted_msg));
        }

        self.orchestrator.execute_action(
            DepartmentType::CustomerSuccess,
            "Send personalized thank you & shipping ETA".to_string(),
            event.tenant_id.clone(),
            risk,
            payload,
        ).await.map(|_| ())
    }

    fn get_config(&self, tenant_id: &str) -> Option<DepartmentConfig> {
        self.configs.get(tenant_id).cloned()
    }

    fn set_config(&mut self, tenant_id: String, config: DepartmentConfig) {
        self.configs.insert(tenant_id, config);
    }

    async fn query_memory(&self, _query: &str) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String> {
        self.orchestrator.execute_action(self.department_type(), description.clone(), tenant_id.clone(), risk, serde_json::json!({})).await
    }
}

#[async_trait::async_trait]
impl BaseAgent for CustomerSuccessAgent {
    fn agent_id(&self) -> String {
        "customer_success_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }

    async fn execute(&self, _payload: Value) -> Result<(), String> {
        Ok(())
    }
}
