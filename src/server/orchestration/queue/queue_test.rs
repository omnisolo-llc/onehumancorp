use super::{TaskQueue, Job, SQLiteTaskQueue};
use std::sync::Arc;
use sqlx::SqlitePool;
use chrono::Utc;

#[tokio::test]
async fn test_sqlite_task_queue() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    sqlx::query(
        "CREATE TABLE sub_agent_queue (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            parent_task_id TEXT NOT NULL,
            payload TEXT,
            status TEXT NOT NULL DEFAULT 'QUEUED',
            worker_id TEXT,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(&pool).await.unwrap();

    let queue = SQLiteTaskQueue::new(Arc::new(pool.clone()));

    let job = Job {
        id: "job-1".to_string(),
        organization_id: "system".to_string(),
        parent_task_id: "parent-1".to_string(),
        agent_role: "test-role".to_string(),
        payload: "{}".to_string(),
        status: "QUEUED".to_string(),
        worker_id: None,
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
    assert_eq!(dequeued.organization_id, "system");

    queue.complete(&dequeued.id).await.unwrap();

    // Check status
    let status: (String,) = sqlx::query_as("SELECT status FROM sub_agent_queue WHERE id = 'job-1'").fetch_one(&pool).await.unwrap();
    assert_eq!(status.0, "COMPLETED");
}

#[tokio::test]
async fn test_sqlite_task_queue_empty_dequeue() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    sqlx::query(
        "CREATE TABLE sub_agent_queue (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            parent_task_id TEXT NOT NULL,
            payload TEXT,
            status TEXT NOT NULL DEFAULT 'QUEUED',
            worker_id TEXT,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(&pool).await.unwrap();

    let queue = SQLiteTaskQueue::new(Arc::new(pool));

    let dequeued = queue.dequeue(vec!["test-role".to_string()], 100, 100).await.unwrap();
    assert!(dequeued.is_none());
}

#[tokio::test]
async fn test_e2e_ui_sub_agent_decomposition_routing() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    sqlx::query(
        "CREATE TABLE sub_agent_queue (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            parent_task_id TEXT NOT NULL,
            payload TEXT,
            status TEXT NOT NULL DEFAULT 'QUEUED',
            worker_id TEXT,
            attempts INTEGER DEFAULT 0,
            max_attempts INTEGER DEFAULT 3,
            run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(&pool).await.unwrap();

    // 1. Simulate UI Login & Event triggering a feature decomposition.
    let ui_organization_id = "tenant-ui-alpha".to_string();
    let decomposed_feature_task_id = "epic-task-id-999".to_string();

    let queue = Arc::new(SQLiteTaskQueue::new(Arc::new(pool.clone())));

    // UI Backend pushes generated sub-agent sub-tasks into queue
    let mut ui_triggered_jobs = vec![];
    for i in 0..5 {
        ui_triggered_jobs.push(Job {
            id: format!("ui-triggered-job-{}", i),
            organization_id: ui_organization_id.clone(),
            parent_task_id: decomposed_feature_task_id.clone(),
            agent_role: "scout".to_string(),
            payload: format!("{{\"action\": \"verify_ui_element\", \"script\": \"echo 'Verified UI element {}'\"}}", i),
            status: "QUEUED".to_string(),
            worker_id: None,
            attempts: 0,
            max_attempts: 3,
            run_after: Utc::now() - chrono::Duration::seconds(1),
            locked_until: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });
    }

    // Include one failing task to test retry/failure isolation
    ui_triggered_jobs.push(Job {
        id: "ui-triggered-job-failing".to_string(),
        organization_id: ui_organization_id.clone(),
        parent_task_id: decomposed_feature_task_id.clone(),
        agent_role: "scout".to_string(),
        payload: "{\"action\": \"fail_test\"}".to_string(),
        status: "QUEUED".to_string(),
        worker_id: None,
        attempts: 0,
        max_attempts: 3,
        run_after: Utc::now() - chrono::Duration::seconds(1),
        locked_until: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    });

    for job in ui_triggered_jobs {
        queue.enqueue(job).await.unwrap();
    }

    // 2. Start Worker Pool to process queued items
    let worker_pool = super::queue::WorkerPool::new(queue.clone(), 3, vec!["scout".to_string()]);
    let (tx, _rx) = tokio::sync::broadcast::channel(1);

    worker_pool.start(tx.clone()).await;

    // Give it time to process all 6 tasks concurrently
    tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;

    let _ = tx.send(());

    // 3. Verify execution outcomes for E2E flow
    let status_success: (String,) = sqlx::query_as("SELECT status FROM sub_agent_queue WHERE id = 'ui-triggered-job-0'").fetch_one(&pool).await.unwrap();
    assert_eq!(status_success.0, "COMPLETED");

    let status_fail: (String,) = sqlx::query_as("SELECT status FROM sub_agent_queue WHERE id = 'ui-triggered-job-failing'").fetch_one(&pool).await.unwrap();
    assert_eq!(status_fail.0, "QUEUED"); // Because attempts went from 0 to 1, backoff queued it

    // Check attempts incremented
    let attempts: (i32,) = sqlx::query_as("SELECT attempts FROM sub_agent_queue WHERE id = 'ui-triggered-job-failing'").fetch_one(&pool).await.unwrap();
    assert_eq!(attempts.0, 1);
}

#[tokio::test]
async fn test_sqlite_worker_pool_routing() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    sqlx::query(
        "CREATE TABLE sub_agent_queue (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            parent_task_id TEXT NOT NULL,
            payload TEXT,
            status TEXT NOT NULL DEFAULT 'QUEUED',
            worker_id TEXT,
            attempts INTEGER DEFAULT 0,
            max_attempts INTEGER DEFAULT 3,
            run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(&pool).await.unwrap();

    let queue = Arc::new(SQLiteTaskQueue::new(Arc::new(pool.clone())));

    let job = Job {
        id: "ui-job-1".to_string(),
        organization_id: "ui-tenant".to_string(),
        parent_task_id: "parent-login".to_string(),
        agent_role: "scout".to_string(),
        payload: "{\"action\": \"success_test\"}".to_string(),
        status: "QUEUED".to_string(),
        worker_id: None,
        attempts: 0,
        max_attempts: 3,
        run_after: Utc::now() - chrono::Duration::seconds(1),
        locked_until: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let job2 = Job {
        id: "ui-job-2".to_string(),
        organization_id: "ui-tenant".to_string(),
        parent_task_id: "parent-login".to_string(),
        agent_role: "scout".to_string(),
        payload: "{\"action\": \"fail_test\"}".to_string(),
        status: "QUEUED".to_string(),
        worker_id: None,
        attempts: 0,
        max_attempts: 3,
        run_after: Utc::now() - chrono::Duration::seconds(1),
        locked_until: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    queue.enqueue(job).await.unwrap();
    queue.enqueue(job2).await.unwrap();

    let worker_pool = super::queue::WorkerPool::new(queue.clone(), 2, vec!["scout".to_string()]);
    let (tx, rx) = tokio::sync::broadcast::channel(1);

    worker_pool.start(tx.clone()).await;

    // Give it time to process
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let _ = tx.send(());

    // Verify
    let status_success: (String,) = sqlx::query_as("SELECT status FROM sub_agent_queue WHERE id = 'ui-job-1'").fetch_one(&pool).await.unwrap();
    assert_eq!(status_success.0, "COMPLETED");

    let status_fail: (String,) = sqlx::query_as("SELECT status FROM sub_agent_queue WHERE id = 'ui-job-2'").fetch_one(&pool).await.unwrap();
    assert_eq!(status_fail.0, "FAILED");
}

#[tokio::test]
async fn test_pg_worker_pool_routing() {
    if let Ok(db_url) = std::env::var("DATABASE_URL") {
        let pool = sqlx::PgPool::connect(&db_url).await.unwrap();

        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS sub_agent_queue (id UUID PRIMARY KEY, organization_id VARCHAR NOT NULL, parent_task_id UUID NOT NULL, payload JSONB, status VARCHAR NOT NULL DEFAULT 'QUEUED', worker_id VARCHAR, created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP)").execute(&pool).await;

        // Add missing columns dynamically just for test mock setup
        let _ = sqlx::query("ALTER TABLE sub_agent_queue ADD COLUMN attempts INTEGER DEFAULT 0").execute(&pool).await;
        let _ = sqlx::query("ALTER TABLE sub_agent_queue ADD COLUMN max_attempts INTEGER DEFAULT 3").execute(&pool).await;
        let _ = sqlx::query("ALTER TABLE sub_agent_queue ADD COLUMN run_after TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP").execute(&pool).await;

        let queue = Arc::new(super::PgTaskQueue::new(Arc::new(pool.clone())));

        let job = Job {
            id: uuid::Uuid::new_v4().to_string(),
            organization_id: "ui-tenant".to_string(),
            parent_task_id: uuid::Uuid::new_v4().to_string(),
            agent_role: "scout".to_string(),
            payload: "{\"action\": \"success_test\"}".to_string(),
            status: "QUEUED".to_string(),
            worker_id: None,
            attempts: 0,
            max_attempts: 3,
            run_after: Utc::now() - chrono::Duration::seconds(1),
            locked_until: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        queue.enqueue(job.clone()).await.unwrap();

        let worker_pool = super::queue::WorkerPool::new(queue.clone(), 2, vec!["scout".to_string()]);
        let (tx, _rx) = tokio::sync::broadcast::channel(1);

        worker_pool.start(tx.clone()).await;

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        let _ = tx.send(());

        let status_success: (String,) = sqlx::query_as("SELECT status FROM sub_agent_queue WHERE id = $1").bind(&job.id).fetch_one(&pool).await.unwrap();
        assert_eq!(status_success.0, "COMPLETED");
    }
}

#[tokio::test]
async fn test_redis_task_queue() {
    // Basic stub test for Redis
    let mock_redis_url = "redis://127.0.0.1:6379/";
    // Only run if redis is available locally
    let client = redis::Client::open(mock_redis_url);
    if let Ok(c) = client {
        if let Ok(mut conn) = c.get_connection() {
            let res: redis::RedisResult<String> = redis::cmd("PING").query(&mut conn);
            if res.is_ok() {
                let queue = super::RedisTaskQueue::new(mock_redis_url, "test_queue").unwrap();
                let job = super::Job {
                    id: "redis-job-1".to_string(),
                    organization_id: "system".to_string(),
                    parent_task_id: "parent-1".to_string(),
                    agent_role: "test-role".to_string(),
                    payload: "{}".to_string(),
                    status: "QUEUED".to_string(),
                    worker_id: None,
                    attempts: 0,
                    max_attempts: 3,
                    run_after: chrono::Utc::now() - chrono::Duration::seconds(1),
                    locked_until: None,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                };

                use super::TaskQueue;
                queue.enqueue(job).await.unwrap();

                let dequeued_opt = queue.dequeue(vec!["test-role".to_string()], 100, 100).await.unwrap();
                assert!(dequeued_opt.is_some());
            }
        }
    }
}

#[tokio::test]
async fn test_sqlite_task_queue_fail_backoff_poison_pill() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    sqlx::query(
        "CREATE TABLE sub_agent_queue (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            parent_task_id TEXT NOT NULL,
            payload TEXT,
            status TEXT NOT NULL DEFAULT 'QUEUED',
            worker_id TEXT,
            attempts INTEGER DEFAULT 0,
            max_attempts INTEGER DEFAULT 3,
            run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(&pool).await.unwrap();

    let queue = SQLiteTaskQueue::new(Arc::new(pool.clone()));

    let job = Job {
        id: "job-poison".to_string(),
        organization_id: "system".to_string(),
        parent_task_id: "parent-1".to_string(),
        agent_role: "test-role".to_string(),
        payload: "{}".to_string(),
        status: "QUEUED".to_string(),
        worker_id: None,
        attempts: 2, // Next fail should hit max_attempts = 3
        max_attempts: 3,
        run_after: Utc::now() - chrono::Duration::seconds(1),
        locked_until: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    queue.enqueue(job).await.unwrap();
    // simulate it was running and updated
    sqlx::query("UPDATE sub_agent_queue SET attempts = 2").execute(&pool).await.unwrap();

    // Fail it
    queue.fail("job-poison", "error").await.unwrap();

    // Check status is FAILED (poison pill)
    let status: (String,) = sqlx::query_as("SELECT status FROM sub_agent_queue WHERE id = 'job-poison'").fetch_one(&pool).await.unwrap();
    assert_eq!(status.0, "FAILED");
    let attempts: (i32,) = sqlx::query_as("SELECT attempts FROM sub_agent_queue WHERE id = 'job-poison'").fetch_one(&pool).await.unwrap();
    assert_eq!(attempts.0, 3);
}
