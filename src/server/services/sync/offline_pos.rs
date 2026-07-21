use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct OfflineMutation {
    pub transaction_id: String,
    pub timestamp: Option<String>,
    pub product_id: String,
    pub quantity_deducted: i32,
    pub amount: Option<i64>,
    pub payment_method: Option<String>,
    pub payment_intent_id: Option<String>,
    pub currency: Option<String>,
    pub mutation_type: Option<String>,
    pub payload: Option<String>,
}

pub struct CRDTOfflineSynchronizer;

impl CRDTOfflineSynchronizer {
    pub async fn process_batch(
        pool: &PgPool,
        tenant_id: &str,
        mutations: &[OfflineMutation],
    ) -> Result<(usize, Vec<String>), String> {
        let mut failed_count = 0;
        let mut failed_transactions = Vec::new();

        for mutation in mutations {
            if mutation.mutation_type.as_deref() == Some("draft_quote") {
                continue;
            }

            let locker: Box<dyn crate::orchestration::locks::DistributedLock> = if crate::is_standalone_runtime() {
                if let Some(p) = crate::db::get_sqlite_pool_if_exists() {
                    Box::new(crate::orchestration::locks::StandaloneLock::with_pool(p))
                } else {
                    Box::new(crate::orchestration::locks::StandaloneLock::new())
                }
            } else {
                if let Some(client) = crate::get_redis_client() {
                    Box::new(crate::orchestration::locks::RedisLock::new(client))
                } else {
                    if let Some(p) = crate::db::get_sqlite_pool_if_exists() {
                        Box::new(crate::orchestration::locks::StandaloneLock::with_pool(p))
                    } else {
                        Box::new(crate::orchestration::locks::StandaloneLock::new())
                    }
                }
            };

            let mut _lock_guard = match locker.acquire_resource(tenant_id, "inventory", &mutation.product_id).await {
                Ok(guard) => guard,
                Err(_) => {
                    tracing::warn!("Failed to acquire lock for offline sync reconciliation: inventory:{}", mutation.product_id);
                    failed_count += 1;
                    failed_transactions.push(mutation.transaction_id.clone());
                    continue;
                }
            };

            let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

            let current_stock_res: Result<(i32,), sqlx::Error> = sqlx::query_as(
                "SELECT available_count FROM inventory_levels WHERE variant_id = $1 AND tenant_id = $2 FOR UPDATE"
            )
            .bind(&mutation.product_id)
            .bind(tenant_id)
            .fetch_one(&mut *tx)
            .await;

            let mut success = false;
            let mut shortage_found = false;

            if let Ok((stock,)) = current_stock_res {
                let qty_i32 = mutation.quantity_deducted as i32;
                if stock < qty_i32 {
                    shortage_found = true;
                    let tx_id = mutation.transaction_id.clone();
                    let product_id = mutation.product_id.clone();

                    // trigger Operations Agent conflict resolution
                    let action_request_id = uuid::Uuid::new_v4().to_string();
                    let action_payload = serde_json::json!({
                        "transaction_id": tx_id,
                        "product_id": product_id,
                        "suggested_action": "Restock Item",
                        "reason": "Lock contention on limited item during checkout sync"
                    }).to_string();

                    let _ = sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, action_type, status, confidence_score, product_id, payload, source, agent_type, created_at, updated_at) VALUES ($1, $2, 'Reorder', 'Pending', 0.95, $3, $4::jsonb, 'pos_sync_service', 'operations', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                        .bind(&action_request_id)
                        .bind(&tenant_id)
                        .bind(&product_id)
                        .bind(&action_payload)
                        .execute(&mut *tx)
                        .await;

                    let cs_action_request_id = uuid::Uuid::new_v4().to_string();
                    let cs_payload = serde_json::json!({
                        "transaction_id": tx_id,
                        "product_id": product_id,
                        "suggested_action": "Notify Customer of Out of Stock",
                        "reason": "Lock contention on limited item during checkout sync"
                    }).to_string();

                    let _ = sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, action_type, status, confidence_score, product_id, payload, source, agent_type, created_at, updated_at) VALUES ($1, $2, 'NotifyCustomer', 'Pending', 0.99, $3, $4::jsonb, 'pos_sync_service', 'customer_success', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                        .bind(&cs_action_request_id)
                        .bind(&tenant_id)
                        .bind(&product_id)
                        .bind(&cs_payload)
                        .execute(&mut *tx)
                        .await;
                }

                let _ = sqlx::query("UPDATE inventory_levels SET available_count = GREATEST(0, available_count - $1) WHERE variant_id = $2 AND tenant_id = $3")
                    .bind(qty_i32)
                    .bind(&mutation.product_id)
                    .bind(&tenant_id)
                    .execute(&mut *tx)
                    .await;

                let inventory_level_res = sqlx::query("SELECT id FROM inventory_levels WHERE variant_id = $1 AND tenant_id = $2")
                    .bind(&mutation.product_id)
                    .bind(&tenant_id)
                    .fetch_optional(&mut *tx)
                    .await;
                if let Ok(Some(row)) = inventory_level_res {
                    use sqlx::Row;
                    let level_id: String = row.get("id");
                    let tx_id_inv = uuid::Uuid::new_v4().to_string();
                    let _ = sqlx::query("INSERT INTO inventory_transactions (id, tenant_id, inventory_level_id, type, quantity_change) VALUES ($1, $2, $3, 'SALE', -$4)")
                        .bind(&tx_id_inv)
                        .bind(&tenant_id)
                        .bind(level_id)
                        .bind(qty_i32)
                        .execute(&mut *tx)
                        .await;
                }

                let _ = sqlx::query("UPDATE products SET pn_counter_n = pn_counter_n + $1, inventory_count = GREATEST(0, pn_counter_p - (pn_counter_n + $1)), available_quantity = GREATEST(0, available_quantity - $1) WHERE id = $2 AND tenant_id = $3")
                    .bind(qty_i32)
                    .bind(&mutation.product_id)
                    .bind(&tenant_id)
                    .execute(&mut *tx)
                    .await;

                if let Some(client) = crate::get_redis_client() {
                    if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                        let invalidation_topic = "cache_invalidation_events";
                        let invalidation_payload = serde_json::json!({
                            "event": "inventory.updated",
                            "tags": [
                                format!("tenant-id:{}", tenant_id),
                                format!("entity:product:{}", mutation.product_id)
                            ]
                        }).to_string();
                        let _: Result<(), _> = redis::cmd("PUBLISH").arg(invalidation_topic).arg(invalidation_payload).query_async(&mut conn).await;
                    }
                }

                success = true;
            } else {
                tracing::warn!("Product {} not found or unauthorized for tenant {}", mutation.product_id, tenant_id); // pii-safe // pii-safe
            }

            if success {
                // Record successful pos offline transaction
                let mutation_ts = mutation.timestamp.clone().unwrap_or_else(|| chrono::Utc::now().to_rfc3339());
                let res = sqlx::query(
                    "INSERT INTO pos_offline_transactions (id, tenant_id, client_id, status, amount_cents, currency, payload, created_at, updated_at, _sync_status, terminal_id)
                        VALUES ($1, $2, $3, 'RESOLVED', $4, $5, $6::jsonb, $7::timestamptz, $7::timestamptz, 'synced', $8)
                        ON CONFLICT (id) DO UPDATE SET status = 'RESOLVED', payload = EXCLUDED.payload, updated_at = EXCLUDED.updated_at, _sync_status = 'synced'
                        WHERE pos_offline_transactions.updated_at < EXCLUDED.updated_at"
                )
                .bind(&mutation.transaction_id)
                .bind(tenant_id)
                .bind(&mutation.transaction_id)
                .bind(mutation.amount.unwrap_or(0))
                .bind(mutation.currency.as_deref().unwrap_or("USD"))
                .bind(serde_json::to_value(mutation).unwrap())
                .bind(&mutation_ts)
                .bind(mutation.terminal_id.clone())
                .execute(&mut *tx)
                .await;

                if res.is_ok() {
                    let _ = tx.commit().await;
                    continue;
                } else {
                    tracing::error!("Failed to record pos transaction: {:?}", res.err());
                }
            }

            // If we reach here, it's a failure
            failed_count += 1;
            failed_transactions.push(mutation.transaction_id.clone());

            let _ = tx.rollback().await;

            // Record failure in pos_offline_transactions
            let mutation_ts = mutation.timestamp.clone().unwrap_or_else(|| chrono::Utc::now().to_rfc3339());
            let _ = sqlx::query(
                "INSERT INTO pos_offline_transactions (id, tenant_id, client_id, status, amount_cents, currency, payload, created_at, updated_at, _sync_status, terminal_id)
                 VALUES ($1, $2, $3, 'FAILED', $4, $5, $6::jsonb, $7::timestamptz, $7::timestamptz, 'synced', $8)
                 ON CONFLICT (id) DO UPDATE SET status = 'FAILED', payload = EXCLUDED.payload, updated_at = EXCLUDED.updated_at, _sync_status = 'synced'
                 WHERE pos_offline_transactions.updated_at < EXCLUDED.updated_at"
            )
            .bind(&mutation.transaction_id)
            .bind(tenant_id)
            .bind(&mutation.transaction_id)
            .bind(mutation.amount.unwrap_or(0))
            .bind(mutation.currency.as_deref().unwrap_or("USD"))
            .bind(serde_json::to_value(mutation).unwrap())
            .bind(&mutation_ts)
            .bind(mutation.terminal_id.clone())
            .execute(pool)
            .await;

            // Enqueue Operations AI Agent job for graceful failure handling
            let agent_payload = serde_json::json!({
                "workflow": "ohc_business_swarm",
                "task": "Handle offline POS sync failure",
                "context": format!("Transaction {} failed to sync offline due to inventory discrepancy or decline.", mutation.transaction_id),
                "action": "OperationsAgent: generate a plain-language alert for the business owner and draft a follow-up message to the customer regarding the declined offline transaction."
            }).to_string();

            let _ = sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload) VALUES ($1, $2, 'agent_task', $3::jsonb)"
            )
            .bind(Uuid::new_v4().to_string())
            .bind(tenant_id)
            .bind(agent_payload)
            .execute(pool)
            .await;
        }

        Ok((failed_count, failed_transactions))
    }
}
