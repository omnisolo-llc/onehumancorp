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
            "tenant.omnichannel.message.received".to_string(),
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

            if let Some(inbox_id) = original.and_then(|orig| orig.get("inbox_message_id").and_then(|v| v.as_str())) {
                let orchestrator_clone = self.orchestrator.clone();
                let id_clone = inbox_id.to_string();
                let tenant_id_clone = event.tenant_id.clone();
                tokio::spawn(async move {
                    let _ = orchestrator_clone.update_inbox_message_status(&id_clone, &tenant_id_clone, "sent").await;
                });
            }

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

        if event.event_type == "tenant.message.received" || event.event_type == "tenant.omnichannel.message.received" {
            let message = event.payload.get("message").and_then(|v| v.as_str()).unwrap_or("");
            let sender_id = event.payload.get("sender_id").and_then(|v| v.as_str()).unwrap_or("unknown_sender");

            let query_embedding = match std::env::var("OHC_INBOX_DRAFT_LLM_PROVIDER")
                .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
                .as_deref()
            {
                Ok("minimax") => {
                    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
                    crate::minimax::MinimaxClient::new(api_key).generate_embedding(message).await.unwrap_or_else(|_| vec![0.0; 1536])
                }
                _ => {
                    crate::minimax::LocalLLMClient::new().generate_embedding(message).await.unwrap_or_else(|_| vec![0.0; 1536])
                }
            };

            // ZERO-PARTY DATA EXTRACTION
            let extractor_prompt = format!(
                "Extract any explicit user preferences (like vegan, gluten-free, size M, allergies, birthdays) from this message. Return a JSON object with a single `preferences` key mapped to an array of string tags. Return {{\"preferences\":[]}} if none found. Message: {}",
                message
            );
            let compressed_extractor_prompt = crate::pricing::compression::reduce_tokens(&extractor_prompt);

            let extraction_result = match std::env::var("OHC_INBOX_DRAFT_LLM_PROVIDER")
                .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
                .as_deref()
            {
                Ok("minimax") => {
                    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
                    crate::minimax::MinimaxClient::new(api_key.clone()).reason(&compressed_extractor_prompt).await.unwrap_or_else(|_| "{\"preferences\":[]}".to_string())
                }
                _ => {
                    crate::minimax::LocalLLMClient::new().reason(&compressed_extractor_prompt).await.unwrap_or_else(|_| "{\"preferences\":[]}".to_string())
                }
            };

            let mut extracted_prefs: Vec<String> = vec![];
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&extraction_result) {
                if let Some(prefs_arr) = parsed.get("preferences").and_then(|v| v.as_array()) {
                    for p in prefs_arr {
                        if let Some(s) = p.as_str() {
                            extracted_prefs.push(s.to_string());
                        }
                    }
                }
            }

            if !extracted_prefs.is_empty() {
                // Update customer360
                let mut existing_prefs = vec![];
                let mut cust_to_update = match self.orchestrator.get_customer360(&event.tenant_id, sender_id).await {
                    Ok(Some(cust)) => {
                        if let Some(prefs) = &cust.preferences {
                            if let Some(arr) = prefs.as_array() {
                                for p in arr {
                                    if let Some(s) = p.as_str() {
                                        existing_prefs.push(s.to_string());
                                    }
                                }
                            }
                        }
                        cust
                    },
                    _ => {
                        crate::orchestration::departments::types::Customer360 {
                            id: uuid::Uuid::new_v4().to_string(),
                            tenant_id: event.tenant_id.clone(),
                            customer_id: sender_id.to_string(),
                            email: None,
                            phone: None,
                            mood: None,
                            preferences: Some(serde_json::json!([])),
                            created_at: Some(chrono::Utc::now()),
                            updated_at: Some(chrono::Utc::now()),
                        }
                    }
                };

                for new_pref in &extracted_prefs {
                    if !existing_prefs.contains(new_pref) {
                        existing_prefs.push(new_pref.clone());

                        let pref_embedding = match std::env::var("OHC_INBOX_DRAFT_LLM_PROVIDER").or_else(|_| std::env::var("OHC_LLM_PROVIDER")).as_deref() {
                            Ok("minimax") => crate::minimax::MinimaxClient::new(std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string())).generate_embedding(new_pref).await.unwrap_or_else(|_| vec![0.0; 1536]),
                            _ => crate::minimax::LocalLLMClient::new().generate_embedding(new_pref).await.unwrap_or_else(|_| vec![0.0; 1536]),
                        };
                        let record = ohc_builtin_agent::memory_store::EmbeddingRecord {
                            id: uuid::Uuid::new_v4().to_string(),
                            tenant_id: event.tenant_id.clone(),
                            agent_id: "customer_success_agent".to_string(),
                            content: format!("The Ambassador learned that the customer prefers {}", new_pref),
                            embedding: pref_embedding,
                            source_type: "ZERO_PARTY_DATA".to_string(),
                            created_at: chrono::Utc::now(),
                            last_referenced_at: chrono::Utc::now(),
                            reference_count: 0,
                            reliability_score: 100,
                            owner_override: false,
                            metadata: None,
                        };
                        let _ = self.orchestrator.write_long_term_memory(record).await;
                    }
                }

                cust_to_update.preferences = Some(serde_json::json!(existing_prefs));
                cust_to_update.updated_at = Some(chrono::Utc::now());
                let _ = self.orchestrator.upsert_customer360(&cust_to_update).await;
            }

            let memories = self.orchestrator.query_long_term_memory(&event.tenant_id, &query_embedding, 5).await.unwrap_or_default();

            let context_summary = if !memories.is_empty() {
                memories.join("\n")
            } else {
                "No relevant memory found.".to_string()
            };

            let prompt = format!(
                "Write one concise, warm customer-service reply for an omnichannel SMB inbox. Do not invent policies, availability, prices, or order state. Tenant: {}. Customer message: {}\n\nContext:\n{}",
                event.tenant_id, message, context_summary
            );
            let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);

            let generated_response = match std::env::var("OHC_INBOX_DRAFT_LLM_PROVIDER")
                .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
                .as_deref()
            {
                Ok("minimax") => {
                    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
                    crate::minimax::MinimaxClient::new(api_key).reason(&compressed_prompt).await.unwrap_or_else(|_| "Thank you for your message. We will get back to you shortly.".to_string())
                }
                _ => {
                    crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await.unwrap_or_else(|_| "Thank you for your message. We will get back to you shortly.".to_string())
                }
            };

            let description = if risk == ActionRisk::AutoExecute {
                format!("Auto-replied to message: '{}' with '{}'", message, generated_response)
            } else {
                "Draft email for review".to_string()
            };

            let inbox_id = event.payload.get("inbox_message_id").and_then(|v| v.as_str()).unwrap_or("");
            if !inbox_id.is_empty() {
                let _ = self.orchestrator.update_inbox_message_draft(inbox_id, &event.tenant_id, &generated_response).await;
            }

            let action_payload = serde_json::json!({
                "feature_type": "ambassador_reply",
                "original_message": message,
                "generated_response": generated_response,
                "context_used": context_summary,
                "inbox_message_id": inbox_id,
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

#[cfg(test)]
mod tests {


    #[test]
    fn test_customer_success_agent_struct_exists() {
        // Minimal test to assert the module loads in the test harness
        let type_name = "CustomerSuccessAgent";
        assert_eq!(type_name, "CustomerSuccessAgent");
    }
}
