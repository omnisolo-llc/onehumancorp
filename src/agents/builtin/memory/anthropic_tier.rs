use std::path::{Path, PathBuf};
use tokio::fs;

/// Anthropic 3-Tier Memory
/// 1) Lightweight index (~150 chars/entry, always loaded in context)
/// 2) Detailed topic files (pulled on demand)
/// 3) Raw transcripts (accessed via search only)
#[derive(Clone, Debug)]
pub struct Anthropic3TierMemory {
    _base_dir: PathBuf,
    index_file: PathBuf,
    topics_dir: PathBuf,
    transcripts_dir: PathBuf,
}

impl Anthropic3TierMemory {
    pub async fn new<P: AsRef<Path>>(base_dir: P) -> std::io::Result<Self> {
        let base_dir = base_dir.as_ref().to_path_buf();
        let index_file = base_dir.join("index.md");
        let topics_dir = base_dir.join("topics");
        let transcripts_dir = base_dir.join("transcripts");

        fs::create_dir_all(&base_dir).await?;
        fs::create_dir_all(&topics_dir).await?;
        fs::create_dir_all(&transcripts_dir).await?;

        Ok(Self {
             _base_dir: base_dir,
            index_file,
            topics_dir,
            transcripts_dir,
        })
    }

    /// Appends a new entry to the lightweight index.
    pub async fn append_to_index(&self, summary: &str, tags: &[String]) -> std::io::Result<()> {
        let truncated_summary = if summary.len() > 150 {
            format!("{}...", &summary[..147])
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
            if path.is_file() {
                if let Ok(content) = fs::read_to_string(&path).await {
                    if content.to_lowercase().contains(&query_lower) {
                        let filename = path.file_name().unwrap_or_default().to_string_lossy();
                        results.push(format!("Transcript {}:\n{}", filename, content));
                    }
                }
            }
        }
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_lightweight_index() {
        let dir = tempdir().unwrap();
        let memory = Anthropic3TierMemory::new(dir.path()).await.unwrap();

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
        let memory = Anthropic3TierMemory::new(dir.path()).await.unwrap();

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
        let memory = Anthropic3TierMemory::new(dir.path()).await.unwrap();

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
}
