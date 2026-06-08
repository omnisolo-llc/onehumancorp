
use std::sync::Arc;
use crate::db::DB;

pub struct PosSyncWorker {
    db: Arc<DB>,
}

impl PosSyncWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn handle(&self, job: crate::queue::Job) -> Result<Result<(), String>, String> {
        let payload: serde_json::Value = serde_json::from_str(&job.payload).unwrap();
        let transaction_id = payload.get("transaction_id").and_then(|v| v.as_str())
            .or_else(|| payload.get("pos_transaction_id").and_then(|v| v.as_str()))
            .unwrap_or("");

        let mut tx = match self.db.pool.begin().await {
            Ok(tx) => tx,
            Err(e) => {
                tracing::error!("Failed to begin transaction: {}", e);
                return Err("Failed to begin db transaction".into());
            }
        };

        if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &job.tenant_id).await {
            tracing::error!("Failed to set org context: {}", e);
            return Err("Failed to set org context".into());
        }

        sqlx::query("UPDATE pos_offline_transactions SET status = 'RESOLVED' WHERE id = $1")
            .bind(transaction_id)
            .execute(&mut *tx)
            .await
            .unwrap();

        if let Some(mutation) = payload.get("mutation") {
            let product_id = mutation["product_id"].as_str().unwrap();
            let quantity_deducted = mutation["quantity_deducted"].as_i64().unwrap();

            let current_stock_res = sqlx::query("SELECT inventory_count FROM products WHERE id = $1 AND tenant_id = $2 FOR UPDATE")
                .bind(product_id)
                .bind(&job.tenant_id)
                .fetch_optional(&mut *tx)
                .await;

            if let Ok(Some(row)) = current_stock_res {
                let stock: i32 = sqlx::Row::get(&row, "inventory_count");
                let is_conflict = stock < quantity_deducted as i32;

                let _ = sqlx::query("UPDATE products SET inventory_count = GREATEST(0, inventory_count - $1) WHERE id = $2 AND tenant_id = $3")
                    .bind(quantity_deducted)
                    .bind(product_id)
                    .bind(&job.tenant_id)
                    .execute(&mut *tx)
                    .await;

                let new_stock = std::cmp::max(0, stock - quantity_deducted as i32);
                if new_stock <= 5 {
                    let existing_action: Option<(String,)> = sqlx::query_as("SELECT id FROM agent_action_requests WHERE product_id = $1 AND tenant_id = $2 AND status = 'Pending' AND action_type = 'RestockItem'")
                        .bind(product_id).bind(&job.tenant_id).fetch_optional(&mut *tx).await.unwrap_or(None);
                    if existing_action.is_none() {
                        let action_id = uuid::Uuid::new_v4().to_string();
                        let action_payload = serde_json::json!({
                            "reason": "Low inventory detected during offline POS sync",
                            "current_stock": new_stock
                        }).to_string();

                        let _ = sqlx::query(
                            "INSERT INTO agent_action_requests (id, tenant_id, action_type, status, confidence_score, product_id, payload) VALUES ($1, $2, 'RestockItem', 'Pending', 0.95, $3, $4::jsonb)"
                        )
                        .bind(&action_id).bind(&job.tenant_id).bind(product_id).bind(&action_payload)
                        .execute(&mut *tx).await;
                    }
                }

                if is_conflict {
                    let ai_task_id = uuid::Uuid::new_v4().to_string();
                    let ai_payload = serde_json::json!({
                        "transaction_id": transaction_id,
                        "product_id": product_id,
                        "expected_stock": quantity_deducted,
                        "actual_stock": stock,
                        "message": format!("Heads up! A pop-up sale overlapped with an online order for {}. Operations has drafted an email to the online customer.", product_id)
                    }).to_string();

                    let _ = sqlx::query(
                        "INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status)
                         VALUES ($1, $2, 'operations', 'PosSyncFailure', $3::jsonb, 'PENDING')"
                    )
                    .bind(&ai_task_id)
                    .bind(&job.tenant_id)
                    .bind(&ai_payload)
                    .execute(&mut *tx)
                    .await;
                }

                let cache = crate::builder::edge::get_edge_cache();
                let _ = cache.invalidate_by_tag(&format!("entity:product:{}", product_id)).await;
                let _ = cache.invalidate_by_tag(&format!("tenant-id:{}", job.tenant_id)).await;
            }
        }

        // Support payload formatted directly for the transaction items array
        if let Some(items) = payload.get("payload") {
            if let Some(items_str) = items.as_str() {
                if let Ok(items_array) = serde_json::from_str::<Vec<serde_json::Value>>(items_str) {
                    for item in items_array {
                        let product_id = item.get("product_id").and_then(|v| v.as_str()).unwrap_or("");
                        let qty = item.get("quantity").and_then(|v| v.as_i64()).unwrap_or(1);
                        if product_id.is_empty() { continue; }

                        let current_stock_res = sqlx::query("SELECT inventory_count FROM products WHERE id = $1 AND tenant_id = $2 FOR UPDATE")
                            .bind(product_id)
                            .bind(&job.tenant_id)
                            .fetch_optional(&mut *tx)
                            .await;

                        if let Ok(Some(row)) = current_stock_res {
                            let stock: i32 = sqlx::Row::get(&row, "inventory_count");
                            let is_conflict = stock < qty as i32;

                            let _ = sqlx::query("UPDATE products SET inventory_count = GREATEST(0, inventory_count - $1) WHERE id = $2 AND tenant_id = $3")
                                .bind(qty)
                                .bind(product_id)
                                .bind(&job.tenant_id)
                                .execute(&mut *tx)
                                .await;

                            let new_stock = std::cmp::max(0, stock - qty as i32);
                            if new_stock <= 5 {
                                let existing_action: Option<(String,)> = sqlx::query_as("SELECT id FROM agent_action_requests WHERE product_id = $1 AND tenant_id = $2 AND status = 'Pending' AND action_type = 'RestockItem'")
                                    .bind(product_id).bind(&job.tenant_id).fetch_optional(&mut *tx).await.unwrap_or(None);
                                if existing_action.is_none() {
                                    let action_id = uuid::Uuid::new_v4().to_string();
                                    let action_payload = serde_json::json!({
                                        "reason": "Low inventory detected during offline POS sync",
                                        "current_stock": new_stock
                                    }).to_string();

                                    let _ = sqlx::query(
                                        "INSERT INTO agent_action_requests (id, tenant_id, action_type, status, confidence_score, product_id, payload) VALUES ($1, $2, 'RestockItem', 'Pending', 0.95, $3, $4::jsonb)"
                                    )
                                    .bind(&action_id).bind(&job.tenant_id).bind(product_id).bind(&action_payload)
                                    .execute(&mut *tx).await;
                                }
                            }

                            if is_conflict {
                                let ai_task_id = uuid::Uuid::new_v4().to_string();
                                let ai_payload = serde_json::json!({
                                    "transaction_id": transaction_id,
                                    "product_id": product_id,
                                    "expected_stock": qty,
                                    "actual_stock": stock,
                                    "message": format!("Heads up! A pop-up sale overlapped with an online order for {}. Operations has drafted an email to the online customer.", product_id)
                                }).to_string();

                                let _ = sqlx::query(
                                    "INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status)
                                     VALUES ($1, $2, 'operations', 'PosSyncFailure', $3::jsonb, 'PENDING')"
                                )
                                .bind(&ai_task_id)
                                .bind(&job.tenant_id)
                                .bind(&ai_payload)
                                .execute(&mut *tx)
                                .await;
                            }

                            let cache = crate::builder::edge::get_edge_cache();
                            let _ = cache.invalidate_by_tag(&format!("entity:product:{}", product_id)).await;
                            let _ = cache.invalidate_by_tag(&format!("tenant-id:{}", job.tenant_id)).await;
                        }
                    }
                }
            }
        }

        sqlx::query("INSERT INTO ohc_universal_ledger (tenant_id, event_type, payload) VALUES ($1, 'offline_pos_sync', $2::jsonb)")
            .bind(&job.tenant_id)
            .bind(&job.payload)
            .execute(&mut *tx)
            .await
            .unwrap();

        tx.commit().await.unwrap();

        Ok(Ok(()))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_pos_sync_worker_logic() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
        let db = Arc::new(DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });
        let worker = PosSyncWorker::new(db.clone());

        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-worker-test', 'Worker Test Tenant') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count) VALUES ('prod-worker-test-1', 'tenant-worker-test', 'Test Prod', 10) ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO pos_offline_transactions (id, tenant_id, transaction_id, status) VALUES ('worker-tx-id', 'tenant-worker-test', 'tx-test-worker', 'PENDING') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();

        let job_payload = serde_json::json!({
            "transaction_id": "tx-test-worker",
            "mutation": {
                "product_id": "prod-worker-test-1",
                "quantity_deducted": 2,
                "amount": 5000,
                "transaction_id": "tx-test-worker"
            }
        });

        let job = crate::queue::Job {
            id: "job-1".to_string(),
            tenant_id: "tenant-worker-test".to_string(),
            job_type: "offline_pos_sync".to_string(),
            payload: job_payload.to_string(),
            status: "PROCESSING".to_string(),
            retry_count: 0,
            max_retries: 3,
            next_retry_at: Utc::now(),
            locked_until: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            parent_task_id: "".to_string(),
        };

        let handle = worker.handle(job);
        let res = handle.await.unwrap();
        assert!(res.is_ok());

        let count: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-worker-test-1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count.0, 8); // 10 - 2 = 8

        let tx_status: (String,) = sqlx::query_as("SELECT status FROM pos_offline_transactions WHERE transaction_id = 'tx-test-worker'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(tx_status.0, "RESOLVED");

        let ledger_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ohc_universal_ledger WHERE event_type = 'offline_pos_sync'")
            .fetch_one(&pool).await.unwrap();
        assert!(ledger_count.0 > 0);
    }

    #[tokio::test]
    async fn test_pos_sync_worker_low_stock_restock() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
        let db = Arc::new(DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });
        let worker = PosSyncWorker::new(db.clone());

        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-worker-test2', 'Worker Test Tenant 2') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count) VALUES ('prod-worker-test-low', 'tenant-worker-test2', 'Low Stock Prod', 6) ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO pos_offline_transactions (id, tenant_id, transaction_id, status) VALUES ('worker-tx-id2', 'tenant-worker-test2', 'tx-test-worker2', 'PENDING') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();

        let job_payload = serde_json::json!({
            "transaction_id": "tx-test-worker2",
            "mutation": {
                "product_id": "prod-worker-test-low",
                "quantity_deducted": 2,
                "amount": 5000,
                "transaction_id": "tx-test-worker2"
            }
        });

        let job = crate::queue::Job {
            id: "job-2".to_string(),
            tenant_id: "tenant-worker-test2".to_string(),
            job_type: "offline_pos_sync".to_string(),
            payload: job_payload.to_string(),
            status: "PROCESSING".to_string(),
            retry_count: 0,
            max_retries: 3,
            next_retry_at: Utc::now(),
            locked_until: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            parent_task_id: "".to_string(),
        };

        let handle = worker.handle(job);
        let res = handle.await.unwrap();
        assert!(res.is_ok());

        let count: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-worker-test-low'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count.0, 4); // 6 - 2 = 4 (<= 5)

        // Verify agent_action_requests was created
        let action_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM agent_action_requests WHERE product_id = 'prod-worker-test-low' AND status = 'Pending'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(action_count.0, 1);
    }
}
