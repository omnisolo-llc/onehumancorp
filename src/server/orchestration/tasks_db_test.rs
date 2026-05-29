use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::orchestration::tasks_db::TasksDB;

#[tokio::test]
async fn test_tasks_db_claim_task_sqlite() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            parent_plan_id TEXT,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'PENDING',
            assigned_agent_id TEXT,
            dependencies JSONB DEFAULT '[]',
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            approval_status TEXT
        );

        CREATE TABLE IF NOT EXISTS state_machine_transitions (
            id TEXT PRIMARY KEY,
            task_id TEXT,
            from_state TEXT,
            to_state TEXT,
            agent_id TEXT,
            transitioned_at TEXT
        );
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/postgres").unwrap();

    let db = DB { pool: dummy_pg_pool, store: DbStore::Sqlite(pool.clone()) };
    let db = Arc::new(db);
    let tasks_db = TasksDB::new(db.clone());

    sqlx::query("INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ('task1', 'org1', 'Test Task 1', 'PENDING')")
        .execute(&pool)
        .await
        .unwrap();

    let claimed_task = tasks_db.claim_task("org1", "agent1").await.unwrap();
    assert!(claimed_task.is_some());
    let claimed = claimed_task.unwrap();
    assert_eq!(claimed.id, "task1");
    assert_eq!(claimed.status, "IN_PROGRESS");
    assert_eq!(claimed.assigned_agent_id.unwrap(), "agent1");

    let empty_claim = tasks_db.claim_task("org1", "agent2").await.unwrap();
    assert!(empty_claim.is_none());
}
