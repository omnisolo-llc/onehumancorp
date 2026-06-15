use std::path::{Path, PathBuf};
use tokio::fs;

/// Anthropic 3-Tier Memory
/// 1) Lightweight index (~150 chars/entry, always loaded in context)
/// 2) Detailed topic files (pulled on demand)
/// 3) Raw transcripts (accessed via search only)
#[derive(Clone, Debug)]
pub struct Anthropic3TierMemoryStore {
    _base_dir: PathBuf,
    index_file: PathBuf,
    topics_dir: PathBuf,
    transcripts_dir: PathBuf,
}

impl Anthropic3TierMemoryStore {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> std::io::Result<Self> {
        let base_dir = base_dir.as_ref().to_path_buf();
        let index_file = base_dir.join("index.md");
        let topics_dir = base_dir.join("topics");
        let transcripts_dir = base_dir.join("transcripts");

        std::fs::create_dir_all(&base_dir)?;
        std::fs::create_dir_all(&topics_dir)?;
        std::fs::create_dir_all(&transcripts_dir)?;

        Ok(Self {
            _base_dir: base_dir,
            index_file,
            topics_dir,
            transcripts_dir,
        })
    }

    /// Appends a new entry to the lightweight index.

    pub async fn update_index(&self, content: &str) -> std::io::Result<()> {
        tokio::fs::write(&self.index_file, content).await
    }

    pub async fn get_lightweight_index(&self) -> Result<String, String> {
        if self.index_file.exists() {
            tokio::fs::read_to_string(&self.index_file)
                .await
                .map_err(|e| e.to_string())
        } else {
            Ok(String::new())
        }
    }

    pub async fn append_to_index(&self, summary: &str, tags: &[String]) -> std::io::Result<()> {
        let char_count = summary.chars().count();
        let truncated_summary = if char_count > 150 {
            let truncated: String = summary.chars().take(147).collect();
            format!("{}...", truncated)
        } else {
            summary.to_string()
        };

        let tags_str = if tags.is_empty() {
            String::new()
        } else {
            format!(" [{}]", tags.join(", "))
        };
        let entry = format!("- {}{}\n", truncated_summary.replace('\n', " "), tags_str);

        let mut current_index = if self.index_file.exists() {
            fs::read_to_string(&self.index_file).await?
        } else {
            String::new()
        };
        current_index.push_str(&entry);

        fs::write(&self.index_file, current_index).await
    }

    /// Reads the entire lightweight index.
    pub async fn read_index(&self) -> std::io::Result<String> {
        if self.index_file.exists() {
            fs::read_to_string(&self.index_file).await
        } else {
            Ok(String::new())
        }
    }

    /// Writes a detailed topic file.
    pub async fn write_topic(&self, topic_name: &str, content: &str) -> std::io::Result<()> {
        let safe_name =
            topic_name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "");
        let path = self.topics_dir.join(format!("{}.md", safe_name));
        fs::write(path, content).await
    }

    /// Reads a detailed topic file.
    pub async fn read_topic(&self, topic_name: &str) -> std::io::Result<Option<String>> {
        let safe_name =
            topic_name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "");
        let path = self.topics_dir.join(format!("{}.md", safe_name));
        if path.exists() {
            Ok(Some(fs::read_to_string(path).await?))
        } else {
            Ok(None)
        }
    }

    /// Appends a turn to a raw transcript file for a specific session.
    pub async fn append_transcript(
        &self,
        session_id: &str,
        turn_content: &str,
    ) -> std::io::Result<()> {
        let path = self.transcripts_dir.join(format!("{}.log", session_id));
        let mut content = if path.exists() {
            fs::read_to_string(&path).await?
        } else {
            String::new()
        };
        content.push_str(turn_content);
        content.push('\n');
        fs::write(path, content).await
    }

    /// Performs a simple substring search across all transcripts.
    pub async fn search_transcripts(&self, query: &str) -> std::io::Result<Vec<String>> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();

        let mut entries = fs::read_dir(&self.transcripts_dir).await?;
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if path.is_file()
                && let Ok(content) = fs::read_to_string(&path).await
                && content.to_lowercase().contains(&query_lower)
            {
                let filename = path.file_name().unwrap_or_default().to_string_lossy();
                results.push(format!("Transcript {}:\n{}", filename, content));
            }
        }
        Ok(results)
    }
}



