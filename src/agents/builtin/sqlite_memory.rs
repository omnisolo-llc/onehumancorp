use crate::memory_store::LongTermMemory;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Memory: Short-term, Long-term (SQLite/Redis), Long-term (Anthropic 3-Tier)
/// A SQLite-backed LongTermMemory implementation for the agent harness.
#[derive(Clone)]
pub struct SqliteMemoryStore {
    llm: std::sync::Arc<dyn crate::llm::LlmClient>,
    pool: SqlitePool,
    cache: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

impl std::fmt::Debug for SqliteMemoryStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SqliteMemoryStore").finish()
    }
}

impl SqliteMemoryStore {
    pub async fn new(db_url: &str, llm: std::sync::Arc<dyn crate::llm::LlmClient>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;

        // Initialize table
        sqlx::query(
            "CREATE VIRTUAL TABLE IF NOT EXISTS agent_memory USING fts5(
                content,
                tags,
                created_at UNINDEXED
            )"
        )
        .execute(&pool)
        .await?;

        Ok(Self {
            llm,
            pool,
            cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Optimized FTS5 query formatting: converts a plain query string
    /// into a tokenized FTS5 AND-search pattern to dramatically improve search accuracy.
    fn format_fts5_query(query: &str) -> String {
        let tokens: Vec<String> = query
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| {
                // Escape quotes to prevent FTS5 syntax errors
                let clean = s.replace("\"", "");
                format!("\"{}\"", clean)
            })
            .collect();

        if tokens.is_empty() {
            return "\"\"".to_string();
        }

        tokens.join(" AND ")
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

        // Invalidate cache upon new insertions to maintain consistency
        let mut cache = self.cache.write().await;
        cache.clear();

        Ok(())
    }

    async fn retrieve(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let cache_key = format!("{}:{}", query, limit);

        // 1. Check Cache
        {
            let cache = self.cache.read().await;
            if let Some(cached_result) = cache.get(&cache_key) {
                return Ok(cached_result.clone());
            }
        }

        // 2. FTS5 session search for long term memory retrieval
        let search_pattern = Self::format_fts5_query(query);
        let rows = sqlx::query_as::<_, (String,)>("SELECT content FROM agent_memory WHERE agent_memory MATCH ? ORDER BY rank LIMIT ?")
            .bind(&search_pattern)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let raw_results: Vec<String> = rows.into_iter().map(|r| r.0).collect();

        if raw_results.is_empty() {
            let result = vec![];
            let mut cache = self.cache.write().await;
            cache.insert(cache_key, result.clone());
            return Ok(result);
        }

        // 3. Hermes Agent Unique Harness Innovations: FTS5 session search: Cross-session recall with LLM summarization
        let combined_memories = raw_results.join("\n\n---\n\n");
        let summarize_prompt = format!(
            "You are a memory condensation agent. Summarize the following past session memories relevant to the query: '{}'.\n\nReturn a single, cohesive, condensed summary. Do not include introductory text.\n\nMemories:\n{}",
            query, combined_memories
        );

        let chat_req = crate::types::ChatRequest {
            model: "default".to_string(), // Can be customized or pulled from config
            system: "You are a helpful assistant that summarizes memory.".to_string(),
            messages: vec![crate::types::Message::user(summarize_prompt)],
            tools: vec![],
            max_tokens: 1000,
            temperature: 0.0,
        };

        let result = match self.llm.chat(chat_req).await {
            Ok(resp) => {
                let summary = resp.message.content.trim().to_string();
                vec![format!("Cross-Session Recall Summary:\n{}", summary)]
            }
            Err(e) => {
                tracing::warn!("Failed to summarize FTS5 results via LLM, returning raw: {}", e);
                raw_results
            }
        };

        // 4. Update Cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(cache_key, result.clone());
        }

        Ok(result)
    }

