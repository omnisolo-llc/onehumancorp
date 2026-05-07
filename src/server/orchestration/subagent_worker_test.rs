#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::orchestration::subagent_worker::{SubAgentWorker, DefaultSubAgentSpawner};
    use crate::orchestration::queue::{SQLiteTaskQueue, Job, TaskQueue};
    use sqlx::SqlitePool;
    use chrono::Utc;
    use crate::orchestration::mesh::TeammateMesh;

    struct DummyMesh;
    #[async_trait::async_trait]
    impl TeammateMesh for DummyMesh {
        async fn publish(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
        async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
        async fn subscribe(&self, _topic: &str, _handler: Box<dyn Fn(ohc_builtin_agent::mesh::transport::Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
        async fn acquire_lock(&self, _resource: &str, _owner: &str, _ttl_seconds: u64) -> Result<bool, String> { Ok(true) }
        async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> { Ok(()) }
        async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> { Ok(()) }
        async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> { Ok(vec![]) }
        async fn ping(&self) -> Result<(), String> { Ok(()) }
        async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
        async fn publish_state_handoff(&self, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
        async fn subscribe_state_handoff(&self, _handler: Box<dyn Fn(ohc_builtin_agent::mesh::transport::Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
    }

    #[tokio::test]
    async fn test_subagent_worker_fail_updates_parent() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

        sqlx::query(
            "CREATE TABLE shared_tasks_decomposition (
                id TEXT PRIMARY KEY,
                status TEXT,
                updated_at DATETIME
            )"
        ).execute(&pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE sub_agent_jobs (
                id TEXT PRIMARY KEY,
                parent_task_id TEXT,
                agent_role TEXT NOT NULL,
                payload TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'QUEUED',
                attempts INTEGER DEFAULT 0,
                max_attempts INTEGER DEFAULT 3,
                run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                locked_until TIMESTAMPTZ,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
            )"
        ).execute(&pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE state_machine_transitions (
                id TEXT PRIMARY KEY,
                task_id TEXT,
                from_state TEXT,
                to_state TEXT,
                agent_id TEXT,
                transitioned_at DATETIME
            )"
        ).execute(&pool).await.unwrap();

        let parent_id = "parent-123";
        sqlx::query("INSERT INTO shared_tasks_decomposition (id, status, updated_at) VALUES (?, 'EXECUTING', CURRENT_TIMESTAMP)")
            .bind(parent_id)
            .execute(&pool)
            .await
            .unwrap();

        let job_id = "job-456";
        sqlx::query("INSERT INTO sub_agent_jobs (id, parent_task_id, agent_role, payload, status, run_after) VALUES (?, ?, 'test-role', '{}', 'RUNNING', CURRENT_TIMESTAMP)")
            .bind(job_id)
            .bind(parent_id)
            .execute(&pool)
            .await
            .unwrap();

        // Create dummy PG pool for DB struct compilation requirement
        let pg_pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap();
        let db = Arc::new(crate::db::DB { pool: pg_pool, store: crate::db::DbStore::Sqlite(pool.clone()) });

        let queue = Arc::new(SQLiteTaskQueue::new(Arc::new(pool.clone())));
        let spawner = Arc::new(DefaultSubAgentSpawner::new(db.clone()));

        let worker = SubAgentWorker::new(queue, spawner, db, Arc::new(DummyMesh), vec!["test-role".to_string()]);

        let job = Job {
            id: job_id.to_string(),
            parent_task_id: parent_id.to_string(),
            agent_role: "test-role".to_string(),
            payload: "{}".to_string(),
            status: "RUNNING".to_string(),
            attempts: 0,
            max_attempts: 3,
            run_after: Utc::now(),
            locked_until: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        worker.fail_job_for_test(&job, "simulated failure").await.unwrap();

        let parent_status: String = sqlx::query_scalar("SELECT status FROM shared_tasks_decomposition WHERE id = ?")
            .bind(parent_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(parent_status, "FAILED");
    }
}
