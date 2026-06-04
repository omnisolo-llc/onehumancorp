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
            let original = payload.get("payload");

            let message = if let Some(orig) = original {
                orig.get("generated_response").and_then(|v| v.as_str()).unwrap_or("Unknown response")
            } else {
                "Unknown response"
            };

            let source = original.and_then(|o| o.get("source")).and_then(|v| v.as_str()).unwrap_or("");
            let sender_id = original.and_then(|o| o.get("sender_id")).and_then(|v| v.as_str()).unwrap_or("");

            tracing::info!("EXECUTING APPROVED DRAFT: Sending message to {} via {}: {}", sender_id, source, message);

            if source == "instagram" && !sender_id.is_empty() {
                let meta_provider = crate::integrations::meta::provider::MetaProvider::new("dummy_token".to_string());
                if let Err(e) = meta_provider.send_message("instagram", sender_id, message).await {
                    tracing::error!("Failed to send meta message: {}", e);
                }
            }

            let content = format!("Sent response to customer: {}", message);

            // Log the action in the agent's memory
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
            self.orchestrator.write_long_term_memory(record).await.map_err(|e| e.to_string())?;

            return Ok(());
        }

        if event.event_type == "tenant.message.received" {
            let message = event.payload.get("message").and_then(|v| v.as_str()).unwrap_or("");
            let source = event.payload.get("source").and_then(|v| v.as_str()).unwrap_or("");
            let inbox_message_id = event.payload.get("inbox_message_id").and_then(|v| v.as_str()).unwrap_or("");
            let sender_id = event.payload.get("sender_id").and_then(|v| v.as_str()).unwrap_or("");

            // 1. Fetch real inventory
            let pool = crate::db::get_pool();
            let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
            crate::common::auth_utils::set_org_context(&mut *tx, &event.tenant_id).await.map_err(|e| e.to_string())?;

            // Using query_as since products table has title, description, inventory_count
            let products: Vec<(String, String, i32)> = sqlx::query_as(
                "SELECT title, COALESCE(description, ''), COALESCE(inventory_count, 0) FROM products WHERE tenant_id = $1"
            )
            .bind(&event.tenant_id)
            .fetch_all(&mut *tx)
            .await
            .unwrap_or_default();

            let _ = tx.commit().await;

            let mut inventory_context = String::new();
            for (title, desc, count) in products {
                inventory_context.push_str(&format!("- {} ({}): {} in stock\n", title, desc, count));
            }
            if inventory_context.is_empty() {
                inventory_context = "No products in inventory.".to_string();
            }

            // 2. Generate Draft Reply using LLM
            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
            let generated_response = if !api_key.is_empty() {
                let prompt = format!(
                    "Write one concise, warm customer-service reply. Business Inventory: {} Customer message: {}",
                    inventory_context, message
                );
                let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);
                let client = crate::minimax::MinimaxClient::new(api_key);
                client.reason(&compressed_prompt).await.unwrap_or_else(|_| "Thank you for reaching out! We will get back to you shortly.".to_string())
            } else {
                "Thank you for your message. We will get back to you shortly.".to_string()
            };

            // 3. Update the inbox_messages row with the generated draft
            if !inbox_message_id.is_empty() {
                let mut tx2 = pool.begin().await.map_err(|e| e.to_string())?;
                crate::common::auth_utils::set_org_context(&mut *tx2, &event.tenant_id).await.map_err(|e| e.to_string())?;
                let _ = sqlx::query("UPDATE inbox_messages SET draft_reply = $1 WHERE id = $2")
                    .bind(&generated_response)
                    .bind(inbox_message_id)
                    .execute(&mut *tx2)
                    .await;
                let _ = tx2.commit().await;
            }

            // 4. Create Approval Action
            let description = if risk == ActionRisk::AutoExecute {
                format!("Auto-replied to message: '{}'", message)
            } else {
                format!("Draft reply for Instagram message from {}: {}", sender_id, message)
            };

            let action_payload = serde_json::json!({
                "feature_type": "ambassador_reply",
                "original_message": message,
                "generated_response": generated_response,
                "source": source,
                "sender_id": sender_id,
                "inbox_message_id": inbox_message_id,
                "context_used": inventory_context,
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
