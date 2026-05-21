#[cfg(test)]
mod tests {
    use crate::orchestration::autodream::*;
    use crate::db::{DB, DbStore};
    use crate::autodream_pipeline::llm_client::MockLLMClient;
    use std::sync::Arc;
    use sqlx::sqlite::SqlitePoolOptions;
    use tokio;

    #[tokio::test]
    async fn test_autodream_worker_consolidation() {
        // Setup in-memory SQLite DB
        let sqlite_pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Create tables
        sqlx::query("CREATE TABLE agent_session_data (session_id TEXT PRIMARY KEY, agent_id TEXT NOT NULL, context_data TEXT NOT NULL)")
            .execute(&sqlite_pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE autodream_memories (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, agent_id TEXT NOT NULL, task_id TEXT NOT NULL, content TEXT NOT NULL, embedding BLOB, source_type TEXT NOT NULL, version INTEGER DEFAULT 1, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, topic TEXT DEFAULT '', _sync_status TEXT DEFAULT 'pending')")
            .execute(&sqlite_pool)
            .await
            .unwrap();

        // Seed data
        sqlx::query("INSERT INTO agent_session_data (session_id, agent_id, context_data) VALUES ('sess1', 'agent1', 'some context data')")
            .execute(&sqlite_pool)
            .await
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(sqlite_pool.clone()),
        });

        let mock_llm = Arc::new(MockLLMClient {
            embedding: vec![0.1; 1536],
        });

        let worker = AutoDreamWorker::new(db.clone(), mock_llm);

        // Run consolidation
        worker.consolidate_memories().await.unwrap();

        // Verify result in autodream_memories
        let row: (String, String) = sqlx::query_as("SELECT task_id, content FROM autodream_memories WHERE task_id = 'sess1'")
            .fetch_one(&sqlite_pool)
            .await
            .unwrap();
        assert_eq!(row.0, "sess1");
        assert_eq!(row.1, "some context data");

        // Verify session data is deleted
        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM agent_session_data WHERE session_id = 'sess1'")
            .fetch_one(&sqlite_pool)
            .await
            .unwrap();
        assert_eq!(count.0, 0);
    }
}
