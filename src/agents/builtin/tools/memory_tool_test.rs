use ohc_builtin_agent_core::types::LongTermMemory;
use ohc_builtin_agent_core::types::EmbeddingRecord;
use crate::tools::memory_tool::{memory_store_tool, memory_search_tool};
use crate::memory_store::{LongTermMemory, EmbeddingRecord};
use std::sync::Arc;
use serde_json::json;

struct MockLtm {
    pub repo: Arc<crate::memory_store::VectorRepository>,
}

impl std::fmt::Debug for MockLtm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MockLtm").finish()
    }
}

#[async_trait::async_trait]
impl LongTermMemory for MockLtm {
    async fn retrieve(&self, _query: &str, limit: usize) -> Result<Vec<String>, String> {
        let records = self.repo.semantic_search("test", &[0.0], limit as i64).await?;
        Ok(records.into_iter().map(|r| r.content).collect())
    }
    async fn store(&self, content: &str, tags: Vec<String>) -> Result<(), String> {
        let record = EmbeddingRecord {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: "test".to_string(),
            agent_id: "test".to_string(),
            content: content.to_string(),
            embedding: vec![0.0],
            source_type: "MANUAL".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 0,
            reliability_score: 100,
            owner_override: true,
            archived: false,
            archived: false,
            metadata: Some(json!(tags).to_string()),
        };
        self.repo.upsert(&record).await
    }
}

async fn setup_sqlite_repo() -> Arc<crate::memory_store::VectorRepository> {
    use std::str::FromStr;
    let conn_opts = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
    let pool = sqlx::sqlite::SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS consolidated_memory (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            agent_id TEXT,
            content TEXT NOT NULL,
            embedding TEXT,
            source_type TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            reference_count INTEGER DEFAULT 0,
            reliability_score INTEGER DEFAULT 50,
            owner_override BOOLEAN DEFAULT FALSE,
            archived BOOLEAN DEFAULT FALSE,
            metadata TEXT
        );"
    ).execute(&pool).await.unwrap();

    Arc::new(crate::memory_store::VectorRepository::new_sqlite(pool))
}

#[tokio::test]
async fn test_memory_tools_integration() {
    let repo = setup_sqlite_repo().await;
    let ltm = Arc::new(MockLtm { repo: repo.clone() });

    let store_tool = memory_store_tool(ltm.clone());
    let search_tool = memory_search_tool(ltm.clone());

    let store_res = store_tool.execute.execute(json!({
        "content": "Maya loves blueberry muffins.",
        "tags": ["preference", "food"]
    })).await.unwrap();
    assert!(store_res.contains("successfully"));
}
