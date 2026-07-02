use sqlx::{Pool, Sqlite};
use crate::db::DbStore;
use crate::workers::booking_reengagement::BookingReengagementWorker;
use uuid::Uuid;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn test_booking_reengagement_worker_dormant_customer() {
    let pool = crate::db::create_sqlite_pool_for_test().await;

    // We create the actual tables since memory db is clean
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS ohc_job_queue (
            id TEXT PRIMARY KEY,
            tenant_id TEXT,
            job_type TEXT,
            status TEXT,
            payload TEXT,
            next_retry_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS bookings (
            id TEXT PRIMARY KEY,
            tenant_id TEXT,
            customer_id TEXT,
            start_time DATETIME
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS customers (
            id TEXT PRIMARY KEY,
            tenant_id TEXT,
            name TEXT
        )"
    ).execute(&pool).await.unwrap();

    // Use full actual schema columns to match the query used by the worker
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            parent_plan_id TEXT,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'PENDING',
            assigned_agent_id TEXT,
            dependencies TEXT DEFAULT '[]',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            _sync_status TEXT DEFAULT 'pending',
            version INTEGER DEFAULT 1,
            auto_dreamed BOOLEAN DEFAULT FALSE,
            action_risk TEXT,
            approval_status TEXT,
            proposed_content TEXT,
            priority TEXT
        )"
    ).execute(&pool).await.unwrap();

    let tenant_id = "tenant-1";
    let customer_id = "customer-1";

    sqlx::query("INSERT INTO customers (id, tenant_id, name) VALUES (?, ?, ?)")
        .bind(customer_id).bind(tenant_id).bind("Test Customer")
        .execute(&pool).await.unwrap();

    // Insert dormant customer (2 bookings, both old)
    sqlx::query("INSERT INTO bookings (id, tenant_id, customer_id, start_time) VALUES (?, ?, ?, datetime('now', '-20 days'))")
        .bind(Uuid::new_v4().to_string()).bind(tenant_id).bind(customer_id)
        .execute(&pool).await.unwrap();

    sqlx::query("INSERT INTO bookings (id, tenant_id, customer_id, start_time) VALUES (?, ?, ?, datetime('now', '-15 days'))")
        .bind(Uuid::new_v4().to_string()).bind(tenant_id).bind(customer_id)
        .execute(&pool).await.unwrap();

    let job_id = Uuid::new_v4().to_string();
    let payload = json!({ "customer_id": customer_id }).to_string();
    sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, status, payload) VALUES (?, ?, 'booking_reengagement_check', 'PENDING', ?)")
        .bind(&job_id).bind(tenant_id).bind(&payload)
        .execute(&pool).await.unwrap();

    let db = Arc::new(crate::db::DB {
        pool: crate::db::secure_pg_pool_options().acquire_timeout(std::time::Duration::from_millis(10)).connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap(),
        store: DbStore::Sqlite(pool.clone()),
    });
    let mut worker = BookingReengagementWorker::new(db);
    worker.poll_interval = std::time::Duration::from_millis(100);
    worker.start();
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    let task_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM shared_tasks WHERE organization_id = ?")
        .bind(tenant_id)
        .fetch_one(&pool).await.unwrap_or(0);

    assert_eq!(task_count, 1, "Expected 1 re-engagement task to be created");
}

#[tokio::test]
async fn test_booking_reengagement_worker_active_customer() {
    let pool = crate::db::create_sqlite_pool_for_test().await;

    // We create the actual tables since memory db is clean
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS ohc_job_queue (
            id TEXT PRIMARY KEY,
            tenant_id TEXT,
            job_type TEXT,
            status TEXT,
            payload TEXT,
            next_retry_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS bookings (
            id TEXT PRIMARY KEY,
            tenant_id TEXT,
            customer_id TEXT,
            start_time DATETIME
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS customers (
            id TEXT PRIMARY KEY,
            tenant_id TEXT,
            name TEXT
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            parent_plan_id TEXT,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'PENDING',
            assigned_agent_id TEXT,
            dependencies TEXT DEFAULT '[]',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            _sync_status TEXT DEFAULT 'pending',
            version INTEGER DEFAULT 1,
            auto_dreamed BOOLEAN DEFAULT FALSE,
            action_risk TEXT,
            approval_status TEXT,
            proposed_content TEXT,
            priority TEXT
        )"
    ).execute(&pool).await.unwrap();

    let tenant_id = "tenant-1";
    let customer_id = "customer-1";

    sqlx::query("INSERT INTO customers (id, tenant_id, name) VALUES (?, ?, ?)")
        .bind(customer_id).bind(tenant_id).bind("Test Customer")
        .execute(&pool).await.unwrap();

    // Insert active customer (1 old, 1 recent)
    sqlx::query("INSERT INTO bookings (id, tenant_id, customer_id, start_time) VALUES (?, ?, ?, datetime('now', '-20 days'))")
        .bind(Uuid::new_v4().to_string()).bind(tenant_id).bind(customer_id)
        .execute(&pool).await.unwrap();

    sqlx::query("INSERT INTO bookings (id, tenant_id, customer_id, start_time) VALUES (?, ?, ?, datetime('now', '-2 days'))")
        .bind(Uuid::new_v4().to_string()).bind(tenant_id).bind(customer_id)
        .execute(&pool).await.unwrap();

    let job_id = Uuid::new_v4().to_string();
    let payload = json!({ "customer_id": customer_id }).to_string();
    sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, status, payload) VALUES (?, ?, 'booking_reengagement_check', 'PENDING', ?)")
        .bind(&job_id).bind(tenant_id).bind(&payload)
        .execute(&pool).await.unwrap();

    let db = Arc::new(crate::db::DB {
        pool: crate::db::secure_pg_pool_options().acquire_timeout(std::time::Duration::from_millis(10)).connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap(),
        store: DbStore::Sqlite(pool.clone()),
    });
    let mut worker = BookingReengagementWorker::new(db);
    worker.poll_interval = std::time::Duration::from_millis(100);
    worker.start();
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    let task_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM shared_tasks WHERE organization_id = ?")
        .bind(tenant_id)
        .fetch_one(&pool).await.unwrap_or(0);

    assert_eq!(task_count, 0, "Expected 0 re-engagement tasks to be created for active customer");
}
