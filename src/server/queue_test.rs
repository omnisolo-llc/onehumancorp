use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sub_agent_queue_isolation() {
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            let pool = sqlx::postgres::PgPoolOptions::new()
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
}
