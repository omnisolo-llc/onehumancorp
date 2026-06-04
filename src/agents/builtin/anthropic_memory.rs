use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::memory_store::LongTermMemory;
use crate::tools::anthropic_memory::MemoryAccessor;

/// Anthropic 3-Tier Memory Architecture
/// 1) Lightweight index (~150 chars/entry, *always* loaded in context)
/// 2) Detailed topic files (pulled on demand)
/// 3) Raw transcripts (accessed via search only)

#[derive(Debug)]
pub struct Anthropic3TierMemory {
    pub tenant_id: String,

    // Tier 1: Always in context
    // Lightweight index of topic names to short summaries
    pub lightweight_index: RwLock<HashMap<String, String>>,

    // Tier 2: Pulled on demand
    pub topic_files: RwLock<HashMap<String, String>>,

    // Tier 3: Transcripts, accessed via search (mocked via simple storage here, could use SQLite FTS5)
    pub transcripts: RwLock<Vec<String>>,
}

impl Anthropic3TierMemory {
    pub fn new(tenant_id: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_string(),
            lightweight_index: RwLock::new(HashMap::new()),
            topic_files: RwLock::new(HashMap::new()),
            transcripts: RwLock::new(Vec::new()),
        }
    }

    pub async fn add_transcript(&self, content: &str) {
        let mut transcripts = self.transcripts.write().await;
        transcripts.push(content.to_string());
    }

    pub async fn update_topic(&self, topic: &str, summary: &str, detailed_content: &str) {
        {
            let mut index = self.lightweight_index.write().await;

            // Limit summary to ~150 chars for the lightweight index
            let truncated_summary = if summary.len() > 150 {
                let mut s = summary.chars().take(147).collect::<String>();
                s.push_str("...");
                s
            } else {
                summary.to_string()
            };
            index.insert(topic.to_string(), truncated_summary);
        }

        {
            let mut files = self.topic_files.write().await;
            files.insert(topic.to_string(), detailed_content.to_string());
        }
    }

    pub async fn get_lightweight_context(&self) -> String {
        let index = self.lightweight_index.read().await;
        if index.is_empty() {
            return String::new();
        }

        let mut out = String::from("Memory Hints (Use TopicRetrieve to read full details):\n");
        for (topic, summary) in index.iter() {
            out.push_str(&format!("- [{}] {}\n", topic, summary));
        }
        out
    }
}

#[async_trait::async_trait]
impl MemoryAccessor for Anthropic3TierMemory {
    async fn retrieve_topic(&self, topic_name: &str) -> Result<String, String> {
        let files = self.topic_files.read().await;
        files.get(topic_name)
            .cloned()
            .ok_or_else(|| format!("Topic '{}' not found", topic_name))
    }

    async fn search_transcripts(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let transcripts = self.transcripts.read().await;
        let query_lower = query.to_lowercase();

        let mut results = Vec::new();
        for transcript in transcripts.iter() {
            if transcript.to_lowercase().contains(&query_lower) {
                results.push(transcript.clone());
                if results.len() >= limit {
                    break;
                }
            }
        }

        Ok(results)
    }
}

// Ensure it implements LongTermMemory so it can be plugged into the Agent
#[async_trait::async_trait]
impl LongTermMemory for Anthropic3TierMemory {
    async fn store(&self, content: &str, _tags: Vec<String>) -> Result<(), String> {
        self.add_transcript(content).await;
        Ok(())
    }

    async fn retrieve(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        MemoryAccessor::search_transcripts(self, query, limit).await
    }

    fn as_anthropic_accessor(&self) -> Option<std::sync::Arc<dyn crate::tools::anthropic_memory::MemoryAccessor>> {
        None // We'll manage this manually in tests or need an Arc of self. We can bypass this by instantiating directly.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_anthropic_3_tier_memory() {
        let mem = Anthropic3TierMemory::new("org_1");

        mem.update_topic("ProjectX", "Important project details.", "Here is the full 10 page document about ProjectX...").await;
        mem.add_transcript("User asked about ProjectX yesterday.").await;

        let index_context = mem.get_lightweight_context().await;
        assert!(index_context.contains("[ProjectX] Important project details."));

        // Topic retrieval
        let topic_content = MemoryAccessor::retrieve_topic(&mem, "ProjectX").await.unwrap();
        assert_eq!(topic_content, "Here is the full 10 page document about ProjectX...");

        // Transcript search
        let search_res = MemoryAccessor::search_transcripts(&mem, "yesterday", 5).await.unwrap();
        assert_eq!(search_res.len(), 1);
        assert_eq!(search_res[0], "User asked about ProjectX yesterday.");
    }
}
