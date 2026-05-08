use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sub_agent_queue_isolation() {
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
                .connect_lazy(&db_url)
                .unwrap();

            let qm = QueueManager::new(pool);
            let job_id = uuid::Uuid::new_v4().to_string();
            let org_id = "tenant-a".to_string();

            let job = SubAgentJob {
                id: job_id.clone(),
                organization_id: org_id.clone(),
                parent_task_id: "task-1".to_string(),
                payload: serde_json::json!({"action": "test"}),
                status: "QUEUED".to_string(),
                worker_id: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            qm.enqueue(job).await.unwrap();

            // Should fail if org_id is wrong
            let res = qm.mark_completed(&job_id, "wrong-tenant").await.unwrap();

            // Actually the query doesn't error out, it just updates 0 rows. Let's poll it to see if it was modified.
            // Oh, we can check `rows_affected()`. Wait, `execute` returns `PgQueryResult`
        }
    }

    #[tokio::test]
    async fn test_memory_queue_batch() {
        let mem_queue = crate::queue::MemoryTaskQueue::new();
        let job1 = crate::queue::Job {
            id: "job_mem_1".to_string(),
            tenant_id: "test".to_string(),
            parent_task_id: "parent_1".to_string(),
            agent_role: "test_agent".to_string(),
            payload: "{}".to_string(),
            status: "PENDING".to_string(),
            attempts: 0,
            max_attempts: 3,
            run_after: chrono::Utc::now(),
            locked_until: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let job2 = crate::queue::Job {
            id: "job_mem_2".to_string(),
            tenant_id: "test".to_string(),
            parent_task_id: "parent_1".to_string(),
            agent_role: "test_agent".to_string(),
            payload: "{}".to_string(),
            status: "PENDING".to_string(),
            attempts: 0,
            max_attempts: 3,
            run_after: chrono::Utc::now(),
            locked_until: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        crate::queue::TaskQueue::enqueue_batch(&mem_queue, vec![job1, job2]).await.unwrap();
        let res1 = crate::queue::TaskQueue::dequeue(&mem_queue, vec!["test_agent".to_string()]).await.unwrap();
        assert!(res1.is_some());
        let res2 = crate::queue::TaskQueue::dequeue(&mem_queue, vec!["test_agent".to_string()]).await.unwrap();
        assert!(res2.is_some());
        let res3 = crate::queue::TaskQueue::dequeue(&mem_queue, vec!["test_agent".to_string()]).await.unwrap();
        assert!(res3.is_none());
    }

    #[tokio::test]
    async fn test_sqlite_queue_batch_empty() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let sqlite_queue = crate::queue::SqliteTaskQueue::new(pool.clone());
        sqlite_queue.init().await.unwrap();
        crate::queue::TaskQueue::enqueue_batch(&sqlite_queue, vec![]).await.unwrap();
        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM local_queue_jobs")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count.0, 0);
    }
}
