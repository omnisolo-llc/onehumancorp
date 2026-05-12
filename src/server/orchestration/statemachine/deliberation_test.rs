use super::deliberation::DeliberationStateMachine;
use crate::db::{DB, DbStore};
use crate::orchestration::mesh::TeammateMesh;
use async_trait::async_trait;
use ohc_builtin_agent::mesh::transport::Message;
use std::sync::Arc;

struct MockMesh;

#[async_trait]
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

#[tokio::test]
async fn test_deliberation_state_machine() {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    let _ = sqlx::query(
        "CREATE TABLE shared_tasks_decomposition (id TEXT PRIMARY KEY, organization_id TEXT, mission_id TEXT, parent_plan_id TEXT, dependencies TEXT, title TEXT, description TEXT, status TEXT, priority TEXT, payload TEXT, locked_until TEXT, ultraplan_phase TEXT, deliberation_log TEXT, depth INTEGER, created_at TEXT, updated_at TEXT, action_risk TEXT, approval_status TEXT, proposed_content TEXT, assigned_agent_id TEXT)"
    ).execute(&pool).await.unwrap();

    let db = Arc::new(DB {
        store: DbStore::Sqlite(pool.clone()),
        pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
    });

    let mesh: Arc<dyn TeammateMesh> = Arc::new(MockMesh);
    let sm = DeliberationStateMachine::new(db, mesh);

    let task_id = "task-123";
    let agent_id = "agent-456";

    // 1. Setup pending task
    sqlx::query("INSERT INTO shared_tasks_decomposition (id, organization_id, status, dependencies, payload, title, priority) VALUES (?, 'org1', 'PENDING', '[]', '{}', 'Test', 'P2')")
        .bind(task_id)
        .execute(&pool).await.unwrap();

    // 2. Start Deliberation
    let t = sm.start_deliberation(task_id, agent_id).await.unwrap();
    assert_eq!(t.status, "DELIBERATING");
    assert_eq!(t.assigned_agent_id.unwrap(), agent_id);

    // 3. Complete Deliberation
    let t = sm.complete_deliberation(task_id, agent_id, r#"["thought1"]"#).await.unwrap();
    assert_eq!(t.status, "DECOMPOSED");
    assert_eq!(t.deliberation_log.unwrap(), r#"["thought1"]"#);

    // 4. Setup new pending task for failure flow
    let fail_task_id = "task-fail-123";
    sqlx::query("INSERT INTO shared_tasks_decomposition (id, organization_id, status, dependencies, payload, title, priority) VALUES (?, 'org1', 'PENDING', '[]', '{}', 'Test2', 'P2')")
        .bind(fail_task_id)
        .execute(&pool).await.unwrap();

    let t = sm.start_deliberation(fail_task_id, agent_id).await.unwrap();
    assert_eq!(t.status, "DELIBERATING");

    // 5. Fail Deliberation
    let t = sm.fail_deliberation(fail_task_id, agent_id, "Bad error").await.unwrap();
    assert_eq!(t.status, "FAILED");
    let payload_val: serde_json::Value = serde_json::from_str(&t.payload).unwrap();
    assert_eq!(payload_val["error"], "Bad error");

    // 6. Test unmet dependency
    let dep_task_id = "dep-task";
    sqlx::query("INSERT INTO shared_tasks_decomposition (id, organization_id, status, dependencies, payload, title, priority) VALUES (?, 'org1', 'PENDING', '[]', '{}', 'Dep', 'P2')")
        .bind(dep_task_id)
        .execute(&pool).await.unwrap();

    let blocked_task_id = "blocked-task";
    sqlx::query("INSERT INTO shared_tasks_decomposition (id, organization_id, status, dependencies, payload, title, priority) VALUES (?, 'org1', 'PENDING', ?, '{}', 'Blocked', 'P2')")
        .bind(blocked_task_id)
        .bind(serde_json::to_string(&vec![dep_task_id]).unwrap())
        .execute(&pool).await.unwrap();

    let err = sm.start_deliberation(blocked_task_id, agent_id).await.unwrap_err();
    assert!(err.contains("unmet dependencies"));
}
