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

        // Initialize table
        // Hermes Agent Unique Harness Innovations: FTS5 session search
        // Cross-session recall with LLM summarization

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS agent_memory (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content TEXT NOT NULL,
                tags TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )"
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            "CREATE VIRTUAL TABLE IF NOT EXISTS agent_memory_fts USING fts5(
                content,
                tags,
                content='agent_memory',
                content_rowid='id'
            )"
        )
        .execute(&pool)
        .await?;

        // Triggers to keep FTS index up to date
        sqlx::query(
            "CREATE TRIGGER IF NOT EXISTS agent_memory_ai AFTER INSERT ON agent_memory BEGIN
                INSERT INTO agent_memory_fts(rowid, content, tags) VALUES (new.id, new.content, new.tags);
            END;"
        ).execute(&pool).await?;

        sqlx::query(
            "CREATE TRIGGER IF NOT EXISTS agent_memory_ad AFTER DELETE ON agent_memory BEGIN
                INSERT INTO agent_memory_fts(agent_memory_fts, rowid, content, tags) VALUES('delete', old.id, old.content, old.tags);
            END;"
        ).execute(&pool).await?;

        sqlx::query(
            "CREATE TRIGGER IF NOT EXISTS agent_memory_au AFTER UPDATE ON agent_memory BEGIN
                INSERT INTO agent_memory_fts(agent_memory_fts, rowid, content, tags) VALUES('delete', old.id, old.content, old.tags);
                INSERT INTO agent_memory_fts(rowid, content, tags) VALUES (new.id, new.content, new.tags);
            END;"
        ).execute(&pool).await?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl LongTermMemory for SqliteMemoryStore {
    async fn store(&self, content: &str, tags: Vec<String>) -> Result<(), String> {
        let tags_json = serde_json::to_string(&tags).map_err(|e| e.to_string())?;
        sqlx::query("INSERT INTO agent_memory (content, tags) VALUES (?, ?)")
            .bind(content)
            .bind(tags_json)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn retrieve(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        // Simple text search for long term memory retrieval
        // Use FTS5 for fast full-text search
        let fts_query = format!("\"{}\"", query); // simple quoting for MATCH
        let rows = sqlx::query_as::<_, (String,)>("SELECT content FROM agent_memory_fts WHERE agent_memory_fts MATCH ? ORDER BY rank LIMIT ?")
            .bind(fts_query)
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
