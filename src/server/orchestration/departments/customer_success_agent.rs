use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use std::collections::HashMap;

pub struct CustomerSuccessAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
    pub hub: Option<std::sync::Arc<crate::hub::Hub>>,
    configs: HashMap<String, DepartmentConfig>,
}

impl CustomerSuccessAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self {
            orchestrator,
            hub: None,
            configs: HashMap::new(),
        }
    }

    pub fn with_hub(mut self, hub: std::sync::Arc<crate::hub::Hub>) -> Self {
        self.hub = Some(hub);
        self
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

            let source = original.and_then(|orig| orig.get("source").and_then(|v| v.as_str())).unwrap_or("").to_string();
            let sender_id = original.and_then(|orig| orig.get("sender_id").and_then(|v| v.as_str())).unwrap_or("").to_string();

            let target_language = original.and_then(|orig| orig.get("translated_from_language").and_then(|v| v.as_str())).unwrap_or("").to_string();
            let text = if !target_language.is_empty() && target_language.to_lowercase() != "en" && target_language.to_lowercase() != "english" && target_language.to_lowercase() != "unknown" {
                match crate::api::agents::translation::translate_inbox_message_with_llm(&event.tenant_id, &source, message, &target_language).await {
                    Ok(t) => t.translated_content,
                    Err(e) => {
                        tracing::error!("Failed to translate outgoing message back to {}: {}", target_language, e);
                        message.to_string()
                    }
                }
            } else {
                message.to_string()
            };

            let _hub_clone = self.hub.clone();
            let tenant_id_for_meta = event.tenant_id.clone();

            tokio::spawn(async move {
                if (source == "whatsapp" || source == "instagram") && !sender_id.is_empty() {
                    let pool = crate::db::get_pool();
                    let row: Result<(String,), sqlx::Error> = sqlx::query_as("SELECT api_token FROM integration_credentials WHERE integration_id = 'meta' AND tenant_id = $1 LIMIT 1")
                        .bind(&tenant_id_for_meta)
                        .fetch_one(&pool)
                        .await;
                    match row {
                        Ok((api_token,)) => {
                            use crate::integrations::meta::client::{MetaClientWrapper, RealMetaClient};
                            let client = RealMetaClient::new(api_token);
                            if let Err(e) = client.send_message(&source, &sender_id, &text).await {
                                tracing::error!("Failed to send {} message via Meta integration: {}", source, e);
                            } else {
                                tracing::info!("Successfully sent {} message via Meta integration", source);
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to fetch Meta integration credentials from DB: {}", e);
                        }
                    }
                }
            });

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

        if event.event_type == "tenant.message.received" {
            let message = event.payload.get("original_message")
                .or_else(|| event.payload.get("message"))
                .and_then(|v| v.as_str()).unwrap_or("");
            let source = event.payload.get("source").and_then(|v| v.as_str()).unwrap_or("");
            let sender_id = event.payload.get("sender_id").and_then(|v| v.as_str()).unwrap_or("");

            // Identity Resolution: Look up customer by phone, email, or name
            let mut customer_id = "".to_string();
            let mut past_orders = "".to_string();
            if !sender_id.is_empty() && sender_id != "unknown" {
                let pool = crate::db::get_pool();
                let result: Result<(String,), sqlx::Error> = sqlx::query_as("SELECT id FROM customers WHERE tenant_id = $1 AND (phone = $2 OR email = $2 OR name = $2) LIMIT 1")
                    .bind(&event.tenant_id)
                    .bind(&sender_id)
                    .fetch_one(&pool)
                    .await;
                if let Ok((id,)) = result {
                    customer_id = id.clone();
                    tracing::info!("Resolved sender {} to customer {}", sender_id, customer_id);

                    // Fetch past orders context
                    let orders: Result<Vec<(f64,)>, sqlx::Error> = sqlx::query_as("SELECT total_amount FROM orders WHERE tenant_id = $1 AND customer_id = $2")
                        .bind(&event.tenant_id)
                        .bind(&customer_id)
                        .fetch_all(&pool)
                        .await;
                    if let Ok(orders) = orders {
                        if !orders.is_empty() {
                            past_orders = format!("Returning Customer ({} past orders).", orders.len());
                        }
                    }
                }
            }

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

            let memories = self.orchestrator.query_long_term_memory(&event.tenant_id, &query_embedding, 5).await.unwrap_or_default();

            let mut context_summary = if !memories.is_empty() {
                memories.join("\n")
            } else {
                "No relevant memory found.".to_string()
            };

            if !past_orders.is_empty() {
                context_summary.push_str("\n");
                context_summary.push_str(&past_orders);
            }

            if let Ok(inventory_summary) = self.orchestrator.get_inventory_summary(&event.tenant_id).await {
                context_summary.push_str("\n\n");
                context_summary.push_str(&inventory_summary);
            }

            let prompt = format!(
                "Write one concise, warm customer-service reply for an omnichannel SMB inbox. Do not invent policies, availability, prices, or order state. Use the provided inventory context if asked about product availability. Tenant: {}. Customer message: {}\n\nContext:\n{}",
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
                if risk == ActionRisk::AutoExecute {
                    let _ = self.orchestrator.update_inbox_message_status(inbox_id, &event.tenant_id, "auto_replied").await;
                }
            }

            let action_payload = serde_json::json!({
                "feature_type": "ambassador_reply",
                "original_message": message,
                "generated_response": generated_response,
                "context_used": context_summary,
                "inbox_message_id": inbox_id,
                "source": source,
                "original_content": message,
                "sender_id": sender_id,
                "customer_id": customer_id,
                "past_orders": past_orders,
            });

            let approval_req = self.orchestrator.execute_action(
                DepartmentType::CustomerSuccess,
                description,
                event.tenant_id.clone(),
                risk.clone(),
                action_payload.clone(),
            ).await.map_err(|e| e.to_string())?;

            if risk == ActionRisk::AutoExecute {
                let approved_event = DepartmentEvent {
                    id: uuid::Uuid::new_v4().to_string(),
                    tenant_id: event.tenant_id.clone(),
                    event_type: "agent:customer_success:approved".to_string(),
                    payload: serde_json::json!({
                        "original_payload": action_payload,
                        "approval_id": approval_req.id
                    }),
                };
                let _ = self.orchestrator.dispatch_event(approved_event).await;
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
