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

        // Initialize Anthropic 3-Tier Memory tables
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS lightweight_index (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content TEXT NOT NULL,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )"
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS topic_files (
                topic_name TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )"
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            "CREATE VIRTUAL TABLE IF NOT EXISTS transcripts USING fts5(
                session_id UNINDEXED,
                query_text,
                content,
                created_at UNINDEXED
            )"
        )
        .execute(&pool)
        .await?;

        Ok(Self { llm, pool })
    }

    pub async fn update_lightweight_index(&self, content: &str) -> Result<(), String> {
        let truncated_content = if content.len() > 150 {
            format!("{}...", &content[..147])
        } else {
            content.to_string()
        };

        sqlx::query("INSERT INTO lightweight_index (content) VALUES (?)")
            .bind(truncated_content)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn upsert_topic(&self, topic_name: &str, content: &str) -> Result<(), String> {
        let safe_name = topic_name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "");
        sqlx::query(
            "INSERT INTO topic_files (topic_name, content, updated_at)
             VALUES (?1, ?2, CURRENT_TIMESTAMP)
             ON CONFLICT(topic_name) DO UPDATE SET
                content = excluded.content,
                updated_at = excluded.updated_at"
        )
        .bind(safe_name)
        .bind(content)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn store_transcript(&self, session_id: &str, query_text: &str, content: &str) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO transcripts (session_id, query_text, content)
             VALUES (?, ?, ?)"
        )
        .bind(session_id)
        .bind(query_text)
        .bind(content)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
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

    async fn get_lightweight_index(&self) -> Result<String, String> {
        let rows = sqlx::query_as::<_, (String,)>("SELECT content FROM lightweight_index ORDER BY updated_at ASC")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut index_content = String::new();
        for row in rows {
            index_content.push_str(&row.0);
            index_content.push('\n');
        }
        Ok(index_content.trim_end().to_string())
    }

    async fn retrieve_topic(&self, topic_name: &str) -> Result<String, String> {
        let safe_name = topic_name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "");
        match sqlx::query_as::<_, (String,)>("SELECT content FROM topic_files WHERE topic_name = ?")
            .bind(safe_name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?
        {
            Some(row) => Ok(row.0),
            None => Err(format!("Topic '{}' not found", topic_name)),
        }
    }

    async fn search_transcripts(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let search_pattern = format!("\"{}\"", query);
        let rows = sqlx::query_as::<_, (String,)>("SELECT content FROM transcripts WHERE transcripts MATCH ? ORDER BY rank LIMIT ?")
            .bind(search_pattern)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(rows.into_iter().map(|r| r.0).collect())
    }

    fn as_anthropic_accessor(&self) -> Option<std::sync::Arc<dyn ohc_builtin_agent_tools::anthropic_memory::MemoryAccessor>> {
        Some(std::sync::Arc::new(self.clone()))
    }
}

#[async_trait]
impl ohc_builtin_agent_tools::anthropic_memory::MemoryAccessor for SqliteMemoryStore {
    async fn retrieve_topic(&self, topic_name: &str) -> Result<String, String> {
        <Self as LongTermMemory>::retrieve_topic(self, topic_name).await
    }

    async fn search_transcripts(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        <Self as LongTermMemory>::search_transcripts(self, query, limit).await
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
        let store = SqliteMemoryStore::new("sqlite::memory:", llm.clone()).await.unwrap();

        store.store("The secret code is 42", vec!["secret".to_string()]).await.unwrap();
        store.store("The weather is sunny", vec!["weather".to_string()]).await.unwrap();

        let results = store.retrieve("secret", 10).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("Summarized cross-session recall"));
    }

    #[tokio::test]
    async fn test_sqlite_memory_store_anthropic_3_tier() {
        use crate::types::{ChatRequest, ChatResponse, Usage, Message};
        use std::sync::Arc;
        use ohc_builtin_agent_tools::anthropic_memory::MemoryAccessor;

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

        // 1) Test lightweight index
        store.update_lightweight_index("Context index 1").await.unwrap();
        store.update_lightweight_index("Context index 2").await.unwrap();

        // Truncation test
        let long_str = "a".repeat(200);
        store.update_lightweight_index(&long_str).await.unwrap();

        let index = <SqliteMemoryStore as LongTermMemory>::get_lightweight_index(&store).await.unwrap();
        assert!(index.contains("Context index 1"));
        assert!(index.contains("Context index 2"));
        assert!(index.contains(&"a".repeat(147)));
        assert!(index.contains("..."));

        // 2) Test detailed topic files
        store.upsert_topic("my_topic", "Detailed knowledge about my_topic").await.unwrap();
        // Test update
        store.upsert_topic("my_topic", "Updated detailed knowledge about my_topic").await.unwrap();

        let topic_content = <SqliteMemoryStore as LongTermMemory>::retrieve_topic(&store, "my_topic").await.unwrap();
        assert_eq!(topic_content, "Updated detailed knowledge about my_topic");

        let missing_topic = <SqliteMemoryStore as LongTermMemory>::retrieve_topic(&store, "missing_topic").await;
        assert!(missing_topic.is_err());

        // 3) Test raw transcripts via FTS5
        store.store_transcript("session_1", "What is the secret?", "The secret code is 42").await.unwrap();
        store.store_transcript("session_2", "Who are you?", "I am an AI agent").await.unwrap();

        let results = <SqliteMemoryStore as LongTermMemory>::search_transcripts(&store, "secret", 10).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "The secret code is 42");

        let results_agent = <SqliteMemoryStore as LongTermMemory>::search_transcripts(&store, "agent", 10).await.unwrap();
        assert_eq!(results_agent.len(), 1);
        assert_eq!(results_agent[0], "I am an AI agent");

        // 4) Test MemoryAccessor trait implementation
        let accessor_opt = <SqliteMemoryStore as LongTermMemory>::as_anthropic_accessor(&store);
        assert!(accessor_opt.is_some());
        let accessor = accessor_opt.unwrap();

        let acc_topic = accessor.retrieve_topic("my_topic").await.unwrap();
        assert_eq!(acc_topic, "Updated detailed knowledge about my_topic");

        let acc_search = accessor.search_transcripts("secret", 10).await.unwrap();
        assert_eq!(acc_search.len(), 1);
        assert_eq!(acc_search[0], "The secret code is 42");
    }
}
