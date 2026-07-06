use std::sync::Arc;
use uuid::Uuid;
use crate::db::{DbStore, DB};
use super::subscription_churn_worker::SubscriptionChurnWorker;
use serde_json::json;

async fn setup_test_db() -> Option<Arc<DB>> {
    let sqlite_pool = crate::db::create_sqlite_pool_for_test().await;
    let pool = crate::db::create_dummy_pg_pool().await;
    let db = DB {
        pool,
        store: DbStore::Sqlite(sqlite_pool.clone()),
    };

    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS tenants (id TEXT PRIMARY KEY, name TEXT, industry TEXT);").execute(&sqlite_pool).await;
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS customers (id UUID PRIMARY KEY, tenant_id TEXT, name TEXT, email TEXT, phone TEXT, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(&sqlite_pool).await;
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS subscribers (id TEXT PRIMARY KEY, tenant_id TEXT, customer_id TEXT, health_score INTEGER DEFAULT 100, last_health_check_at TIMESTAMP, churn_risk_status TEXT DEFAULT 'healthy');").execute(&sqlite_pool).await;
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS bookings (id TEXT PRIMARY KEY, tenant_id TEXT, customer_id UUID, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(&sqlite_pool).await;
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS ohc_job_queue (id TEXT PRIMARY KEY, tenant_id TEXT, job_type TEXT, payload TEXT, status TEXT, next_retry_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(&sqlite_pool).await;
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS agent_feed_items (id TEXT PRIMARY KEY, tenant_id TEXT, event_source TEXT, context_payload TEXT, proposed_action TEXT, lifecycle_state TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(&sqlite_pool).await;

    Some(Arc::new(db))
}

#[tokio::test]
async fn test_score_drops_after_inactivity() {
    let db = setup_test_db().await.unwrap();
    let pool = match &db.store {
        DbStore::Sqlite(p) => p.clone(),
        _ => panic!("Expected Sqlite store"),
    };

    let tenant_id = "tenant-1";
    let customer_id = Uuid::new_v4();
    let subscriber_id = "sub-1";

    let _ = sqlx::query("INSERT INTO customers (id, tenant_id, name) VALUES ($1, $2, 'Alex')")
        .bind(customer_id)
        .bind(tenant_id)
        .execute(&pool).await.unwrap();

    let _ = sqlx::query("INSERT INTO subscribers (id, tenant_id, customer_id, health_score) VALUES ($1, $2, $3, 60)")
        .bind(subscriber_id)
        .bind(tenant_id)
        .bind(customer_id.to_string())
        .execute(&pool).await.unwrap();

    let job_id = Uuid::new_v4().to_string();
    let payload = json!({
        "subscriber_id": subscriber_id,
        "customer_id": customer_id.to_string()
    }).to_string();

    let _ = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'subscription_churn_check', $3, 'PENDING')")
        .bind(&job_id)
        .bind(tenant_id)
        .bind(&payload)
        .execute(&pool).await.unwrap();

    let worker = SubscriptionChurnWorker::new(db.clone());
    let processed = worker.poll().await.unwrap();
    assert!(processed);

    let (score, status): (i32, String) = sqlx::query_as("SELECT health_score, churn_risk_status FROM subscribers WHERE id = $1")
        .bind(subscriber_id)
        .fetch_one(&pool).await.unwrap();

    // 60 - 20 = 40, which is <= 50, so status should be at_risk
    assert_eq!(score, 40);
    assert_eq!(status, "at_risk");
}

#[tokio::test]
async fn test_ambassador_creates_action_card() {
    let db = setup_test_db().await.unwrap();
    let pool = match &db.store {
        DbStore::Sqlite(p) => p.clone(),
        _ => panic!("Expected Sqlite store"),
    };

    let tenant_id = "tenant-2";
    let customer_id = Uuid::new_v4();
    let subscriber_id = "sub-2";

    let _ = sqlx::query("INSERT INTO customers (id, tenant_id, name) VALUES ($1, $2, 'Alex')")
        .bind(customer_id)
        .bind(tenant_id)
        .execute(&pool).await.unwrap();

    let _ = sqlx::query("INSERT INTO subscribers (id, tenant_id, customer_id, health_score) VALUES ($1, $2, $3, 60)")
        .bind(subscriber_id)
        .bind(tenant_id)
        .bind(customer_id.to_string())
        .execute(&pool).await.unwrap();

    let job_id = Uuid::new_v4().to_string();
    let payload = json!({
        "subscriber_id": subscriber_id,
        "customer_id": customer_id.to_string()
    }).to_string();

    let _ = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'subscription_churn_check', $3, 'PENDING')")
        .bind(&job_id)
        .bind(tenant_id)
        .bind(&payload)
        .execute(&pool).await.unwrap();

    let worker = SubscriptionChurnWorker::new(db.clone());
    worker.poll().await.unwrap();

    let (event_source, lifecycle_state, proposed_action): (String, String, String) = sqlx::query_as("SELECT event_source, lifecycle_state, proposed_action FROM agent_feed_items WHERE tenant_id = $1")
        .bind(tenant_id)
        .fetch_one(&pool).await.unwrap();

    assert_eq!(event_source, "The Ambassador");
    assert_eq!(lifecycle_state, "PENDING_APPROVAL");

    let action_json: serde_json::Value = serde_json::from_str(&proposed_action).unwrap();
    assert_eq!(action_json["action_type"], "Draft Reply");
    let draft_text = action_json["draft_reply"].as_str().unwrap_or("");
    // Given the offline mock behavior, check for either "Alex" or the default message from the code fallback.
    assert!(draft_text.contains("Alex") || draft_text.contains("we noticed you haven't booked"), "Draft text didn't contain expected strings: {}", draft_text);
}
