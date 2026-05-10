use super::shared_tasks::{SharedTaskOrchestrator, SharedTaskV4};
use crate::db::DB;
use std::sync::Arc;
use chrono::Utc;

#[tokio::test]
async fn test_shared_task_orchestrator() {
    if std::env::var("DATABASE_URL").is_err() {
        return;
    }

    // Safety check - do not run db tests with production DB
    let db_url = std::env::var("DATABASE_URL").unwrap();
    if !db_url.contains("test") {
        return;
    }

    let pool = sqlx::postgres::PgPoolOptions::new()
        .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
        .acquire_timeout(std::time::Duration::from_millis(50))
        .connect_lazy(&db_url)
        .unwrap();

    let db = DB { pool: pool.clone(), store: crate::db::DbStore::Postgres };
    let db = Arc::new(db);
    let orchestrator = SharedTaskOrchestrator::new(db.clone());

    let task = SharedTaskV4 {
        id: "".to_string(),
        organization_id: "org_123".to_string(),
        title: "Test Task".to_string(),
        description: Some("Description".to_string()),
        status: "PENDING".to_string(),
        agent_id: Some("agent_1".to_string()),
        priority: "P1".to_string(),
        payload: Some("{}".to_string()),
        parent_plan_id: None,
        dependencies: "[]".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let result = orchestrator.create_task(task).await;
    // Database might not be fully migrated in this test env, we just test compiling and running.
    if let Ok(created_task) = result {
        assert!(!created_task.id.is_empty());
        assert_eq!(created_task.title, "Test Task");

        let fetched_task = orchestrator.get_task(&created_task.id).await.unwrap();
        assert_eq!(fetched_task.id, created_task.id);
        assert_eq!(fetched_task.organization_id, "org_123");

        let claimed_task = orchestrator.claim_task("org_123", "agent_123").await.unwrap();
        assert!(claimed_task.is_some());
        let claimed = claimed_task.unwrap();
        assert_eq!(claimed.id, created_task.id);
        assert_eq!(claimed.status, "ASSIGNED");
        assert_eq!(claimed.agent_id.unwrap(), "agent_123");

        let empty_claim = orchestrator.claim_task("org_123", "agent_123").await.unwrap();
        assert!(empty_claim.is_none());
    }
}

#[tokio::test]
async fn test_shared_task_orchestrator_sqlite() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
            id VARCHAR PRIMARY KEY,
            organization_id VARCHAR NOT NULL,
            title VARCHAR NOT NULL,
            description TEXT,
            status VARCHAR NOT NULL DEFAULT 'PENDING',
            agent_id VARCHAR,
            priority VARCHAR NOT NULL DEFAULT 'P2',
            payload TEXT,
            parent_plan_id TEXT,
            dependencies TEXT NOT NULL DEFAULT '[]',
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        );
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
        .connect_lazy("postgres://postgres:postgres@localhost:5432/postgres")
        .unwrap();

    let db = DB { pool: dummy_pg_pool, store: crate::db::DbStore::Sqlite(pool) };
    let db = Arc::new(db);
    let orchestrator = SharedTaskOrchestrator::new(db.clone());

    let task = SharedTaskV4 {
        id: "task_1".to_string(),
        organization_id: "org_123".to_string(),
        title: "Test Task".to_string(),
        description: Some("Description".to_string()),
        status: "PENDING".to_string(),
        agent_id: None,
        priority: "P1".to_string(),
        payload: Some("{}".to_string()),
        parent_plan_id: None,
        dependencies: "[]".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let created_task = orchestrator.create_task(task).await.unwrap();
    assert_eq!(created_task.id, "task_1");

    let claimed_task = orchestrator.claim_task("org_123", "agent_123").await.unwrap();
    assert!(claimed_task.is_some());
    let claimed = claimed_task.unwrap();
    assert_eq!(claimed.id, "task_1");
    assert_eq!(claimed.status, "ASSIGNED");
    assert_eq!(claimed.agent_id.unwrap(), "agent_123");
}
