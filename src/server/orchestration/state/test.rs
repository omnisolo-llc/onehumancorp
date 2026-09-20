use super::{StateManager, standalone::StandaloneStateManager};
use crate::db::{DB, DbStore};

use std::sync::Arc;

use sqlx::sqlite::SqlitePoolOptions;

use crate::orchestration::mesh::TeammateMesh;
use async_trait::async_trait;
use omnisolo_builtin_agent::mesh::transport::{InProcessTransport, MeshTransport, Message};

struct MockMesh {
    transport: InProcessTransport,
}

impl MockMesh {
    fn new() -> Self {
        Self {
            transport: InProcessTransport::new(),
        }
    }
}

#[async_trait]
impl TeammateMesh for MockMesh {
    async fn publish(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> {
        Ok(())
    }
    async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> {
        Ok(())
    }
    async fn subscribe(
        &self,
        _topic: &str,
        _handler: Box<dyn Fn(Message) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        Ok(Box::new(|| {}))
    }
    async fn acquire_lock(
        &self,
        resource: &str,
        owner: &str,
        ttl_seconds: u64,
    ) -> Result<bool, String> {
        self.transport
            .acquire_lock(resource, owner, ttl_seconds)
            .await
    }
    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        self.transport.release_lock(resource, owner).await
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
        _handler: Box<dyn Fn(Message) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        Ok(Box::new(|| {}))
    }
}

async fn setup_db() -> Arc<DB> {
    let db_id = uuid::Uuid::new_v4().to_string();
    let uri = format!("sqlite:file:{}?mode=memory&cache=shared", db_id);
    let sqlite_pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&uri)
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE swarm_tasks (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL DEFAULT 'test_org',
            mission_id TEXT NOT NULL,
            parent_plan_id TEXT,
            dependencies TEXT NOT NULL DEFAULT '[]',
            title TEXT NOT NULL,
            description TEXT,
            priority TEXT,
            status TEXT NOT NULL DEFAULT 'PENDING',
            assigned_agent_id TEXT,
            locked_until TEXT,
            payload TEXT,
            created_at TEXT,
            updated_at TEXT
        );
        "#,
    )
    .execute(&sqlite_pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE state_machine_transitions (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL DEFAULT 'test_org',
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
    .execute(&sqlite_pool)
    .await
    .unwrap();

    let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new()
        .after_release(|conn, _meta| {
            Box::pin(async move {
                use sqlx::Executor;
                conn.execute("DISCARD ALL").await?;
                Ok(true)
            })
        })
        .after_release(|conn, _meta| {
            Box::pin(async move {
                use sqlx::Executor;
                conn.execute("DISCARD ALL").await?;
                Ok(true)
            })
        })
        .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
        .unwrap();

    Arc::new(DB {
        pool: dummy_pg_pool,
        store: DbStore::Sqlite(sqlite_pool),
    })
}

#[tokio::test]
async fn test_single_agent_flow() {
    let db = setup_db().await;
    let mesh: Arc<dyn TeammateMesh> = Arc::new(MockMesh::new());
    let state_manager = StandaloneStateManager::new(db.clone(), mesh);

    let task_id = uuid::Uuid::new_v4().to_string();

    if let DbStore::Sqlite(pool) = &db.store {
        sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status, tenant_id) VALUES (?, 'm1', 't1', 'PENDING', 'default_tenant')")
            .bind(&task_id)
            .execute(pool)
            .await
            .unwrap();
    }

    let result = state_manager
        .transition_state(
            &task_id,
            "default_tenant",
            "PENDING",
            "IN_PROGRESS",
            Some("agent_1"),
            None,
        )
        .await;
    tracing::info!("Result: {:?}", result);
    assert!(result.is_ok());

    if let DbStore::Sqlite(pool) = &db.store {
        let status: String = sqlx::query_scalar("SELECT status FROM swarm_tasks WHERE id = ?")
            .bind(&task_id)
            .fetch_one(pool)
            .await
            .unwrap();
        assert_eq!(status, "IN_PROGRESS");
    }
}

