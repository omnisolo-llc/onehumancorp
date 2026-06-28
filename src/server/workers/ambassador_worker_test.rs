use crate::db::DbStore;
use std::sync::Arc;
use crate::db::DB;

async fn setup_test_db() -> Arc<DB> {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();
    let _ = sqlx::query("CREATE TABLE tenants (id TEXT PRIMARY KEY, name TEXT, industry TEXT);").execute(&pool).await;
    let _ = sqlx::query("CREATE TABLE ohc_job_queue (id TEXT PRIMARY KEY, tenant_id TEXT, job_type TEXT, payload TEXT, status TEXT, next_retry_at TIMESTAMP, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(&pool).await;
    let _ = sqlx::query("CREATE TABLE unified_threads (id TEXT PRIMARY KEY, tenant_id TEXT, customer_id TEXT, channel TEXT, status TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(&pool).await;
    let _ = sqlx::query("CREATE TABLE unified_messages (id TEXT PRIMARY KEY, tenant_id TEXT, thread_id TEXT, sender_type TEXT, content TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(&pool).await;
    let _ = sqlx::query("CREATE TABLE unified_triage_actions (id TEXT PRIMARY KEY, tenant_id TEXT, thread_id TEXT, action_type TEXT, action_payload TEXT, status TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(&pool).await;
    let _ = sqlx::query("CREATE TABLE inventory_items (id TEXT PRIMARY KEY, tenant_id TEXT, name TEXT, quantity INTEGER);").execute(&pool).await;

    // Use dummy PgPool to satisfy struct requirement for test (using sqlite internally)
    let pg_pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy:dummy@localhost:5432/dummy").unwrap();

    let db = DB {
        pool: pg_pool,
        store: DbStore::Sqlite(pool),
    };
    Arc::new(db)
}

#[tokio::test]
async fn test_ambassador_worker_processes_job_and_creates_draft() {
    let db = setup_test_db().await;
    if let DbStore::Sqlite(pool) = &db.store {
        sqlx::query("INSERT INTO tenants (id, name, industry) VALUES ('tenant1', 'Maya Bakery', 'Bakery')")
            .execute(&*pool).await.unwrap();

        sqlx::query("INSERT INTO inventory_items (id, tenant_id, name, quantity) VALUES ('inv1', 'tenant1', 'Vegan Chocolate Cake', 5)")
            .execute(&*pool).await.unwrap();

        sqlx::query("INSERT INTO unified_threads (id, tenant_id, customer_id, channel, status) VALUES ('thread1', 'tenant1', 'cust1', 'instagram', 'open')")
            .execute(&*pool).await.unwrap();

        let payload = serde_json::json!({
            "message_id": "msg1",
            "thread_id": "thread1",
            "customer_id": "cust1",
            "source": "instagram",
            "content": "Do you have vegan chocolate cake available?"
        });

        sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES ('job1', 'tenant1', 'ambassador_intent', ?, 'PENDING', CURRENT_TIMESTAMP)")
            .bind(payload.to_string())
            .execute(&*pool).await.unwrap();
    }

    let processed = super::ambassador_worker::AmbassadorWorker::poll(&db).await.unwrap();
    assert!(processed);

    if let DbStore::Sqlite(pool) = &db.store {
        use sqlx::Row;
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM unified_triage_actions WHERE thread_id = 'thread1'")
            .fetch_one(&*pool).await.unwrap();
        assert_eq!(count, 1);

        let row = sqlx::query("SELECT action_type, action_payload, status FROM unified_triage_actions WHERE thread_id = 'thread1'")
            .fetch_one(&*pool).await.unwrap();

        let action_type: String = row.get("action_type");
        assert_eq!(action_type, "DRAFT_REPLY");

        let status: String = row.get("status");
        assert_eq!(status, "pending");

        let action_payload: String = row.get("action_payload");
        assert!(action_payload.contains("draft_reply"));
        assert!(action_payload.contains("Inventory Checked"));

        let job_status: String = sqlx::query_scalar("SELECT status FROM ohc_job_queue WHERE id = 'job1'")
            .fetch_one(&*pool).await.unwrap();
        assert_eq!(job_status, "COMPLETED");
    }
}
