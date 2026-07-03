/// Master Catalog B.3. Memory: Long-term (OpenAI/LangGraph): Sessions backed by SQLite/Redis, or namespace-organized JSON Stores. Hermes Agent Unique Harness Innovations: FTS5 session search: Cross-session recall with LLM summarization.
use crate::memory_store::LongTermMemory;
use async_trait::async_trait;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

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
    pub async fn new(
        db_url: &str,
        llm: std::sync::Arc<dyn crate::llm::LlmClient>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
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
            )",
        )
        .execute(&pool)
        .await?;

        // FTS5 Session Messages Table (Hermes-style)
        sqlx::query(
            "CREATE VIRTUAL TABLE IF NOT EXISTS session_messages_fts USING fts5(
                session_id UNINDEXED,
                role UNINDEXED,
                content,
                timestamp UNINDEXED
            )",
        )
        .execute(&pool)
        .await?;

        Ok(Self { llm, pool })
    }

    /// Stores a raw message into the session search FTS5 table
    pub async fn store_session_message(
        &self,
        session_id: &str,
        role: &str,
        content: &str,
    ) -> Result<(), String> {
        let timestamp = chrono::Utc::now().timestamp();
        sqlx::query("INSERT INTO session_messages_fts (session_id, role, content, timestamp) VALUES (?, ?, ?, ?)")
            .bind(session_id)
            .bind(role)
            .bind(content)
            .bind(timestamp)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Searches session messages using FTS5 MATCH, returning ranked snippets, and optionally summarizing them using the LLM.
    pub async fn search_session_messages(
        &self,
        session_id: &str,
        query: &str,
        limit: usize,
        summarize: bool,
    ) -> Result<Vec<String>, String> {
        let search_pattern = format!("\"{}\"", query);
        // Using SQLite FTS5 snippet function to highlight matches
        let rows = sqlx::query_as::<_, (String,)>("SELECT snippet(session_messages_fts, -1, '[', ']', '...', 64) FROM session_messages_fts WHERE session_id = ? AND session_messages_fts MATCH ? ORDER BY rank LIMIT ?")
            .bind(session_id)
            .bind(&search_pattern)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let raw_results: Vec<String> = rows.into_iter().map(|r| r.0).collect();

        if raw_results.is_empty() || !summarize {
            return Ok(raw_results);
        }

        let combined = raw_results.join("\n\n");
        let summarize_prompt = format!(
            "You are a memory condensation agent. Summarize the following session message snippets relevant to the query: '{}'.\n\nReturn a single, cohesive summary. Do not include introductory text.\n\nSnippets:\n{}",
            query, combined
        );

        let chat_req = crate::types::ChatRequest {
            model: "default".to_string(),
            system: "You are a helpful assistant that summarizes session context.".to_string(),
            messages: vec![crate::types::Message::user(summarize_prompt)],
            tools: vec![],
            max_tokens: 1000,
            temperature: 0.0,
        };

        match self.llm.chat(chat_req).await {
            Ok(resp) => {
                let summary = resp.message.content.trim().to_string();
                Ok(vec![format!("Session Search Summary:\n{}", summary)])
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to summarize session messages via LLM, returning raw snippets: {}",
                    e
                );
                Ok(raw_results)
            }
        }
    }
}

