use super::statemachine_v2::{StateMachine, State};
use super::locks::StandaloneLock;
use crate::db::DbStore;
use crate::orchestration::mesh::TeammateMesh;
// use ohc_builtin_agent::mesh::transport::Message;
use ohc_builtin_agent::mesh::transport::Message;
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use sqlx::Row;
use serde_json::Value;

struct MockMesh;

#[async_trait::async_trait]
impl TeammateMesh for MockMesh {
    async fn publish(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe(&self, _topic: &str, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
    async fn acquire_lock(&self, _resource: &str, _owner: &str, _ttl_seconds: u64) -> Result<bool, String> { Ok(true) }
    async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> { Ok(()) }
    async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> { Ok(()) }
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> { Ok(vec![]) }
    async fn ping(&self) -> Result<(), String> { Ok(()) }
    async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
    async fn publish_state_handoff(&self, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe_state_handoff(&self, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
}

async fn setup_db() -> sqlx::SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE shared_tasks_decomposition (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'PENDING',
            assigned_agent_id TEXT,
            updated_at TEXT
        );
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE state_machine_transitions (
            id TEXT PRIMARY KEY,
            tenant_id TEXT DEFAULT 'system',
            entity_id TEXT,
            entity_type TEXT,
            from_state TEXT NOT NULL,
            to_state TEXT NOT NULL,
            agent_id TEXT,
            reason TEXT,
            occurred_at TEXT DEFAULT CURRENT_TIMESTAMP,
            task_id TEXT,
            transitioned_at TEXT,
            _sync_status TEXT DEFAULT 'pending',
            version INTEGER DEFAULT 1
        );
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}

#[tokio::test]
async fn test_statemachine_valid_transitions() {
    let pool = setup_db().await;
    let lock = Arc::new(StandaloneLock::new());
    let mesh = Arc::new(MockMesh);
    let sm = StateMachine::new(DbStore::Sqlite(pool.clone()), lock, mesh);

    let task_id = "task1";
    let org_id = "org1";

    sqlx::query("INSERT INTO shared_tasks_decomposition (id, organization_id, title, status) VALUES (?, ?, 'Task 1', 'PENDING')")
        .bind(task_id)
        .bind(org_id)
        .execute(&pool)
        .await
        .unwrap();

    // Pending -> Ready
    sm.transition_to_ready(task_id).await.unwrap();

    let row = sqlx::query("SELECT status FROM shared_tasks_decomposition WHERE id = ?")
        .bind(task_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    let status: String = row.get("status");
    assert_eq!(status, "READY");

    let row = sqlx::query("SELECT from_state, to_state FROM state_machine_transitions WHERE entity_id = ? ORDER BY occurred_at DESC LIMIT 1")
        .bind(task_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    let from_state: String = row.get("from_state");
    let to_state: String = row.get("to_state");
    assert_eq!(from_state, "PENDING");
    assert_eq!(to_state, "READY");

    // Ready -> InProgress
    sm.transition_to_in_progress(task_id, "agent1").await.unwrap();
    let row = sqlx::query("SELECT status, assigned_agent_id FROM shared_tasks_decomposition WHERE id = ?")
        .bind(task_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    let status: String = row.get("status");
    let agent: String = row.get("assigned_agent_id");
    assert_eq!(status, "IN_PROGRESS");
    assert_eq!(agent, "agent1");

    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM state_machine_transitions WHERE entity_id = ?")
        .bind(task_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 2);
}

#[tokio::test]
async fn test_statemachine_invalid_transition() {
    let pool = setup_db().await;
    let lock = Arc::new(StandaloneLock::new());
    let mesh = Arc::new(MockMesh);
    let sm = StateMachine::new(DbStore::Sqlite(pool.clone()), lock, mesh);

    let task_id = "task2";
    let org_id = "org1";

    sqlx::query("INSERT INTO shared_tasks_decomposition (id, organization_id, title, status) VALUES (?, ?, 'Task 2', 'PENDING')")
        .bind(task_id)
        .bind(org_id)
        .execute(&pool)
        .await
        .unwrap();

    // Pending -> InProgress (Invalid)
    let err = sm.transition_to_in_progress(task_id, "agent1").await;
    assert!(err.is_err());

    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM state_machine_transitions WHERE entity_id = ?")
        .bind(task_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0); // Ensure no audit log was created on failure
}

#[tokio::test]
async fn test_statemachine_concurrent_transitions() {
    let pool = setup_db().await;
    let lock = Arc::new(StandaloneLock::new());
    let mesh = Arc::new(MockMesh);
    let sm = Arc::new(StateMachine::new(DbStore::Sqlite(pool.clone()), lock, mesh));

    let task_id = "task3";
    let org_id = "org1";

    sqlx::query("INSERT INTO shared_tasks_decomposition (id, organization_id, title, status) VALUES (?, ?, 'Task 3', 'READY')")
        .bind(task_id)
        .bind(org_id)
        .execute(&pool)
        .await
        .unwrap();

    let sm1 = sm.clone();
    let sm2 = sm.clone();

    let t1 = tokio::spawn(async move {
        sm1.transition_to_in_progress("task3", "agent1").await
    });

    let t2 = tokio::spawn(async move {
        sm2.transition_to_in_progress("task3", "agent2").await
    });

    let res1 = t1.await.unwrap();
    let res2 = t2.await.unwrap();

    let mut success_count = 0;
    if res1.is_ok() { success_count += 1; }
    if res2.is_ok() { success_count += 1; }

    assert_eq!(success_count, 1, "Only one transition should succeed");
}
