use crate::memory_store::LongTermMemory;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use async_trait::async_trait;

/// Memory: Short-term, Long-term (SQLite/Redis), Long-term (Anthropic 3-Tier)
/// A SQLite-backed LongTermMemory implementation for the agent harness.
#[derive(Clone)]
pub struct SqliteMemoryStore {
    llm: std::sync::Arc<dyn crate::llm::LlmClient>,
    pool: SqlitePool,
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

        Ok(Self { llm, pool })
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

        let raw_results: Vec<String> = rows.into_iter().map(|r| r.0).collect();

        if raw_results.is_empty() {
            return Ok(vec![]);
        }

        // Hermes Agent Unique Harness Innovations: FTS5 session search: Cross-session recall with LLM summarization
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

        match self.llm.chat(chat_req).await {
            Ok(resp) => {
                let summary = resp.message.content.trim().to_string();
                Ok(vec![format!("Cross-Session Recall Summary:\n{}", summary)])
            }
            Err(e) => {
                tracing::warn!("Failed to summarize FTS5 results via LLM, returning raw: {}", e);
                Ok(raw_results)
            }
        }
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
        use crate::types::{ChatRequest, ChatResponse, Usage, Message};
        use std::sync::Arc;

        struct MockLlm;
        #[async_trait::async_trait]
        impl crate::llm::LlmClient for MockLlm {
            async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
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
        let llm = Arc::new(MockLlm);
        let store = SqliteMemoryStore::new("sqlite::memory:", llm).await.unwrap();

        store.store("The secret code is 42", vec!["secret".to_string()]).await.unwrap();
        store.store("The weather is sunny", vec!["weather".to_string()]).await.unwrap();

        let results = store.retrieve("secret", 10).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("Summarized cross-session recall"));
    }
}
