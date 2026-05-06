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

    queue.complete(&dequeued.id).await.unwrap();
}

#[tokio::test]
async fn test_sqlite_task_queue_empty_dequeue() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    sqlx::query(
        "CREATE TABLE sub_agent_jobs (
            id TEXT PRIMARY KEY,
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

#[tokio::test]
async fn test_sqlite_task_queue_prune_stuck_jobs() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    sqlx::query(
        "CREATE TABLE sub_agent_jobs (
            id TEXT PRIMARY KEY,
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

    let queue = SQLiteTaskQueue::new(Arc::new(pool.clone()));

    // Insert a job that's been RUNNING for over 2 hours
    let old_time = Utc::now() - chrono::Duration::hours(2);
    sqlx::query("INSERT INTO sub_agent_jobs (id, parent_task_id, agent_role, payload, status, updated_at) VALUES (?, ?, ?, ?, ?, ?)")
        .bind("stuck-job")
        .bind("parent")
        .bind("role")
        .bind("{}")
        .bind("RUNNING")
        .bind(old_time)
        .execute(&pool).await.unwrap();

    // Insert a job that's RUNNING but only 5 minutes old
    let recent_time = Utc::now() - chrono::Duration::minutes(5);
    sqlx::query("INSERT INTO sub_agent_jobs (id, parent_task_id, agent_role, payload, status, updated_at) VALUES (?, ?, ?, ?, ?, ?)")
        .bind("active-job")
        .bind("parent")
        .bind("role")
        .bind("{}")
        .bind("RUNNING")
        .bind(recent_time)
        .execute(&pool).await.unwrap();

    queue.prune_stuck_jobs().await.unwrap();

    let stuck_status: String = sqlx::query_scalar("SELECT status FROM sub_agent_jobs WHERE id = 'stuck-job'")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(stuck_status, "FAILED");

    let active_status: String = sqlx::query_scalar("SELECT status FROM sub_agent_jobs WHERE id = 'active-job'")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(active_status, "RUNNING");
}