#[async_trait::async_trait]
impl crate::tools::anthropic_memory::MemoryAccessor for Anthropic3TierMemoryStore {
    async fn write_topic(&self, topic_name: &str, content: &str) -> Result<(), String> {
        let safe_name =
            topic_name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "");
        let path = self.topics_dir.join(format!("{}.md", safe_name));
        tokio::fs::write(&path, content)
            .await
            .map_err(|e| e.to_string())?;

        let mut existing_index = self.get_lightweight_index().await?;
        let char_count = content.chars().count();
        let truncated_content = if char_count > 150 {
            let truncated: String = content.chars().take(147).collect();
            format!("{}...", truncated)
        } else {
            content.to_string()
        };
        let new_entry = format!(
            "- {}: {}\n",
            safe_name,
            truncated_content.replace(char::from(10), " ")
        );
        if !existing_index.contains(&safe_name) {
            existing_index.push_str(&new_entry);
            self.update_index(&existing_index).await.map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    async fn retrieve_topic(&self, topic_name: &str) -> Result<String, String> {
        let safe_name =
            topic_name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "");
        let path = self.topics_dir.join(format!("{}.md", safe_name));
        if path.exists() {
            tokio::fs::read_to_string(&path)
                .await
                .map_err(|e| e.to_string())
        } else {
            Err(format!("Topic '{}' not found", safe_name))
        }
    }

    async fn search_transcripts(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        <Anthropic3TierMemoryStore as crate::memory_store::LongTermMemory>::search_transcripts(
            self, query, limit,
        )
        .await
    }
}

#[async_trait::async_trait]
impl crate::memory_store::LongTermMemory for Anthropic3TierMemoryStore {
    async fn store_session_message(
        &self,
        session_id: &str,
        role: &str,
        content: &str,
    ) -> Result<(), String> {
        let turn = format!("{}: {}", role, content);
        self.append_transcript(session_id, &turn).await.map_err(|e| e.to_string())
    }

    async fn search_session_messages(
        &self,
        _session_id: &str,
        query: &str,
        limit: usize,
        _summarize: bool,
    ) -> Result<Vec<String>, String> {
        crate::tools::anthropic_memory::MemoryAccessor::search_transcripts(self, query, limit).await
    }

    async fn retrieve(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let mut results = Vec::new();

        if !self.topics_dir.exists() {
            return Ok(results);
        }

        let mut dir = tokio::fs::read_dir(&self.topics_dir)
            .await
            .map_err(|e| e.to_string())?;
        while let Ok(Some(entry)) = dir.next_entry().await {
            let content = tokio::fs::read_to_string(entry.path())
                .await
                .map_err(|e| e.to_string())?;
            if content.to_lowercase().contains(&query.to_lowercase()) {
                results.push(content);
                if results.len() >= limit {
                    break;
                }
            }
        }

        Ok(results)
    }

    async fn store(&self, content: &str, tags: Vec<String>) -> Result<(), String> {
        let mut existing_index = self.get_lightweight_index().await?;

        let char_count2 = content.chars().count();
        let truncated_content = if char_count2 > 150 {
            {
                let truncated: String = content.chars().take(147).collect();
                format!("{}...", truncated)
            }
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
        self.update_index(&existing_index).await.map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_lightweight_index(&self) -> Result<String, String> {
        Anthropic3TierMemoryStore::get_lightweight_index(self).await
    }

    async fn retrieve_topic(&self, topic_name: &str) -> Result<String, String> {
        let safe_name =
            topic_name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "");
        let path = self.topics_dir.join(format!("{}.md", safe_name));
        if path.exists() {
            tokio::fs::read_to_string(&path)
                .await
                .map_err(|e| e.to_string())
        } else {
            Err(format!("Topic '{}' not found", safe_name))
        }
    }

    async fn search_transcripts(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();
        let mut dir = tokio::fs::read_dir(&self.transcripts_dir)
            .await
            .map_err(|e| e.to_string())?;

        'outer: while let Ok(Some(entry)) = dir.next_entry().await {
            let content = tokio::fs::read_to_string(entry.path())
                .await
                .map_err(|e| e.to_string())?;

            let filename = entry.file_name().to_string_lossy().to_string();

            // Chunk by paragraphs (double newline) to avoid loading huge files
            for chunk in content.split("\n\n") {
                if chunk.to_lowercase().contains(&query_lower) {
                    results.push(format!("Transcript {} snippet:\n{}", filename, chunk.trim()));
                    if results.len() >= limit {
                        break 'outer;
                    }
                }
            }
        }
        Ok(results)
    }

