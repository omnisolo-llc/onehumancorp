use super::{OHCJobQueue, RedisLock, TaskQueue};
use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;

#[tokio::test]
async fn test_ohc_job_queue_e2e() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        unsafe { std::env::set_var("OHC_DATABASE_URL", "postgres://ohc:ohc@localhost:5432/ohc"); }
    }

    let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
    let pool = match PgPoolOptions::new().max_connections(5).connect(&database_url).await { Ok(p) => p, Err(_) => return, };

    let queue = OHCJobQueue::new(Arc::new(pool.clone()));

    // Ensure table is clean for tests
    sqlx::query("DELETE FROM ohc_job_queue").execute(&pool).await.unwrap();

    let tenant_id = "tenant_test_1";
    let job_type = "test_job_type";
    let payload = serde_json::json!({"key": "value"});

    // 1. Enqueue job
    let job_id = queue.enqueue(tenant_id, job_type, &payload).await.unwrap();

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
        unsafe { std::env::set_var("OHC_DATABASE_URL", "postgres://ohc:ohc@localhost:5432/ohc"); }
    }

    let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
    let pool = match PgPoolOptions::new().max_connections(5).connect(&database_url).await { Ok(p) => p, Err(_) => return, };

    let queue = OHCJobQueue::new(Arc::new(pool.clone()));

    sqlx::query("DELETE FROM ohc_job_queue").execute(&pool).await.unwrap();

    let tenant_id = "tenant_test_2";
    let job_type = "fail_job_type";
    let payload = serde_json::json!({});

    let job_id = queue.enqueue(tenant_id, job_type, &payload).await.unwrap();

    queue.dequeue(vec![job_type]).await.unwrap().unwrap();

    // 1st fail
    queue.fail(&job_id, 3, "fail 1").await.unwrap();
    let status_retry: (String, i32) = sqlx::query_as("SELECT status, retry_count FROM ohc_job_queue WHERE id = $1")
        .bind(&job_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(status_retry.0, "PENDING");
    assert_eq!(status_retry.1, 1);

    // 2nd fail
    queue.dequeue(vec![job_type]).await.unwrap(); // might fail if next_retry_at is in future, but assuming time hasn't passed it will skip, let's update it for test
    sqlx::query("UPDATE ohc_job_queue SET next_retry_at = '2020-01-01T00:00:00Z' WHERE id = $1").bind(&job_id).execute(&pool).await.unwrap();
    queue.dequeue(vec![job_type]).await.unwrap();

    queue.fail(&job_id, 3, "fail 2").await.unwrap();
    let status_retry2: (String, i32) = sqlx::query_as("SELECT status, retry_count FROM ohc_job_queue WHERE id = $1")
        .bind(&job_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(status_retry2.0, "PENDING");
    assert_eq!(status_retry2.1, 2);

    // 3rd fail (should dead letter)
    sqlx::query("UPDATE ohc_job_queue SET next_retry_at = '2020-01-01T00:00:00Z' WHERE id = $1").bind(&job_id).execute(&pool).await.unwrap();
    queue.dequeue(vec![job_type]).await.unwrap();
    queue.fail(&job_id, 3, "fail 3").await.unwrap();

    let status_retry3: (String, i32) = sqlx::query_as("SELECT status, retry_count FROM ohc_job_queue WHERE id = $1")
        .bind(&job_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(status_retry3.0, "FAILED");
    assert_eq!(status_retry3.1, 3);
}

use super::worker_pool::{WorkerPool, JobHandler};
use super::ohc_universal_ledger::OHCUniversalLedger;

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
        unsafe { std::env::set_var("OHC_DATABASE_URL", "postgres://ohc:ohc@localhost:5432/ohc"); }
    }

    let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
    let pool = match PgPoolOptions::new().max_connections(5).connect(&database_url).await { Ok(p) => p, Err(_) => return, };
    let pool_arc = Arc::new(pool.clone());

    let queue = Arc::new(OHCJobQueue::new(pool_arc.clone()));
    let ledger = Arc::new(OHCUniversalLedger::new(pool_arc.clone()));

    sqlx::query("DELETE FROM ohc_job_queue").execute(&pool).await.unwrap();
    sqlx::query("DELETE FROM ohc_universal_ledger").execute(&pool).await.unwrap();

    let handler = Arc::new(TestHandler { ledger: ledger.clone() });

    let worker_pool = WorkerPool::new(queue.clone(), 2, vec!["test_worker_job".to_string()], handler);

    let tenant_id = "tenant_worker_test";
    queue.enqueue(tenant_id, "test_worker_job", &serde_json::json!({})).await.unwrap();

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
        unsafe { std::env::set_var("OHC_DATABASE_URL", "postgres://ohc:ohc@localhost:5432/ohc"); }
    }

    let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
    let pool = match PgPoolOptions::new().max_connections(5).connect(&database_url).await { Ok(p) => p, Err(_) => return, };
    let pool_arc = Arc::new(pool.clone());

    let queue = Arc::new(OHCJobQueue::new(pool_arc.clone()));

    sqlx::query("DELETE FROM ohc_job_queue").execute(&pool).await.unwrap();

    let handler = Arc::new(TimeoutTestHandler { sleep_ms: 300 });

    let worker_pool = WorkerPool::new_with_timeout(queue.clone(), 1, vec!["chaos_timeout_job".to_string()], handler, 150);

    let tenant_id = "tenant_chaos_timeout";
    let job_id = queue.enqueue(tenant_id, "chaos_timeout_job", &serde_json::json!({})).await.unwrap();

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

