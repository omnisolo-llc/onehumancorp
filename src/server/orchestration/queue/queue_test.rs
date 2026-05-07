use super::{TaskQueue, Job, SQLiteTaskQueue};
use std::sync::Arc;
use sqlx::SqlitePool;
use chrono::Utc;

#[tokio::test]
async fn test_sqlite_task_queue() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    sqlx::query(
        "CREATE TABLE sub_agent_jobs (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL DEFAULT '',
            parent_task_id TEXT,
            agent_role TEXT NOT NULL,
            payload TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'QUEUED',
            attempts INTEGER DEFAULT 0,
            max_attempts INTEGER DEFAULT 3,
            run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            locked_until TIMESTAMPTZ,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(&pool).await.unwrap();

    let queue = SQLiteTaskQueue::new(Arc::new(pool));

    let job = Job {
        id: "job-1".to_string(),
        tenant_id: "tenant-1".to_string(),
        parent_task_id: "parent-1".to_string(),
        agent_role: "test-role".to_string(),
        payload: "{}".to_string(),
        status: "QUEUED".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: Utc::now() - chrono::Duration::seconds(1),
        locked_until: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    queue.enqueue(job).await.unwrap();

    let dequeued_opt = queue.dequeue(vec!["test-role".to_string()]).await.unwrap();
    let dequeued = dequeued_opt.unwrap();
    assert_eq!(dequeued.id, "job-1");
    assert_eq!(dequeued.tenant_id, "tenant-1");

    queue.complete(&dequeued.id).await.unwrap();
}

#[tokio::test]
async fn test_sqlite_task_queue_empty_dequeue() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    sqlx::query(
        "CREATE TABLE sub_agent_jobs (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL DEFAULT '',
            parent_task_id TEXT,
            agent_role TEXT NOT NULL,
            payload TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'QUEUED',
            attempts INTEGER DEFAULT 0,
            max_attempts INTEGER DEFAULT 3,
            run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            locked_until TIMESTAMPTZ,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(&pool).await.unwrap();

    let queue = SQLiteTaskQueue::new(Arc::new(pool));

    let dequeued = queue.dequeue(vec!["test-role".to_string()]).await.unwrap();
    assert!(dequeued.is_none());
}

use super::PostgresTaskQueue;
use sqlx::postgres::PgPoolOptions;

#[tokio::test]
async fn test_postgres_task_queue() {
    if let Ok(db_url) = std::env::var("DATABASE_URL") {
        let pool = PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy(&db_url)
            .unwrap();

        // Ensure table exists for test
        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS sub_agent_jobs (
                id TEXT PRIMARY KEY,
                organization_id TEXT NOT NULL DEFAULT '',
                parent_task_id TEXT,
                agent_role TEXT NOT NULL,
                payload TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'QUEUED',
                attempts INTEGER DEFAULT 0,
                max_attempts INTEGER DEFAULT 3,
                run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                locked_until TIMESTAMPTZ,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
            )"
        ).execute(&pool).await;

        let queue = PostgresTaskQueue::new(Arc::new(pool));

        let job_id = uuid::Uuid::new_v4().to_string();
        let job = Job {
            id: job_id.clone(),
            tenant_id: "tenant-pg-1".to_string(),
            parent_task_id: "parent-1".to_string(),
            agent_role: "pg-test-role".to_string(),
            payload: "{}".to_string(),
            status: "QUEUED".to_string(),
            attempts: 0,
            max_attempts: 3,
            run_after: Utc::now() - chrono::Duration::seconds(1),
            locked_until: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        queue.enqueue(job).await.unwrap();

        let dequeued_opt = queue.dequeue(vec!["pg-test-role".to_string()]).await.unwrap();
        let dequeued = dequeued_opt.unwrap();
        assert_eq!(dequeued.id, job_id);
        assert_eq!(dequeued.tenant_id, "tenant-pg-1");

        queue.complete(&dequeued.id).await.unwrap();
    }
}
