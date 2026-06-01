use std::sync::Arc;
use crate::memory_store::LongTermMemory;

/// Memory: Implemented across multiple timescales:
/// - Short-term: Conversation history in the active session.
/// - Long-term (OpenAI/LangGraph): Sessions backed by SQLite/Redis, or namespace-organized JSON Stores.
/// - Long-term (Anthropic 3-Tier): 1) Lightweight index (~150 chars/entry, always loaded in context), 2) Detailed topic files (pulled on demand), 3) Raw transcripts (accessed via search only).
/// Crucial rule: Agent must treat memory as a "hint" and verify against actual state before acting.
pub struct MemoryManager {
    pub short_term_history: Vec<crate::types::Message>,
    pub long_term_store: Option<Arc<dyn LongTermMemory>>,
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            short_term_history: Vec::new(),
            long_term_store: None,
        }
    }

    pub fn with_long_term_store(mut self, store: Arc<dyn LongTermMemory>) -> Self {
        self.long_term_store = Some(store);
        self
    }

    pub fn add_short_term(&mut self, message: crate::types::Message) {
        self.short_term_history.push(message);
    }

    pub async fn store_long_term(&self, content: &str, tags: Vec<String>) -> Result<(), String> {
        if let Some(store) = &self.long_term_store {
            store.store(content, tags).await
        } else {
            Ok(())
        }
    }

    pub async fn retrieve_long_term(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        if let Some(store) = &self.long_term_store {
            store.retrieve(query, limit).await
        } else {
            Ok(Vec::new())
        }
    }
}