#[tokio::test]
async fn test_ohc_job_queue_fail_max_retries_dead_letter() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        unsafe { std::env::set_var("OHC_DATABASE_URL", "postgres://ohc:ohc@localhost:5432/ohc"); }
    }

    let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
    let pool = match PgPoolOptions::new().max_connections(5).connect(&database_url).await { Ok(p) => p, Err(_) => return, };

    let queue = OHCJobQueue::new(Arc::new(pool.clone()));

    sqlx::query("DELETE FROM ohc_job_queue").execute(&pool).await.unwrap();
    sqlx::query("DELETE FROM department_dead_letters").execute(&pool).await.unwrap();

    let tenant_id = "tenant_test_dead_letter";
    let job_type = "fail_job_type_dead_letter";
    let payload = serde_json::json!({"test": "payload"});

    let job_id = queue.enqueue(tenant_id, job_type, &payload).await.unwrap();

    // Directly set retry_count to max_retries - 1
    sqlx::query("UPDATE ohc_job_queue SET retry_count = 2 WHERE id = $1")
        .bind(&job_id)
        .execute(&pool)
        .await
        .unwrap();

    // Trigger failure that exceeds max_retries
    queue.fail(&job_id, 3, "fail").await.unwrap();

    // Verify job is marked as FAILED
    let status_retry: (String, i32) = sqlx::query_as("SELECT status, retry_count FROM ohc_job_queue WHERE id = $1")
        .bind(&job_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(status_retry.0, "FAILED");

    // Verify dead letter was created
    use sqlx::Row;
    let dl_row = sqlx::query("SELECT tenant_id, event_type, department, payload, error_message FROM department_dead_letters LIMIT 1").fetch_one(&pool).await.unwrap();
    let dl_tenant_id: String = dl_row.get("tenant_id");
    let dl_event_type: String = dl_row.get("event_type");
    let dl_department: String = dl_row.get("department");
    let dl_payload: String = dl_row.get("payload");
    let dl_error_message: String = dl_row.get("error_message");

    assert_eq!(dl_tenant_id, tenant_id);
    assert_eq!(dl_event_type, "cleanup");
    assert_eq!(dl_department, "job_queue");
    assert_eq!(dl_payload, "{\"test\":\"payload\"}");
    assert_eq!(dl_error_message, "Max retries exceeded");
}

#[tokio::test]
async fn test_chaos_redis_lock_race_condition() {
    let redis_url = std::env::var("OHC_REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    if redis::Client::open(redis_url.clone()).and_then(|c| c.get_connection()).is_err() {
        return;
    }

    let redis_lock = Arc::new(RedisLock::new(&redis_url).unwrap());

    let tenant_id = "tenant_chaos_lock";
    let resource_type = "agent-lock";
    let resource_id = "test_race_condition";

    let lock_clone1 = redis_lock.clone();
    let lock_clone2 = redis_lock.clone();

    // Start two concurrent tasks trying to acquire the exact same lock
    let t1 = tokio::spawn(async move {
        lock_clone1.acquire_lock(tenant_id, resource_type, resource_id, 10).await
    });

    let t2 = tokio::spawn(async move {
        lock_clone2.acquire_lock(tenant_id, resource_type, resource_id, 10).await
    });

    let (res1, res2) = tokio::join!(t1, t2);

    let l1 = res1.unwrap().unwrap();
    let l2 = res2.unwrap().unwrap();

    // Exactly one should succeed in getting the lock, the other should be None
    assert!(l1.is_some() ^ l2.is_some(), "Only one task should acquire the lock concurrently");

    if let Some(lock_val) = l1 {
        redis_lock.release_lock(tenant_id, resource_type, resource_id, &lock_val).await.unwrap();
    } else if let Some(lock_val) = l2 {
        redis_lock.release_lock(tenant_id, resource_type, resource_id, &lock_val).await.unwrap();
    }
}

