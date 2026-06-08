use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;
use std::sync::Arc;

pub struct FinanceAgent {
    orchestrator: Arc<DepartmentOrchestrator>,
}

impl FinanceAgent {
    pub fn new(orchestrator: Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait::async_trait]
impl Department for FinanceAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Finance
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec!["tenant.payment.received".to_string()]
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

        if event.event_type == "tenant.payment.received" {
            let amount = event.payload.get("amount").and_then(Value::as_f64).unwrap_or(0.0);

            // Assume an estimated 10% tax liability on the payment
            let tax_liability = amount * 0.10;

            let advisory_payload = serde_json::json!({
                "original_amount": amount,
                "tax_liability_estimate": tax_liability,
                "virtual_envelope": "Tax Savings",
                "action": "Move funds to tax savings"
            });

            // Store in DB - wait we can't do that easily without DbStore from orchestrator,
            // but we can generate the action for it to be picked up by the executor/orchestrator
            return self.orchestrator.execute_action(
                DepartmentType::Finance,
                format!("You have collected ${} in sales tax this month. Move to tax savings?", tax_liability),
                event.tenant_id.clone(),
                risk,
                advisory_payload,
            ).await.map(|_| ());
        }

        self.orchestrator.execute_action(
            DepartmentType::Finance,
            "Record deposit and track payment".to_string(),
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
        Ok(vec![])
    }

    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String> {
        self.orchestrator.execute_action(self.department_type(), description.clone(), tenant_id.clone(), risk, serde_json::json!({})).await
    }
}

#[async_trait::async_trait]
impl BaseAgent for FinanceAgent {
    fn agent_id(&self) -> String {
        "finance_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::Scheduled
    }

    async fn execute(&self, _payload: Value) -> Result<(), String> {
        // Run scheduled worker to aggregate weekly sales data.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestration::departments::orchestrator::{DepartmentOrchestrator};
    use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, ActionRisk};
    use crate::db::DbStore;
    use sqlx::sqlite::SqlitePoolOptions;
    use std::sync::Arc;
    use serde_json::json;

    // We can't easily mock DepartmentOrchestrator since it's a concrete struct and not a trait.
    // Instead we will test using a real instance with an in-memory DB.

    async fn setup_test_db() -> crate::db::DB {
        let sqlite_pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE ai_budgets (
                tenant_id TEXT PRIMARY KEY,
                token_allowance INTEGER NOT NULL,
                tokens_used INTEGER NOT NULL DEFAULT 0,
                cost_usd DOUBLE PRECISION NOT NULL DEFAULT 0.0,
                overage_allowed BOOLEAN NOT NULL DEFAULT FALSE,
                reset_date TEXT NOT NULL
            );
            CREATE TABLE agent_approvals (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                department TEXT NOT NULL,
                description TEXT NOT NULL,
                risk_level TEXT NOT NULL,
                payload JSON NOT NULL,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE agent_configs (
                tenant_id TEXT,
                department TEXT,
                config JSON,
                PRIMARY KEY (tenant_id, department)
            );
            CREATE TABLE ohc_universal_ledger (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                event_type TEXT NOT NULL,
                department TEXT NOT NULL,
                payload JSON DEFAULT '{}',
                created_at TEXT NOT NULL
            );"
        )
        .execute(&sqlite_pool)
        .await
        .unwrap();

        crate::db::DB {
            pool: sqlx::PgPoolOptions::new()
                .connect_lazy("postgres://dummy")
                .unwrap(),
            store: DbStore::Sqlite(sqlite_pool),
        }
    }

    #[tokio::test]
    async fn test_finance_agent_payment_received() {
        let db = setup_test_db().await;
        // Construct the orchestrator properly
        let mut _memory_mock = ohc_builtin_agent::memory_store::VectorRepository::new();
        // Just mock the minimum needed or check if the actual event returns ok
        let orchestrator = Arc::new(DepartmentOrchestrator::new(Arc::new(db), _memory_mock));
        let agent = FinanceAgent::new(orchestrator.clone());

        let payload = json!({
            "amount": 500.0,
        });

        let event = DepartmentEvent {
            id: "test_event".to_string(),
            tenant_id: "test_tenant".to_string(),
            event_type: "tenant.payment.received".to_string(),
            payload,
            created_at: chrono::Utc::now(),
        };

        let result = agent.handle_event(&event).await;
        assert!(result.is_ok());

        // Verify the approval request was created with correct tax math
        // In reality, because sqlite is passed, we can query agent_approvals
        let mut approvals = orchestrator.get_pending_approvals("test_tenant").await.unwrap();
        assert_eq!(approvals.len(), 1);

        let approval = approvals.pop().unwrap();
        assert_eq!(approval.department, DepartmentType::Finance);
        assert!(approval.description.contains("You have collected $50")); // 10% of 500

        let payload_json = approval.payload;
        assert_eq!(payload_json["tax_liability_estimate"], 50.0);
        assert_eq!(payload_json["virtual_envelope"], "Tax Savings");
    }
}
