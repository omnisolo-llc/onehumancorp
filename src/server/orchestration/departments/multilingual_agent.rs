use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use std::collections::HashMap;
use std::sync::Arc;

pub struct MultilingualAgent {
    orchestrator: Arc<DepartmentOrchestrator>,
    configs: HashMap<String, DepartmentConfig>,
}

impl MultilingualAgent {
    pub fn new(orchestrator: Arc<DepartmentOrchestrator>) -> Self {
        Self {
            orchestrator,
            configs: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl Department for MultilingualAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Operations
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec!["tenant.omnichannel.message.received".to_string()]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "tenant.omnichannel.message.received" {
            let source = event.payload.get("source").and_then(|v| v.as_str()).unwrap_or("");
            if source != "walkup" {
                return Ok(()); // Only process walkup orders
            }

            let message = event.payload.get("message").and_then(|v| v.as_str()).unwrap_or("");
            if message.is_empty() {
                return Ok(());
            }

            let tenant_id = event.tenant_id.clone();
            let pool = crate::db::get_pool();

            let target_language: String = {
                let prefs_row = sqlx::query(
                    "SELECT language_preference FROM tenants WHERE id = $1"
                )
                .bind(&tenant_id)
                .fetch_optional(&pool)
                .await
                .unwrap_or(None);

                match prefs_row {
                    Some(r) => {
                        use sqlx::Row;
                        let lang: Option<String> = r.get("language_preference");
                        lang.unwrap_or_else(|| "en".to_string())
                    }
                    None => "en".to_string(),
                }
            };

            let prompt = format!(
                "You are a Multilingual Order Interceptor for a small business. Detect the language of the following input, translate it to {}, extract the intent (Order, Query, Status Check, etc.), and extract any items and quantities.\nInput: {}\nReturn JSON format exactly like: {{\"intent\": \"Order\", \"translated_text\": \"3x Chicken Tacos\", \"items\": [\"3x Chicken Tacos\"]}}",
                target_language, message
            );

            let raw_response = match std::env::var("OHC_TRANSLATION_LLM_PROVIDER")
                .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
                .as_deref()
            {
                Ok("minimax") => {
                    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                    if api_key.trim().is_empty() {
                        crate::minimax::LocalLLMClient::new().reason(&crate::pricing::compression::reduce_tokens(&prompt)).await.unwrap_or_default()
                    } else {
                        crate::minimax::MinimaxClient::new(api_key).reason(&crate::pricing::compression::reduce_tokens(&prompt)).await.unwrap_or_default()
                    }
                },
                _ => crate::minimax::LocalLLMClient::new().reason(&crate::pricing::compression::reduce_tokens(&prompt)).await.unwrap_or_default(),
            };

            let clean_res = raw_response.trim_matches('`').trim_start_matches("json\n").trim_end();
            if let Ok(translated_json) = serde_json::from_str::<serde_json::Value>(clean_res) {
                let intent = translated_json.get("intent").and_then(|v| v.as_str()).unwrap_or("Query");
                let translated_text = translated_json.get("translated_text").and_then(|v| v.as_str()).unwrap_or(message);

                if intent == "Order" {
                    let _ = sqlx::query(
                        "INSERT INTO triage_items (id, tenant_id, source, priority, context, status) VALUES ($1, $2, 'Multilingual Interceptor Agent', 'high', $3, 'pending')"
                    )
                    .bind(uuid::Uuid::new_v4().to_string())
                    .bind(&tenant_id)
                    .bind(translated_text)
                    .execute(&pool)
                    .await;
                }
            }
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
        self.orchestrator.execute_action(self.department_type(), description.clone(), tenant_id.clone(), risk, serde_json::json!({})).await
    }
}

#[async_trait::async_trait]
impl BaseAgent for MultilingualAgent {
    fn agent_id(&self) -> String {
        "multilingual_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }
}