    fn as_anthropic_accessor(&self) -> Option<std::sync::Arc<dyn ohc_builtin_agent_tools::anthropic_memory::MemoryAccessor>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChatRequest, ChatResponse, Usage, Message};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockLlm {
        pub call_count: AtomicUsize,
        pub fail_llm: bool,
    }

    #[async_trait::async_trait]
    impl crate::llm::LlmClient for MockLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            self.call_count.fetch_add(1, Ordering::SeqCst);
            if self.fail_llm {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Mock LLM failure")));
            }
            Ok(ChatResponse {
                message: Message::assistant("Summarized cross-session recall"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: None,
            })
        }
        async fn generate_embedding(&self, _text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(vec![])
        }
    }

    #[tokio::test]
    async fn test_sqlite_memory_store_optimizations() {
        let llm = Arc::new(MockLlm { call_count: AtomicUsize::new(0), fail_llm: false });
        let store = SqliteMemoryStore::new("sqlite::memory:", llm.clone()).await.unwrap();

        store.store("The secret code is 42", vec!["secret".to_string()]).await.unwrap();
        store.store("The weather is sunny", vec!["weather".to_string()]).await.unwrap();

        // 1. Initial Retrieve (Cache Miss -> DB hit + LLM hit)
        let results = store.retrieve("secret code", 10).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("Summarized cross-session recall"));
        assert_eq!(llm.call_count.load(Ordering::SeqCst), 1);

        // 2. Cached Retrieve (Cache Hit -> NO LLM hit)
        let results2 = store.retrieve("secret code", 10).await.unwrap();
        assert_eq!(results2.len(), 1);
        assert!(results2[0].contains("Summarized cross-session recall"));
        assert_eq!(llm.call_count.load(Ordering::SeqCst), 1); // call count shouldn't increase

        // 3. New Store clears cache
        store.store("The new secret is 43", vec!["secret".to_string()]).await.unwrap();

        // 4. Retrieve again (Cache Miss -> LLM hit again)
        let results3 = store.retrieve("secret code", 10).await.unwrap();
        assert_eq!(results3.len(), 1);
        assert!(results3[0].contains("Summarized cross-session recall"));
        assert_eq!(llm.call_count.load(Ordering::SeqCst), 2); // call count increases

        // Test query formatting directly
        assert_eq!(SqliteMemoryStore::format_fts5_query("hello world"), "\"hello\" AND \"world\"");
        assert_eq!(SqliteMemoryStore::format_fts5_query("   "), "\"\"");
    }

    #[tokio::test]
    async fn test_sqlite_memory_store_empty_results() {
        let llm = Arc::new(MockLlm { call_count: AtomicUsize::new(0), fail_llm: false });
        let store = SqliteMemoryStore::new("sqlite::memory:", llm.clone()).await.unwrap();

        // DB is empty, should return empty immediately and hit cache next time
        let results = store.retrieve("nonexistent", 10).await.unwrap();
        assert!(results.is_empty());
        assert_eq!(llm.call_count.load(Ordering::SeqCst), 0); // NO LLM call since raw_results empty

        // Retrieve again to ensure empty is cached correctly
        let results2 = store.retrieve("nonexistent", 10).await.unwrap();
        assert!(results2.is_empty());
        assert_eq!(llm.call_count.load(Ordering::SeqCst), 0); // STILL NO LLM call
    }

    #[tokio::test]
    async fn test_sqlite_memory_store_llm_failure() {
        let llm = Arc::new(MockLlm { call_count: AtomicUsize::new(0), fail_llm: true });
        let store = SqliteMemoryStore::new("sqlite::memory:", llm.clone()).await.unwrap();

        store.store("The secret code is 42", vec!["secret".to_string()]).await.unwrap();

        let results = store.retrieve("secret", 10).await.unwrap();
        // Since LLM fails, it returns the raw result
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "The secret code is 42");
        assert_eq!(llm.call_count.load(Ordering::SeqCst), 1);

        // Ensure the fallback is cached too
        let results2 = store.retrieve("secret", 10).await.unwrap();
        assert_eq!(results2.len(), 1);
        assert_eq!(results2[0], "The secret code is 42");
        assert_eq!(llm.call_count.load(Ordering::SeqCst), 1);
    }
}
