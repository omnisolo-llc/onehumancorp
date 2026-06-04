use crate::orchestration::tasks_db::TaskDbService;
use crate::db::DB;
use std::sync::Arc;

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
            dependencies TEXT DEFAULT '[]',
            created_at TEXT,
            updated_at TEXT,
            locked_until TEXT,
            _sync_status TEXT DEFAULT 'pending',
            version INTEGER DEFAULT 1
        );
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    let dummy_pg = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy("postgres://dummy")
        .unwrap();

    let db = Arc::new(DB {
        pool: dummy_pg,
        store: crate::db::DbStore::Sqlite(pool.clone()),
    });

    let service = TaskDbService::new(db);

    sqlx::query(
        "INSERT INTO shared_tasks (id, organization_id, title, status, created_at, updated_at) VALUES ('1', 'org1', 'Task 1', 'PENDING', '2023-01-01T00:00:00Z', '2023-01-01T00:00:00Z')"
    )
    .execute(&pool)
    .await
    .unwrap();

    let task = service.claim_task("agent1").await.unwrap();
    assert!(task.is_some());
    let task = task.unwrap();
    assert_eq!(task.id, "1");
    assert_eq!(task.status, "ASSIGNED");
    assert_eq!(task.assigned_agent_id, Some("agent1".to_string()));

    let task2 = service.claim_task("agent2").await.unwrap();
    assert!(task2.is_none());
}

#[tokio::test]
async fn test_tasks_db_claim_task_concurrent_sqlite() {
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
            dependencies TEXT DEFAULT '[]',
            created_at TEXT,
            updated_at TEXT,
            locked_until TEXT,
            _sync_status TEXT DEFAULT 'pending',
            version INTEGER DEFAULT 1
        );
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    let dummy_pg = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy("postgres://dummy")
        .unwrap();

    let db = Arc::new(DB {
        pool: dummy_pg,
        store: crate::db::DbStore::Sqlite(pool.clone()),
    });

    let service = Arc::new(TaskDbService::new(db));

    sqlx::query(
        "INSERT INTO shared_tasks (id, organization_id, title, status, created_at, updated_at) VALUES ('concurrent1', 'org1', 'Concurrent Task 1', 'PENDING', '2023-01-01T00:00:00Z', '2023-01-01T00:00:00Z')"
    )
    .execute(&pool)
    .await
    .unwrap();

    let mut handles = vec![];
    for i in 0..10 {
        let s = service.clone();
        handles.push(tokio::spawn(async move {
            s.claim_task(&format!("agent{}", i)).await.unwrap()
        }));
    }

    let mut claimed = 0;
    for handle in handles {
        if let Ok(Some(_)) = handle.await {
            claimed += 1;
        }
    }

    assert_eq!(claimed, 1, "Only one agent should be able to claim the single pending task concurrently");
}


#[tokio::test]
async fn test_task_timeout_and_retry() {
    let timeout = crate::orchestration::tasks::task_claim_timeout();
    assert_eq!(timeout.as_secs(), 60, "Must be configured with a 60-second timeout");
}
