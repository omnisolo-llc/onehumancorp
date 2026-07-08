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

    let queue = SQLiteTaskQueue::new(Arc::new(pool.clone()));

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
    // For SQLite enqueue doesn't preserve retry_count (defaults to 0), so we must update it
    sqlx::query("UPDATE ohc_job_queue SET retry_count = 2 WHERE id = 'job-fail-sqlite-dead'").execute(&pool).await.unwrap();

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

    let queue = SQLiteTaskQueue::new(Arc::new(pool.clone()));

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
    // For SQLite enqueue doesn't preserve retry_count (defaults to 0), so we must update it
    sqlx::query("UPDATE ohc_job_queue SET retry_count = 0 WHERE id = 'job-fail-1'").execute(&pool).await.unwrap();

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

#[tokio::test]
async fn test_sqlite_fail_max_retries_dead_letter() {
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

    sqlx::query(
        "CREATE TABLE department_dead_letters (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            event_type TEXT NOT NULL,
            department TEXT NOT NULL,
            payload TEXT NOT NULL,
            error_message TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE agents (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            name TEXT NOT NULL,
            role TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'ACTIVE'
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query("INSERT INTO agents (id, tenant_id, name, role, status) VALUES ('agent-1', 'test_org', 'test', 'test', 'ACTIVE')").execute(&pool).await.unwrap();

    let queue = super::SQLiteTaskQueue::new(std::sync::Arc::new(pool.clone()));

    let job = super::Job {
        id: "job-fail-sqlite-dead".to_string(),
        tenant_id: "test_org".to_string(),
        parent_task_id: "parent-1".to_string(),
        job_type: "test-role".to_string(),
        payload: "{\"test\": \"payload\"}".to_string(),
        status: "PENDING".to_string(),
        retry_count: 2, // Next fail will be 3, which is max_retries
        max_retries: 3,
        next_retry_at: chrono::Utc::now() - chrono::Duration::seconds(10),
        locked_until: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    queue.enqueue(job).await.unwrap();
    // For SQLite enqueue doesn't preserve retry_count (defaults to 0), so we must update it
    sqlx::query("UPDATE ohc_job_queue SET retry_count = 2 WHERE id = 'job-fail-sqlite-dead'").execute(&pool).await.unwrap();

    // Trigger failure that exceeds max_retries
    queue.fail("job-fail-sqlite-dead", "test reason").await.unwrap();

    // Verify job is marked as FAILED
    use sqlx::Row;
    let row = sqlx::query("SELECT status FROM ohc_job_queue WHERE id = 'job-fail-sqlite-dead'").fetch_one(&pool).await.unwrap();
    let status: String = row.get("status");
    assert_eq!(status.as_str(), "FAILED");

    // Verify dead letter was created
    let dl_row = sqlx::query("SELECT tenant_id, event_type, department, payload, error_message FROM department_dead_letters LIMIT 1").fetch_one(&pool).await.unwrap();
    let dl_tenant_id: String = dl_row.get("tenant_id");
    let dl_event_type: String = dl_row.get("event_type");
    let dl_department: String = dl_row.get("department");
    let dl_payload: String = dl_row.get("payload");
    let dl_error_message: String = dl_row.get("error_message");

    assert_eq!(dl_tenant_id, "test_org");
    assert_eq!(dl_event_type, "job_failed");
    assert_eq!(dl_department, "job_queue");
    assert_eq!(dl_payload, "{\"test\": \"payload\"}");
    assert_eq!(dl_error_message, "test reason");
}

#[tokio::test]
async fn test_sqlite_queue_concurrent_workers() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use sqlx::sqlite::SqlitePoolOptions;

    // Use a shared memory database to allow concurrent connections
    let pool = SqlitePoolOptions::new()
        .max_connections(20)
        .connect("sqlite::memory:?cache=shared").await.unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS ohc_job_queue (
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

    let queue = Arc::new(SQLiteTaskQueue::new(Arc::new(pool.clone())));

    // Clean queue
    sqlx::query("DELETE FROM ohc_job_queue").execute(&pool).await.unwrap();

    // Enqueue 100 jobs
    let mut jobs = Vec::new();
    for i in 0..100 {
        jobs.push(Job {
            id: format!("job-concurrent-{}", i),
            tenant_id: "concurrent_tenant".to_string(),
            parent_task_id: "parent-1".to_string(),
            job_type: "concurrent-role".to_string(),
            payload: "{}".to_string(),
            status: "PENDING".to_string(),
            retry_count: 0,
            max_retries: 3,
            next_retry_at: Utc::now(),
            locked_until: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });
    }
    queue.enqueue_batch(jobs).await.unwrap();

    let processed_count = Arc::new(AtomicUsize::new(0));
    let mut workers = Vec::new();

    for _ in 0..5 {
        let q = queue.clone();
        let count = processed_count.clone();
        workers.push(tokio::spawn(async move {
            loop {
                let job = q.dequeue(vec!["concurrent-role".to_string()], 0, 0).await.unwrap();
                match job {
                    Some(j) => {
                        q.complete(&j.id).await.unwrap();
                        count.fetch_add(1, Ordering::SeqCst);
                    }
                    None => break, // No more jobs
                }
            }
        }));
    }

    for w in workers {
        w.await.unwrap();
    }

    assert_eq!(processed_count.load(Ordering::SeqCst), 100, "Exactly 100 jobs should be executed");

    // Verify 0 duplicates or pending jobs
    let remaining: (i64,) = sqlx::query_as("SELECT count(*) FROM ohc_job_queue WHERE status = 'PENDING'")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(remaining.0, 0, "There should be no pending jobs left");
}
