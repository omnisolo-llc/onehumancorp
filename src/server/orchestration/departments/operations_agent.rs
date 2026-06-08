use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct RescheduleIntent {
    pub original_message: String,
    pub suggested_time: String,
    pub booking_id: Option<String>,
}

#[async_trait::async_trait]
pub trait BookingIntentPlanner: Send + Sync {
    async fn plan_reschedule_intent(
        &self,
        tenant_id: &str,
        payload: &Value,
    ) -> Result<Option<RescheduleIntent>, String>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OperationsIntentBackend {
    Local,
    Minimax { api_key: String },
}

impl OperationsIntentBackend {
    pub fn from_env() -> Self {
        match std::env::var("OHC_OPERATIONS_LLM_PROVIDER")
            .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
            .as_deref()
        {
            Ok("minimax") => {
                let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                if api_key.trim().is_empty() {
                    Self::Local
                } else {
                    Self::Minimax { api_key }
                }
            }
            _ => Self::Local,
        }
    }
}

struct RuntimeBookingIntentPlanner {
    backend: OperationsIntentBackend,
}

impl RuntimeBookingIntentPlanner {
    fn from_env() -> Self {
        Self {
            backend: OperationsIntentBackend::from_env(),
        }
    }
}

#[async_trait::async_trait]
impl BookingIntentPlanner for RuntimeBookingIntentPlanner {
    async fn plan_reschedule_intent(
        &self,
        tenant_id: &str,
        payload: &Value,
    ) -> Result<Option<RescheduleIntent>, String> {
        let original_message = payload
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim();
        if original_message.is_empty() {
            return Ok(None);
        }

        let payload_json = serde_json::to_string(payload).map_err(|e| e.to_string())?;
        let prompt = format!(
            "You are the OneHumanCorp operations planner. Decide whether a customer message is asking to reschedule an appointment. Return strict JSON only with keys intent, suggested_time, confidence, booking_id, and original_message. intent must be reschedule or none. confidence is 0.0 to 1.0. suggested_time must be the ISO8601 string of the requested time. booking_id is the string ID of the booking if present in context. Tenant: {tenant_id}. Payload: {payload_json}"
        );

        let raw = match &self.backend {
            OperationsIntentBackend::Minimax { api_key } => {
                crate::minimax::MinimaxClient::new(api_key.clone())
                    .reason(&prompt)
                    .await
            }
            OperationsIntentBackend::Local => crate::minimax::LocalLLMClient::new()
                .reason(&prompt)
                .await,
        }?;

        let parsed: Value = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
        if parsed.get("intent").and_then(|i| i.as_str()) == Some("reschedule") {
            let confidence = parsed.get("confidence").and_then(|c| c.as_f64()).unwrap_or(0.0);
            if confidence > 0.6 {
                return Ok(Some(RescheduleIntent {
                    original_message: original_message.to_string(),
                    suggested_time: parsed.get("suggested_time").and_then(|t| t.as_str()).unwrap_or("unknown").to_string(),
                    booking_id: parsed.get("booking_id").and_then(|t| t.as_str()).map(|s| s.to_string()),
                }));
            }
        }
        Ok(None)
    }
}

pub struct OperationsAgent {
    orchestrator: Arc<DepartmentOrchestrator>,
    booking_intent_planner: Arc<dyn BookingIntentPlanner>,
}

impl OperationsAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self {
            orchestrator,
            booking_intent_planner: Arc::new(RuntimeBookingIntentPlanner::from_env()),
        }
    }

    pub fn with_planner(
        orchestrator: Arc<DepartmentOrchestrator>,
        booking_intent_planner: Arc<dyn BookingIntentPlanner>,
    ) -> Self {
        Self {
            orchestrator,
            booking_intent_planner,
        }
    }
}