#[tokio::test]
async fn test_dag_workflow() {
    let db = setup_db().await;
    let mesh: Arc<dyn TeammateMesh> = Arc::new(MockMesh::new());
    let state_manager = StandaloneStateManager::new(db.clone(), mesh);

    let parent_id = uuid::Uuid::new_v4().to_string();
    let child_id = uuid::Uuid::new_v4().to_string();
    let deps = format!(r#"["{}"]"#, parent_id);

    if let DbStore::Sqlite(pool) = &db.store {
        sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status, tenant_id) VALUES (?, 'm1', 'parent', 'PENDING', 'default_tenant')")
            .bind(&parent_id)
            .execute(pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status, dependencies, tenant_id) VALUES (?, 'm1', 'child', 'PENDING', ?, 'default_tenant')")
            .bind(&child_id)
            .bind(&deps)
            .execute(pool)
            .await
            .unwrap();
    }

    // Since pull_available_tasks now updates them to IN_PROGRESS directly
    let tasks = state_manager.pull_available_tasks(10).await.unwrap();

    // Parent should be available, child should not because parent is PENDING (now IN_PROGRESS)
    assert!(tasks.iter().any(|t| t.id == parent_id));
    assert!(!tasks.iter().any(|t| t.id == child_id));

    // Complete parent - parent was moved to IN_PROGRESS by pull_available_tasks
    state_manager
        .transition_state(
            &parent_id,
            "default_tenant",
            "IN_PROGRESS",
            "COMPLETED",
            Some("agent_1"),
            None,
        )
        .await
        .unwrap();

    // Now child should be available
    let tasks_after = state_manager.pull_available_tasks(10).await.unwrap();
    assert!(tasks_after.iter().any(|t| t.id == child_id));
}

