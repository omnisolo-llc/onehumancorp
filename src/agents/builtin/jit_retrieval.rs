/// Master Catalog B.4. Context Management (Preventing Context Rot)
/// Just-in-Time (JIT) Retrieval Mechanic
/// "Never load full files. Implement tools that act like grep, glob, head, and tail."
/// And dynamically pull relevant past sessions, tool docs, or code snippets before LLM calls.

use crate::memory_store::LongTermMemory;
use ohc_builtin_agent_core::types::Message;
use std::sync::Arc;

pub struct JitContextRetriever {
    memory_store: Arc<dyn LongTermMemory>,
    session_id: String,
}

impl JitContextRetriever {
    pub fn new(memory_store: Arc<dyn LongTermMemory>, session_id: String) -> Self {
        Self {
            memory_store,
            session_id,
        }
    }

    /// Analyzes the recent conversation history to extract keywords and pulls relevant
    /// JIT context from the FTS5-backed LongTermMemory.
    pub async fn retrieve_context(&self, messages: &[Message]) -> Option<String> {
        // Simple keyword extraction heuristic: take the last user message
        let last_user_msg = messages.iter().rev().find(|m| m.role == ohc_builtin_agent_core::types::Role::User)?;

        let content = &last_user_msg.content;

        // Skip short messages
        if content.len() < 10 {
            return None;
        }

        // Basic keyword extraction: remove common stop words and keep long words
        let stop_words = ["the", "and", "a", "an", "is", "in", "to", "of", "for", "with", "on", "this", "that", "it", "as", "at", "by", "be", "this", "which", "or", "from", "but", "not", "are", "was"];

        let mut keywords: Vec<String> = content
            .split_whitespace()
            .map(|s| s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>())
            .filter(|s| !s.is_empty() && s.len() > 3 && !stop_words.contains(&s.as_str()))
            .collect();

        keywords.sort();
        keywords.dedup();

        if keywords.is_empty() {
            return None;
        }

        // Take top 3 keywords
        let query = keywords.into_iter().take(3).collect::<Vec<_>>().join(" OR ");

        // 1. Try to search general memory
        let mut combined_context = String::new();

        if let Ok(results) = self.memory_store.retrieve(&query, 3).await
            && !results.is_empty() {
                combined_context.push_str("Relevant General Knowledge (JIT Retrieval):\n");
                for res in results {
                    combined_context.push_str(&res);
                    combined_context.push_str("\n---\n");
                }
            }

        // 2. Try to search past session messages
        if let Ok(results) = self.memory_store.search_session_messages(&self.session_id, &query, 3, false).await
            && !results.is_empty() {
                if !combined_context.is_empty() {
                    combined_context.push('\n');
                }
                combined_context.push_str("Relevant Past Session Context (JIT Retrieval):\n");
                for res in results {
                    combined_context.push_str(&res);
                    combined_context.push_str("\n---\n");
                }
            }

        if combined_context.is_empty() {
            None
        } else {
            Some(format!("[System: The following Just-In-Time (JIT) context was automatically retrieved based on your recent message to help you.]\n{}", combined_context.trim_end_matches("\n---\n")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    #[derive(Debug)]
    struct MockMemoryStore;

    #[async_trait]
    impl LongTermMemory for MockMemoryStore {
        async fn retrieve(&self, query: &str, _limit: usize) -> Result<Vec<String>, String> {
            if query.contains("rust") {
                Ok(vec!["Rust is a systems programming language.".to_string()])
            } else {
                Ok(vec![])
            }
        }

        async fn store(&self, _content: &str, _tags: Vec<String>) -> Result<(), String> {
            Ok(())
        }

        async fn search_session_messages(&self, _session_id: &str, query: &str, _limit: usize, _summarize: bool) -> Result<Vec<String>, String> {
             if query.contains("error") {
                Ok(vec!["User previously encountered a compiler error in main.rs".to_string()])
            } else {
                Ok(vec![])
            }
        }
    }

    #[tokio::test]
    async fn test_jit_retrieval_general() {
        let store = Arc::new(MockMemoryStore);
        let retriever = JitContextRetriever::new(store, "test_session".to_string());

        let messages = vec![Message::user("How do I write a fast server in Rust?")];

        let context = retriever.retrieve_context(&messages).await.unwrap();
        assert!(context.contains("Relevant General Knowledge (JIT Retrieval)"));
        assert!(context.contains("Rust is a systems programming language."));
    }

    #[tokio::test]
    async fn test_jit_retrieval_session() {
        let store = Arc::new(MockMemoryStore);
        let retriever = JitContextRetriever::new(store, "test_session".to_string());

        let messages = vec![Message::user("I got an error when compiling")];

        let context = retriever.retrieve_context(&messages).await.unwrap();
        assert!(context.contains("Relevant Past Session Context (JIT Retrieval)"));
        assert!(context.contains("User previously encountered a compiler error in main.rs"));
    }

    #[tokio::test]
    async fn test_jit_retrieval_empty() {
        let store = Arc::new(MockMemoryStore);
        let retriever = JitContextRetriever::new(store, "test_session".to_string());

        let messages = vec![Message::user("Hello")]; // Too short / no keywords

        let context = retriever.retrieve_context(&messages).await;
        assert!(context.is_none());
    }
}