#[async_trait::async_trait]
impl Department for OperationsAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Operations
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.quote.accepted".to_string(),
            "tenant.booking.reschedule_requested".to_string(),
            "tenant.order.created".to_string(),
            "tenant.subscription.fulfillment_batch.created".to_string(),
            "LowStockAlert".to_string(),
            "PosSyncFailure".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        let config = self.get_config(&event.tenant_id);
        let risk = if let Some(cfg) = config {
            if cfg.auto_approve_limits > 0.0 {
                ActionRisk::AutoExecute
            } else {
                ActionRisk::DraftForReview
            }
        } else {
            ActionRisk::DraftForReview
        };

        let mut payload = event.payload.clone();

        let action_description = match event.event_type.as_str() {
            "tenant.booking.reschedule_requested" => {
                if let Ok(Some(intent)) = self.booking_intent_planner.plan_reschedule_intent(&event.tenant_id, &event.payload).await {
                    let mut obj = payload.as_object_mut().unwrap();
                    obj.insert("suggested_time".to_string(), Value::String(intent.suggested_time.clone()));
                    if let Some(booking_id) = intent.booking_id {
                        obj.insert("booking_id".to_string(), Value::String(booking_id));
                    }
                    format!("Review Reschedule Request for {}", intent.suggested_time)
                } else {
                    "Review Reschedule Request".to_string()
                }
            },
            "tenant.order.created" => "Process Order & Update Inventory".to_string(),
            "LowStockAlert" => {
                let product_id = event.payload.get("product_id").and_then(|v| v.as_str()).unwrap_or("unknown");
                format!("Draft a restock order for product {} due to low stock", product_id)
            },
            "PosSyncFailure" => {
                let transaction_id = event.payload.get("transaction_id").and_then(|v| v.as_str()).unwrap_or("unknown");
                format!("Review POS offline sync discrepancy for transaction {}", transaction_id)
            },
            "tenant.subscription.fulfillment_batch.created" => {
                let batch_id = event
                    .payload
                    .get("batch_id")
                    .and_then(|value| value.as_str())
                    .unwrap_or("unknown batch");
                let subscriber_count = event
                    .payload
                    .get("subscriber_count")
                    .and_then(|value| value.as_i64())
                    .unwrap_or(0);
                format!(
                    "Prepare subscription fulfillment batch {} for {} subscribers",
                    batch_id, subscriber_count
                )
            }
            _ => "Create order and booking".to_string(),
        };

        self.orchestrator.execute_action(
            DepartmentType::Operations,
            action_description,
            event.tenant_id.clone(),
            risk,
            payload,
        ).await?;

        if event.event_type == "tenant.subscription.fulfillment_batch.created" {
            return Ok(());
        }

        // Dispatch event for customer success agent
        let cs_event = DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: event.tenant_id.clone(),
            event_type: "tenant.order.fulfillment_ready".to_string(),
            payload: event.payload.clone(),
        };
        self.orchestrator.dispatch_event(cs_event).await
    }

    fn get_config(&self, _tenant_id: &str) -> Option<DepartmentConfig> {
        Some(DepartmentConfig { tone_of_voice: "professional".to_string(), auto_approve_limits: 10.0 })
    }

    fn set_config(&mut self, _tenant_id: String, _config: DepartmentConfig) {
    }

    async fn query_memory(&self, _query: &str) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String> {
        self.orchestrator.execute_action(self.department_type(), description.clone(), tenant_id.clone(), risk, serde_json::json!({})).await
    }
}

#[async_trait::async_trait]
impl BaseAgent for OperationsAgent {
    fn agent_id(&self) -> String {
        "operations_agent".to_string()
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
    use super::*;
    use crate::orchestration::departments::orchestrator::Department;
    use crate::orchestration::departments::types::{ApprovalStatus, DepartmentType};
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
        sqlx::query(
            "CREATE TABLE tenants (
                tenant_id TEXT PRIMARY KEY,
                business_name TEXT,
                tier TEXT
            )",
        )
        .execute(&sqlite_pool)
        .await
        .unwrap();
        sqlx::query(
            "CREATE TABLE tenant_ai_budgets (
                tenant_id TEXT NOT NULL,
                year_month TEXT NOT NULL,
                actions_used INTEGER NOT NULL DEFAULT 0,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY (tenant_id, year_month)
            )",
        )
        .execute(&sqlite_pool)
        .await
        .unwrap();
        sqlx::query(
            "CREATE TABLE agent_approvals (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                department TEXT NOT NULL,
                description TEXT NOT NULL,
                status TEXT NOT NULL,
                action_risk TEXT NOT NULL,
                payload TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .execute(&sqlite_pool)
        .await
        .unwrap();
        sqlx::query("INSERT INTO tenants (tenant_id, business_name, tier) VALUES ('tenant-ops', 'Ops Test', 'starter')")
            .execute(&sqlite_pool)
            .await
            .unwrap();

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
    async fn operations_agent_consumes_subscription_fulfillment_batch_events() {
        let orchestrator = test_orchestrator().await;
        let agent = OperationsAgent::new(orchestrator.clone());

        assert!(agent
            .subscribed_events()
            .contains(&"tenant.subscription.fulfillment_batch.created".to_string()));

        let event = DepartmentEvent {
            id: "evt-batch".to_string(),
            tenant_id: "tenant-ops".to_string(),
            event_type: "tenant.subscription.fulfillment_batch.created".to_string(),
            payload: serde_json::json!({
                "batch_id": "batch-123",
                "subscription_plan_id": "plan-123",
                "fulfillment_date": "2026-06-15",
                "subscriber_count": 2
            }),
        };

        agent.handle_event(&event).await.unwrap();

        let approvals = orchestrator.get_activity_feed("tenant-ops", None, 10).await;
        let approval = approvals
            .iter()
            .find(|approval| approval.description.contains("Prepare subscription fulfillment batch"))
            .expect("fulfillment batch should create an operations action");

        assert_eq!(approval.status, ApprovalStatus::Approved);
        assert_eq!(approval.department, DepartmentType::Operations);
        assert_eq!(
            approval.payload.as_ref().unwrap()["batch_id"],
            serde_json::json!("batch-123")
        );
        assert_eq!(
            approval.payload.as_ref().unwrap()["subscriber_count"],
            serde_json::json!(2)
        );
    }
}
