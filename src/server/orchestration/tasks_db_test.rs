use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::orchestration::tasks_db::TasksDB;

#[tokio::test]
async fn test_claim_task_sqlite() {
    let database_url = "sqlite::memory:";
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(database_url)
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE shared_tasks (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            parent_plan_id TEXT,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'PENDING',
            assigned_agent_id TEXT,
            dependencies TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        );
        "#
    ).execute(&pool).await.unwrap();

    sqlx::query(
        "INSERT INTO shared_tasks (id, organization_id, title, status, dependencies, created_at, updated_at) VALUES ('1', 'org-1', 'Test', 'PENDING', '[]', '2023-01-01T00:00:00Z', '2023-01-01T00:00:00Z')"
    ).execute(&pool).await.unwrap();

    let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap();
    let db = Arc::new(DB { pool: dummy_pg_pool, store: DbStore::Sqlite(pool) });
    let tasks_db = TasksDB::new(db);

    let result = tasks_db.claim_task("org-1", "agent-1").await.unwrap();
    assert!(result.is_some());
    let task = result.unwrap();
    assert_eq!(task.id, "1");
    assert_eq!(task.status, "ASSIGNED");
    assert_eq!(task.assigned_agent_id, Some("agent-1".to_string()));

    let empty_result = tasks_db.claim_task("org-1", "agent-2").await.unwrap();
    assert!(empty_result.is_none());
}
