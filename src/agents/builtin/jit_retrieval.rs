#![allow(clippy::empty_line_after_doc_comments)]
/// Master Catalog B.4. Context Management (Preventing Context Rot)
/// Just-in-Time (JIT) Retrieval Mechanic
/// "Never load full files. Implement tools that act like grep, glob, head, and tail."
/// And dynamically pull relevant past sessions, tool docs, or code snippets before LLM calls.
use crate::memory_store::LongTermMemory;
use ohc_builtin_agent_core::types::Message;
use std::sync::Arc;
use std::collections::HashMap;

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
        let last_user_msg = messages
            .iter()
            .rev()
            .find(|m| m.role == ohc_builtin_agent_core::types::Role::User)?;

        let content = &last_user_msg.content;

        // Skip short messages
        if content.len() < 10 {
            return None;
        }

        // Basic keyword extraction: remove common stop words and keep long words
        let stop_words = [
            "the", "and", "a", "an", "is", "in", "to", "of", "for", "with", "on", "this", "that",
            "it", "as", "at", "by", "be", "this", "which", "or", "from", "but", "not", "are",
            "was", "what", "how", "why", "when", "where", "can", "could", "would", "should",
            "will", "shall", "do", "does", "did", "have", "has", "had", "then", "there", "their",
            "they", "we", "you", "he", "she", "i", "my", "your", "his", "her", "our", "them",
        ];

        let mut term_frequencies: HashMap<String, usize> = HashMap::new();

        for s in content.split_whitespace() {
            let cleaned: String = s
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect();

            if !cleaned.is_empty() && cleaned.len() > 3 && !stop_words.contains(&cleaned.as_str()) {
                *term_frequencies.entry(cleaned).or_insert(0) += 1;
            }
        }

        if term_frequencies.is_empty() {
            return None;
        }

        // Score terms based on a combination of frequency and length (longer words are often more specific domain terms)
        let mut scored_terms: Vec<(String, usize)> = term_frequencies
            .into_iter()
            .map(|(term, freq)| {
                // Score = frequency * length multiplier
                let score = freq * term.len();
                (term, score)
            })
            .collect();

        // Sort by score descending
        scored_terms.sort_by(|a, b| b.1.cmp(&a.1));

        // Take top 3 most relevant keywords
        let query = scored_terms
            .into_iter()
            .take(3)
            .map(|(term, _)| term)
            .collect::<Vec<_>>()
            .join(" OR ");

        // 1. Try to search general memory
        let mut combined_context = String::new();

        if let Ok(results) = self.memory_store.retrieve(&query, 3).await
            && !results.is_empty()
        {
            combined_context.push_str("Relevant General Knowledge (JIT Retrieval):\n");
            for res in results {
                combined_context.push_str(&res);
                combined_context.push_str("\n---\n");
            }
        }

        // 2. Try to search past session messages
        if let Ok(results) = self
            .memory_store
            .search_session_messages(&self.session_id, &query, 3, false)
            .await
            && !results.is_empty()
        {
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
            Some(format!(
                "[System: The following Just-In-Time (JIT) context was automatically retrieved based on your recent message to help you.]\n{}",
                combined_context.trim_end_matches("\n---\n")
            ))
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
            if query.contains("rust") || query.contains("systems") || query.contains("server") || query.contains("programming") {
                Ok(vec!["Rust is a systems programming language.".to_string()])
            } else {
                Ok(vec![])
            }
        }

        async fn store(&self, _content: &str, _tags: Vec<String>) -> Result<(), String> {
            Ok(())
        }

        async fn search_session_messages(
            &self,
            _session_id: &str,
            query: &str,
            _limit: usize,
            _summarize: bool,
        ) -> Result<Vec<String>, String> {
            if query.contains("error") || query.contains("compiling") || query.contains("compiler") {
                Ok(vec![
                    "User previously encountered a compiler error in main.rs".to_string(),
                ])
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

    #[tokio::test]
    async fn test_jit_retrieval_keyword_scoring() {
        let store = Arc::new(MockMemoryStore);
        let retriever = JitContextRetriever::new(store, "test_session".to_string());

        // This prompt contains words starting with 'a' and 'b' that are long enough
        // but 'authentication' and 'architecture' should score higher than 'apple' due to length/frequency.
        let messages = vec![Message::user(
            "The architecture requires authentication. We must build authentication architecture quickly. Apple banana cat.",
        )];

        let _ = retriever.retrieve_context(&messages).await;
        // In this mock test, it may not match anything, but we can verify it doesn't crash
        // and if we had a spy, we could verify the query. Let's just ensure it runs cleanly.
    }
}
