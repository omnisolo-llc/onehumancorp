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
    pub fn new_sync<P: AsRef<Path>>(base_dir: P) -> std::io::Result<Self> {
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

    /// Overwrites the entire lightweight index.
    pub async fn update_index(&self, content: &str) -> std::io::Result<()> {
        fs::write(&self.index_file, content).await
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
    pub async fn search_transcripts(
        &self,
        query: &str,
        limit: usize,
    ) -> std::io::Result<Vec<String>> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();

        let mut entries = fs::read_dir(&self.transcripts_dir).await?;
        while let Ok(Some(entry)) = entries.next_entry().await {
            if results.len() >= limit {
                break;
            }
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

    /// Performs a substring search across all transcripts, returning context-aware snippets
    /// with matching lines and a custom number of padding lines around the match.
    /// This prevents context rot and token explosion by loading only relevant snippets.
    pub async fn search_transcripts_with_snippets(
        &self,
        query: &str,
        limit: usize,
        context_lines: usize,
    ) -> std::io::Result<Vec<String>> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();

        let mut entries = fs::read_dir(&self.transcripts_dir).await?;
        while let Ok(Some(entry)) = entries.next_entry().await {
            if results.len() >= limit {
                break;
            }
            let path = entry.path();
            if path.is_file() {
                if let Ok(content) = fs::read_to_string(&path).await {
                    if content.to_lowercase().contains(&query_lower) {
                        let filename = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                        let lines: Vec<&str> = content.lines().collect();
                        let mut snippets = Vec::new();

                        for (idx, line) in lines.iter().enumerate() {
                            if line.to_lowercase().contains(&query_lower) {
                                let start = idx.saturating_sub(context_lines);
                                let end = std::cmp::min(lines.len(), idx + context_lines + 1);

                                let mut snippet_lines = Vec::new();
                                for i in start..end {
                                    let indicator = if i == idx { ">>> " } else { "    " };
                                    snippet_lines.push(format!("{}{:4}: {}", indicator, i + 1, lines[i]));
                                }
                                snippets.push(snippet_lines.join("\n"));
                            }
                        }

                        if !snippets.is_empty() {
                            results.push(format!(
                                "Transcript {} (Matches: {}):\n---\n{}\n---",
                                filename,
                                snippets.len(),
                                snippets.join("\n\n[...]\n\n")
                            ));
                        }
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
        let results = memory.search_transcripts("weather", 5).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("session2.log"));
        assert!(results[0].contains("What's the weather?"));

        // Search match across sessions (if applicable, though here only session1 has "hello")
        let results2 = memory.search_transcripts("hello", 5).await.unwrap();
        assert_eq!(results2.len(), 1);
        assert!(results2[0].contains("session1.log"));

        // Search no match
        let results3 = memory.search_transcripts("bananas", 5).await.unwrap();
        assert_eq!(results3.len(), 0);
    }

    #[tokio::test]
    async fn test_search_transcripts_with_snippets() {
        let dir = tempdir().unwrap();
        let memory = Anthropic3TierMemory::new(dir.path()).await.unwrap();

        let log_content = "Line 1: preamble\nLine 2: setup\nLine 3: target query match here\nLine 4: cleanup\nLine 5: postamble";
        memory
            .append_transcript("session_snippets", log_content)
            .await
            .unwrap();

        // Search with 1 line of context padding
        let results = memory
            .search_transcripts_with_snippets("target query", 5, 1)
            .await
            .unwrap();

        assert_eq!(results.len(), 1);
        let snippet = &results[0];
        assert!(snippet.contains("session_snippets.log"));
        assert!(snippet.contains("Matches: 1"));

        // Line 2, 3, 4 should be present
        assert!(snippet.contains("Line 2: setup"));
        assert!(snippet.contains(">>>    3: Line 3: target query match here"));
        assert!(snippet.contains("    4: Line 4: cleanup"));

        // Line 1 and 5 should NOT be present (outside 1-line context window)
        assert!(!snippet.contains("Line 1: preamble"));
        assert!(!snippet.contains("Line 5: postamble"));
    }

    #[tokio::test]
    async fn test_snippet_extraction_boundaries() {
        let dir = tempdir().unwrap();
        let memory = Anthropic3TierMemory::new(dir.path()).await.unwrap();

        let log_content = "target query on first line\nLine 2\nLine 3\nLine 4\ntarget query on last line";
        memory
            .append_transcript("session_boundaries", log_content)
            .await
            .unwrap();

        // Search with 1 line of context padding
        let results = memory
            .search_transcripts_with_snippets("target query", 5, 1)
            .await
            .unwrap();

        assert_eq!(results.len(), 1);
        let snippet = &results[0];
        assert!(snippet.contains("Matches: 2"));

        // First match boundary (index 0 saturating_sub 1 -> 0, idx+2 -> lines 0 and 1)
        assert!(snippet.contains(">>>    1: target query on first line"));
        assert!(snippet.contains("    2: Line 2"));
        assert!(!snippet.contains("Line 3"));

        // Second match boundary (index 4 saturating_sub 1 -> 3, idx+2 -> lines 3 and 4)
        assert!(snippet.contains("    4: Line 4"));
        assert!(snippet.contains(">>>    5: target query on last line"));
    }
}
