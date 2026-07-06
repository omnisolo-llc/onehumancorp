use super::subscription_retention_worker::SubscriptionRetentionWorker;
use crate::db::{DB, DbStore};
use std::sync::Arc;
use uuid::Uuid;

async fn setup_test_db() -> Option<Arc<DB>> {
    let sqlite_pool = crate::db::create_sqlite_pool_for_test().await;
    let pool = crate::db::create_dummy_pg_pool().await;
    let db = DB {
        pool,
        store: DbStore::Sqlite(sqlite_pool.clone()),
    };

    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS tenants (id TEXT PRIMARY KEY, name TEXT);").execute(&sqlite_pool).await;
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS customers (id TEXT PRIMARY KEY, tenant_id TEXT, name TEXT);").execute(&sqlite_pool).await;
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS ohc_job_queue (id TEXT PRIMARY KEY, tenant_id TEXT, job_type TEXT, payload TEXT, status TEXT, next_retry_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(&sqlite_pool).await;
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS work_item (id TEXT PRIMARY KEY, tenant_id TEXT, customer_id TEXT, source TEXT, payload TEXT, status TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(&sqlite_pool).await;
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS agent_draft (id TEXT PRIMARY KEY, work_item_id TEXT, response TEXT, status TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(&sqlite_pool).await;
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS ohc_universal_ledger (id TEXT PRIMARY KEY, tenant_id TEXT, department TEXT, action_type TEXT, state_change TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(&sqlite_pool).await;

    Some(Arc::new(db))
}

#[tokio::test]
async fn test_subscription_retention_worker_processes_job() {
    let db = match setup_test_db().await {
        Some(db) => db,
        None => return,
    };

    let pool = match &db.store {
        DbStore::Sqlite(p) => p.clone(),
        _ => panic!("Expected Sqlite store"),
    };

    let tenant_id = Uuid::new_v4().to_string();
    let customer_id = Uuid::new_v4().to_string();
    let job_id = Uuid::new_v4().to_string();

    let _ = sqlx::query("INSERT INTO tenants (id, name) VALUES (?, 'Test Tenant')")
        .bind(&tenant_id)
        .execute(&pool).await;

    let _ = sqlx::query("INSERT INTO customers (id, tenant_id, name) VALUES (?, ?, 'AtRisk Customer')")
        .bind(&customer_id)
        .bind(&tenant_id)
        .execute(&pool).await;

    let _ = sqlx::query("INSERT INTO ohc_universal_ledger (id, tenant_id, department, action_type, state_change) VALUES (?, ?, 'Operations', 'booking', '{\"booking_id\":\"b123\"}')")
        .bind(Uuid::new_v4().to_string())
        .bind(&tenant_id)
        .execute(&pool).await;

    let payload = serde_json::json!({
        "subscription_id": "sub_123",
        "customer_id": customer_id
    });

    let _ = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'subscription_retention', ?, 'PENDING')")
        .bind(&job_id)
        .bind(&tenant_id)
        .bind(serde_json::to_string(&payload).unwrap())
        .execute(&pool).await;

    let worker = SubscriptionRetentionWorker::new(db.clone());
    let processed = worker.poll().await.unwrap();

    assert!(processed, "Worker should have processed the job");

    // Verify job status
    let status: String = sqlx::query_scalar("SELECT status FROM ohc_job_queue WHERE id = ?")
        .bind(&job_id)
        .fetch_one(&pool).await.unwrap();
    assert_eq!(status, "COMPLETED");

    // Skip testing insert statements that require RLS bypassing since sqlite testing harness skips it.
}
