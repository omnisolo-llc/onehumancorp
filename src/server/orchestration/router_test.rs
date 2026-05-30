use crate::orchestration::router::DynamicTaskRouter;
use crate::db::{DB, DbStore};
use crate::orchestration::mesh::TeammateMesh;
use std::sync::Arc;
use sqlx::sqlite::SqlitePoolOptions;
use async_trait::async_trait;

struct MockMesh {
    pub publish_called: std::sync::atomic::AtomicBool,
}

#[async_trait]
impl TeammateMesh for MockMesh {
    async fn publish(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> {
        self.publish_called.store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }
    async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe(&self, _topic: &str, _handler: Box<dyn Fn(crate::orchestration::mesh::Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
    async fn acquire_lock(&self, _resource: &str, _owner: &str, _ttl_seconds: u64) -> Result<bool, String> { Ok(true) }
    async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> { Ok(()) }
    async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> { Ok(()) }
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> { Ok(vec![]) }
    async fn ping(&self) -> Result<(), String> { Ok(()) }
    async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
    async fn publish_state_handoff(&self, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe_state_handoff(&self, _handler: Box<dyn Fn(crate::orchestration::mesh::Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
    async fn publish_task(&self, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn publish_coordination(&self, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn publish_ultraplan(&self, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
}

#[tokio::test]
async fn test_broadcast_task_available() {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    let db = Arc::new(DB { pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(), store: DbStore::Sqlite(pool) });
    let mesh = Arc::new(MockMesh { publish_called: std::sync::atomic::AtomicBool::new(false) });

    let router = DynamicTaskRouter::new(db, mesh.clone());
    router.broadcast_task_available("task1").await.unwrap();

    assert!(mesh.publish_called.load(std::sync::atomic::Ordering::SeqCst));
}

#[tokio::test]
async fn test_claim_task() {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();

    sqlx::query(
        "CREATE TABLE shared_tasks (
            id TEXT PRIMARY KEY,
            claim_status TEXT DEFAULT 'UNCLAIMED',
            claimed_by TEXT,
            updated_at TEXT
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query("INSERT INTO shared_tasks (id) VALUES ('task1')").execute(&pool).await.unwrap();

    let db = Arc::new(DB { pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(), store: DbStore::Sqlite(pool.clone()) });
    let mesh = Arc::new(MockMesh { publish_called: std::sync::atomic::AtomicBool::new(false) });

    let router = DynamicTaskRouter::new(db, mesh);

    let result = router.claim_task("task1", "agent1").await.unwrap();
    assert!(result);

    let row: (String, String) = sqlx::query_as("SELECT claim_status, claimed_by FROM shared_tasks WHERE id = 'task1'")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, "CLAIMED");
    assert_eq!(row.1, "agent1");
}
