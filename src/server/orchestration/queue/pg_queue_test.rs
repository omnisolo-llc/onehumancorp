use super::{TaskQueue, Job, PgTaskQueue};
use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;
use chrono::Utc;

#[tokio::test]
async fn test_pg_fail_backoff() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        return;
    }

    let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
        .unwrap();

    let queue = PgTaskQueue::new(Arc::new(pool.clone()));

    // Ensure table is clean for tests
    sqlx::query("DELETE FROM ohc_job_queue").execute(&pool).await.unwrap();

    let job = Job {
        id: "job-fail-pg-1".to_string(),
        tenant_id: "test_org".to_string(),
        parent_task_id: "parent-1".to_string(),
        job_type: "test-role".to_string(),
        payload: "{}".to_string(),
        status: "PENDING".to_string(),
        retry_count: 0,
        max_retries: 3,
        next_retry_at: Utc::now() - chrono::Duration::seconds(10),
        locked_until: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    queue.enqueue(job).await.unwrap();

    let before_fail = chrono::Utc::now();
    queue.fail("job-fail-pg-1", "test").await.unwrap();

    // After fail, retry_count should be 1, status PENDING, and next_retry_at should be updated
    use sqlx::Row;
    let row = sqlx::query("SELECT retry_count, status, next_retry_at FROM ohc_job_queue WHERE id = 'job-fail-pg-1'").fetch_one(&pool).await.unwrap();

    let retry_count: i32 = row.get("retry_count");
    assert_eq!(retry_count, 1);

    let status: String = row.get("status");
    assert_eq!(status, "PENDING");

    let next_retry_at: chrono::DateTime<chrono::Utc> = row.get("next_retry_at");

    // Verify next_retry_at is approximately now + 2 seconds (1 << 1 attempt)
    let backoff_duration = chrono::Duration::seconds(2);
    let expected_time = before_fail + backoff_duration;

    let diff = next_retry_at.signed_duration_since(expected_time).num_milliseconds().abs();
    assert!(diff < 1000, "next_retry_at timestamp should be roughly Utc::now() + backoff time, but got difference of {} ms", diff);
}

#[tokio::test]
async fn test_pg_fail_max_retries_dead_letter() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        return;
    }

    let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
        .unwrap();

    let queue = PgTaskQueue::new(Arc::new(pool.clone()));

    // Ensure tables are clean for tests
    sqlx::query("DELETE FROM ohc_job_queue").execute(&pool).await.unwrap();
    sqlx::query("DELETE FROM department_dead_letters").execute(&pool).await.unwrap();

    let job = Job {
        id: "job-fail-pg-dead".to_string(),
        tenant_id: "test_org".to_string(),
        parent_task_id: "parent-1".to_string(),
        job_type: "test-role".to_string(),
        payload: "{\"test\": \"payload\"}".to_string(),
        status: "PENDING".to_string(),
        retry_count: 2, // Next fail will be 3, which is max_retries
        max_retries: 3,
        next_retry_at: Utc::now(),
        locked_until: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    queue.enqueue(job).await.unwrap();

    // Trigger failure that exceeds max_retries
    queue.fail("job-fail-pg-dead", "test reason").await.unwrap();

    // Verify job is marked as FAILED
    use sqlx::Row;
    let row = sqlx::query("SELECT status FROM ohc_job_queue WHERE id = 'job-fail-pg-dead'").fetch_one(&pool).await.unwrap();
    let status: String = row.get("status");
    assert_eq!(status, "FAILED");

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
    assert_eq!(dl_payload, "{\"test\":\"payload\"}");
    assert_eq!(dl_error_message, "test reason");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_pg_queue_concurrent_dequeue() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        return;
    }

    let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
        .unwrap();

    let queue = Arc::new(PgTaskQueue::new(Arc::new(pool.clone())));

    // Ensure table is clean
    sqlx::query("DELETE FROM ohc_job_queue").execute(&pool).await.unwrap();

    let num_jobs = 100;
    for i in 0..num_jobs {
        let job = Job {
            id: format!("job-concurrent-{}", i),
            tenant_id: format!("tenant-{}", i % 5),
            parent_task_id: format!("parent-{}", i),
            job_type: "concurrent-role".to_string(),
            payload: "{}".to_string(),
            status: "PENDING".to_string(),
            retry_count: 0,
            max_retries: 3,
            next_retry_at: Utc::now() - chrono::Duration::seconds(10),
            locked_until: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        queue.enqueue(job).await.unwrap();
    }

    let mut handles = vec![];
    let num_workers = 2;

    for _ in 0..num_workers {
        let q_clone = queue.clone();
        handles.push(tokio::spawn(async move {
            let mut processed = 0;
            loop {
                let job_opt = q_clone.dequeue(vec!["concurrent-role".to_string()], 0, 0).await.unwrap();
                match job_opt {
                    Some(job) => {
                        // Simulate work
                        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
                        q_clone.complete(&job.id).await.unwrap();
                        processed += 1;
                    }
                    None => break,
                }
            }
            processed
        }));
    }

    let mut total_processed = 0;
    for handle in handles {
        total_processed += handle.await.unwrap();
    }

    assert_eq!(total_processed, num_jobs);

    // Verify all are completed
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ohc_job_queue WHERE status = 'COMPLETED'")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(count.0 as usize, num_jobs);
}