#[tokio::test]
async fn test_chaos_redis_mailbox_corruption() {
    let redis_url = std::env::var("OHC_REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let client = match redis::Client::open(redis_url.clone()) {
        Ok(c) => c,
        Err(_) => return,
    };

    let mut conn = match client.get_multiplexed_tokio_connection().await {
        Ok(c) => c,
        Err(_) => {
            return;
        }
    };

    let _tenant_id = "tenant_chaos_mailbox";
    let queue_name = "test_queue";

    // Corrupt the queue by pushing a non-JSON / invalid string
    // RedisTaskQueue uses a sorted set (ZSET), so we use ZADD.
    let _: () = redis::cmd("ZADD")
        .arg(queue_name)
        .arg(0)
        .arg("THIS_IS_CORRUPT_DATA_NOT_JSON!!!")
        .query_async(&mut conn)
        .await
        .unwrap();

    // Now try to dequeue from Redis queue. Assuming we test the resilience of RedisTaskQueue
    let queue = super::RedisTaskQueue::new(&redis_url, queue_name).unwrap();

    // Attempting to dequeue should gracefully handle the corrupt data (e.g., skip it or return error, but NOT panic)
    // Actually, RedisTaskQueue dequeues using LPOP and then parsing.
    // Let's test what happens when we dequeue it
    let result = queue.dequeue(vec!["test_role".to_string()], 1, 100).await;

    // Since it's corrupt data, it might return an error or skip. The critical condition is NO panic.
    assert!(result.is_err() || result.unwrap().is_none(), "Corrupt data should not panic, and should yield an error or None");
}

#[tokio::test]
async fn test_cleanup_stagnant_pending_jobs() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        unsafe { std::env::set_var("OHC_DATABASE_URL", "postgres://ohc:ohc@localhost:5432/ohc"); }
    }

    let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
    let pool = match PgPoolOptions::new().max_connections(5).connect(&database_url).await { Ok(p) => p, Err(_) => return, };
    let queue = OHCJobQueue::new(Arc::new(pool.clone()));

    sqlx::query("DELETE FROM ohc_job_queue").execute(&pool).await.unwrap();
    sqlx::query("DELETE FROM department_dead_letters").execute(&pool).await.unwrap();

    let tenant_id = "tenant_test_stagnant";
    let job_type = "test_stagnant_job";
    let payload = serde_json::json!({"test": "stagnant"});

    // Insert a normal recent PENDING job
    queue.enqueue(tenant_id, job_type, &payload).await.unwrap();

    // Insert a stagnant PENDING job
    let stagnant_job_id = "stagnant_job_123";
    sqlx::query(
        "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, created_at, updated_at, next_retry_at)
         VALUES ($1, $2, $3, $4, 'PENDING', CURRENT_TIMESTAMP - INTERVAL '25 hours', CURRENT_TIMESTAMP - INTERVAL '25 hours', CURRENT_TIMESTAMP)"
    )
    .bind(stagnant_job_id)
    .bind(tenant_id)
    .bind(job_type)
    .bind(&payload)
    .execute(&pool)
    .await
    .unwrap();

    // Run cleanup
    let cleaned = queue.cleanup_stale_jobs().await.unwrap();
    assert_eq!(cleaned, 1);

    // Verify stagnant job is FAILED
    let stagnant_row = sqlx::query("SELECT status FROM ohc_job_queue WHERE id = $1").bind(stagnant_job_id).fetch_optional(&pool).await.unwrap();
    assert!(stagnant_row.is_none());

    // Verify recent PENDING job is still PENDING
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ohc_job_queue WHERE status = 'PENDING'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);

    // Verify dead letter was created
    let dl_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM department_dead_letters WHERE id = $1")
        .bind(stagnant_job_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(dl_count, 1);
}