#[async_trait]
impl LongTermMemory for SqliteMemoryStore {
    async fn search_cross_session_messages(
        &self,
        query: &str,
        limit: usize,
        summarize: bool,
    ) -> Result<Vec<String>, String> {
        let search_pattern = format!("\"{}\"", query);
        // Using SQLite FTS5 snippet function, but omitting the `session_id = ?` filter
        let rows = sqlx::query_as::<_, (String, String)>("SELECT session_id, snippet(session_messages_fts, -1, '[', ']', '...', 64) FROM session_messages_fts WHERE session_messages_fts MATCH ? ORDER BY rank LIMIT ?")
            .bind(&search_pattern)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let raw_results: Vec<String> = rows
            .into_iter()
            .map(|(sid, snippet)| format!("[Session: {}] {}", sid, snippet))
            .collect();

        if raw_results.is_empty() || !summarize {
            return Ok(raw_results);
        }

        let combined = raw_results.join("\n\n");
        let summarize_prompt = format!(
            "You are a cross-session memory condensation agent. Summarize the following session message snippets relevant to the query: '{}'.\n\nThese snippets come from multiple past sessions. Synthesize the insights across these different sessions to provide a comprehensive, cohesive summary. Mention session context if relevant. Do not include introductory text.\n\nCross-Session Snippets:\n{}",
            query, combined
        );

        let chat_req = crate::types::ChatRequest {
            model: "default".to_string(),
            system: "You are a helpful assistant that synthesizes and summarizes context across multiple past sessions.".to_string(),
            messages: vec![crate::types::Message::user(summarize_prompt)],
            tools: vec![],
            max_tokens: 1500,
            temperature: 0.0,
        };

        match self.llm.chat(chat_req).await {
            Ok(resp) => {
                let summary = resp.message.content.trim().to_string();
                Ok(vec![format!("Cross-Session Search Summary:\n{}", summary)])
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to summarize cross-session messages via LLM, returning raw snippets: {}",
                    e
                );
                Ok(raw_results)
            }
        }
    }

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
        let rows = sqlx::query_as::<_, (String,)>(
            "SELECT content FROM agent_memory WHERE agent_memory MATCH ? ORDER BY rank LIMIT ?",
        )
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
                tracing::warn!(
                    "Failed to summarize FTS5 results via LLM, returning raw: {}",
                    e
                );
                Ok(raw_results)
            }
        }
    }

    async fn store_session_message(
        &self,
        session_id: &str,
        role: &str,
        content: &str,
    ) -> Result<(), String> {
        self.store_session_message(session_id, role, content).await
    }

    async fn search_session_messages(
        &self,
        session_id: &str,
        query: &str,
        limit: usize,
        summarize: bool,
    ) -> Result<Vec<String>, String> {
        self.search_session_messages(session_id, query, limit, summarize)
            .await
    }

    fn as_anthropic_accessor(
        &self,
    ) -> Option<std::sync::Arc<dyn crate::tools::anthropic_memory::MemoryAccessor>> {
        Some(std::sync::Arc::new(self.clone()))
    }
}

#[async_trait]
impl crate::tools::anthropic_memory::MemoryAccessor for SqliteMemoryStore {
    async fn retrieve_topic(&self, _topic_name: &str) -> Result<String, String> {
        Err("Topic retrieval not supported in SQLite FTS5 store directly.".to_string())
    }

    async fn search_transcripts(&self, _query: &str, _limit: usize) -> Result<Vec<String>, String> {
        // Fallback to cross-session search
        <Self as crate::memory_store::LongTermMemory>::search_cross_session_messages(
            self, _query, _limit, false,
        )
        .await
    }

    async fn search_cross_session_messages(
        &self,
        query: &str,
        limit: usize,
        summarize: bool,
    ) -> Result<Vec<String>, String> {
        <Self as crate::memory_store::LongTermMemory>::search_cross_session_messages(
            self, query, limit, summarize,
        )
        .await
    }

    async fn write_topic(&self, _topic_name: &str, _content: &str) -> Result<(), String> {
        Err("Topic writing not supported in SQLite FTS5 store directly.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sqlite_memory_store() {
        use crate::types::{ChatRequest, ChatResponse, Message, Usage};
        use std::sync::Arc;

        struct MockLlm;
        #[async_trait::async_trait]
        impl crate::llm::LlmClient for MockLlm {
            async fn chat(
                &self,
                _req: ChatRequest,
            ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                Ok(ChatResponse {
                    message: Message::assistant("Summarized cross-session recall"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: None,
                })
            }
            async fn generate_embedding(
                &self,
                _text: &str,
            ) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
                Ok(vec![])
            }
        }
        let llm = Arc::new(MockLlm);
        let store = SqliteMemoryStore::new("sqlite::memory:", llm)
            .await
            .unwrap();

        store
            .store("The secret code is 42", vec!["secret".to_string()])
            .await
            .unwrap();
        store
            .store("The weather is sunny", vec!["weather".to_string()])
            .await
            .unwrap();

        let results = store.retrieve("secret", 10).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("Summarized cross-session recall"));
    }

