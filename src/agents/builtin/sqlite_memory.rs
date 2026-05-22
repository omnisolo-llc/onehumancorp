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
        use std::str::FromStr;
        use sqlx::sqlite::SqliteConnectOptions;

        let mut conn_opts = SqliteConnectOptions::from_str(db_url)?
            .create_if_missing(true);

        let key = std::env::var("OHC_SQLITE_KEY").expect("CRITICAL SECURITY ERROR: OHC_SQLITE_KEY must be set in Standalone Mode to ensure secure, encrypted SQLite storage.");
        if key.is_empty() {
            panic!("CRITICAL SECURITY ERROR: OHC_SQLITE_KEY is empty. Encrypted storage is mandatory in Standalone Mode.");
        }

        let pragma_key = format!("'{}'", key.replace('\'', "''"));
        conn_opts = conn_opts.pragma("key", pragma_key);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(conn_opts)
            .await?;

        // Initialize table
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
        let search_pattern = format!("%{}%", query);
        let rows = sqlx::query_as::<_, (String,)>("SELECT content FROM agent_memory WHERE content LIKE ? LIMIT ?")
            .bind(search_pattern)
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
        unsafe { std::env::set_var("OHC_SQLITE_KEY", "test_key"); }
        let store = SqliteMemoryStore::new("sqlite::memory:").await.unwrap();

        store.store("The secret code is 42", vec!["secret".to_string()]).await.unwrap();
        store.store("The weather is sunny", vec!["weather".to_string()]).await.unwrap();

        let results = store.retrieve("secret", 10).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("42"));
    }
}
