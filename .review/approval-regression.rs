//! Behavior regressions for owner approval and standalone audit paths.
use server_lib::orchestration::departments::orchestrator::DepartmentOrchestrator;
use server_lib::orchestration::departments::types::ApprovalStatus;
use server_lib::db::{DB, DbStore};
use server_lib::orchestration::mesh::CentrifugeNode;
use omnisolo_builtin_agent::mesh::transport::InProcessTransport;
use serde_json::{Value, json};
use sqlx::SqlitePool;
use std::sync::Arc;

async fn standalone() -> (DepartmentOrchestrator, SqlitePool) {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1).connect("sqlite::memory:").await.unwrap();
    sqlx::raw_sql("CREATE TABLE agent_feed_items (
        id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, event_source TEXT NOT NULL,
        context_payload TEXT, proposed_action TEXT, lifecycle_state TEXT NOT NULL,
        created_at TEXT DEFAULT CURRENT_TIMESTAMP, updated_at TEXT DEFAULT CURRENT_TIMESTAMP);
        CREATE TABLE ohc_universal_ledger (
        id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, department TEXT NOT NULL,
        action_type TEXT NOT NULL, state_change TEXT NOT NULL,
        created_at TEXT DEFAULT CURRENT_TIMESTAMP);
        CREATE TABLE inbox_messages (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL,
        draft_reply TEXT, status TEXT NOT NULL);
        CREATE TABLE omni_inbox_messages (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL,
        draft_reply TEXT, status TEXT NOT NULL);")
        .execute(&pool).await.unwrap();
    let db = Arc::new(DB {
        pool: sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://unused:unused@127.0.0.1:1/unused").unwrap(),
        store: DbStore::Sqlite(pool.clone()),
    });
    let mesh = Arc::new(CentrifugeNode::new(Arc::new(InProcessTransport::new())));
    (DepartmentOrchestrator::new(db, mesh), pool)
}

async fn draft(pool: &SqlitePool, id: &str, tenant: &str, payload: Value) {
    sqlx::query("INSERT INTO agent_feed_items (id, tenant_id, event_source,
        context_payload, proposed_action, lifecycle_state)
        VALUES (?, ?, 'finance', '{\"description\":\"Owner review\"}', ?, 'PENDING_APPROVAL')")
        .bind(id).bind(tenant).bind(payload.to_string()).execute(pool).await.unwrap();
}
async fn state(pool: &SqlitePool, id: &str) -> String {
    sqlx::query_scalar("SELECT lifecycle_state FROM agent_feed_items WHERE id = ?")
        .bind(id).fetch_one(pool).await.unwrap()
}

#[tokio::test]
async fn review_regression_sqlite_ledger_returns_persisted_tenant_rows() {
    let (service, pool) = standalone().await;
    for (id, tenant, date) in [("a", "one", "2026-01-01 00:00:00"),
        ("b", "one", "2026-01-02 00:00:00"), ("c", "other", "2026-01-03 00:00:00")] {
        sqlx::query("INSERT INTO ohc_universal_ledger
            (id, tenant_id, department, action_type, state_change, created_at)
            VALUES (?, ?, 'finance', 'approval_decision', '{\"approved\":true}', ?)")
            .bind(id).bind(tenant).bind(date).execute(&pool).await.unwrap();
    }
    let entries = service.get_ledger_entries("one", 1).await.unwrap();
    assert_eq!(entries.len(), 1, "stored standalone records must not become empty success");
    assert_eq!(entries[0].id, "b");
    assert_eq!(entries[0].tenant_id, "one");
    assert_eq!(entries[0].event_type, "approval_decision");
    assert_eq!(serde_json::from_str::<Value>(&entries[0].payload).unwrap(), json!({"approved":true}));
}
#[tokio::test]
async fn review_regression_ledger_storage_failure_is_not_an_empty_success() {
    let (service, pool) = standalone().await;
    sqlx::query("DROP TABLE ohc_universal_ledger").execute(&pool).await.unwrap();
    assert!(service.get_ledger_entries("one", 10).await.is_err());
}
#[tokio::test]
async fn review_regression_ledger_rejects_unbounded_or_unscoped_queries() {
    let (service, _) = standalone().await;
    for (tenant, limit) in [("", 10), ("   ", 10), ("one", -1), ("one", 0), ("one", 501)] {
        assert!(service.get_ledger_entries(tenant, limit).await.is_err());
    }
}
#[tokio::test]
async fn review_regression_invalid_commercial_input_does_not_consume_approval() {
    let (service, pool) = standalone().await;
    draft(&pool, "approval", "one", json!({
        "feature_type":"invoice_draft", "customer_id":"", "amount_cents":100
    })).await;
    assert!(service.decide_approval("approval", "one", true, None).await.is_err());
    assert_eq!(state(&pool, "approval").await, "PENDING_APPROVAL");
}
#[tokio::test]
async fn review_regression_edits_cannot_replace_action_or_customer_identity() {
    let (service, pool) = standalone().await;
    let original = json!({"feature_type":"invoice_followup", "customer_id":"customer-one",
        "invoice_id":"invoice-one", "generated_response":"A prepared reminder"});
    draft(&pool, "approval", "one", original.clone()).await;
    let edited = json!({"feature_type":"unrelated_action", "customer_id":"customer-two"});
    assert!(service.decide_approval("approval", "one", true, Some(edited)).await.is_err());
    assert_eq!(state(&pool, "approval").await, "PENDING_APPROVAL");
    let saved: String = sqlx::query_scalar("SELECT proposed_action FROM agent_feed_items WHERE id='approval'")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(serde_json::from_str::<Value>(&saved).unwrap(), original);
}
#[tokio::test]
async fn review_regression_interrupted_execution_is_not_shown_as_pending_approval() {
    let (service, pool) = standalone().await;
    draft(&pool, "approval", "one", json!({"feature_type":"invoice_draft"})).await;
    sqlx::query("UPDATE agent_feed_items SET lifecycle_state='RECONCILIATION_REQUIRED'")
        .execute(&pool).await.unwrap();
    let activity = service.get_activity_feed("one", None, 10).await;
    assert_eq!(activity.len(), 1);
    assert_eq!(activity[0].status, ApprovalStatus::Paused);
}
#[tokio::test]
async fn review_regression_a_prepared_reply_is_not_sent() {
    let (service, pool) = standalone().await;
    sqlx::query("INSERT INTO inbox_messages (id, tenant_id, status) VALUES ('message', 'one', 'unread')")
        .execute(&pool).await.unwrap();
    draft(&pool, "approval", "one", json!({"feature_type":"ambassador_reply",
        "inbox_message_id":"message", "generated_response":"Prepared reply"})).await;
    service.decide_approval("approval", "one", true, None).await.unwrap();
    let (reply, status): (String, String) = sqlx::query_as(
        "SELECT draft_reply, status FROM inbox_messages WHERE id='message'")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(reply, "Prepared reply");
    assert_eq!(status, "draft", "saving a reply does not prove delivery");
}
#[tokio::test]
async fn review_regression_rejection_is_audited_in_standalone_mode() {
    let (service, pool) = standalone().await;
    draft(&pool, "approval", "one", json!({"feature_type":"invoice_draft"})).await;
    service.decide_approval("approval", "one", false, None).await.unwrap();
    assert_eq!(state(&pool, "approval").await, "REJECTED");
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ohc_universal_ledger WHERE tenant_id='one'")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(count, 1);
}
