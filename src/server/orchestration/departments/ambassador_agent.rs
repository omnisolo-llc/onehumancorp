use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use std::collections::HashMap;

pub struct AmbassadorAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
    pub hub: Option<std::sync::Arc<crate::hub::Hub>>,
    configs: HashMap<String, DepartmentConfig>,
}

impl AmbassadorAgent {
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

    async fn get_knowledge_base_context(&self, tenant_id: &str, query: &str) -> String {
        let pool = crate::db::get_pool();
        let query_lower = format!("%{}%", query.to_lowercase());

        let rows: Result<Vec<(String, String)>, sqlx::Error> = match &self.orchestrator.db().store {
            crate::db::DbStore::Postgres => {
                sqlx::query_as("SELECT title, content_markdown FROM help_articles WHERE tenant_id = $1 AND (LOWER(title) LIKE $2 OR LOWER(content_markdown) LIKE $2) LIMIT 3")
                    .bind(tenant_id)
                    .bind(&query_lower)
                    .fetch_all(&pool)
                    .await
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as("SELECT title, content_markdown FROM help_articles WHERE tenant_id = ? AND (LOWER(title) LIKE ? OR LOWER(content_markdown) LIKE ?) LIMIT 3")
                    .bind(tenant_id)
                    .bind(&query_lower)
                    .bind(&query_lower)
                    .fetch_all(sqlite_pool)
                    .await
            }
        };

        if let Ok(articles) = rows {
            if articles.is_empty() {
                return "No relevant help articles found.".to_string();
            }
            let mut context = String::from("Knowledge Base Articles:\n");
            for (title, content) in articles {
                context.push_str(&format!("- {}: {}\n", title, content));
            }
            context
        } else {
            "No knowledge base context available.".to_string()
        }
    }
}

#[async_trait::async_trait]
impl Department for AmbassadorAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Ambassador
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.omnichannel.message.received".to_string(),
            "agent:ambassador:approved".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "agent:ambassador:approved" {
            let payload = &event.payload;
            let original = payload.get("original_payload");
            let message = if let Some(orig) = original {
                orig.get("generated_response").and_then(|v| v.as_str()).unwrap_or("Unknown response")
            } else {
                "Unknown response"
            };
            tracing::info!("AMBASSADOR EXECUTING APPROVED DRAFT: Sending message: {}", message);

            let source = original.and_then(|orig| orig.get("source").and_then(|v| v.as_str())).unwrap_or("").to_string();
            let sender_id = original.and_then(|orig| orig.get("sender_id").and_then(|v| v.as_str())).unwrap_or("").to_string();

            let tenant_id = event.tenant_id.clone();

            // Execute outbound communication
            if (source == "whatsapp" || source == "instagram") && !sender_id.is_empty() {
                let pool = crate::db::get_pool();
                let row: Result<(String,), sqlx::Error> = sqlx::query_as("SELECT api_token FROM integration_credentials WHERE integration_id = 'meta' AND tenant_id = $1 LIMIT 1")
                    .bind(&tenant_id)
                    .fetch_one(&pool)
                    .await;
                match row {
                    Ok((api_token,)) => {
                        use crate::integrations::meta::client::{MetaClientWrapper, RealMetaClient};
                        let client = RealMetaClient::new(api_token);
                        if let Err(e) = client.send_message(&source, &sender_id, &message).await {
                            tracing::error!("Ambassador failed to send {} message via Meta integration: {}", source, e);
                        } else {
                            tracing::info!("Ambassador successfully sent {} message via Meta integration", source);
                        }
                    }
                    Err(e) => {
                        tracing::error!("Ambassador failed to fetch Meta integration credentials from DB: {}", e);
                    }
                }
            }

            if let Some(inbox_id) = original.and_then(|orig| orig.get("inbox_message_id").and_then(|v| v.as_str())) {
                let _ = self.orchestrator.update_inbox_message_status(inbox_id, &tenant_id, "sent").await;
            }

            return Ok(());
        }

        if event.event_type == "tenant.omnichannel.message.received" {
            let message = event.payload.get("content")
                .or_else(|| event.payload.get("message"))
                .and_then(|v| v.as_str()).unwrap_or("");
            let source = event.payload.get("source").and_then(|v| v.as_str()).unwrap_or("unknown");
            let sender_id = event.payload.get("sender_id").and_then(|v| v.as_str()).unwrap_or("unknown");
            let inbox_id = event.payload.get("inbox_message_id")
                .or_else(|| event.payload.get("message_id"))
                .and_then(|v| v.as_str()).unwrap_or("");

            if message.is_empty() {
                return Ok(());
            }

            // 1. Retrieve Context (RAG)
            let inventory_summary = self.orchestrator.get_inventory_summary(&event.tenant_id).await.unwrap_or_else(|_| "No inventory data available.".to_string());
            let kb_context = self.get_knowledge_base_context(&event.tenant_id, message).await;

            let context = format!("{}\n\n{}", inventory_summary, kb_context);

            // 2. Draft Response using LLM
            let prompt = format!(
                "You are 'The Ambassador', a native social inbox auto-responder for a small business.
Your goal is to draft a helpful, warm, and concise reply to a customer message.
Do not invent policies or prices. Use the provided context for inventory and policies.

Tenant ID: {}
Source: {}
Customer Message: '{}'

Context:
{}

Drafted Reply:",
                event.tenant_id, source, message, context
            );

            let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);

            let generated_response = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                Ok("minimax") => {
                    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
                    crate::minimax::MinimaxClient::new(api_key).reason(&compressed_prompt).await.unwrap_or_else(|_| "Thank you for reaching out! We will get back to you soon.".to_string())
                }
                _ => {
                    crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await.unwrap_or_else(|_| "Thank you for reaching out! We will get back to you soon.".to_string())
                }
            };

            // 3. Push Action Card to Agent Feed
            let action_payload = serde_json::json!({
                "feature_type": "ambassador_reply",
                "original_message": message,
                "generated_response": generated_response,
                "context_used": context,
                "inbox_message_id": inbox_id,
                "source": source,
                "sender_id": sender_id,
            });

            let _ = self.orchestrator.execute_action(
                DepartmentType::Ambassador,
                format!("The Ambassador: Drafted reply for {}", source),
                event.tenant_id.clone(),
                ActionRisk::DraftForReview,
                action_payload,
            ).await?;

            return Ok(());
        }
        Ok(())
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
        self.orchestrator.execute_action(self.department_type(), description, tenant_id, risk, serde_json::json!({})).await
    }
}

#[async_trait::async_trait]
impl BaseAgent for AmbassadorAgent {
    fn agent_id(&self) -> String {
        "ambassador_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }
}
