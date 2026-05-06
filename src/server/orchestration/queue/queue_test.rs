use super::{TaskQueue, SQLiteTaskQueue};
use crate::ohc::orchestration::Job;
use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
use std::str::FromStr;
use std::sync::Arc;

#[tokio::test]
async fn test_sqlite_queue() {
    let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:")
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .connect_with(conn_opts)
        .await
        .unwrap();

    sqlx::query(
        "CREATE TABLE jobs (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            agent_role TEXT NOT NULL,
            status TEXT NOT NULL,
            run_after DATETIME NOT NULL,
            locked_until DATETIME,
            created_at DATETIME NOT NULL,
            protobuf_blob BLOB NOT NULL
        )"
    ).execute(&pool).await.unwrap();

    let queue = SQLiteTaskQueue::new(Arc::new(pool));

    let job = Job {
        id: "job1".to_string(),
        tenant_id: "system".to_string(),
        parent_task_id: "task1".to_string(),
        agent_role: "researcher".to_string(),
        payload: "{}".to_string(),
        status: "pending".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: chrono::Utc::now().timestamp(),
        locked_until: 0,
        created_at: chrono::Utc::now().timestamp(),
        updated_at: chrono::Utc::now().timestamp(),
    };

    queue.enqueue(job.clone()).await.unwrap();

    let dequeued = queue.dequeue(vec!["researcher".to_string()]).await.unwrap();
    assert!(dequeued.is_some());
    let d = dequeued.unwrap();
    assert_eq!(d.id, "job1");

    // Should be locked now
    let dequeued2 = queue.dequeue(vec!["researcher".to_string()]).await.unwrap();
    assert!(dequeued2.is_none());

    queue.complete("job1").await.unwrap();
}

#[tokio::test]
async fn test_sqlite_queue_roles() {
    let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:")
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .connect_with(conn_opts)
        .await
        .unwrap();

    sqlx::query(
        "CREATE TABLE jobs (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            agent_role TEXT NOT NULL,
            status TEXT NOT NULL,
            run_after DATETIME NOT NULL,
            locked_until DATETIME,
            created_at DATETIME NOT NULL,
            protobuf_blob BLOB NOT NULL
        )"
    ).execute(&pool).await.unwrap();

    let queue = SQLiteTaskQueue::new(Arc::new(pool));

    let job = Job {
        id: "job1".to_string(),
        tenant_id: "system".to_string(),
        parent_task_id: "task1".to_string(),
        agent_role: "implementer".to_string(),
        payload: "{}".to_string(),
        status: "pending".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: chrono::Utc::now().timestamp(),
        locked_until: 0,
        created_at: chrono::Utc::now().timestamp(),
        updated_at: chrono::Utc::now().timestamp(),
    };

    queue.enqueue(job).await.unwrap();

    // Wrong role should not dequeue
    let dequeued = queue.dequeue(vec!["researcher".to_string()]).await.unwrap();
    assert!(dequeued.is_none());

    // Correct role should dequeue
    let dequeued2 = queue.dequeue(vec!["implementer".to_string()]).await.unwrap();
    assert!(dequeued2.is_some());
}
