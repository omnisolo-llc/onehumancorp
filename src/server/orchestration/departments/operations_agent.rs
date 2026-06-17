use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};

pub struct OperationsAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl OperationsAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
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
            "tenant.order.created".to_string(),
            "tenant.order.updated".to_string(),
            "tenant.subscription.fulfillment_batch.created".to_string(),
            "LowStockAlert".to_string(),
            "InventoryConflictEvent".to_string(),
            "tenant.inventory.updated".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "tenant.inventory.updated" {
            let product_id = event.payload.get("product_id").and_then(|v| v.as_str()).unwrap_or("");
            let cache = crate::builder::edge::get_edge_cache();
            cache.invalidate_by_tag(&format!("tenant-id:{}", event.tenant_id)).await;
            if !product_id.is_empty() {
                cache.invalidate_by_tag(&format!("entity:product:{}", product_id)).await;
            }
        }

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

        let action_description = match event.event_type.as_str() {
            "tenant.order.created" => {
                let notes = event.payload.get("notes").and_then(|v| v.as_str()).unwrap_or("");
                if !notes.is_empty() {
                    // Extract tenant language preference here if available, defaulting to English/Arabic for now.
                    format!("Translate order notes to the tenant's preferred language for the kitchen: {}", notes)
                } else {
                    "Process Order & Update Inventory".to_string()
                }
            },
            "tenant.order.updated" => {
                let status = event.payload.get("status").and_then(|v| v.as_str()).unwrap_or("");
                let order_id = event.payload.get("order_id").and_then(|v| v.as_str()).unwrap_or("unknown");
                if status == "Ready" {
                    format!("Notify customer that order {} is ready for pickup via SMS/WhatsApp", order_id)
                } else {
                    format!("Order {} status updated to {}", order_id, status)
                }
            },
            "LowStockAlert" => {
                let product_id = event.payload.get("product_id").and_then(|v| v.as_str()).unwrap_or("unknown");
                let remaining_stock = event.payload.get("remaining_stock").and_then(|v| v.as_i64()).unwrap_or(0);
                let _msg = event.payload.get("message").and_then(|v| v.as_str()).unwrap_or("");

                let product_name = event.payload.get("product_title").and_then(|v| v.as_str()).unwrap_or("unknown item");

                // Enrich payload with Quartermaster agent supply order details
                let mut new_payload = event.payload.clone();
                if let Some(obj) = new_payload.as_object_mut() {
                    obj.insert("feature_type".to_string(), serde_json::json!("supply_order"));
                    obj.insert("vendor_name".to_string(), serde_json::json!("Local Supplier"));
                    obj.insert("vendor_contact".to_string(), serde_json::json!("Sam (WhatsApp)"));
                    obj.insert("est_runout_days".to_string(), serde_json::json!(2));
                    obj.insert("suggested_reorder_quantity".to_string(), serde_json::json!(500));
                    obj.insert("draft_message".to_string(), serde_json::json!(format!("Hi Sam, please send 500 more {} to the Main St location.", product_name)));
                    obj.insert("description".to_string(), serde_json::json!(format!("Supply Alert: {} running low. Order drafted.", product_name)));
                }

                return self.orchestrator.execute_action(
                    DepartmentType::Operations,
                    format!("Supply Alert: {} running low. Order drafted.", product_name),
                    event.tenant_id.clone(),
                    risk,
                    new_payload,
                ).await.map(|_| ());
            },
            "InventoryConflictEvent" => {
                let msg = event.payload.get("message").and_then(|v| v.as_str()).unwrap_or("");
                if msg.contains("Operations has drafted an email to the online customer") {
                    msg.to_string()
                } else {
                    let transaction_id = event.payload.get("transaction_id").and_then(|v| v.as_str()).unwrap_or("unknown");
                    let product_id = event.payload.get("product_id").and_then(|v| v.as_str()).unwrap_or("unknown");
                    let expected = event.payload.get("expected_stock").and_then(|v| v.as_i64()).unwrap_or(0);
                    let actual = event.payload.get("actual_stock").and_then(|v| v.as_i64()).unwrap_or(0);
                    let deficit = expected - actual; // e.g. quantity_deducted if offline stock was 0, but actually pos_sync_worker passes quantity_deducted as expected_stock

                    let llm_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                    let prompt = format!("Context: We have an offline sync conflict. The user tried to sell/deduct {} of item {} but the actual stock is {}. Transaction ID: {}. Please analyze this business conflict. If it can be safely merged (e.g., small negative stock allowed based on typical policies), output exactly 'AUTO_RESOLVE'. Otherwise, formulate a brief, polite question for the business owner to decide how to handle it (e.g. asking to cancel or restock).", expected, product_id, actual, transaction_id);

                    let llm_response = if !llm_key.is_empty() {
                        let llm = crate::minimax::MinimaxClient::new(llm_key);
                        llm.reason(&prompt).await.unwrap_or_else(|_| format!("We oversold the item {} by {}. Should I cancel the online order or draft a rush supply order for transaction {}?", product_id, deficit, transaction_id))
                    } else {
                        format!("We oversold the item {} by {}. Should I cancel the online order or draft a rush supply order for transaction {}?", product_id, deficit, transaction_id)
                    };

                    if llm_response.contains("AUTO_RESOLVE") {
                        // Let's create an auto-resolution action
                        let _ = self.orchestrator.execute_action(
                            DepartmentType::Operations,
                            format!("Auto-resolving inventory conflict for {} (tx: {})", product_id, transaction_id),
                            event.tenant_id.clone(),
                            ActionRisk::AutoExecute,
                            event.payload.clone(),
                        ).await;
                        return Ok(());
                    }

                    llm_response
                }
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
            event.payload.clone(),
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
