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
            "tenant.escalation.resolved".to_string(),
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

        if event.event_type == "tenant.escalation.resolved" {
            let human_response = event.payload.get("human_response").and_then(|v| v.as_str()).unwrap_or("");
            let original_message = event.payload.get("original_message").and_then(|v| v.as_str()).unwrap_or("");

            tracing::info!("LEARNING FROM ESCALATION: Human responded with: {}", human_response);

            let content = format!("Learned from manual escalation. For customer inquiry: '{}', the owner responded: '{}'. Will use this context for future identical edge-cases.", original_message, human_response);

            let record = ohc_builtin_agent::memory_store::EmbeddingRecord {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: event.tenant_id.clone(),
                agent_id: "customer_success_agent".to_string(),
                content,
                embedding: vec![0.0; 1536],
                source_type: "HUMAN_ESCALATION_OVERRIDE".to_string(),
                created_at: chrono::Utc::now(),
                last_referenced_at: chrono::Utc::now(),
                reference_count: 0,
                reliability_score: 100,
                owner_override: true,
                metadata: None,
            };
            self.orchestrator.write_long_term_memory(record).await.map_err(|e| e.to_string())?;

            // We simulate the API returning control to AI
            return Ok(());
        }

        if event.event_type == "tenant.message.received" {
            let message = event.payload.get("message").and_then(|v| v.as_str()).unwrap_or("");

            let is_escalation = message.to_lowercase().contains("custom") ||
                                message.to_lowercase().contains("owner") ||
                                message.to_lowercase().contains("human") ||
                                message.to_lowercase().contains("complex") ||
                                message.to_lowercase().contains("photo");

            // Dummy query embedding for simulation
            let query_embedding = vec![0.5; 1536];
            let memories = self.orchestrator.query_long_term_memory(&event.tenant_id, &query_embedding, 5).await.unwrap_or_default();

            let context_summary = if !memories.is_empty() {
                memories.join("
")
            } else {
                "No relevant memory found.".to_string()
            };

            let generated_response = if is_escalation {
                "That sounds amazing! Let me double-check the specifics for you really quick."
            } else if message.to_lowercase().contains("vegan") && context_summary.to_lowercase().contains("vegan") {
                "Yes, we do vegan cakes!"
            } else {
                "Thank you for your message. We will get back to you shortly."
            };

            let description = if is_escalation {
                format!("Escalation: Action Required for complex request: '{}'", message)
            } else if risk == ActionRisk::AutoExecute {
                format!("Auto-replied to message: '{}' with '{}'", message, generated_response)
            } else {
                "Draft email for review".to_string()
            };

            let action_payload = serde_json::json!({
                "feature_type": if is_escalation { "ambassador_escalation" } else { "ambassador_reply" },
                "original_message": message,
                "generated_response": generated_response,
                "context_used": context_summary,
                "is_escalation": is_escalation,
                "best_guess_draft": "We can certainly help with this custom request. Let me know the exact details.",
            });

            // If it's an escalation, we could notify via push notification service here.
            // For now, we execute the action which surfaces it to the owner's mobile feed.
            self.orchestrator.execute_action(
                DepartmentType::CustomerSuccess,
                description,
                event.tenant_id.clone(),
                if is_escalation { ActionRisk::DraftForReview } else { risk },
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
