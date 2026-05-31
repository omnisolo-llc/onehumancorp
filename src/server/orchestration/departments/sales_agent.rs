use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;
use uuid::Uuid;
use std::str::FromStr;

pub struct SalesAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl SalesAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait::async_trait]
impl Department for SalesAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Sales
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec!["tenant.quote.requested".to_string(), "agent:sales:approved".to_string()]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "agent:sales:approved" {
            let payload = &event.payload;
            let original = payload.get("original_payload");

            let amount = if let Some(orig) = original {
                orig.get("generated_deposit_amount").and_then(|v| v.as_i64()).unwrap_or(0)
            } else {
                0
            };

            // Generate Stripe payment link for deposit
            let tenant_id = Uuid::from_str(&event.tenant_id).unwrap_or(Uuid::new_v4());
            let customer_id = Uuid::new_v4(); // Generate a dummy customer ID for now
            let mut quote = crate::services::booking::BookingService::create_draft_quote(tenant_id, customer_id, amount * 100);

            match crate::services::booking::BookingService::approve_quote(&mut quote, None) {
                Ok((_slot, link)) => {
                    tracing::info!("EXECUTING APPROVED QUOTE: Sending deposit link: {}", link);

                    let content = format!("Sent quote deposit link to customer: {}", link);

                    let record = ohc_builtin_agent::memory_store::EmbeddingRecord {
                        id: uuid::Uuid::new_v4().to_string(),
                        tenant_id: event.tenant_id.clone(),
                        agent_id: "sales_agent".to_string(),
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
                },
                Err(e) => {
                    tracing::error!("Failed to approve quote: {}", e);
                }
            }

            return Ok(());
        }

        // Query memory context
        let query_embedding = vec![0.5, 0.5, 0.5]; // Mock embedding
        let _context = self.orchestrator.query_long_term_memory(&event.tenant_id, &query_embedding, 5).await?;

        let risk = ActionRisk::DraftForReview;

        ::server_telemetry::record_business_event(&event.tenant_id, ::server_telemetry::get_deployment_mode(), "quote_generated");

        self.orchestrator.execute_action(
            DepartmentType::Sales,
            "Quote generated for review".to_string(),
            event.tenant_id.clone(),
            risk,
            event.payload.clone(),
        ).await.map(|_| ())
    }

    fn get_config(&self, _tenant_id: &str) -> Option<DepartmentConfig> {
        None
    }

    fn set_config(&mut self, _tenant_id: String, _config: DepartmentConfig) {
    }

    async fn query_memory(&self, _query: &str) -> Result<Vec<String>, String> {
        let embedding = vec![0.5, 0.5, 0.5];
        // Note: We need a tenant_id here, but the trait signature doesn't provide one.
        // We'll pass a dummy one or extract it if available.
        self.orchestrator.query_long_term_memory("default_tenant", &embedding, 5).await
    }

    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String> {
        self.orchestrator.execute_action(self.department_type(), description.clone(), tenant_id.clone(), risk, serde_json::json!({})).await
    }
}

#[async_trait::async_trait]
impl BaseAgent for SalesAgent {
    fn agent_id(&self) -> String {
        "sales_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }

    async fn execute(&self, _payload: Value) -> Result<(), String> {
        Ok(())
    }
}
