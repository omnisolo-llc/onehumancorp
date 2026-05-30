use super::statemachine_v2::StateMachine;
use super::locks::StandaloneLock;
use std::sync::Arc;
use crate::db::{DB, DbStore};
use sqlx::sqlite::SqlitePoolOptions;
use super::mesh::TeammateMesh;

struct MockMesh;

#[async_trait::async_trait]
impl TeammateMesh for MockMesh {
    async fn publish(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> {
        Ok(())
    }
    async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }
    async fn subscribe(&self, _topic: &str) -> Result<Box<dyn super::mesh::Subscription>, String> {
        Err("not implemented".to_string())
    }
}

async fn setup_db() -> sqlx::SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'PENDING',
            assigned_agent_id TEXT,
            priority TEXT NOT NULL DEFAULT 'P2',
            payload TEXT,
            parent_plan_id TEXT,
            dependencies TEXT NOT NULL DEFAULT '[]',
            locked_until TEXT,
            tokens_consumed INTEGER DEFAULT 0,
            agent_role TEXT,
            model TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        );
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS state_machine_transitions (
            id TEXT PRIMARY KEY,
            entity_id TEXT NOT NULL,
            entity_type TEXT NOT NULL,
            from_state TEXT NOT NULL,
            to_state TEXT NOT NULL,
            agent_id TEXT,
            occurred_at TEXT DEFAULT CURRENT_TIMESTAMP
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
    let db = Arc::new(DB {
        pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
        store: DbStore::Sqlite(pool.clone()),
    });
    let mesh = Arc::new(MockMesh);
    let lock = Arc::new(StandaloneLock::new());
    let sm = StateMachine::new(db, mesh, lock);

    let task_id = "task1";
    sqlx::query("INSERT INTO shared_tasks_decomposition (id, organization_id, title) VALUES ('task1', 'org1', 't1')").execute(&pool).await.unwrap();

    // Pending -> Ready
    sm.transition(task_id, "READY", "", None).await.unwrap();
    let row: (String,) = sqlx::query_as("SELECT status FROM shared_tasks_decomposition WHERE id = 'task1'").fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, "READY");

    // Ready -> InProgress
    sm.transition(task_id, "IN_PROGRESS", "agent1", None).await.unwrap();
    let row: (String,) = sqlx::query_as("SELECT status FROM shared_tasks_decomposition WHERE id = 'task1'").fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, "IN_PROGRESS");

    // InProgress -> Blocked
    sm.transition(task_id, "BLOCKED", "", None).await.unwrap();
    let row: (String,) = sqlx::query_as("SELECT status FROM shared_tasks_decomposition WHERE id = 'task1'").fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, "BLOCKED");

    // Blocked -> InProgress
    sm.transition(task_id, "IN_PROGRESS", "agent1", None).await.unwrap();
    let row: (String,) = sqlx::query_as("SELECT status FROM shared_tasks_decomposition WHERE id = 'task1'").fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, "IN_PROGRESS");

    // InProgress -> Completed
    sm.transition(task_id, "COMPLETED", "", None).await.unwrap();
    let row: (String,) = sqlx::query_as("SELECT status FROM shared_tasks_decomposition WHERE id = 'task1'").fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, "COMPLETED");
}

#[tokio::test]
async fn test_statemachine_invalid_transition() {
    let pool = setup_db().await;
    let db = Arc::new(DB {
        pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
        store: DbStore::Sqlite(pool.clone()),
    });
    let mesh = Arc::new(MockMesh);
    let lock = Arc::new(StandaloneLock::new());
    let sm = StateMachine::new(db, mesh, lock);

    let task_id = "task2";
    sqlx::query("INSERT INTO shared_tasks_decomposition (id, organization_id, title) VALUES ('task2', 'org1', 't2')").execute(&pool).await.unwrap();

    // Pending -> Blocked (Invalid)
    let err = sm.transition(task_id, "BLOCKED", "agent1", None).await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_statemachine_concurrent_transitions() {
    let pool = setup_db().await;
    let db = Arc::new(DB {
        pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
        store: DbStore::Sqlite(pool.clone()),
    });
    let mesh = Arc::new(MockMesh);
    let lock = Arc::new(StandaloneLock::new());
    let sm = Arc::new(StateMachine::new(db, mesh, lock));

    let task_id = "task3";
    sqlx::query("INSERT INTO shared_tasks_decomposition (id, organization_id, title, status) VALUES ('task3', 'org1', 't3', 'READY')").execute(&pool).await.unwrap();

    let sm1 = sm.clone();
    let sm2 = sm.clone();

    let t1 = tokio::spawn(async move {
        sm1.transition("task3", "IN_PROGRESS", "agent1", None).await
    });

    let t2 = tokio::spawn(async move {
        // PENDING is invalid from READY, but IN_PROGRESS->PENDING is also invalid. So whichever is second fails.
        sm2.transition("task3", "PENDING", "agent2", None).await
    });

    let res1 = t1.await.unwrap();
    let res2 = t2.await.unwrap();
    let mut success_count = 0;
    if res1.is_ok() { success_count += 1; }
    if res2.is_ok() { success_count += 1; }
    assert!(success_count > 0, "At least one transition should succeed");
}
