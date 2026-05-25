use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message};
use ohc_builtin_agent_llm::LlmClient;
use std::sync::Arc;
use std::collections::HashMap;

/// Ruflo Unique Harness Innovations: SONA neural patterns
/// Self-learning trajectory patterns for repeated tasks.
///
/// SONA (Self-Organizing Neural Architecture) records agent trajectories (the sequence
/// of thoughts and tool uses) for completed tasks, distills them into generalizable "patterns",
/// and stores them. When encountering similar tasks, the agent can retrieve the pattern and
/// apply it as a structural guide.

#[derive(Debug, Clone)]
pub struct SonaPattern {
    pub id: String,
    pub task_description: String,
    pub extracted_pattern: String,
    pub successful_uses: usize,
}

pub struct SonaPatternStore {
    patterns: tokio::sync::RwLock<HashMap<String, SonaPattern>>,
    llm: Arc<dyn LlmClient>,
}

impl SonaPatternStore {
    pub fn new(llm: Arc<dyn LlmClient>) -> Self {
        Self {
            patterns: tokio::sync::RwLock::new(HashMap::new()),
            llm,
        }
    }

    /// Extract a generalizable pattern from a completed task trajectory
    pub async fn learn_from_trajectory(&self, task: &str, trajectory: &str) -> Result<String, String> {
        let system_prompt = "You are a trajectory extraction engine. Distill the given specific execution trajectory for the task into a generalized, reusable step-by-step pattern (a SONA neural pattern). Focus on the sequence of tool usage and reasoning, ignoring specific input values.";
        let req = ChatRequest {
            model: "default".to_string(),
            system: system_prompt.to_string(),
            messages: vec![Message::user(format!("Task: {}\nTrajectory:\n{}", task, trajectory))],
            tools: vec![],
            max_tokens: 1500,
            temperature: 0.1,
        };

        match self.llm.chat(req).await {
            Ok(resp) => {
                let pattern = resp.message.content;
                let id = uuid::Uuid::new_v4().to_string();

                let mut store = self.patterns.write().await;
                store.insert(id.clone(), SonaPattern {
                    id: id.clone(),
                    task_description: task.to_string(),
                    extracted_pattern: pattern,
                    successful_uses: 1,
                });

                Ok(id)
            }
            Err(e) => Err(format!("Failed to extract SONA pattern: {}", e)),
        }
    }

    /// Retrieve the most relevant pattern for a given task
    /// For this mock implementation, we use an LLM call to find the best match or simply a keyword match
    pub async fn retrieve_pattern(&self, task: &str) -> Option<SonaPattern> {
        let store = self.patterns.read().await;
        // Basic keyword matching for simplicity in this implementation
        // A real SONA would use vector similarity search (like HNSW)
        for (_, pattern) in store.iter() {
            let task_words: Vec<&str> = task.split_whitespace().collect();
            let mut matches = 0;
            for word in &task_words {
                if pattern.task_description.contains(word) {
                    matches += 1;
                }
            }
            if matches > task_words.len() / 2 {
                return Some(pattern.clone());
            }
        }

        None
    }

    /// Feedback loop: increment success count if the pattern was useful
    pub async fn reinforce_pattern(&self, id: &str) {
        let mut store = self.patterns.write().await;
        if let Some(pattern) = store.get_mut(id) {
            pattern.successful_uses += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::Usage;

    struct SonaMockLlmClient {
        resp: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for SonaMockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant(&self.resp),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_sona_learn_from_trajectory() {
        let llm = Arc::new(SonaMockLlmClient {
            resp: "1. Read File\n2. Extract Data\n3. Write File".to_string(),
        });

        let store = SonaPatternStore::new(llm);

        let id = store.learn_from_trajectory("Extract user emails from a log file", "I used read_file on access.log, then parsed it, then write_file to emails.txt").await.unwrap();

        let patterns = store.patterns.read().await;
        assert!(patterns.contains_key(&id));
        assert_eq!(patterns.get(&id).unwrap().extracted_pattern, "1. Read File\n2. Extract Data\n3. Write File");
    }

    #[tokio::test]
    async fn test_sona_retrieve_and_reinforce() {
        let llm = Arc::new(SonaMockLlmClient {
            resp: "Step 1, Step 2".to_string(),
        });

        let store = SonaPatternStore::new(llm);

        let id = store.learn_from_trajectory("Generate a python script to calculate fibonacci", "trajectory...").await.unwrap();

        let retrieved = store.retrieve_pattern("Generate a python script to find prime numbers").await;
        assert!(retrieved.is_some());

        store.reinforce_pattern(&id).await;

        let patterns = store.patterns.read().await;
        assert_eq!(patterns.get(&id).unwrap().successful_uses, 2);
    }
}
