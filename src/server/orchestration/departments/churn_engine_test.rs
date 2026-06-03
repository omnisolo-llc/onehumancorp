use crate::orchestration::departments::churn_engine::ChurnPredictionEngine;
use crate::orchestration::departments::orchestrator::AgentOrchestrator;
use crate::orchestration::departments::types::{Customer360, TimelineEvent};
use crate::db::{Database, DbStore};
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use chrono::{Utc, Duration};

#[tokio::test]
async fn test_churn_engine_calculates_cadence_and_triggers_winback() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    sqlx::query("CREATE TABLE customer360 (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, customer_id TEXT NOT NULL, email TEXT, phone TEXT, mood TEXT, preferences TEXT, created_at TIMESTAMP, updated_at TIMESTAMP, status TEXT, expected_purchase_cadence_days REAL)")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("CREATE TABLE interaction_timeline (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, customer_id TEXT NOT NULL, event_type TEXT, source TEXT, content TEXT, metadata TEXT, occurred_at TIMESTAMP)")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("CREATE TABLE approval_requests (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, department TEXT, description TEXT, status TEXT, action_risk TEXT, payload TEXT, created_at TIMESTAMP, updated_at TIMESTAMP)")
        .execute(&pool)
        .await
        .unwrap();

    let db = Arc::new(crate::db::DB { store: DbStore::Sqlite(pool.clone()) });
    let orchestrator = AgentOrchestrator::new(db, Arc::new(crate::queue::SqliteTaskQueue::new(pool.clone())), Arc::new(crate::queue::SqliteTaskQueue::new(pool.clone())), Arc::new(crate::sip::SipDB::new(pool.clone(), "test".to_string())));

    // Create a customer
    let mut c = Customer360 {
        id: "c1".to_string(),
        tenant_id: "t1".to_string(),
        customer_id: "cust1".to_string(),
        email: None,
        phone: None,
        mood: None,
        preferences: None,
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
        status: Some("Active".to_string()),
        expected_purchase_cadence_days: None,
    };
    orchestrator.upsert_customer360(&c).await.unwrap();

    // Insert interactions 10 days apart
    let now = Utc::now();
    let past1 = now - Duration::days(20);
    let past2 = now - Duration::days(30);

    sqlx::query("INSERT INTO interaction_timeline (id, tenant_id, customer_id, event_type, source, content, occurred_at) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind("i1").bind("t1").bind("cust1").bind("order").bind("web").bind("").bind(past1)
        .execute(&pool).await.unwrap();

    sqlx::query("INSERT INTO interaction_timeline (id, tenant_id, customer_id, event_type, source, content, occurred_at) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind("i2").bind("t1").bind("cust1").bind("order").bind("web").bind("").bind(past2)
        .execute(&pool).await.unwrap();

    // Run engine
    ChurnPredictionEngine::run_nightly_job(&orchestrator, "t1").await.unwrap();

    // Fetch updated customer
    let updated_c = orchestrator.get_customer360("t1", "cust1").await.unwrap().unwrap();

    // Cadence should be ~10 days.
    // Since last order was 20 days ago, deviation is 2.0x, which is > 1.5x -> At-Risk
    assert_eq!(updated_c.expected_purchase_cadence_days.unwrap(), 10.0);
    assert_eq!(updated_c.status.unwrap(), "At-Risk");

    // Ensure approval request is created
    let approvals = orchestrator.list_pending_approvals("t1").await.unwrap();
    assert_eq!(approvals.len(), 1);
    assert_eq!(approvals[0].description, "Winback Opportunity for cust1");
}
