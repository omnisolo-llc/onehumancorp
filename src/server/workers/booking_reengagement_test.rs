use std::sync::Arc;
use crate::db::DB;
use super::booking_reengagement::BookingReengagementWorker;
use uuid::Uuid;
use sqlx::Row;
use serde_json::json;

#[tokio::test]
async fn test_booking_reengagement_worker_drafts_message() {
    // For tests, use an in-memory SQLite pool properly initialized
        let sqlite_pool = crate::db::create_sqlite_pool_for_test().await;
    let _ = sqlx::query("CREATE TABLE customers (id TEXT PRIMARY KEY, tenant_id TEXT, name TEXT, email TEXT, phone TEXT, preferences TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, is_subscribable BOOLEAN DEFAULT FALSE, subscription_frequency TEXT, subscription_discount_percent INTEGER DEFAULT 0, _sync_status TEXT DEFAULT 'pending', version INTEGER DEFAULT 1)").execute(&sqlite_pool).await.unwrap();
    let _ = sqlx::query("CREATE TABLE bookings (id TEXT PRIMARY KEY, tenant_id TEXT, customer_id TEXT, product_id TEXT, quote_id TEXT, service_id TEXT, start_time TIMESTAMP, end_time TIMESTAMP, status TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, is_subscribable BOOLEAN DEFAULT FALSE, subscription_frequency TEXT, subscription_discount_percent INTEGER DEFAULT 0, _sync_status TEXT DEFAULT 'pending', version INTEGER DEFAULT 1)").execute(&sqlite_pool).await.unwrap();
    let _ = sqlx::query("CREATE TABLE ohc_job_queue (id TEXT PRIMARY KEY, tenant_id TEXT, job_type TEXT, payload TEXT, status TEXT, next_retry_at TIMESTAMP, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)").execute(&sqlite_pool).await.unwrap();
    let _ = sqlx::query("CREATE TABLE shared_tasks (id TEXT PRIMARY KEY, organization_id TEXT, title TEXT, description TEXT, status TEXT, priority TEXT, action_risk TEXT, approval_status TEXT, proposed_content TEXT)").execute(&sqlite_pool).await.unwrap();

    let db = Arc::new(crate::db::DB { pool: crate::db::create_dummy_pg_pool().await, store: crate::db::DbStore::Sqlite(sqlite_pool) });


    let worker = BookingReengagementWorker::new(db.clone());

    let tenant_id = Uuid::new_v4().to_string();
    let customer_id = Uuid::new_v4().to_string();

    // Setup customer
    let query = match &db.store {
        crate::db::DbStore::Postgres => "INSERT INTO customers (id, tenant_id, name) VALUES ($1, $2, 'Test Customer')",
        crate::db::DbStore::Sqlite(_) => "INSERT INTO customers (id, tenant_id, name) VALUES (?, ?, 'Test Customer')",
    };
    match &db.store {
        crate::db::DbStore::Postgres => {
             let _ = sqlx::query(query).bind(&customer_id).bind(&tenant_id).execute(&db.pool).await.unwrap();
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
             let _ = sqlx::query(query).bind(&customer_id).bind(&tenant_id).execute(sqlite_pool).await.unwrap();
        }
    }

    // Setup past bookings (older than 14 days)
    let query1 = match &db.store {
        crate::db::DbStore::Postgres => "INSERT INTO bookings (id, tenant_id, customer_id, start_time, end_time, status) VALUES ($1, $2, $3, CURRENT_TIMESTAMP - INTERVAL '20 days', CURRENT_TIMESTAMP - INTERVAL '20 days' + INTERVAL '1 hour', 'COMPLETED')",
        crate::db::DbStore::Sqlite(_) => "INSERT INTO bookings (id, tenant_id, customer_id, start_time, end_time, status) VALUES (?, ?, ?, strftime('%Y-%m-%d %H:%M:%S', datetime('now', '-20 days')), strftime('%Y-%m-%d %H:%M:%S', datetime('now', '-20 days', '+1 hour')), 'COMPLETED')",
    };
    match &db.store {
        crate::db::DbStore::Postgres => {
             let _ = sqlx::query(query1).bind(Uuid::new_v4().to_string()).bind(&tenant_id).bind(&customer_id).execute(&db.pool).await.unwrap();
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
             let _ = sqlx::query(query1).bind(Uuid::new_v4().to_string()).bind(&tenant_id).bind(&customer_id).execute(sqlite_pool).await.unwrap();
        }
    }
    let query2 = match &db.store {
        crate::db::DbStore::Postgres => "INSERT INTO bookings (id, tenant_id, customer_id, start_time, end_time, status) VALUES ($1, $2, $3, CURRENT_TIMESTAMP - INTERVAL '15 days', CURRENT_TIMESTAMP - INTERVAL '15 days' + INTERVAL '1 hour', 'COMPLETED')",
        crate::db::DbStore::Sqlite(_) => "INSERT INTO bookings (id, tenant_id, customer_id, start_time, end_time, status) VALUES (?, ?, ?, strftime('%Y-%m-%d %H:%M:%S', datetime('now', '-15 days')), strftime('%Y-%m-%d %H:%M:%S', datetime('now', '-15 days', '+1 hour')), 'COMPLETED')",
    };
    match &db.store {
        crate::db::DbStore::Postgres => {
             let _ = sqlx::query(query2).bind(Uuid::new_v4().to_string()).bind(&tenant_id).bind(&customer_id).execute(&db.pool).await.unwrap();
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
             let _ = sqlx::query(query2).bind(Uuid::new_v4().to_string()).bind(&tenant_id).bind(&customer_id).execute(sqlite_pool).await.unwrap();
        }
    }

    // Queue job
    let job_id = Uuid::new_v4().to_string();
    let payload = json!({"customer_id": customer_id});
    let query3 = match &db.store {
        crate::db::DbStore::Postgres => "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES ($1, $2, 'booking_reengagement_check', $3, 'PENDING', CURRENT_TIMESTAMP)",
        crate::db::DbStore::Sqlite(_) => "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES (?, ?, 'booking_reengagement_check', ?, 'PENDING', datetime('now'))",
    };
    match &db.store {
        crate::db::DbStore::Postgres => {
             let _ = sqlx::query(query3).bind(&job_id).bind(&tenant_id).bind(payload.to_string()).execute(&db.pool).await.unwrap();
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
             let _ = sqlx::query(query3).bind(&job_id).bind(&tenant_id).bind(payload.to_string()).execute(sqlite_pool).await.unwrap();
        }
    }

    // Run worker for a bit
    worker.start();
    tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
    // Check shared tasks
    let query4 = "SELECT title, proposed_content FROM shared_tasks";

    // The worker might take some time to poll and process
    let mut found = false;
    let mut title_res = "".to_string();
    let mut proposed_content_res = "".to_string();

    for _ in 0..30 {
        let res: Result<(String, String), sqlx::Error> = match &db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query_as(query4).fetch_one(&db.pool).await
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as(query4).fetch_one(sqlite_pool).await
            }
        };

        if let Ok((t, c)) = res {
            found = true;
            title_res = t;
            proposed_content_res = c;
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }



    assert!(found, "Re-engagement task should have been created");
    assert!(title_res.contains("Approve Re-engagement"));
    assert!(proposed_content_res.contains("Test Customer"));

    // Check job is completed
    let query5 = match &db.store {
        crate::db::DbStore::Postgres => "SELECT status FROM ohc_job_queue WHERE id = $1",
        crate::db::DbStore::Sqlite(_) => "SELECT status FROM ohc_job_queue WHERE id = ?",
    };
    let job_status: String = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query_scalar(query5).bind(&job_id).fetch_one(&db.pool).await.unwrap()
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query_scalar(query5).bind(&job_id).fetch_one(sqlite_pool).await.unwrap()
        }
    };
    assert_eq!(job_status, "COMPLETED");
}
