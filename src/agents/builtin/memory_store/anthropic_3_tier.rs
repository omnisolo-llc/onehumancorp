use crate::memory_store::LongTermMemory;
/// Anthropic 3-Tier Memory Store implementation
/// 1) Lightweight index (~150 chars/entry, always loaded in context)
/// 2) Detailed topic files (pulled on demand)
/// 3) Raw transcripts (accessed via search only)
#[derive(Clone)]
pub struct Anthropic3TierMemoryStore {
    #[allow(dead_code)]
    base_dir: std::path::PathBuf,
    index_file: std::path::PathBuf,
    topics_dir: std::path::PathBuf,
    transcripts_dir: std::path::PathBuf,
}

impl std::fmt::Debug for Anthropic3TierMemoryStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Anthropic3TierMemoryStore").finish()
    }
}

impl Anthropic3TierMemoryStore {
    pub fn new<P: AsRef<std::path::Path>>(base_dir: P) -> Result<Self, String> {
        let base_dir = base_dir.as_ref().to_path_buf();
        let index_file = base_dir.join("index.md");
        let topics_dir = base_dir.join("topics");
        let transcripts_dir = base_dir.join("transcripts");

        std::fs::create_dir_all(&base_dir).map_err(|e| e.to_string())?;
        std::fs::create_dir_all(&topics_dir).map_err(|e| e.to_string())?;
        std::fs::create_dir_all(&transcripts_dir).map_err(|e| e.to_string())?;

        Ok(Self {
            base_dir,
            index_file,
            topics_dir,
            transcripts_dir,
        })
    }

    pub async fn update_index(&self, content: &str) -> Result<(), String> {
        tokio::fs::write(&self.index_file, content).await.map_err(|e| e.to_string())
    }

    pub async fn write_topic(&self, topic_name: &str, content: &str) -> Result<(), String> {
        let safe_name = topic_name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "");
        let path = self.topics_dir.join(format!("{}.md", safe_name));
        tokio::fs::write(path, content).await.map_err(|e| e.to_string())
    }

    pub async fn append_transcript(&self, session_id: &str, turn_content: &str) -> Result<(), String> {
        let path = self.transcripts_dir.join(format!("{}.log", session_id));
        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::OpenOptions::new().create(true).append(true).open(path).await.map_err(|e| e.to_string())?;
        file.write_all(format!("{}\n\n", turn_content).as_bytes()).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[async_trait::async_trait]
#[async_trait::async_trait]
impl ohc_builtin_agent_tools::anthropic_memory::MemoryAccessor for Anthropic3TierMemoryStore {
    async fn retrieve_topic(&self, topic_name: &str) -> Result<String, String> {
        let safe_name = topic_name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "");
        let path = self.topics_dir.join(format!("{}.md", safe_name));
        if path.exists() {
            tokio::fs::read_to_string(&path).await.map_err(|e| e.to_string())
        } else {
            Err(format!("Topic '{}' not found", safe_name))
        }
    }

    async fn search_transcripts(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let mut results = Vec::new();
        let mut dir = tokio::fs::read_dir(&self.transcripts_dir).await.map_err(|e| e.to_string())?;
        while let Ok(Some(entry)) = dir.next_entry().await {
            let content = tokio::fs::read_to_string(entry.path()).await.map_err(|e| e.to_string())?;
            for par in content.split("\n\n") {
                if par.to_lowercase().contains(&query.to_lowercase()) {
                    results.push(par.to_string());
                    if results.len() >= limit {
                        return Ok(results);
                    }
                }
            }
        }
        Ok(results)
    }
}

#[async_trait::async_trait]
#[async_trait::async_trait]
impl LongTermMemory for Anthropic3TierMemoryStore {
    async fn retrieve(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let mut results = Vec::new();

        if !self.topics_dir.exists() {
            return Ok(results);
        }

        let mut dir = tokio::fs::read_dir(&self.topics_dir).await.map_err(|e| e.to_string())?;
        while let Ok(Some(entry)) = dir.next_entry().await {
            let content = tokio::fs::read_to_string(entry.path()).await.map_err(|e| e.to_string())?;
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

        let truncated_content = if content.len() > 150 {
            format!("{}...", &content[..147])
        } else {
            content.to_string()
        };

        let tags_str = if tags.is_empty() { String::new() } else { format!(" [{}]", tags.join(", ")) };
        let new_entry = format!("- {}{}\n", truncated_content.replace('\n', " "), tags_str);

        existing_index.push_str(&new_entry);
        self.update_index(&existing_index).await?;

        Ok(())
    }

    async fn get_lightweight_index(&self) -> Result<String, String> {
        if self.index_file.exists() {
            tokio::fs::read_to_string(&self.index_file).await.map_err(|e| e.to_string())
        } else {
            Ok(String::new())
        }
    }

    async fn retrieve_topic(&self, topic_name: &str) -> Result<String, String> {
        let safe_name = topic_name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "");
        let path = self.topics_dir.join(format!("{}.md", safe_name));
        if path.exists() {
            tokio::fs::read_to_string(&path).await.map_err(|e| e.to_string())
        } else {
            Err(format!("Topic '{}' not found", safe_name))
        }
    }

    async fn search_transcripts(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let mut results = Vec::new();
        let mut dir = tokio::fs::read_dir(&self.transcripts_dir).await.map_err(|e| e.to_string())?;
        while let Ok(Some(entry)) = dir.next_entry().await {
            let content = tokio::fs::read_to_string(entry.path()).await.map_err(|e| e.to_string())?;
            for par in content.split("\n\n") {
                if par.to_lowercase().contains(&query.to_lowercase()) {
                    results.push(par.to_string());
                    if results.len() >= limit {
                        return Ok(results);
                    }
                }
            }
        }
        Ok(results)
    }
    fn as_anthropic_accessor(&self) -> Option<std::sync::Arc<dyn ohc_builtin_agent_tools::anthropic_memory::MemoryAccessor>> {
        Some(std::sync::Arc::new(self.clone()))
    }
}
