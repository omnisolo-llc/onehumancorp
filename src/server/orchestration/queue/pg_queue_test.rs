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
        .max_connections(5)
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
