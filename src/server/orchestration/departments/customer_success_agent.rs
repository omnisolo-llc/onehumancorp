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
            let payload = &event.payload;
            let original = payload.get("original_payload");
            let message = if let Some(orig) = original {
                orig.get("generated_response").and_then(|v| v.as_str()).unwrap_or("Unknown response")
            } else {
                "Unknown response"
            };
            tracing::info!("EXECUTING APPROVED DRAFT: Sending message: {}", message);

            let content = format!("Sent response to customer: {}", message);

            // Log the action in the agent's memory, handling errors and using proper defaults
            // Assuming we don't have an embedding service here, we use a zero vector
            // but properly await and map the error.
            let record = ohc_builtin_agent::memory_store::EmbeddingRecord {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: event.tenant_id.clone(),
                agent_id: "customer_success_agent".to_string(),
                content,
                embedding: vec![0.0; 1536], // Simple dummy embedding since we don't have an embedder
                source_type: "AGENT_ACTION".to_string(),
                created_at: chrono::Utc::now(),
                last_referenced_at: chrono::Utc::now(),
                reference_count: 0,
                reliability_score: 100,
                owner_override: false,
                metadata: None,
            };
            self.orchestrator.write_long_term_memory(record).await.map_err(|e| e.to_string())?;

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

            let mut final_risk = risk;

            // Intelligent intent parsing using LLM instead of simple string containment
            let prompt = format!(
                "Classify the customer intent from this message. Respond with EXACTLY one of these labels, followed by a colon and the extracted order ID if applicable:
                - ORDER_STATUS_REQUEST: <order_id>
                - VEGAN_INQUIRY
                - GENERAL_INQUIRY

                Message: '{}'", message
            );

            let intent_result = if let Ok(provider) = std::env::var("OHC_LLM_PROVIDER") {
                if provider == "minimax" {
                    let minimax_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                    crate::minimax::MinimaxClient::new(minimax_key).reason(&prompt).await.unwrap_or_else(|_| "GENERAL_INQUIRY".to_string())
                } else {
                    crate::minimax::LocalLLMClient::new().reason(&prompt).await.unwrap_or_else(|_| "GENERAL_INQUIRY".to_string())
                }
            } else {
                crate::minimax::LocalLLMClient::new().reason(&prompt).await.unwrap_or_else(|_| "GENERAL_INQUIRY".to_string())
            };

            let generated_response = if intent_result.starts_with("ORDER_STATUS_REQUEST") {
                final_risk = ActionRisk::AutoExecute;
                let parts: Vec<&str> = intent_result.split(':').collect();
                if parts.len() > 1 {
                    let order_id = parts[1].trim();
                    if order_id.is_empty() || order_id.to_lowercase() == "none" || order_id.to_lowercase() == "null" || order_id == "<order_id>" {
                        "Could you please provide your order number?".to_string()
                    } else {
                        let status = self.orchestrator.get_order_status(&event.tenant_id, order_id).await.unwrap_or(None);
                        if let Some(s) = status {
                            format!("Your order {} is currently: {}.", order_id, s)
                        } else {
                            format!("I couldn't find an active order with ID {}.", order_id)
                        }
                    }
                } else {
                    "Could you please provide your order number?".to_string()
                }
            } else if intent_result.starts_with("VEGAN_INQUIRY") || (message.to_lowercase().contains("vegan") && context_summary.to_lowercase().contains("vegan")) {
                "Yes, we do vegan cakes!".to_string()
            } else {
                "Thank you for your message. We will get back to you shortly.".to_string()
            };

            let description = if final_risk == ActionRisk::AutoExecute {
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
                final_risk,
                action_payload,
            ).await.map(|_| ())?;

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
