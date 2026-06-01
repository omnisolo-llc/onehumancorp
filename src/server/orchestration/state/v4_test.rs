use super::v4::V4StateMachine;
use crate::db::{DB, DbStore};
use crate::orchestration::mesh::TeammateMesh;
use std::sync::Arc;
use sqlx::sqlite::SqlitePoolOptions;
use async_trait::async_trait;

struct DummyMesh;

#[async_trait]
impl TeammateMesh for DummyMesh {
    async fn publish(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> {
        Ok(())
    }
    async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> {
        Ok(())
    }
    // We stub out the type in the handler to allow compilation even if ohc_builtin_agent fails
    // Alternatively we can use a generic type or a type that exists in crate::orchestration::mesh
    async fn subscribe(
        &self,
        _topic: &str,
        _handler: Box<dyn Fn(ohc_builtin_agent::mesh::transport::Message) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        Ok(Box::new(|| {}))
    }
    async fn acquire_lock(
        &self,
        _resource: &str,
        _owner: &str,
        _ttl_seconds: u64,
    ) -> Result<bool, String> {
        Ok(true)
    }
    async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> {
        Ok(())
    }
    async fn register_presence(
        &self,
        _agent_id: &str,
        _status: &str,
        _ttl_seconds: u64,
    ) -> Result<(), String> {
        Ok(())
    }
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        Ok(vec![])
    }
    async fn ping(&self) -> Result<(), String> {
        Ok(())
    }
    async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        Ok(Box::new(|| {}))
    }
    async fn publish_state_handoff(&self, _payload: Vec<u8>) -> Result<(), String> {
        Ok(())
    }
    async fn subscribe_state_handoff(
        &self,
        _handler: Box<dyn Fn(ohc_builtin_agent::mesh::transport::Message) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        Ok(Box::new(|| {}))
    }
}

async fn setup_db() -> Arc<DB> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
            id VARCHAR PRIMARY KEY,
            organization_id VARCHAR NOT NULL,
            title VARCHAR NOT NULL,
            status VARCHAR NOT NULL DEFAULT 'PENDING',
            agent_id VARCHAR,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sub_agent_queue (
            id VARCHAR PRIMARY KEY,
            tenant_id VARCHAR NOT NULL DEFAULT 'system',
            status VARCHAR NOT NULL DEFAULT 'PENDING',
            worker_id VARCHAR,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS state_machine_transitions (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL DEFAULT 'system',
            entity_id TEXT NOT NULL,
            entity_type TEXT NOT NULL,
            from_state TEXT NOT NULL,
            to_state TEXT NOT NULL,
            agent_id TEXT,
            reason TEXT,
            occurred_at TEXT
        );
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
        .unwrap();

    Arc::new(DB {
        pool: dummy_pg_pool,
        store: DbStore::Sqlite(pool),
    })
}

// TODO: These tests will fail to compile/run globally until `ohc_builtin_agent` and other unrelated module imports are fixed in `src/server/lib.rs`.
#[tokio::test]
async fn test_transition_task_v4() {
    let db = setup_db().await;
    let mesh: Arc<dyn TeammateMesh> = Arc::new(DummyMesh);
    let sm = V4StateMachine::new(db.clone(), mesh);

    if let DbStore::Sqlite(pool) = &db.store {
        sqlx::query("INSERT INTO shared_tasks_v4 (id, organization_id, title, status) VALUES ('task1', 'org1', 'Test', 'PENDING')")
            .execute(pool)
            .await
            .unwrap();
    }

    let res = sm
        .transition_task_v4("task1", "org1", "PENDING", "IN_PROGRESS", Some("agent1"), None)
        .await;
    assert!(res.is_ok());

    if let DbStore::Sqlite(pool) = &db.store {
        let status: String = sqlx::query_scalar("SELECT status FROM shared_tasks_v4 WHERE id = 'task1'")
            .fetch_one(pool)
            .await
            .unwrap();
        assert_eq!(status, "IN_PROGRESS");

        let trans_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM state_machine_transitions WHERE entity_id = 'task1' AND to_state = 'IN_PROGRESS'")
            .fetch_one(pool)
            .await
            .unwrap();
        assert_eq!(trans_count, 1);
    }
}

// TODO: These tests will fail to compile/run globally until `ohc_builtin_agent` and other unrelated module imports are fixed in `src/server/lib.rs`.
#[tokio::test]
async fn test_transition_sub_agent_queue() {
    let db = setup_db().await;
    let mesh: Arc<dyn TeammateMesh> = Arc::new(DummyMesh);
    let sm = V4StateMachine::new(db.clone(), mesh);

    if let DbStore::Sqlite(pool) = &db.store {
        sqlx::query("INSERT INTO sub_agent_queue (id, tenant_id, status) VALUES ('queue1', 'org1', 'QUEUED')")
            .execute(pool)
            .await
            .unwrap();
    }

    let res = sm
        .transition_sub_agent_queue("queue1", "org1", "QUEUED", "RUNNING", Some("worker1"), None)
        .await;
    assert!(res.is_ok());

    if let DbStore::Sqlite(pool) = &db.store {
        let status: String = sqlx::query_scalar("SELECT status FROM sub_agent_queue WHERE id = 'queue1'")
            .fetch_one(pool)
            .await
            .unwrap();
        assert_eq!(status, "RUNNING");

        let trans_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM state_machine_transitions WHERE entity_id = 'queue1' AND to_state = 'RUNNING'")
            .fetch_one(pool)
            .await
            .unwrap();
        assert_eq!(trans_count, 1);
    }
}
