use crate::memory_store::LongTermMemory;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use async_trait::async_trait;
use ohc_builtin_agent_core::types::{ChatRequest, Message};

/// Memory: Short-term, Long-term (SQLite/Redis), Long-term (Anthropic 3-Tier)
/// A SQLite-backed LongTermMemory implementation for the agent harness.
#[derive(Clone)]
pub struct SqliteMemoryStore {
    pool: SqlitePool,
    llm: std::sync::Arc<dyn crate::llm::LlmClient>,
}

impl std::fmt::Debug for SqliteMemoryStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SqliteMemoryStore")
            .finish_non_exhaustive()
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

        Ok(Self { pool, llm })
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
        // FTS5 session search for long term memory retrieval
        let search_pattern = format!("\"{}\"", query);
        let rows = sqlx::query_as::<_, (String,)>("SELECT content FROM agent_memory WHERE agent_memory MATCH ? ORDER BY rank LIMIT ?")
            .bind(search_pattern)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if rows.is_empty() {
            return Ok(vec![]);
        }

        let context: Vec<String> = rows.into_iter().map(|r| r.0).collect();
        let combined_context = context.join("\n\n");

        let system_prompt = "You are a memory retrieval assistant. Your task is to perform cross-session recall. You are provided with a set of memory snippets matching a query. Synthesize these snippets into a concise, unified summary relevant to the query. Do not add any new facts, just summarize what is there.";
        let user_prompt = format!("Query: {}\n\nMemory Snippets:\n{}", query, combined_context);

        let req = ChatRequest {
            model: "default".to_string(),
            system: system_prompt.to_string(),
            messages: vec![Message::user(user_prompt)],
            tools: vec![],
            max_tokens: 1000,
            temperature: 0.1,
        };

        let resp = self.llm.chat(req).await.map_err(|e| e.to_string())?;

        Ok(vec![resp.message.content])
    }

    fn as_anthropic_accessor(&self) -> Option<std::sync::Arc<dyn ohc_builtin_agent_tools::anthropic_memory::MemoryAccessor>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatResponse, Usage};

    struct MockLlmClient;

    #[async_trait::async_trait]
    impl crate::llm::LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant("Summarized: 42 is the secret code."),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_sqlite_memory_store() {
        let store = SqliteMemoryStore::new("sqlite::memory:", std::sync::Arc::new(MockLlmClient)).await.unwrap();

        store.store("The secret code is 42", vec!["secret".to_string()]).await.unwrap();
        store.store("The weather is sunny", vec!["weather".to_string()]).await.unwrap();

        let results = store.retrieve("secret", 10).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("42"));
    }
}
