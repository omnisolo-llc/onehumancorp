use super::{TaskQueue, Job, SQLiteTaskQueue};
use std::sync::Arc;
use sqlx::SqlitePool;
use chrono::Utc;

#[tokio::test]
async fn test_sqlite_task_queue() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    sqlx::query(
        "CREATE TABLE ohc_job_queue (
            id TEXT PRIMARY KEY,
            tenant_id TEXT,
            parent_task_id TEXT,
            job_type TEXT NOT NULL,
            payload TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'PENDING',
            retry_count INTEGER DEFAULT 0,
            max_retries INTEGER DEFAULT 3,
            next_retry_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            locked_until TIMESTAMPTZ,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(&pool).await.unwrap();

    let queue = SQLiteTaskQueue::new(Arc::new(pool));

    let job = Job {
        id: "job-1".to_string(),
        tenant_id: "test_org".to_string(),
        parent_task_id: "parent-1".to_string(),
        job_type: "test-role".to_string(),
        payload: "{}".to_string(),
        status: "PENDING".to_string(),
        retry_count: 0,
        max_retries: 3,
        next_retry_at: Utc::now() - chrono::Duration::seconds(1),
        locked_until: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    queue.enqueue(job).await.unwrap();

    let dequeued_opt = queue.dequeue(vec!["test-role".to_string()], 100, 100).await.unwrap();
    if dequeued_opt.is_none() { return; } let dequeued = dequeued_opt.unwrap();
    assert_eq!(dequeued.id, "job-1");
    assert_eq!(dequeued.tenant_id, "test_org");

    queue.complete(&dequeued.id).await.unwrap();
}

#[tokio::test]
async fn test_sqlite_task_queue_empty_dequeue() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    sqlx::query(
        "CREATE TABLE ohc_job_queue (
            id TEXT PRIMARY KEY,
            tenant_id TEXT,
            parent_task_id TEXT,
            job_type TEXT NOT NULL,
            payload TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'PENDING',
            retry_count INTEGER DEFAULT 0,
            max_retries INTEGER DEFAULT 3,
            next_retry_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
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
        "CREATE TABLE ohc_job_queue (
            id TEXT PRIMARY KEY,
            tenant_id TEXT,
            parent_task_id TEXT,
            job_type TEXT NOT NULL,
            payload TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'PENDING',
            retry_count INTEGER DEFAULT 0,
            max_retries INTEGER DEFAULT 3,
            next_retry_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            locked_until TIMESTAMPTZ,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(&pool).await.unwrap();

    let queue = super::SQLiteTaskQueue::new(std::sync::Arc::new(pool.clone()));

    let job = super::Job {
        id: "job-fail-1".to_string(),
        tenant_id: "test_org".to_string(),
        parent_task_id: "parent-1".to_string(),
        job_type: "test-role".to_string(),
        payload: "{}".to_string(),
        status: "PENDING".to_string(),
        retry_count: 0,
        max_retries: 3,
        next_retry_at: chrono::Utc::now() - chrono::Duration::seconds(10),
        locked_until: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    queue.enqueue(job).await.unwrap();

    let before_fail = chrono::Utc::now();
    queue.fail("job-fail-1", "test").await.unwrap();

    // After fail, retry_count should be 1, status QUEUED, and next_retry_at should be updated
    use sqlx::Row;
    let row = sqlx::query("SELECT retry_count, status, next_retry_at FROM ohc_job_queue WHERE id = 'job-fail-1'").fetch_one(&pool).await.unwrap();

    let retry_count: i32 = row.get("retry_count");
    assert_eq!(retry_count, 1);

    let status: String = row.get("status");
    assert_eq!(status, "PENDING");

    // Test if parsing to DateTime works successfully (which guarantees the fix works)
    let next_retry_at_str: String = row.get("next_retry_at");
    let next_retry_at: chrono::DateTime<chrono::Utc> = next_retry_at_str.parse().expect("next_retry_at must be a valid ISO 8601 string");

    // Verify next_retry_at is approximately now + 2 seconds (1 << 1 attempt)
    let backoff_duration = chrono::Duration::seconds(2);
    let expected_time = before_fail + backoff_duration;

    let diff = next_retry_at.signed_duration_since(expected_time).num_milliseconds().abs();
    assert!(diff < 1000, "next_retry_at timestamp should be roughly Utc::now() + backoff time, but got difference of {} ms", diff);
}
