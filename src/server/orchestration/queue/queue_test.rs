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
            organization_id TEXT,
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
        tenant_id: "system".to_string(),
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

    let dequeued_opt = queue.dequeue(vec!["test-role".to_string()], 100, 100).await.unwrap();
    let dequeued = dequeued_opt.unwrap();
    assert_eq!(dequeued.id, "job-1");
    assert_eq!(dequeued.tenant_id, "system");

    queue.complete(&dequeued.id).await.unwrap();
}

#[tokio::test]
async fn test_sqlite_task_queue_empty_dequeue() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    sqlx::query(
        "CREATE TABLE sub_agent_jobs (
            id TEXT PRIMARY KEY,
            organization_id TEXT,
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

    let dequeued = queue.dequeue(vec!["test-role".to_string()], 100, 100).await.unwrap();
    assert!(dequeued.is_none());
}

#[tokio::test]
async fn test_sqlite_fail_backoff() {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();

    sqlx::query(
        "CREATE TABLE sub_agent_jobs (
            id TEXT PRIMARY KEY,
            organization_id TEXT,
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

    let queue = super::SQLiteTaskQueue::new(std::sync::Arc::new(pool.clone()));

    let job = super::Job {
        id: "job-fail-1".to_string(),
        tenant_id: "system".to_string(),
        parent_task_id: "parent-1".to_string(),
        agent_role: "test-role".to_string(),
        payload: "{}".to_string(),
        status: "QUEUED".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: chrono::Utc::now() - chrono::Duration::seconds(10),
        locked_until: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    queue.enqueue(job).await.unwrap();

    let before_fail = chrono::Utc::now();
    queue.fail("job-fail-1", "test").await.unwrap();

    // After fail, attempts should be 1, status QUEUED, and run_after should be updated
    use sqlx::Row;
    let row = sqlx::query("SELECT attempts, status, run_after FROM sub_agent_jobs WHERE id = 'job-fail-1'").fetch_one(&pool).await.unwrap();

    let attempts: i32 = row.get("attempts");
    assert_eq!(attempts, 1);

    let status: String = row.get("status");
    assert_eq!(status, "QUEUED");

    // Test if parsing to DateTime works successfully (which guarantees the fix works)
    let run_after_str: String = row.get("run_after");
    let run_after: chrono::DateTime<chrono::Utc> = run_after_str.parse().expect("run_after must be a valid ISO 8601 string");

    // Verify run_after is approximately now + 2 seconds (1 << 1 attempt)
    let backoff_duration = chrono::Duration::seconds(2);
    let expected_time = before_fail + backoff_duration;

    let diff = run_after.signed_duration_since(expected_time).num_milliseconds().abs();
    assert!(diff < 1000, "run_after timestamp should be roughly Utc::now() + backoff time, but got difference of {} ms", diff);
}

#[tokio::test]
async fn test_sqlite_complete() {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();

    sqlx::query(
        "CREATE TABLE sub_agent_jobs (
            id TEXT PRIMARY KEY,
            organization_id TEXT,
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

    let queue = super::SQLiteTaskQueue::new(std::sync::Arc::new(pool.clone()));

    let job = super::Job {
        id: "job-complete-1".to_string(),
        tenant_id: "system".to_string(),
        parent_task_id: "parent-1".to_string(),
        agent_role: "test-role".to_string(),
        payload: "{}".to_string(),
        status: "QUEUED".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: chrono::Utc::now() - chrono::Duration::seconds(10),
        locked_until: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    queue.enqueue(job).await.unwrap();

    queue.complete("job-complete-1").await.unwrap();

    use sqlx::Row;
    let row = sqlx::query("SELECT status FROM sub_agent_jobs WHERE id = 'job-complete-1'").fetch_one(&pool).await.unwrap();

    let status: String = row.get("status");
    assert_eq!(status, "COMPLETED");
}
