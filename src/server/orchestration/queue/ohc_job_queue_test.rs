use super::{OHCJobQueue, RedisLock};
use std::sync::Arc;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

#[tokio::test]
async fn test_ohc_job_queue_e2e() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        return; // Skip if database is not available
    }

    let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let queue = OHCJobQueue::new(Arc::new(pool.clone()));

    // Ensure table is clean for tests
    sqlx::query("DELETE FROM ohc_job_queue").execute(&pool).await.unwrap();

    let tenant_id = "tenant_test_1";
    let job_type = "test_job_type";
    let payload = serde_json::json!({"key": "value"});

    // 1. Enqueue job
    let job_id = queue.enqueue(tenant_id, job_type, &payload, None, 3).await.unwrap();

    // 2. Dequeue job
    let job = queue.dequeue(vec![job_type]).await.unwrap().expect("Job should be available");
    assert_eq!(job.id, job_id);
    assert_eq!(job.status, "PROCESSING");
    assert_eq!(job.tenant_id, tenant_id);
    assert_eq!(job.job_type, job_type);

    // 3. Complete job
    queue.complete(&job_id).await.unwrap();

    // Verify completion
    let status: (String,) = sqlx::query_as("SELECT status FROM ohc_job_queue WHERE id = $1")
        .bind(&job_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(status.0, "COMPLETED");
}

#[tokio::test]
async fn test_ohc_job_queue_fail_backoff() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        return;
    }

    let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let queue = OHCJobQueue::new(Arc::new(pool.clone()));

    sqlx::query("DELETE FROM ohc_job_queue").execute(&pool).await.unwrap();

    let tenant_id = "tenant_test_2";
    let job_type = "fail_job_type";
    let payload = serde_json::json!({});

    let job_id = queue.enqueue(tenant_id, job_type, &payload, None, 3).await.unwrap();

    queue.dequeue(vec![job_type]).await.unwrap().unwrap();

    // 1st fail
    queue.fail(&job_id).await.unwrap();
    let status_retry: (String, i32) = sqlx::query_as("SELECT status, retry_count FROM ohc_job_queue WHERE id = $1")
        .bind(&job_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(status_retry.0, "PENDING");
    assert_eq!(status_retry.1, 1);

    // 2nd fail
    queue.dequeue(vec![job_type]).await.unwrap(); // might fail if next_retry_at is in future, but assuming time hasn't passed it will skip, let's update it for test
    sqlx::query("UPDATE ohc_job_queue SET next_retry_at = CURRENT_TIMESTAMP WHERE id = $1").bind(&job_id).execute(&pool).await.unwrap();
    queue.dequeue(vec![job_type]).await.unwrap();

    queue.fail(&job_id).await.unwrap();
    let status_retry2: (String, i32) = sqlx::query_as("SELECT status, retry_count FROM ohc_job_queue WHERE id = $1")
        .bind(&job_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(status_retry2.0, "PENDING");
    assert_eq!(status_retry2.1, 2);

    // 3rd fail (should dead letter)
    sqlx::query("UPDATE ohc_job_queue SET next_retry_at = CURRENT_TIMESTAMP WHERE id = $1").bind(&job_id).execute(&pool).await.unwrap();
    queue.dequeue(vec![job_type]).await.unwrap();
    queue.fail(&job_id).await.unwrap();

    let status_retry3: (String, i32) = sqlx::query_as("SELECT status, retry_count FROM ohc_job_queue WHERE id = $1")
        .bind(&job_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(status_retry3.0, "FAILED");
    assert_eq!(status_retry3.1, 3);
}

use super::worker_pool::{WorkerPool, JobHandler};
use super::ohc_universal_ledger::{OHCUniversalLedger, OHCLedgerEntry};

struct TestHandler {
    ledger: Arc<OHCUniversalLedger>,
}

#[async_trait::async_trait]
impl JobHandler for TestHandler {
    fn handle(&self, job: super::ohc_job_queue::OHCJob) -> tokio::task::JoinHandle<Result<(), String>> {
        let ledger = self.ledger.clone();
        tokio::spawn(async move {
            ledger.append_entry(&job.tenant_id, "test_job_completed", "test_dept", &serde_json::json!({"job_id": job.id})).await?;
            Ok(())
        })
    }
}

#[tokio::test]
async fn test_worker_pool_and_ledger() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        return;
    }

    let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
    let pool_arc = Arc::new(pool.clone());

    let queue = Arc::new(OHCJobQueue::new(pool_arc.clone()));
    let ledger = Arc::new(OHCUniversalLedger::new(pool_arc.clone()));

    sqlx::query("DELETE FROM ohc_job_queue").execute(&pool).await.unwrap();
    sqlx::query("DELETE FROM ohc_universal_ledger").execute(&pool).await.unwrap();

    let handler = Arc::new(TestHandler { ledger: ledger.clone() });

    let worker_pool = WorkerPool::new(queue.clone(), 2, vec!["test_worker_job".to_string()], handler);

    let tenant_id = "tenant_worker_test";
    queue.enqueue(tenant_id, "test_worker_job", &serde_json::json!({}), None, 3).await.unwrap();

    // Give the worker pool time to process
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    worker_pool.shutdown().await;

    let entries = ledger.get_entries(tenant_id, 10).await.unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].event_type, "test_job_completed");
}

struct TimeoutTestHandler {
    sleep_ms: u64,
}

#[async_trait::async_trait]
impl JobHandler for TimeoutTestHandler {
    fn handle(&self, _job: super::ohc_job_queue::OHCJob) -> tokio::task::JoinHandle<Result<(), String>> {
        let sleep_ms = self.sleep_ms;
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(sleep_ms)).await;
            Ok(())
        })
    }
}

#[tokio::test]
async fn test_worker_pool_chaos_timeout() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        return;
    }

    let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
    let pool_arc = Arc::new(pool.clone());

    let queue = Arc::new(OHCJobQueue::new(pool_arc.clone()));

    sqlx::query("DELETE FROM ohc_job_queue").execute(&pool).await.unwrap();

    let handler = Arc::new(TimeoutTestHandler { sleep_ms: 300 });

    let worker_pool = WorkerPool::new_with_timeout(queue.clone(), 1, vec!["chaos_timeout_job".to_string()], handler, 150);

    let tenant_id = "tenant_chaos_timeout";
    let job_id = queue.enqueue(tenant_id, "chaos_timeout_job", &serde_json::json!({}), None, 3).await.unwrap();

    // Give the worker pool time to process and timeout (should take ~150ms to timeout + overhead)
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    worker_pool.shutdown().await;

    let status_retry: (String, i32) = sqlx::query_as("SELECT status, retry_count FROM ohc_job_queue WHERE id = $1")
        .bind(&job_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(status_retry.0, "PENDING");
    assert_eq!(status_retry.1, 1);
}