#[tokio::test]
async fn test_missing_dependency_blocking() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(
        "CREATE TABLE swarm_tasks (
            id TEXT PRIMARY KEY,
            mission_id TEXT,
            title TEXT NOT NULL,
            status TEXT NOT NULL,
            dependencies TEXT,
            tenant_id TEXT NOT NULL DEFAULT 'test_org',
            payload TEXT,
            created_at TEXT,
            updated_at TEXT
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE state_machine_transitions (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            entity_id TEXT NOT NULL,
            entity_type TEXT NOT NULL,
            from_state TEXT NOT NULL,
            to_state TEXT NOT NULL,
            agent_id TEXT,
            reason TEXT,
            occurred_at TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    let db = Arc::new(DB {
        pool: crate::db::get_pool(),
        store: DbStore::Sqlite(pool.clone()),
    });

    let mock_mesh: Arc<dyn TeammateMesh> = Arc::new(MockMesh::new());

    let state_manager = StandaloneStateManager::new(db, mock_mesh);

    // Insert task 1 that depends on non-existent task "missing-id"
    sqlx::query(
        "INSERT INTO swarm_tasks (id, title, status, dependencies) VALUES (?, 'Task 1', 'PENDING', '[\"missing-id\"]')"
    )
    .bind("task-1")
    .execute(&pool)
    .await
    .unwrap();

    // Verify it is NOT pulled because dependency is missing
    let tasks = state_manager.pull_available_tasks(10).await.unwrap();
    assert_eq!(
        tasks.len(),
        0,
        "Task 1 should be blocked by missing dependency"
    );

    // Insert the missing dependency, but as PENDING
    sqlx::query(
        "INSERT INTO swarm_tasks (id, title, status, dependencies) VALUES (?, 'Missing Task', 'PENDING', '[]')"
    )
    .bind("missing-id")
    .execute(&pool)
    .await
    .unwrap();

    // Verify task 1 is still blocked because dependency is not COMPLETED
    let mut tasks = state_manager.pull_available_tasks(10).await.unwrap();
    // Only the missing-id task should be pulled now
    assert_eq!(tasks.len(), 1, "Only dependency task should be pulled");
    assert_eq!(tasks[0].id, "missing-id");

    // Mark dependency as COMPLETED
    sqlx::query("UPDATE swarm_tasks SET status = 'COMPLETED' WHERE id = ?")
        .bind("missing-id")
        .execute(&pool)
        .await
        .unwrap();

    // Now Task 1 should be available
    tasks = state_manager.pull_available_tasks(10).await.unwrap();
    assert_eq!(
        tasks.len(),
        1,
        "Task 1 should be pulled now that dependency is completed"
    );
    assert_eq!(tasks[0].id, "task-1");
}

// Exercise persisted DAG pause/replay semantics; SQLite is not cloud certification.
#[tokio::test]
async fn paused_parent_blocks_child_and_replay_cannot_duplicate_transition() {
    let db = setup_db().await;
    let mesh: Arc<dyn TeammateMesh> = Arc::new(MockMesh::new());
    let state_manager = StandaloneStateManager::new(db.clone(), mesh);

    let parent_id = uuid::Uuid::new_v4().to_string();
    let child_id = uuid::Uuid::new_v4().to_string();
    let deps = format!(r#"["{}"]"#, parent_id);

    if let DbStore::Sqlite(pool) = &db.store {
        sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status, tenant_id) VALUES (?, 'm1', 'parent', 'PENDING', 'default_tenant')")
            .bind(&parent_id)
            .execute(pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status, dependencies, tenant_id) VALUES (?, 'm1', 'child', 'PENDING', ?, 'default_tenant')")
            .bind(&child_id)
            .bind(&deps)
            .execute(pool)
            .await
            .unwrap();
    }

    state_manager
        .transition_state(
            &parent_id,
            "default_tenant",
            "PENDING",
            "PAUSED",
            Some("agent-a"),
            Some("provider unavailable"),
        )
        .await
        .unwrap();
    assert!(
        state_manager
            .pull_available_tasks(10)
            .await
            .unwrap()
            .is_empty()
    );
    assert!(
        state_manager
            .transition_state(
                &child_id,
                "default_tenant",
                "PENDING",
                "IN_PROGRESS",
                Some("agent-a"),
                None
            )
            .await
            .is_err()
    );
    assert!(
        state_manager
            .transition_state(
                &parent_id,
                "another-tenant",
                "PAUSED",
                "COMPLETED",
                None,
                None
            )
            .await
            .is_err()
    );
    state_manager
        .transition_state(
            &parent_id,
            "default_tenant",
            "PAUSED",
            "COMPLETED",
            Some("agent-a"),
            Some("owner resolved exception"),
        )
        .await
        .unwrap();
    assert!(
        state_manager
            .transition_state(
                &parent_id,
                "default_tenant",
                "PAUSED",
                "COMPLETED",
                Some("agent-a"),
                None
            )
            .await
            .is_err()
    );
    let ready = state_manager.pull_available_tasks(10).await.unwrap();
    assert_eq!(ready.len(), 1);
    assert_eq!(ready[0].id, child_id);
    let DbStore::Sqlite(pool) = &db.store else {
        panic!("Expected the isolated SQLite fixture");
    };
    let transitions: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM state_machine_transitions WHERE entity_id=?")
            .bind(&parent_id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(
        transitions, 2,
        "Rejected/replayed transitions must not produce duplicate evidence"
    );
}

struct SleepingMockMesh;

#[async_trait]
impl TeammateMesh for SleepingMockMesh {
    async fn publish(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> {
        Ok(())
    }
    async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> {
        Ok(())
    }
    async fn subscribe(
        &self,
        _topic: &str,
        _handler: Box<dyn Fn(Message) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        Ok(Box::new(|| {}))
    }
    async fn acquire_lock(
        &self,
        _resource: &str,
        _owner: &str,
        _ttl_seconds: u64,
    ) -> Result<bool, String> {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
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
        _handler: Box<dyn Fn(Message) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        Ok(Box::new(|| {}))
    }
}

#[tokio::test]
async fn test_degradation_fallback_standalone() {
    unsafe {
        std::env::set_var("OMNISOLO_STATE_MANAGER_TIMEOUT_MS", "50");
    }

    let db = setup_db().await;
    let mesh: Arc<dyn TeammateMesh> = Arc::new(SleepingMockMesh);
    let state_manager = StandaloneStateManager::new(db.clone(), mesh);

    // Testing the fail-safe behavior via mocked timeout
    // The acquire_lock on the MockMesh sleeps past the configured timeout.
    let start = std::time::Instant::now();
    let tasks = state_manager.pull_available_tasks(10).await.unwrap();
    let elapsed = start.elapsed();

    // It should time out around the configured threshold, not the full lock wait.
    assert!(elapsed < std::time::Duration::from_millis(100));
    assert!(elapsed > std::time::Duration::from_millis(40));

    // And returned empty list fail-safe
    assert_eq!(tasks.len(), 0);

    unsafe {
        std::env::remove_var("OMNISOLO_STATE_MANAGER_TIMEOUT_MS");
    }
}
