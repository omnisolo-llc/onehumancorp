use crate::autodream_pipeline::consolidator::AutoDreamConsolidator;
use crate::db::{DB, DbStore};
use crate::autodream_pipeline::llm_client::LLMClient;
use std::sync::Arc;
use sqlx::sqlite::SqlitePoolOptions;
use async_trait::async_trait;

struct MockLLMClient;
#[async_trait]
impl LLMClient for MockLLMClient {
    async fn generate_embedding(&self, _text: &str) -> Result<Vec<f32>, String> {
        Ok(vec![0.1, 0.2, 0.3])
    }
}

#[tokio::test]
async fn test_process_agent_session_data() {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();

    sqlx::query(
        "CREATE TABLE agent_session_data (
            session_id TEXT PRIMARY KEY,
            agent_id TEXT NOT NULL,
            context_data TEXT NOT NULL,
            _sync_status TEXT DEFAULT 'pending'
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE autodream_memories (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            agent_id TEXT NOT NULL,
            task_id TEXT NOT NULL,
            content TEXT NOT NULL,
            embedding TEXT,
            source_type TEXT NOT NULL
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query("INSERT INTO agent_session_data (session_id, agent_id, context_data) VALUES ('session1', 'agent1', 'test context')")
        .execute(&pool).await.unwrap();

    let db = Arc::new(DB { pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(), store: DbStore::Sqlite(pool.clone()) });
    let llm_client = Arc::new(MockLLMClient);

    let consolidator = AutoDreamConsolidator::new(db, llm_client);

    consolidator.process_agent_session_data().await.unwrap();

    let mem_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM autodream_memories").fetch_one(&pool).await.unwrap();
    assert_eq!(mem_count.0, 1);

    let sync_status: (String,) = sqlx::query_as("SELECT _sync_status FROM agent_session_data WHERE session_id = 'session1'")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(sync_status.0, "processed");
}