    fn as_anthropic_accessor(
        &self,
    ) -> Option<std::sync::Arc<dyn crate::tools::anthropic_memory::MemoryAccessor>> {
        Some(std::sync::Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_lightweight_index() {
        let dir = tempdir().unwrap();
        let memory = Anthropic3TierMemoryStore::new(dir.path()).unwrap();

        let summary = "This is a very long summary that should exceed one hundred and fifty characters so we can test the truncation logic properly. Let's add some more text just to be absolutely sure it goes over the limit.";
        memory
            .append_to_index(summary, &["tag1".to_string(), "tag2".to_string()])
            .await
            .unwrap();

        let index = memory.read_index().await.unwrap();
        assert!(index.contains("... [tag1, tag2]\n"));
        assert!(index.len() < 200);

        // Test appending short summary
        memory.append_to_index("Short summary", &[]).await.unwrap();
        let index2 = memory.read_index().await.unwrap();
        assert!(index2.contains("- Short summary\n"));
    }

    #[tokio::test]
    async fn test_topics() {
        let dir = tempdir().unwrap();
        let memory = Anthropic3TierMemoryStore::new(dir.path()).unwrap();

        // Write
        memory
            .write_topic("Database Architecture", "PostgreSQL is used.")
            .await
            .unwrap();

        // Read existing
        let content = memory.read_topic("Database Architecture").await.unwrap();
        assert_eq!(content.unwrap(), "PostgreSQL is used.");

        // Read non-existing
        let none = memory.read_topic("Missing Topic").await.unwrap();
        assert!(none.is_none());
    }

    #[tokio::test]
    async fn test_transcripts() {
        let dir = tempdir().unwrap();
        let memory = Anthropic3TierMemoryStore::new(dir.path()).unwrap();

        memory
            .append_transcript("session1", "User: Hello")
            .await
            .unwrap();
        memory
            .append_transcript("session1", "Assistant: Hi there!")
            .await
            .unwrap();
        memory
            .append_transcript("session2", "User: What's the weather?")
            .await
            .unwrap();

        // Search match
        let results = memory.search_transcripts("weather").await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("session2.log"));
        assert!(results[0].contains("What's the weather?"));

        // Search match across sessions (if applicable, though here only session1 has "hello")
        let results2 = memory.search_transcripts("hello").await.unwrap();
        assert_eq!(results2.len(), 1);
        assert!(results2[0].contains("session1.log"));

        // Search no match
        let results3 = memory.search_transcripts("bananas").await.unwrap();
        assert_eq!(results3.len(), 0);
    }

    #[tokio::test]
    async fn test_anthropic_3_tier_memory_flow() {
        let dir = tempfile::tempdir().unwrap();
        let memory = Anthropic3TierMemoryStore::new(dir.path()).unwrap();

        // 1. Write an explicit topic
        crate::tools::anthropic_memory::MemoryAccessor::write_topic(
            &memory,
            "architecture",
            "The system uses a 3-tier architecture with Rust and React.",
        )
        .await
        .unwrap();

        // 2. Add long-term memory (stores index + files)
        <Anthropic3TierMemoryStore as crate::memory_store::LongTermMemory>::store(
            &memory,
            "User prefers Postgres over MySQL.",
            vec!["db".to_string()],
        )
        .await
        .unwrap();

        // 3. Store a conversation turn
        <Anthropic3TierMemoryStore as crate::memory_store::LongTermMemory>::store_session_message(
            &memory,
            "sess-123",
            "User",
            "How do I scale the DB?",
        )
        .await
        .unwrap();

        // Check index
        let index = <Anthropic3TierMemoryStore as crate::memory_store::LongTermMemory>::get_lightweight_index(&memory)
            .await
            .unwrap();
        assert!(index.contains("architecture: The system uses a 3-tier"));
        assert!(index.contains("User prefers Postgres"));

        // Check retrieval
        let retrieved = <Anthropic3TierMemoryStore as crate::memory_store::LongTermMemory>::retrieve(&memory, "architecture", 5)
            .await
            .unwrap();
        assert!(!retrieved.is_empty());
        assert!(retrieved[0].contains("3-tier architecture"));

        // Search transcripts
        let trans = <Anthropic3TierMemoryStore as crate::memory_store::LongTermMemory>::search_session_messages(&memory, "sess-123", "scale", 5, false)
            .await
            .unwrap();
        assert!(!trans.is_empty());
        assert!(trans[0].contains("scale the DB"));
    }
}
