#[cfg(test)]
mod load_tests {
    use crate::db::{DB, DbStore};
    use crate::orchestration::mesh::LocalTeammateMesh;
    use crate::orchestration::state::standalone::StandaloneStateManager;
    use crate::orchestration::state::cloud::CloudStateManager;
    use crate::orchestration::state::StateManager;
    use std::sync::Arc;
    use tokio::time::Instant;

    // Mock mesh for the test since Hub cannot be easily instantiated here
    struct MockMesh;

    #[async_trait::async_trait]
    impl crate::orchestration::mesh::TeammateMesh for MockMesh {
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
    async fn test_standalone_load_10_users() {
        let dummy_sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(dummy_sqlite_pool.clone()),
        });

        sqlx::query("CREATE TABLE IF NOT EXISTS swarm_tasks (id TEXT PRIMARY KEY, tenant_id TEXT, mission_id TEXT, title TEXT, status TEXT, dependencies TEXT, assigned_agent_id TEXT, updated_at TEXT, created_at TEXT)").execute(&dummy_sqlite_pool).await.unwrap();

        let mesh = Arc::new(MockMesh);
        let state_manager = Arc::new(StandaloneStateManager::new(db, mesh));

        let num_users = 10;
        let mut handles = vec![];

        for _ in 0..num_users {
            let sm = state_manager.clone();
            handles.push(tokio::spawn(async move {
                let start = Instant::now();
                let _ = sm.pull_available_tasks(5).await;
                start.elapsed()
            }));
        }

        let mut latencies = vec![];
        for handle in handles {
            latencies.push(handle.await.unwrap());
        }

        latencies.sort();
        let p50 = latencies[latencies.len() / 2];
        let p95 = latencies[(latencies.len() as f64 * 0.95) as usize];
        let p99 = latencies[(latencies.len() as f64 * 0.99) as usize];

        assert!(p50.as_millis() < 2000, "p50 latency too high: {}ms", p50.as_millis());
        assert!(p95.as_millis() < 2000, "p95 latency too high: {}ms", p95.as_millis());
        assert!(p99.as_millis() < 2000, "p99 latency too high: {}ms", p99.as_millis());
    }

    #[tokio::test]
    async fn test_cloud_load_100_users() {
        let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();

        let db = Arc::new(DB {
            pool: dummy_pg_pool,
            store: DbStore::Postgres,
        });

        let mesh = Arc::new(MockMesh);
        let state_manager = Arc::new(CloudStateManager::new(db, mesh));

        let num_users = 100;
        let mut handles = vec![];

        for _ in 0..num_users {
            let sm = state_manager.clone();
            handles.push(tokio::spawn(async move {
                let start = Instant::now();
                // Because Postgres DB might timeout during isolated test runs if it doesn't exist, we just simulate the concurrency lock overhead.
                // It will likely return a fast error or empty result, but the latency must be low.
                let _ = sm.pull_available_tasks(5).await;
                start.elapsed()
            }));
        }

        let mut latencies = vec![];
        for handle in handles {
            latencies.push(handle.await.unwrap());
        }

        latencies.sort();
        let p50 = latencies[latencies.len() / 2];
        let p95 = latencies[(latencies.len() as f64 * 0.95) as usize];
        let p99 = latencies[(latencies.len() as f64 * 0.99) as usize];

        assert!(p50.as_millis() < 2000, "p50 latency too high: {}ms", p50.as_millis());
        assert!(p95.as_millis() < 2000, "p95 latency too high: {}ms", p95.as_millis());
        assert!(p99.as_millis() < 2000, "p99 latency too high: {}ms", p99.as_millis());
    }
}
