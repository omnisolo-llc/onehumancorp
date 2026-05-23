use crate::memory_store::LongTermMemory;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use async_trait::async_trait;

/// Memory: Short-term, Long-term (SQLite/Redis), Long-term (Anthropic 3-Tier)
/// A SQLite-backed LongTermMemory implementation for the agent harness.
#[derive(Debug, Clone)]
pub struct SqliteMemoryStore {
    pool: SqlitePool,
}

impl SqliteMemoryStore {
    pub async fn new(db_url: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;

        // Initialize table using FTS5 for full-text search as per Hermes Agent mechanic.
        // Master Catalog: Hermes Agent Unique Harness Innovations: FTS5 session search
        sqlx::query(
            "CREATE VIRTUAL TABLE IF NOT EXISTS agent_memory USING fts5(
                content,
                tags,
                created_at UNINDEXED
            )"
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl LongTermMemory for SqliteMemoryStore {
    async fn store(&self, content: &str, tags: Vec<String>) -> Result<(), String> {
        let tags_json = serde_json::to_string(&tags).map_err(|e| e.to_string())?;
        sqlx::query("INSERT INTO agent_memory (content, tags, created_at) VALUES (?, ?, CURRENT_TIMESTAMP)")
            .bind(content)
            .bind(tags_json)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn retrieve(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        // Master Catalog: Hermes Agent Unique Harness Innovations: FTS5 session search: Cross-session recall
        // Uses FTS5 MATCH for high-performance text search
        let rows = sqlx::query_as::<_, (String,)>("SELECT content FROM agent_memory WHERE agent_memory MATCH ? ORDER BY rank LIMIT ?")
            .bind(query)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(rows.into_iter().map(|r| r.0).collect())
    }

    fn as_anthropic_accessor(&self) -> Option<std::sync::Arc<dyn ohc_builtin_agent_tools::anthropic_memory::MemoryAccessor>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sqlite_memory_store() {
        let store = SqliteMemoryStore::new("sqlite::memory:").await.unwrap();

        store.store("The secret code is 42", vec!["secret".to_string()]).await.unwrap();
        store.store("The weather is sunny", vec!["weather".to_string()]).await.unwrap();

        let results = store.retrieve("secret", 10).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("42"));
    }
}
