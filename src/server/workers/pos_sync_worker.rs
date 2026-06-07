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
        let transaction_id = payload.get("pos_transaction_id").or_else(|| payload.get("transaction_id")).and_then(|v| v.as_str()).unwrap_or("");

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

        if let Some(payload_str) = payload.get("payload").and_then(|v| v.as_str()) {
            if let Ok(mutations) = serde_json::from_str::<serde_json::Value>(payload_str) {
                if let Some(mutations_array) = mutations.as_array() {
                    let cache = crate::builder::edge::get_edge_cache();
                    cache.invalidate_by_tag(&format!("tenant-id:{}", job.tenant_id)).await;

                    for mutation in mutations_array {
                        if let Some(product_id) = mutation.get("product_id").and_then(|v| v.as_str()) {
                            let quantity_deducted = mutation.get("quantity")
                                .or_else(|| mutation.get("quantity_deducted"))
                                .and_then(|v| v.as_i64())
                                .unwrap_or(1);

                            sqlx::query("UPDATE products SET inventory_count = GREATEST(0, inventory_count - $1) WHERE id = $2")
                                .bind(quantity_deducted)
                                .bind(product_id)
                                .execute(&mut *tx)
                                .await
                                .unwrap();

                            cache.invalidate_by_tag(&format!("entity:product:{}", product_id)).await;
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
            "payload": "[{\"product_id\": \"prod-worker-test-1\", \"quantity\": 2}]",
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
}
