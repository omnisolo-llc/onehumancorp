use crate::orchestration::departments::orchestrator::{
    AgentTriggerType, BaseAgent, Department, DepartmentOrchestrator,
};
use crate::orchestration::departments::types::{
    ActionRisk, ApprovalRequest, DepartmentConfig, DepartmentEvent, DepartmentType,
};
use serde_json::Value;
use std::collections::HashMap;

// Note: In actual implementation the OutboundDispatcher and ChatClient would be full featured modules
// Since we are mocking them here temporarily to not break the build dependencies, they remain here but
// should be injected via config in a real app.
#[async_trait::async_trait]
pub trait ChatClient: Send + Sync {
    async fn generate_response(&self, prompt: &str) -> Result<(String, f64), String>;
}

pub struct DummyGeminiClient {}

#[async_trait::async_trait]
impl ChatClient for DummyGeminiClient {
    async fn generate_response(&self, prompt: &str) -> Result<(String, f64), String> {
        let msg_lower = prompt.to_lowercase();

        if msg_lower.contains("vegan") {
            Ok(("Yes, we do vegan cakes!".to_string(), 95.0))
        } else if msg_lower.contains("status") {
            Ok(("Your order is currently processing.".to_string(), 92.0))
        } else {
            Ok(("Thank you for your message. We will get back to you shortly.".to_string(), 85.0))
        }
    }
}

// Add an outbound message dispatcher trait
#[async_trait::async_trait]
pub trait OutboundDispatcher: Send + Sync {
    async fn send_message(&self, tenant_id: &str, platform: &str, recipient: &str, content: &str) -> Result<(), String>;
}

pub struct DefaultOutboundDispatcher {}

#[async_trait::async_trait]
impl OutboundDispatcher for DefaultOutboundDispatcher {
    async fn send_message(&self, tenant_id: &str, platform: &str, _recipient: &str, content: &str) -> Result<(), String> {
        tracing::info!("[OUTBOUND_DISPATCHER] Sending message for tenant {} on platform {}: {}", tenant_id, platform, content);
        Ok(())
    }
}

pub struct CustomerSuccessAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
    configs: HashMap<String, DepartmentConfig>,
    llm: std::sync::Arc<dyn ChatClient>,
    dispatcher: std::sync::Arc<dyn OutboundDispatcher>,
}

impl CustomerSuccessAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        // Ideally we would pull the real gemini client or similar here, using dummy for current scope implementation
        Self {
            orchestrator,
            configs: HashMap::new(),
            llm: std::sync::Arc::new(DummyGeminiClient {}),
            dispatcher: std::sync::Arc::new(DefaultOutboundDispatcher {}),
        }
    }

    #[allow(dead_code)]
    pub fn with_deps(
        orchestrator: std::sync::Arc<DepartmentOrchestrator>,
        llm: std::sync::Arc<dyn ChatClient>,
        dispatcher: std::sync::Arc<dyn OutboundDispatcher>
    ) -> Self {
        Self {
            orchestrator,
            configs: HashMap::new(),
            llm,
            dispatcher,
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

        if event.event_type == "agent:customer_success:approved" {
            let payload = &event.payload;
            let original = payload.get("original_payload");

            let message = if let Some(orig) = original {
                orig.get("generated_response")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown response")
            } else {
                "Unknown response"
            };

            let platform = if let Some(orig) = original {
                orig.get("platform")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
            } else {
                "unknown"
            };

            tracing::info!("EXECUTING APPROVED DRAFT: Sending message to platform {}: {}", platform, message);

            // Execute the outbound message sending
            if let Err(e) = self.dispatcher.send_message(&event.tenant_id, platform, "customer", message).await {
                tracing::error!("Failed to send outbound message: {}", e);
            }

            let content = format!("Sent response to customer: {}", message);

            let record = ohc_builtin_agent::memory_store::EmbeddingRecord {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: event.tenant_id.clone(),
                agent_id: "customer_success_agent".to_string(),
                content,
                embedding: vec![0.0; 1536],
                source_type: "AGENT_ACTION".to_string(),
                created_at: chrono::Utc::now(),
                last_referenced_at: chrono::Utc::now(),
                reference_count: 0,
                reliability_score: 100,
                owner_override: false,
                metadata: None,
            };
            self.orchestrator
                .write_long_term_memory(record)
                .await
                .map_err(|e| e.to_string())?;

            return Ok(());
        }

        if event.event_type == "tenant.message.received" {
            let message = event
                .payload
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let platform = event
                .payload
                .get("platform")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");

            let attachments = event.payload.get("attachments").and_then(|a| a.as_array());
            let has_images = attachments.map_or(false, |arr| {
                arr.iter().any(|item| {
                    item.get("media_type").and_then(|t| t.as_str()).map_or(false, |t| t.starts_with("image/"))
                })
            });

            // Dummy query embedding for simulation
            let query_embedding = vec![0.5; 1536];
            let memories = self
                .orchestrator
                .query_long_term_memory(&event.tenant_id, &query_embedding, 5)
                .await
                .unwrap_or_default();

            let context_summary = if !memories.is_empty() {
                memories.join("\n")
            } else {
                "No relevant memory found.".to_string()
            };

            let tone = config.clone().map(|c| c.tone_of_voice).unwrap_or_else(|| "professional and helpful".to_string());

            let mut full_prompt = format!("You are an AI customer success ambassador. Tone: {}. Context: {}. Message: {}", tone, context_summary, message);
            if has_images {
                full_prompt.push_str("\nNote: Image attachments were included in the message. Analyze them and incorporate into the response.");
            }

            let (generated_response, confidence_score) = match self.llm.generate_response(&full_prompt).await {
                Ok((resp, conf)) => (resp, conf),
                Err(e) => {
                    tracing::error!("LLM generation failed: {}", e);
                    ("Thank you for your message. We will get back to you shortly.".to_string(), 85.0)
                }
            };

            let risk = if confidence_score >= 90.0 {
                ActionRisk::AutoExecute
            } else {
                ActionRisk::DraftForReview
            };

            let description = if risk == ActionRisk::AutoExecute {
                format!(
                    "Auto-replied to message on {}: '{}' with '{}'",
                    platform, message, generated_response
                )
            } else {
                format!("Draft reply for review on {}", platform)
            };

            let action_payload = serde_json::json!({
                "feature_type": "ambassador_reply",
                "platform": platform,
                "original_message": message,
                "generated_response": generated_response,
                "context_used": context_summary,
                "confidence_score": confidence_score
            });

            self.orchestrator
                .execute_action(
                    DepartmentType::CustomerSuccess,
                    description,
                    event.tenant_id.clone(),
                    risk,
                    action_payload,
                )
                .await
                .map(|_| ())?;

            return Ok(());
        }

        let risk = if let Some(cfg) = &config {
            if cfg.auto_approve_limits > 0.0 {
                ActionRisk::AutoExecute
            } else {
                ActionRisk::DraftForReview
            }
        } else {
            ActionRisk::DraftForReview
        };

        self.orchestrator
            .execute_action(
                DepartmentType::CustomerSuccess,
                "Send personalized thank you & shipping ETA".to_string(),
                event.tenant_id.clone(),
                risk,
                event.payload.clone(),
            )
            .await
            .map(|_| ())
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

    async fn request_approval(
        &self,
        description: String,
        tenant_id: String,
        risk: ActionRisk,
    ) -> Result<ApprovalRequest, String> {
        self.orchestrator
            .execute_action(
                self.department_type(),
                description.clone(),
                tenant_id.clone(),
                risk,
                serde_json::json!({}),
            )
            .await
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
