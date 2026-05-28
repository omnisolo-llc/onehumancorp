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
        let risk = if let Some(cfg) = &config {
            if cfg.auto_approve_limits > 0.0 {
                ActionRisk::AutoExecute
            } else {
                ActionRisk::DraftForReview
            }
        } else {
            ActionRisk::DraftForReview
        };

        if event.event_type == "agent:customer_success:approved" {
            // Actual logic to send the message when approved.
            // For now, we simulate sending the message.
            tracing::info!("Simulating sending approved message for tenant");
            return Ok(());
        }

                if event.event_type == "tenant.message.received" {
            let message = event.payload.get("message").and_then(|v| v.as_str()).unwrap_or("");
            let customer_id = event.payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("unknown");

            // Query Unified Customer Timeline
            let timeline = self.orchestrator.get_customer_timeline(&event.tenant_id, customer_id).await.unwrap_or_default();

            // Dummy query embedding for simulation
            let query_embedding = vec![0.5; 1536];
            let memories = self.orchestrator.query_long_term_memory(&event.tenant_id, &query_embedding, 5).await.unwrap_or_default();

            let mut context_summary = String::new();
            if !timeline.is_empty() {
                context_summary.push_str("Recent Activity:\n");
                context_summary.push_str(&timeline.join("\n"));
                context_summary.push_str("\n\n");
            }
            if !memories.is_empty() {
                context_summary.push_str("Memory:\n");
                context_summary.push_str(&memories.join("\n"));
            }
            if context_summary.is_empty() {
                context_summary = "No relevant memory or timeline found.".to_string();
            }

            // We fall back to standard prompt logic, but use minimax logic if configured in the webhook payload, or just use a standard check for now.
            // For a production system this would use an LLM API to generate a response based on the message and the unified context_summary.
            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
            let generated_response = if !api_key.is_empty() {
                let prompt = format!(
                    "You are a friendly agent for a small business. Here is the context about this customer: {}. Draft a concise, warm reply to their message: '{}'",
                    context_summary, message
                );
                let client = crate::minimax::MinimaxClient::new(api_key);
                client.reason(&prompt).await.unwrap_or_else(|_| "Thank you for reaching out! We will get back to you shortly.".to_string())
            } else {
                if message.to_lowercase().contains("vegan") || context_summary.to_lowercase().contains("vegan") {
                    "Yes, we do vegan cakes! Would you like me to reserve one?".to_string()
                } else if message.to_lowercase().contains("reserve") || message.to_lowercase().contains("order") {
                    "Great! Here is the link to complete your order: [Link]".to_string()
                } else {
                    "Thank you for your message. We will get back to you shortly.".to_string()
                }
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
                description.clone(),
                event.tenant_id.clone(),
                risk.clone(),
                action_payload.clone(),
            ).await?;

            if risk == ActionRisk::AutoExecute {
                let _ = self.orchestrator.log_customer_action(
                    &event.tenant_id,
                    customer_id,
                    "The Ambassador",
                    "auto_reply",
                    &description,
                    action_payload,
                ).await;
            }

            return Ok(());
        }

        self.orchestrator.execute_action(
            DepartmentType::CustomerSuccess,
            "Send personalized thank you & shipping ETA".to_string(),
            event.tenant_id.clone(),
            risk,
            event.payload.clone(),
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
