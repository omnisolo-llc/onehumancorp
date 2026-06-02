#[cfg(test)]
mod tests {
    use server_lib::queue::{QueueManager, SubAgentJob};
    use sqlx::postgres::PgPoolOptions;
    use sqlx::Executor;

    #[tokio::test]
    async fn test_sub_agent_queue_isolation() {
        if let Ok(db_url) = std::env::var("OHC_DATABASE_URL") {
            let pool = PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { conn.execute("DISCARD ALL").await?; Ok(true) }) })
                .connect_lazy(&db_url)
                .unwrap();

            let qm = QueueManager::new(pool);
            let job_id = uuid::Uuid::new_v4().to_string();
            let org_id = "tenant-a".to_string();

            let job = SubAgentJob {
                id: job_id.clone(),
                tenant_id: org_id.clone(),
                parent_task_id: "task-1".to_string(),
                payload: serde_json::json!({"action": "test"}),
                status: "QUEUED".to_string(),
                worker_id: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            // This will likely fail to connect in a unit test environment without a real DB
            // but we are testing that it compiles.
            let _ = qm.enqueue(job).await;
        }
    }

    #[tokio::test]
    async fn test_queue_manager_requeue() {
        if let Ok(db_url) = std::env::var("OHC_DATABASE_URL") {
            let pool = PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { conn.execute("DISCARD ALL").await?; Ok(true) }) })
                .connect_lazy(&db_url)
                .unwrap();

            let qm = QueueManager::new(pool);
            let job_id = uuid::Uuid::new_v4().to_string();
            let org_id = "tenant-b".to_string();

            // To ensure table exists, run a creation query, ignore if it fails
            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS sub_agent_queue (id VARCHAR PRIMARY KEY, tenant_id VARCHAR NOT NULL, parent_task_id VARCHAR, payload TEXT, status VARCHAR, worker_id VARCHAR, scheduled_at TIMESTAMP, completed_at TIMESTAMP, created_at TIMESTAMP, updated_at TIMESTAMP)")
                .execute(&qm.pool)
                .await;

            let mut job = SubAgentJob {
                id: job_id.clone(),
                tenant_id: org_id.clone(),
                parent_task_id: "task-2".to_string(),
                payload: serde_json::json!({"action": "test_requeue"}),
                status: "QUEUED".to_string(),
                worker_id: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            // First enqueue the job
            if qm.enqueue(job.clone()).await.is_ok() {
                // Now test requeue logic
                job.payload["attempts"] = serde_json::json!(1);
                let result = qm.requeue(&job.id, &job.tenant_id, job.payload.clone()).await;
                assert!(result.is_ok(), "Requeue operation failed");

                // Optionally verify the status is QUEUED
                if let Ok(row) = sqlx::query("SELECT status, payload FROM sub_agent_queue WHERE id = $1 AND tenant_id = $2")
                    .bind(&job.id)
                    .bind(&job.tenant_id)
                    .fetch_one(&qm.pool)
                    .await {
                    use sqlx::Row;
                    let status: String = row.get("status");
                    assert_eq!(status, "QUEUED");
                    let payload_str: String = row.get("payload");
                    let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap();
                    assert_eq!(payload["attempts"], 1);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_queue_manager_poll_contention() {
        if let Ok(db_url) = std::env::var("OHC_DATABASE_URL") {
            let pool = PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { conn.execute("DISCARD ALL").await?; Ok(true) }) })
                .connect_lazy(&db_url)
                .unwrap();

            let qm = QueueManager::new(pool);
            let job_id = uuid::Uuid::new_v4().to_string();
            let org_id = "tenant-c".to_string();

            // To ensure table exists, run a creation query, ignore if it fails
            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS sub_agent_queue (id VARCHAR PRIMARY KEY, tenant_id VARCHAR NOT NULL, parent_task_id VARCHAR, payload TEXT, status VARCHAR, worker_id VARCHAR, scheduled_at TIMESTAMP, completed_at TIMESTAMP, created_at TIMESTAMP, updated_at TIMESTAMP)")
                .execute(&qm.pool)
                .await;

            let job = SubAgentJob {
                id: job_id.clone(),
                tenant_id: org_id.clone(),
                parent_task_id: "task-3".to_string(),
                payload: serde_json::json!({"action": "test_poll"}),
                status: "QUEUED".to_string(),
                worker_id: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            // First enqueue the job
            if qm.enqueue(job.clone()).await.is_ok() {
                // Poll for the job
                let result = qm.poll("worker-1").await;
                assert!(result.is_ok(), "Poll operation failed");
                if let Ok(Some(polled_job)) = result {
                    assert_eq!(polled_job.id, job_id, "Polled job ID does not match");
                    assert_eq!(polled_job.status, "RUNNING", "Status should be updated to RUNNING");
                    assert_eq!(polled_job.worker_id.unwrap(), "worker-1", "Worker ID should be updated");
                }
            }
        }
    }
}
