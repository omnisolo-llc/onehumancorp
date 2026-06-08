/// Master Catalog B.3. Memory
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

        // Anthropic 3-Tier Memory Tables
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS anthropic_index_kv (
                key TEXT PRIMARY KEY,
                value TEXT
            )",
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS anthropic_topics (
                topic_name TEXT PRIMARY KEY,
                content TEXT
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
    async fn store(&self, content: &str, tags: Vec<String>) -> Result<(), String> {
        let tags_json = serde_json::to_string(&tags).map_err(|e| e.to_string())?;
        sqlx::query("INSERT INTO agent_memory (content, tags) VALUES (?, ?)")
            .bind(content)
            .bind(tags_json)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        // Append to the lightweight index
        let mut existing_index = self.get_lightweight_index().await?;

        let truncated_content = if content.len() > 150 {
            format!("{}...", &content[..147])
        } else {
            content.to_string()
        };

        let tags_str = if tags.is_empty() {
            String::new()
        } else {
            format!(" [{}]", tags.join(", "))
        };
        let new_entry = format!("- {}{}\n", truncated_content.replace('\n', " "), tags_str);

        existing_index.push_str(&new_entry);
        self.update_index(&existing_index).await?;

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

    async fn get_lightweight_index(&self) -> Result<String, String> {
        let row: Option<(String,)> =
            sqlx::query_as("SELECT value FROM anthropic_index_kv WHERE key = 'index'")
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

        Ok(row.map(|r| r.0).unwrap_or_default())
    }

    async fn retrieve_topic(&self, topic_name: &str) -> Result<String, String> {
        let safe_name =
            topic_name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "");
        let row: Option<(String,)> =
            sqlx::query_as("SELECT content FROM anthropic_topics WHERE topic_name = ?")
                .bind(&safe_name)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

        row.map(|r| r.0)
            .ok_or_else(|| format!("Topic '{}' not found", safe_name))
    }

    async fn search_transcripts(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let search_pattern = format!("\"{}\"", query);
        let rows = sqlx::query_as::<_, (String,)>("SELECT snippet(session_messages_fts, -1, '[', ']', '...', 64) FROM session_messages_fts WHERE session_messages_fts MATCH ? ORDER BY rank LIMIT ?")
            .bind(&search_pattern)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(rows.into_iter().map(|r| r.0).collect())
    }

    fn as_anthropic_accessor(
        &self,
    ) -> Option<std::sync::Arc<dyn crate::tools::anthropic_memory::MemoryAccessor>> {
        Some(std::sync::Arc::new(self.clone()))
    }
}

impl SqliteMemoryStore {
    pub async fn update_index(&self, content: &str) -> Result<(), String> {
        sqlx::query("INSERT INTO anthropic_index_kv (key, value) VALUES ('index', ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value")
            .bind(content)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn write_topic(&self, topic_name: &str, content: &str) -> Result<(), String> {
        let safe_name =
            topic_name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "");
        sqlx::query("INSERT INTO anthropic_topics (topic_name, content) VALUES (?, ?) ON CONFLICT(topic_name) DO UPDATE SET content = excluded.content")
            .bind(&safe_name)
            .bind(content)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[async_trait]
impl crate::tools::anthropic_memory::MemoryAccessor for SqliteMemoryStore {
    async fn retrieve_topic(&self, topic_name: &str) -> Result<String, String> {
        LongTermMemory::retrieve_topic(self, topic_name).await
    }

    async fn search_transcripts(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        LongTermMemory::search_transcripts(self, query, limit).await
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
    async fn test_sqlite_3_tier_memory() {
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
                    message: Message::assistant("Summarized"),
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

        // 1. Test lightweight index and `store` mechanic
        store
            .store(
                "The architectural decision is to use Glassmorphism.",
                vec!["ui".to_string(), "design".to_string()],
            )
            .await
            .unwrap();
        let index = store.get_lightweight_index().await.unwrap();
        assert!(index.contains("Glassmorphism"));
        assert!(index.contains("[ui, design]"));

        // 2. Test topics
        store
            .write_topic(
                "system_architecture",
                "Detailed DB schema information here.",
            )
            .await
            .unwrap();
        let topic_content = store.retrieve_topic("system_architecture").await.unwrap();
        assert_eq!(topic_content, "Detailed DB schema information here.");
        assert!(store.retrieve_topic("nonexistent").await.is_err());

        // 3. Test transcripts
        store
            .store_session_message(
                "session_999",
                "user",
                "How do I configure the memory store?",
            )
            .await
            .unwrap();
        store
            .store_session_message(
                "session_999",
                "assistant",
                "You use the 3-Tier Anthropic Memory store.",
            )
            .await
            .unwrap();

        let transcripts = store.search_transcripts("Anthropic", 5).await.unwrap();
        assert_eq!(transcripts.len(), 1);
        assert!(transcripts[0].contains("[Anthropic]")); // highlighting from FTS snippet
    }
}