    #[tokio::test]
    async fn test_sqlite_session_messages_fts() {
        use crate::types::{ChatRequest, ChatResponse, Message, Usage};
        use std::sync::Arc;

        struct MockLlm;
        #[async_trait::async_trait]
        impl crate::llm::LlmClient for MockLlm {
            async fn chat(
                &self,
                _req: ChatRequest,
            ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                Ok(ChatResponse {
                    message: Message::assistant("Summarized session context regarding plans"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: None,
                })
            }
            async fn generate_embedding(
                &self,
                _text: &str,
            ) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
                Ok(vec![])
            }
        }
        let llm = Arc::new(MockLlm);
        let store = SqliteMemoryStore::new("sqlite::memory:", llm)
            .await
            .unwrap();

        let session_id = "session_123";
        store
            .store_session_message(
                session_id,
                "user",
                "I need help planning my Q3 roadmap. We will launch 2 new features.",
            )
            .await
            .unwrap();
        store
            .store_session_message(
                session_id,
                "assistant",
                "Sure, what are the features? I can help structure the plan.",
            )
            .await
            .unwrap();
        store
            .store_session_message("other_session", "user", "Unrelated planning info")
            .await
            .unwrap();

        // Test raw snippets (no summarization)
        let raw_snippets = store
            .search_session_messages(session_id, "planning", 5, false)
            .await
            .unwrap();
        assert_eq!(raw_snippets.len(), 1);
        assert!(raw_snippets[0].contains("[planning]")); // Snippet highlights

        // Test summarized results
        let summarized = store
            .search_session_messages(session_id, "planning", 5, true)
            .await
            .unwrap();
        assert_eq!(summarized.len(), 1);
        assert!(summarized[0].contains("Summarized session context regarding plans"));
    }

    #[tokio::test]
    async fn test_sqlite_cross_session_messages_fts() {
        use crate::types::{ChatRequest, ChatResponse, Message, Usage};
        use std::sync::Arc;

        struct MockLlm;
        #[async_trait::async_trait]
        impl crate::llm::LlmClient for MockLlm {
            async fn chat(
                &self,
                req: ChatRequest,
            ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                // Ensure the prompt includes text that confirms it is combining multiple sessions
                let prompt = &req.messages[0].content;
                assert!(prompt.contains("cross-session memory condensation agent"));
                assert!(prompt.contains("[Session: session_a]"));
                assert!(prompt.contains("[Session: session_b]"));

                Ok(ChatResponse {
                    message: Message::assistant(
                        "Summarized cross-session insights regarding apples",
                    ),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: None,
                })
            }
            async fn generate_embedding(
                &self,
                _text: &str,
            ) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
                Ok(vec![])
            }
        }
        let llm = Arc::new(MockLlm);
        let store = SqliteMemoryStore::new("sqlite::memory:", llm)
            .await
            .unwrap();

        // Store messages in different sessions
        store
            .store_session_message("session_a", "user", "I like to eat green apples.")
            .await
            .unwrap();
        store
            .store_session_message("session_b", "user", "I like to eat red apples.")
            .await
            .unwrap();
        store
            .store_session_message("session_c", "user", "I like to eat bananas.")
            .await
            .unwrap();

        // Search across all sessions
        let summarized = store
            .search_cross_session_messages("apples", 5, true)
            .await
            .unwrap();

        assert_eq!(summarized.len(), 1);
        assert!(summarized[0].contains("Cross-Session Search Summary"));
        assert!(summarized[0].contains("Summarized cross-session insights regarding apples"));
    }
}
