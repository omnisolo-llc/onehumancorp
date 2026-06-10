use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use std::collections::HashMap;
use std::sync::Arc;
use crate::db::DbStore;

pub struct TranslationAgent {
    orchestrator: Arc<DepartmentOrchestrator>,
    configs: HashMap<String, DepartmentConfig>,
}

impl TranslationAgent {
    pub fn new(orchestrator: Arc<DepartmentOrchestrator>) -> Self {
        Self {
            orchestrator,
            configs: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl Department for TranslationAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Operations // or maybe CustomerSuccess, but Operations is a catch-all often
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.omnichannel.message.received".to_string(),
            "tenant.product.created".to_string(),
            "tenant.product.updated".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "tenant.omnichannel.message.received" {
            let message = event.payload.get("original_message").and_then(|v| v.as_str()).unwrap_or("");
            let source = event.payload.get("source").and_then(|v| v.as_str()).unwrap_or("");
            let target_language = event.payload.get("target_language").and_then(|v| v.as_str()).unwrap_or("English");
            let inbox_id = event.payload.get("inbox_message_id").and_then(|v| v.as_str()).unwrap_or("");

            if inbox_id.is_empty() || message.is_empty() {
                return Ok(());
            }

            let translation = match crate::api::agents::translation::translate_inbox_message_with_llm(
                &event.tenant_id,
                source,
                message,
                target_language,
            ).await {
                Ok(t) => t,
                Err(e) => {
                    tracing::error!("Translation failed: {}", e);
                    crate::api::agents::translation::InboxTranslation {
                        translated_content: message.to_string(),
                        source_language: Some("Unknown".to_string()),
                        target_language: target_language.to_string(),
                        original_content: message.to_string(),
                    }
                }
            };

            let pool = crate::db::get_pool();
            let _ = sqlx::query(
                r#"
                UPDATE inbox_messages
                SET content = $1, translated_from_language = $2
                WHERE id = $3 AND tenant_id = $4
                "#
            )
            .bind(&translation.translated_content)
            .bind(&translation.source_language)
            .bind(inbox_id)
            .bind(&event.tenant_id)
            .execute(&pool)
            .await;

            let new_event = DepartmentEvent {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: event.tenant_id.clone(),
                event_type: "tenant.message.received".to_string(),
                payload: serde_json::json!({
                    "source": source,
                    "original_message": translation.original_content,
                    "message": translation.translated_content,
                    "translated_from_language": translation.source_language,
                    "inbox_message_id": inbox_id,
                }),
            };
            self.orchestrator.dispatch_event(new_event).await.map(|_| ())?;

            return Ok(());
        }

        if event.event_type == "tenant.product.created" || event.event_type == "tenant.product.updated" {
            let product_id = event.payload.get("product_id").and_then(|v| v.as_str()).unwrap_or("");
            let name = event.payload.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let description = event.payload.get("description").and_then(|v| v.as_str()).unwrap_or("");
            let tenant_id = event.tenant_id.clone();

            if product_id.is_empty() {
                return Ok(());
            }

            let pool = crate::db::get_pool();

            let target_languages: Vec<String> = {
                let prefs_row = sqlx::query(
                    "SELECT target_languages FROM ohc_translation_preferences WHERE tenant_id = $1"
                )
                .bind(&tenant_id)
                .fetch_optional(&pool)
                .await
                .unwrap_or(None);

                match prefs_row {
                    Some(r) => {
                        use sqlx::Row;
                        let langs_val: serde_json::Value = r.get("target_languages");
                        serde_json::from_value(langs_val).unwrap_or_default()
                    }
                    None => vec![],
                }
            };

            if target_languages.is_empty() {
                return Ok(());
            }

            for lang in target_languages {
                let prompt = format!("Translate the following product name and description into language code '{}'.\nName: {}\nDescription: {}\nReturn JSON format: {{\"name\": \"translated name\", \"description\": \"translated description\"}}", lang, name, description);

                let mut translated_name = format!("[{}] {}", lang, name);
                let mut translated_desc = format!("[{}] {}", lang, description);

                let raw_response = match std::env::var("OHC_TRANSLATION_LLM_PROVIDER")
                    .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
                    .as_deref()
                {
                    Ok("minimax") => {
                        let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                        if api_key.trim().is_empty() {
                            crate::minimax::LocalLLMClient::new().reason(&prompt).await.unwrap_or_default()
                        } else {
                            crate::minimax::MinimaxClient::new(api_key).reason(&prompt).await.unwrap_or_default()
                        }
                    },
                    _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await.unwrap_or_default(),
                };

                let clean_res = raw_response.trim_matches('`').trim_start_matches("json\n").trim_end();
                if let Ok(translated_json) = serde_json::from_str::<serde_json::Value>(clean_res) {
                    translated_name = translated_json.get("name").and_then(|v| v.as_str()).unwrap_or(&translated_name).to_string();
                    translated_desc = translated_json.get("description").and_then(|v| v.as_str()).unwrap_or(&translated_desc).to_string();
                }

                let name_key = format!("product:{}:name", product_id);
                let desc_key = format!("product:{}:description", product_id);

                let _ = sqlx::query(
                    "INSERT INTO ohc_i18n_strings (id, tenant_id, locale, key, value) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (tenant_id, locale, key) DO UPDATE SET value = EXCLUDED.value"
                )
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(&tenant_id)
                .bind(&lang)
                .bind(&name_key)
                .bind(&translated_name)
                .execute(&pool).await;

                let _ = sqlx::query(
                    "INSERT INTO ohc_i18n_strings (id, tenant_id, locale, key, value) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (tenant_id, locale, key) DO UPDATE SET value = EXCLUDED.value"
                )
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(&tenant_id)
                .bind(&lang)
                .bind(&desc_key)
                .bind(&translated_desc)
                .execute(&pool).await;
            }

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
        self.orchestrator.execute_action(self.department_type(), description.clone(), tenant_id.clone(), risk, serde_json::json!({})).await
    }
}

#[async_trait::async_trait]
impl BaseAgent for TranslationAgent {
    fn agent_id(&self) -> String {
        "translation_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }
}
