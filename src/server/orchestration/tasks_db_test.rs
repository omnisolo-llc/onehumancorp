use crate::orchestration::tasks_db::TaskDbService;
use crate::db::DB;
use std::sync::Arc;

// Existing code...

#[tokio::test]
async fn test_tasks_db_claim_task_postgres() {
    // Setting up a dummy test for postgres
    let pg_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
    if !pg_url.starts_with("postgres") {
        return;
    }
    let pool = match sqlx::postgres::PgPoolOptions::new().connect(&pg_url).await {
        Ok(p) => p,
        Err(_) => return, // skip test if no db available
    };

    // We create a temp table or use shared_tasks
    let _ = sqlx::query(
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
            created_at TIMESTAMPTZ,
            updated_at TIMESTAMPTZ,
            locked_until TIMESTAMPTZ,
            _sync_status TEXT DEFAULT 'pending',
            version INTEGER DEFAULT 1,
            mission_id TEXT,
            priority TEXT DEFAULT 'NORMAL',
            payload TEXT
        );
        "#
    ).execute(&pool).await;

    let _ = sqlx::query("DELETE FROM shared_tasks").execute(&pool).await;

    let db = Arc::new(DB {
        pool: pool.clone(),
        store: crate::db::DbStore::Postgres,
    });

    let service = TaskDbService::new(db);

    sqlx::query(
        "INSERT INTO shared_tasks (id, organization_id, title, status, created_at, updated_at) VALUES ('1', 'org1', 'Task 1', 'PENDING', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
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
async fn test_tasks_db_claim_task_sqlite_null_dates() {
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

    let db = std::sync::Arc::new(crate::db::DB {
        pool: dummy_pg,
        store: crate::db::DbStore::Sqlite(pool.clone()),
    });

    let service = TaskDbService::new(db);

    sqlx::query(
        "INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ('1', 'org1', 'Task 1', 'PENDING')"
    )
    .execute(&pool)
    .await
    .unwrap();

    let task = service.claim_task("agent1").await.unwrap();
    assert!(task.is_some());
}

#[tokio::test]
async fn test_tasks_db_claim_task_sqlite_with_locked_until() {
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

    let db = std::sync::Arc::new(crate::db::DB {
        pool: dummy_pg,
        store: crate::db::DbStore::Sqlite(pool.clone()),
    });

    let service = crate::orchestration::tasks_db::TaskDbService::new(db);

    sqlx::query(
        "INSERT INTO shared_tasks (id, organization_id, title, status, locked_until) VALUES ('1', 'org1', 'Task 1', 'PENDING', '2025-01-01T00:00:00Z')"
    )
    .execute(&pool)
    .await
    .unwrap();

    let task = service.claim_task("agent1").await.unwrap();
    assert!(task.is_some());
    let task = task.unwrap();
    assert!(task.locked_until.is_some());
}
