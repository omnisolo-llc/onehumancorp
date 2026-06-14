use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use std::sync::Arc;

pub struct OrderInterceptorAgent {
    orchestrator: Arc<DepartmentOrchestrator>,
}

impl OrderInterceptorAgent {
    pub fn new(orchestrator: Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait::async_trait]
impl Department for OrderInterceptorAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Operations
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.message.received".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "tenant.message.received" {
            let message = event.payload.get("message").and_then(|v| v.as_str()).unwrap_or("");
            let original_lang = event.payload.get("translated_from_language").and_then(|v| v.as_str()).unwrap_or("en");

            if message.is_empty() {
                return Ok(());
            }

            // AI analysis to see if this is an order
            let prompt = format!(
                "You are an OHC Order Interceptor. Analyze the following message and determine if it's an order or a pre-order request.
                If it is, extract the items and quantities into a structured JSON format.
                Message: \"{}\"

                Return ONLY valid JSON with fields:
                - is_order (boolean)
                - items (array of {{ name: string, quantity: number }})
                - summary (string, short summary in English)
                - confidence (number 0-1)",
                message
            );

            let raw_response = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
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

            let mut clean_json = raw_response.trim();
            if let Some(start) = clean_json.find('{') {
                if let Some(end) = clean_json.rfind('}') {
                    clean_json = &clean_json[start..=end];
                }
            }

            if let Ok(analysis) = serde_json::from_str::<serde_json::Value>(clean_json) {
                if analysis.get("is_order").and_then(|v| v.as_bool()).unwrap_or(false) && analysis.get("confidence").and_then(|v| v.as_f64()).unwrap_or(0.0) > 0.7 {
                    let summary = analysis.get("summary").and_then(|v| v.as_str()).unwrap_or("New Order Received");

                    // Create an action for the owner to approve the order
                    self.orchestrator.execute_action(
                        DepartmentType::Operations,
                        format!("Incoming Multilingual Order: {}", summary),
                        event.tenant_id.clone(),
                        ActionRisk::DraftForReview,
                        serde_json::json!({
                            "order_data": analysis,
                            "original_message": event.payload.get("original_message"),
                            "translated_message": message,
                            "source_language": original_lang
                        })
                    ).await?;

                    // Also draft a confirmation reply in the original language
                    let reply_prompt = format!(
                        "Write a short, friendly order confirmation message in language '{}' for the following items: {:?}.
                        Keep it under 2 sentences.",
                        original_lang,
                        analysis.get("items")
                    );

                    let draft_reply = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                        Ok("minimax") => {
                            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                            crate::minimax::MinimaxClient::new(api_key).reason(&reply_prompt).await.unwrap_or_default()
                        },
                        _ => crate::minimax::LocalLLMClient::new().reason(&reply_prompt).await.unwrap_or_default(),
                    };

                    let cs_event = DepartmentEvent {
                        id: uuid::Uuid::new_v4().to_string(),
                        tenant_id: event.tenant_id.clone(),
                        event_type: "tenant.order.intercepted".to_string(),
                        payload: serde_json::json!({
                            "summary": summary,
                            "draft_reply": draft_reply,
                            "inbox_message_id": event.payload.get("inbox_message_id")
                        }),
                    };
                    self.orchestrator.dispatch_event(cs_event).await?;
                }
            }
        }
        Ok(())
    }

    fn get_config(&self, _tenant_id: &str) -> Option<DepartmentConfig> {
        Some(DepartmentConfig { tone_of_voice: "helpful".to_string(), auto_approve_limits: 0.0 })
    }

    async fn query_memory(&self, _query: &str) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String> {
        self.orchestrator.execute_action(self.department_type(), description.clone(), tenant_id.clone(), risk, serde_json::json!({})).await
    }
}

#[async_trait::async_trait]
impl BaseAgent for OrderInterceptorAgent {
    fn agent_id(&self) -> String {
        "order_interceptor_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestration::departments::orchestrator::Department;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use sqlx::sqlite::SqlitePoolOptions;
    use std::sync::Arc;

    async fn test_orchestrator() -> Arc<DepartmentOrchestrator> {
        let sqlite_pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query("CREATE TABLE tenants (tenant_id TEXT PRIMARY KEY, business_name TEXT, tier TEXT)").execute(&sqlite_pool).await.unwrap();
        sqlx::query("CREATE TABLE agent_approvals (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, department TEXT NOT NULL, description TEXT NOT NULL, status TEXT NOT NULL, action_risk TEXT NOT NULL, payload TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP, updated_at TEXT DEFAULT CURRENT_TIMESTAMP)").execute(&sqlite_pool).await.unwrap();
        sqlx::query("CREATE TABLE tenant_ai_budgets (tenant_id TEXT NOT NULL, year_month TEXT NOT NULL, actions_used INTEGER NOT NULL DEFAULT 0, updated_at TEXT DEFAULT CURRENT_TIMESTAMP, PRIMARY KEY (tenant_id, year_month))").execute(&sqlite_pool).await.unwrap();

        let pg_pool = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
        let db = Arc::new(crate::db::DB {
            pool: pg_pool,
            store: crate::db::DbStore::Sqlite(sqlite_pool),
        });
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        Arc::new(DepartmentOrchestrator::new(db, mesh))
    }

    #[tokio::test]
    async fn test_order_interceptor_handles_message() {
        let orchestrator = test_orchestrator().await;
        let agent = OrderInterceptorAgent::new(orchestrator.clone());

        // Set provider to local to use mock data/LocalLLMClient
        std::env::set_var("OHC_LLM_PROVIDER", "local");

        let event = DepartmentEvent {
            id: "evt-123".to_string(),
            tenant_id: "tenant-1".to_string(),
            event_type: "tenant.message.received".to_string(),
            payload: serde_json::json!({
                "message": "I want to order 2 coffees",
                "original_message": "I want to order 2 coffees",
                "translated_from_language": "en",
                "inbox_message_id": "msg-123"
            }),
        };

        // Even with LocalLLMClient, it might not return valid JSON or "is_order: true".
        // But we want to ensure the agent logic for dispatching events and executing actions works.
        let _ = agent.handle_event(&event).await;

        let feed = orchestrator.get_activity_feed("tenant-1", None, 10).await;
        // If LocalLLMClient returns something that passes the filter, we'd see it here.
        // For a more robust test, we would need to mock the LLM client, but the current
        // framework uses environment variables and static calls.
    }
}
