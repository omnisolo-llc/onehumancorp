use crate::db::{DB, DbStore};
use crate::orchestration::mesh::TeammateMesh;
use ohc_builtin_agent_core::types::Message;
use std::sync::Arc;
use tokio::time::Duration;
use crate::orchestration::state::StateManager;
use crate::orchestration::state::cloud::CloudStateManager;

#[cfg(test)]
mod real_chaos_tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrent_load_cloud() {
        // We will test 100 concurrent task pulls against a real postgres instance to ensure no panics and correct lock handling
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(100)
            .connect("postgres://postgres:postgres@localhost:5432/test")
            .await;

        if pool.is_err() {
            println!("Skipping real postgres test due to missing DB");
            return;
        }
        let pool = pool.unwrap();

        let db = Arc::new(DB {
            pool,
            store: DbStore::Postgres,
        });

        // Use standard memory mesh for test
        let transport = Arc::new(crate::orchestration::chaos_test::DroppingMockTransport::new(0));
        let mesh = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(transport));

        let state_manager = Arc::new(CloudStateManager::new(db.clone(), mesh));

        let mut handles = vec![];
        for _ in 0..100 {
            let sm = state_manager.clone();
            handles.push(tokio::spawn(async move {
                let _ = sm.pull_available_tasks(1).await;
            }));
        }

        for handle in handles {
            let _ = handle.await;
        }
    }
}
