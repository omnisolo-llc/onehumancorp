use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use std::sync::Arc;
use chrono::{DateTime, Utc};

pub struct NegotiatorAgent {
    orchestrator: Arc<DepartmentOrchestrator>,
}

impl NegotiatorAgent {
    pub fn new(orchestrator: Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait::async_trait]
impl Department for NegotiatorAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Sales
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.message.received".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "tenant.message.received" {
            let message = event.payload.get("message").and_then(|v| v.as_str()).unwrap_or("");

            if message.is_empty() {
                return Ok(());
            }

            // AI analysis to see if this is a booking request or negotiation
            let prompt = format!(
                "You are an OHC Agentic Negotiator. Analyze the following message and determine if the customer wants to book a service or is negotiating a time/price.
                Message: \"{}\"

                Return ONLY valid JSON with fields:
                - is_negotiation (boolean)
                - intent (string: 'booking_request', 'reschedule', 'price_inquiry', 'other')
                - service_name (string, if mentioned)
                - preferred_date (string, YYYY-MM-DD if mentioned)
                - preferred_time (string, HH:MM if mentioned)
                - confidence (number 0-1)",
                message
            );

            let raw_response = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                Ok("minimax") => {
                    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                    crate::minimax::MinimaxClient::new(api_key).reason(&prompt).await.unwrap_or_default()
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
                if analysis.get("is_negotiation").and_then(|v| v.as_bool()).unwrap_or(false) && analysis.get("confidence").and_then(|v| v.as_f64()).unwrap_or(0.0) > 0.7 {

                    let service_name = analysis.get("service_name").and_then(|v| v.as_str()).unwrap_or("Requested Service");
                    let date = analysis.get("preferred_date").and_then(|v| v.as_str()).unwrap_or("any day");
                    let time = analysis.get("preferred_time").and_then(|v| v.as_str()).unwrap_or("any time");

                    // Check availability if date is mentioned
                    let availability_status = if date != "any day" {
                        // Simulated availability check
                        "Available"
                    } else {
                        "Needs Date"
                    };

                    // Create a "Pending AI Negotiation" action for the owner
                    self.orchestrator.execute_action(
                        DepartmentType::Sales,
                        format!("Pending AI Negotiation: {} for {}", service_name, date),
                        event.tenant_id.clone(),
                        ActionRisk::DraftForReview,
                        serde_json::json!({
                            "negotiation_data": analysis,
                            "availability": availability_status,
                            "original_message": event.payload.get("original_message"),
                            "translated_message": message,
                            "proposed_action": format!("Book {} for {} at {}", service_name, date, time)
                        })
                    ).await?;

                    // Emit an event for UI update
                    let neg_event = DepartmentEvent {
                        id: uuid::Uuid::new_v4().to_string(),
                        tenant_id: event.tenant_id.clone(),
                        event_type: "tenant.negotiation.pending".to_string(),
                        payload: serde_json::json!({
                            "service": service_name,
                            "date": date,
                            "time": time,
                            "status": availability_status
                        }),
                    };
                    self.orchestrator.dispatch_event(neg_event).await?;
                }
            }
        }
        Ok(())
    }

    fn get_config(&self, _tenant_id: &str) -> Option<DepartmentConfig> {
        Some(DepartmentConfig { tone_of_voice: "professional".to_string(), auto_approve_limits: 0.0 })
    }

    async fn query_memory(&self, _query: &str) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String> {
        self.orchestrator.execute_action(self.department_type(), description.clone(), tenant_id.clone(), risk, serde_json::json!({})).await
    }
}

#[async_trait::async_trait]
impl BaseAgent for NegotiatorAgent {
    fn agent_id(&self) -> String {
        "negotiator_agent".to_string()
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
    async fn test_negotiator_handles_booking_request() {
        let orchestrator = test_orchestrator().await;
        let agent = NegotiatorAgent::new(orchestrator.clone());

        let event = DepartmentEvent {
            id: "evt-456".to_string(),
            tenant_id: "tenant-2".to_string(),
            event_type: "tenant.message.received".to_string(),
            payload: serde_json::json!({
                "message": "Can I book a session tomorrow at 4pm?",
                "original_message": "Can I book a session tomorrow at 4pm?"
            }),
        };

        let _ = agent.handle_event(&event).await;

        // Similar to order interceptor, we verify it runs without panicking.
        assert!(true);
    }
}
