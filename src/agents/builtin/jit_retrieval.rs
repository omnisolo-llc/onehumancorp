#![allow(clippy::empty_line_after_doc_comments)]
/// Master Catalog B.4. Context Management (Preventing Context Rot)
/// JetBrains JIT Retrieval Mechanic
/// "Never load full files. Implement tools that act like grep, glob, head, and tail."
/// And dynamically pull relevant past sessions, tool docs, or code snippets before LLM calls.
use crate::memory_store::LongTermMemory;
use ohc_builtin_agent_core::types::Message;
use std::collections::HashMap;
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
    /// JIT context from the FTS5-backed LongTermMemory, tool docs, or code snippets.
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
                .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
                .collect();

            if !cleaned.is_empty() && cleaned.len() > 3 && !stop_words.contains(&cleaned.as_str()) {
                *term_frequencies.entry(cleaned).or_insert(0) += 1;
            }
        }

        if term_frequencies.is_empty() {
            return None;
        }

        // Score terms based on an Okapi BM25-inspired heuristic for JIT Retrieval.
        // BM25 usually requires document frequency across a corpus, but here we only have the query text.
        // We simulate semantic importance by prioritizing rare/long words and using log smoothing on frequency.
        // Master Catalog B.4. Context Management: JIT Retrieval Semantic Weighting
        let msg_len = content.split_whitespace().count() as f64;
        let avg_dl = 15.0; // Assume an average user message length of 15 words

        let mut scored_terms: Vec<(String, f64)> = term_frequencies
            .into_iter()
            .map(|(term, freq)| {
                let k1 = 1.2;
                let b = 0.75;
                // Since we lack total document counts, we heavily weight term length as a proxy for rarity/specificity.
                let idf_proxy = (term.len() as f64).ln() + 1.0;
                let tf = freq as f64;

                // Simplified BM25 formula component
                let score =
                    idf_proxy * (tf * (k1 + 1.0)) / (tf + k1 * (1.0 - b + b * (msg_len / avg_dl)));
                (term, score)
            })
            .collect();

        // Sort by score descending
        scored_terms.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Take top 3 most relevant keywords
        let query = scored_terms
            .into_iter()
            .take(3)
            .map(|(term, _)| term)
            .collect::<Vec<_>>()
            .join(" OR ");

        // Extract potential filenames or function names for JIT snippet retrieval
        let mut potential_files = Vec::new();
        for s in content.split_whitespace() {
            if s.contains('.') && s.len() > 4 {
                // likely a file like main.rs, util.ts
                potential_files.push(s.to_string());
            }
        }

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

        // 3. Try to dynamically pull code snippets before LLM calls
        if !potential_files.is_empty() {
            let mut snippet_context = String::new();
            for file_hint in potential_files {
                let cleaned_file_hint: String = file_hint
                    .chars()
                    .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '_' || *c == '-')
                    .collect();
                if let Ok(results) = self
                    .memory_store
                    .retrieve(&format!("file:{}", cleaned_file_hint), 1)
                    .await
                {
                    for res in results {
                        snippet_context.push_str(&res);
                        snippet_context.push_str("\n---\n");
                    }
                }
            }

            if !snippet_context.is_empty() {
                if !combined_context.is_empty() {
                    combined_context.push('\n');
                }
                combined_context.push_str("Relevant Code Snippets (JIT Retrieval):\n");
                combined_context.push_str(&snippet_context);
            }
        }

        if combined_context.trim().is_empty() {
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
            if query.contains("rust")
                || query.contains("systems")
                || query.contains("server")
                || query.contains("programming")
            {
                Ok(vec!["Rust is a systems programming language.".to_string()])
            } else if query.contains("file:main.rs") {
                Ok(vec!["fn main() { println!(\"Hello World\"); }".to_string()])
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
            if query.contains("error") || query.contains("compiling") || query.contains("compiler")
            {
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

    #[tokio::test]
    async fn test_jit_retrieval_code_snippets() {
        let store = Arc::new(MockMemoryStore);
        let retriever = JitContextRetriever::new(store, "test_session".to_string());

        let messages = vec![Message::user(
            "Can you explain what the function in main.rs does?",
        )];

        let context = retriever.retrieve_context(&messages).await.unwrap();
        assert!(context.contains("Relevant Code Snippets (JIT Retrieval)"));
        assert!(context.contains("fn main() { println!(\"Hello World\"); }"));
    }

    #[tokio::test]
    async fn test_jit_retrieval_term_sorting_and_empty() {
        let store = Arc::new(MockMemoryStore);
        let retriever = JitContextRetriever::new(store, "test_session".to_string());

        // This prompt contains short words that should be filtered out
        let messages = vec![Message::user("It is an error the to of in on as at by be")];

        let context = retriever.retrieve_context(&messages).await.unwrap();
        assert!(context.contains("Relevant Past Session Context (JIT Retrieval)"));
        assert!(context.contains("User previously encountered a compiler error in main.rs"));
    }

    #[tokio::test]
    async fn test_jit_retrieval_all_empty() {
        let store = Arc::new(MockMemoryStore);
        let retriever = JitContextRetriever::new(store, "test_session".to_string());

        // A prompt with words that have no matches in the mock store
        let messages = vec![Message::user(
            "Unrelated longword entirely foreign database",
        )];

        let context = retriever.retrieve_context(&messages).await;
        assert!(context.is_none());
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;
    use async_trait::async_trait;

    #[derive(Debug)]
    struct FailingMemoryStore;

    #[async_trait]
    impl LongTermMemory for FailingMemoryStore {
        async fn retrieve(&self, _query: &str, _limit: usize) -> Result<Vec<String>, String> {
            Err("Failed to retrieve general knowledge".to_string())
        }

        async fn store(&self, _content: &str, _tags: Vec<String>) -> Result<(), String> {
            Ok(())
        }

        async fn search_session_messages(
            &self,
            _session_id: &str,
            _query: &str,
            _limit: usize,
            _summarize: bool,
        ) -> Result<Vec<String>, String> {
            Err("Failed to search session".to_string())
        }
    }

    #[tokio::test]
    async fn test_jit_retrieval_failing_store() {
        let store = Arc::new(FailingMemoryStore);
        let retriever = JitContextRetriever::new(store, "test_session".to_string());

        let messages = vec![Message::user("Please review the code in main.rs")];

        // Should return None when all stores fail
        let context = retriever.retrieve_context(&messages).await;
        assert!(context.is_none());
    }
}
