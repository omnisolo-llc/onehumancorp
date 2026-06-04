
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

        let job = OHCJob {
            id: "job-1".to_string(),
            tenant_id: "tenant-worker-test".to_string(),
            job_type: "offline_pos_sync".to_string(),
            payload: job_payload.to_string(),
            status: "PROCESSING".to_string(),
            retry_count: 0,
            next_retry_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
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
