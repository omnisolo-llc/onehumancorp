use crate::orchestration::departments::orchestrator::{
    AgentTriggerType, BaseAgent, Department, DepartmentOrchestrator,
};
use crate::orchestration::departments::types::{
    ActionRisk, ApprovalRequest, DepartmentConfig, DepartmentEvent, DepartmentType,
};

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
            "inventory.sync.conflict".to_string(),
            "tenant.inventory.updated".to_string(),
            "pos_sales".to_string(),
            "tenant.pricing.updated".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "tenant.quote.draft_requested" {
            let _start_time_str = event
                .payload
                .get("suggested_time")
                .and_then(|v| v.as_str())
                .unwrap_or("Tomorrow at 2 PM");
            let start_time = chrono::Utc::now() + chrono::Duration::days(1);
            let end_time = start_time + chrono::Duration::hours(2);

            let service_name = event
                .payload
                .get("service")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
                .to_string();
            let proposed_slot_id = uuid::Uuid::new_v4().to_string();

            let pool = crate::db::get_pool();

            // 1. DB check for overlaps
            if std::env::var("OHC_DATABASE_URL").is_ok() || std::env::var("DATABASE_URL").is_ok() {
                let overlap_count: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM bookings WHERE tenant_id = $1 AND service_id = $2 AND start_time < $4 AND end_time > $3 AND COALESCE(status, 'pending') <> 'cancelled'"
                )
                .bind(&event.tenant_id)
                .bind(&service_name)
                .bind(start_time)
                .bind(end_time)
                .fetch_one(&pool)
                .await.unwrap_or(0);

                if overlap_count > 0 {
                    tracing::warn!("Time slot already booked in DB for {}", event.tenant_id);
                    return Err("Time slot already booked".to_string());
                }
            }

            // 2. Redis lock
            let redis_url =
                std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
            if let Ok(redis_client) = redis::Client::open(redis_url.clone()) {
                if let Ok(mut conn) = redis_client.get_multiplexed_tokio_connection().await {
                    let key = format!(
                        "ohc:lock:{}:capacity:{}:{}_{}",
                        event.tenant_id,
                        service_name,
                        start_time.timestamp(),
                        end_time.timestamp()
                    );
                    let acquired: Result<bool, redis::RedisError> = redis::cmd("SET")
                        .arg(&key)
                        .arg(&proposed_slot_id)
                        .arg("NX")
                        .arg("EX")
                        .arg(300)
                        .query_async(&mut conn)
                        .await;
                    match acquired {
                        Ok(true) => tracing::info!("Acquired Redis capacity lock: {}", key),
                        _ => {
                            tracing::warn!("Failed to acquire Redis capacity lock for {}", key);
                            return Err(
                                "Time slot is currently being held by another request".to_string()
                            );
                        }
                    }
                }
            }

            // 3. DB soft lock
            if std::env::var("OHC_DATABASE_URL").is_ok() || std::env::var("DATABASE_URL").is_ok() {
                let res = sqlx::query(
                    "INSERT INTO booking_slots (id, tenant_id, service_id, start_time, end_time, status) VALUES ($1, $2, $3, $4, $5, 'soft_locked')"
                )
                .bind(&proposed_slot_id)
                .bind(&event.tenant_id)
                .bind(&service_name)
                .bind(start_time)
                .bind(end_time)
                .execute(&pool)
                .await;

                if let Err(e) = res {
                    tracing::error!("OperationsAgent failed to insert soft lock into DB: {}", e);
                    return Err(e.to_string());
                }
            }

            let mut new_payload = event.payload.clone();
            if let Some(obj) = new_payload.as_object_mut() {
                obj.insert(
                    "proposed_slot_id".to_string(),
                    serde_json::json!(proposed_slot_id),
                );
                obj.insert(
                    "start_time".to_string(),
                    serde_json::json!(start_time.to_rfc3339()),
                );
                obj.insert(
                    "end_time".to_string(),
                    serde_json::json!(end_time.to_rfc3339()),
                );
            }

            let ready_event = crate::orchestration::departments::types::DepartmentEvent {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: event.tenant_id.clone(),
                event_type: "tenant.quote.ready_for_review".to_string(),
                payload: new_payload,
            };

            return self
                .orchestrator
                .dispatch_event(ready_event)
                .await
                .map_err(|e| e.to_string());
        }

        if event.event_type == "tenant.quote.approved" {
            let proposed_slot_id = event
                .payload
                .get("proposed_slot_id")
                .and_then(|v| v.as_str());

            if let Some(slot_id) = proposed_slot_id {
                let pool = crate::db::get_pool();
                let update_res = sqlx::query(
                    "UPDATE booking_slots SET status = 'booked' WHERE id = $1 AND tenant_id = $2 AND status = 'soft_locked' RETURNING service_id, start_time, end_time"
                )
                .bind(slot_id)
                .bind(&event.tenant_id)
                .fetch_optional(&pool)
                .await;

                match update_res {
                    Ok(Some(row)) => {
                        use sqlx::Row;
                        let service_id: String = row.try_get("service_id").unwrap_or_default();
                        let start_time: chrono::DateTime<chrono::Utc> =
                            row.try_get("start_time").unwrap_or(chrono::Utc::now());
                        let end_time: chrono::DateTime<chrono::Utc> =
                            row.try_get("end_time").unwrap_or(chrono::Utc::now());
                        let customer_id = event
                            .payload
                            .get("customer_id")
                            .and_then(|v| v.as_str())
                            .and_then(|v| uuid::Uuid::parse_str(v).ok());

                        let booking_id = uuid::Uuid::new_v4().to_string();
                        let checkout_url =
                            event.payload.get("checkout_url").and_then(|v| v.as_str());
                        let initial_status = if checkout_url.is_some() {
                            "pending_payment"
                        } else {
                            "pending"
                        };

                        let _ = sqlx::query(
                            "INSERT INTO bookings (id, tenant_id, customer_id, service_id, start_time, end_time, status) VALUES ($1, $2, $3, $4, $5, $6, $7)"
                        )
                        .bind(&booking_id)
                        .bind(&event.tenant_id)
                        .bind(customer_id)
                        .bind(service_id)
                        .bind(start_time)
                        .bind(end_time)
                        .bind(initial_status)
                        .execute(&pool)
                        .await;

                        tracing::info!("Operations Agent: Transitioned slot {} to booked and created booking {}", slot_id, booking_id);
                    }
                    Ok(None) => tracing::error!(
                        "Operations Agent: Proposed slot {} not found or not soft locked",
                        slot_id
                    ),
                    Err(e) => {
                        tracing::error!("Operations Agent: Failed to update booking slot: {}", e)
                    }
                }
            }
            return Ok(());
        }

        if event.event_type == "tenant.inventory.updated"
            || event.event_type == "tenant.pricing.updated"
        {
            let product_id = event
                .payload
                .get("product_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let cache = crate::builder::edge::get_edge_cache();
            cache
                .invalidate_by_tag(&format!("tenant-id:{}", event.tenant_id))
                .await;
            if !product_id.is_empty() {
                cache
                    .invalidate_by_tag(&format!("entity:product:{}", product_id))
                    .await;
            }

            // Pre-warm (regenerate) cache in background
            if let Ok(tenant_uuid) = uuid::Uuid::parse_str(&event.tenant_id) {
                let pool = crate::db::get_pool();
                let cache_clone = cache.clone();
                tokio::spawn(async move {
                    if let Ok(sites) = crate::builder::db::list_sites(&pool, tenant_uuid).await {
                        if let Some(site) = sites.first() {
                            let site_id = site.id;
                            let cache_key = format!("edge_site_{}_{}", tenant_uuid, site_id);
                            let _ = crate::builder::edge::regenerate_cache(
                                pool.clone(),
                                tenant_uuid,
                                site_id,
                                cache_key,
                                cache_clone,
                            )
                            .await;
                        }
                    }
                });
            }
        }

        if event.event_type == "POS_SALE_COMPLETED" {
            tracing::info!(
                "Operations Agent: Handling POS sale completion for tenant {}",
                event.tenant_id
            );
            return Ok(());
        }

        if event.event_type == "tenant.inventory.updated" {
            let product_id = event
                .payload
                .get("product_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let cache = crate::builder::edge::get_edge_cache();
            cache
                .invalidate_by_tag(&format!("tenant-id:{}", event.tenant_id))
                .await;
            if !product_id.is_empty() {
                cache
                    .invalidate_by_tag(&format!("entity:product:{}", product_id))
                    .await;
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
                let notes = event
                    .payload
                    .get("notes")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if !notes.is_empty() {
                    // Extract tenant language preference here if available, defaulting to English/Arabic for now.
                    format!("Translate order notes to the tenant's preferred language for the kitchen: {}", notes)
                } else {
                    "Process Order & Update Inventory".to_string()
                }
            }
            "tenant.order.updated" => {
                let status = event
                    .payload
                    .get("status")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let order_id = event
                    .payload
                    .get("order_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                if status == "Ready" {
                    format!(
                        "Notify customer that order {} is ready for pickup via SMS/WhatsApp",
                        order_id
                    )
                } else {
                    format!("Order {} status updated to {}", order_id, status)
                }
            }
            "LowStockAlert" => {
                let _product_id = event
                    .payload
                    .get("product_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                let remaining_stock = event
                    .payload
                    .get("remaining_stock")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                let _msg = event
                    .payload
                    .get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                let product_name = event
                    .payload
                    .get("product_title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown item");

                // Enrich payload with Quartermaster agent supply order details
                let mut new_payload = event.payload.clone();
                if let Some(obj) = new_payload.as_object_mut() {
                    obj.insert(
                        "feature_type".to_string(),
                        serde_json::json!("supply_order"),
                    );
                    obj.insert(
                        "vendor_name".to_string(),
                        serde_json::json!("Local Supplier"),
                    );
                    obj.insert(
                        "vendor_contact".to_string(),
                        serde_json::json!("Sam (WhatsApp)"),
                    );
                    obj.insert("est_runout_days".to_string(), serde_json::json!(2));
                    obj.insert(
                        "suggested_reorder_quantity".to_string(),
                        serde_json::json!(500),
                    );
                    obj.insert(
                        "draft_message".to_string(),
                        serde_json::json!(format!(
                            "Hi Sam, please send 500 more {} to the Main St location.",
                            product_name
                        )),
                    );
                    if remaining_stock == 0 {
                        obj.insert(
                            "description".to_string(),
                            serde_json::json!(format!(
                                "{} sold out. Would you like to draft a restock order?",
                                product_name
                            )),
                        );
                    } else {
                        obj.insert(
                            "description".to_string(),
                            serde_json::json!(format!(
                                "Supply Alert: {} running low. Order drafted.",
                                product_name
                            )),
                        );
                    }
                }

                let desc = if remaining_stock == 0 {
                    format!(
                        "{} sold out. Would you like to draft a restock order?",
                        product_name
                    )
                } else {
                    format!("Supply Alert: {} running low. Order drafted.", product_name)
                };

                // Trigger push notification directly for owner visibility
                let _ = self
                    .orchestrator
                    .notify_owner(&event.tenant_id, &desc)
                    .await;

                return self
                    .orchestrator
                    .execute_action(
                        DepartmentType::Operations,
                        desc,
                        event.tenant_id.clone(),
                        risk,
                        new_payload,
                    )
                    .await
                    .map(|_| ());
            }
            "inventory.sync.conflict" => {
                let msg = event
                    .payload
                    .get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if msg.contains("Operations has drafted an email to the online customer") {
                    msg.to_string()
                } else {
                    let transaction_id = event
                        .payload
                        .get("transaction_id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown");
                    let product_id = event
                        .payload
                        .get("product_id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown");
                    let expected = event
                        .payload
                        .get("expected_stock")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);
                    let actual = event
                        .payload
                        .get("actual_stock")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);
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
                        let _ = self
                            .orchestrator
                            .execute_action(
                                DepartmentType::Operations,
                                format!(
                                    "Auto-resolving inventory conflict for {} (tx: {})",
                                    product_id, transaction_id
                                ),
                                event.tenant_id.clone(),
                                ActionRisk::AutoExecute,
                                event.payload.clone(),
                            )
                            .await;
                        return Ok(());
                    }

                    // Create an actionable task in the owner's Triage Feed asking how to resolve it
                    let pool = crate::db::get_pool();
                    let triage_id = uuid::Uuid::new_v4().to_string();
                    let action_id = uuid::Uuid::new_v4().to_string();

                    let context_json = serde_json::json!({
                        "message": llm_response,
                        "transaction_id": transaction_id,
                        "product_id": product_id,
                        "expected_stock": expected,
                        "actual_stock": actual
                    })
                    .to_string();

                    if let Err(e) = sqlx::query(
                        "INSERT INTO triage_items (id, tenant_id, source, priority, context, status) VALUES ($1, $2, 'Operations Agent', 'high', $3, 'pending')"
                    )
                    .bind(&triage_id)
                    .bind(&event.tenant_id)
                    .bind(&context_json)
                    .execute(&pool)
                    .await {
                        tracing::error!("Failed to insert triage item for inventory conflict: {}", e);
                    }

                    let triage_payload = serde_json::json!({
                        "action": "resolve_inventory_conflict",
                        "transaction_id": transaction_id,
                        "product_id": product_id
                    })
                    .to_string();

                    if let Err(e) = sqlx::query(
                        "INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload) VALUES ($1, $2, $3, 'operations_decision', $4::jsonb)"
                    )
                    .bind(&action_id)
                    .bind(&triage_id)
                    .bind(&event.tenant_id)
                    .bind(&triage_payload)
                    .execute(&pool)
                    .await {
                        tracing::error!("Failed to insert triage proposed action for inventory conflict: {}", e);
                    }

                    llm_response
                }
            }
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

        self.orchestrator
            .execute_action(
                DepartmentType::Operations,
                action_description,
                event.tenant_id.clone(),
                risk,
                event.payload.clone(),
            )
            .await?;

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
        Some(DepartmentConfig {
            tone_of_voice: "professional".to_string(),
            auto_approve_limits: 10.0,
        })
    }

    async fn query_memory(&self, _query: &str) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    async fn request_approval(
        &self,
        description: String,
        tenant_id: String,
        risk: ActionRisk,
    ) -> Result<ApprovalRequest, String> {
        self.orchestrator
            .execute_action(
                self.department_type(),
                description.clone(),
                tenant_id.clone(),
                risk,
                serde_json::json!({}),
            )
            .await
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
                id TEXT PRIMARY KEY,
                business_name TEXT,
                plan_tier TEXT
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
        sqlx::query("INSERT INTO tenants (id, business_name, plan_tier) VALUES ('tenant-ops', 'Ops Test', 'starter')")
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
            .find(|approval| {
                approval
                    .description
                    .contains("Prepare subscription fulfillment batch")
            })
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
