use crate::agent::{Agent, AgentEvent, AgentRunConfig};
#[allow(unused_imports)]
use ohc_builtin_agent_core::types::{Message, Role};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::fs;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct RalphProgress {
    pub task_description: String,
    pub features: Vec<Feature>,
    pub current_feature_index: usize,
    pub is_complete: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Feature {
    pub name: String,
    pub status: String, // "pending", "in_progress", "completed"
}

/// The Ralph Loop for long-running asynchronous tasks spanning multiple context windows.
pub struct RalphLoop {
    pub agent: Arc<Agent>,
    pub config: AgentRunConfig,
    pub progress_file_path: String,
}

impl RalphLoop {
    pub fn new(agent: Arc<Agent>, config: AgentRunConfig, progress_file_path: &str) -> Self {
        Self {
            agent,
            config,
            progress_file_path: progress_file_path.to_string(),
        }
    }

    /// Run the full Ralph Loop
    pub async fn run(&self, initial_task: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Phase 1: Initializer
        let mut progress = self.initialize(initial_task).await?;
        
        // Phase 2: Coding Agent Loop
        while !progress.is_complete {
            if progress.current_feature_index >= progress.features.len() {
                progress.is_complete = true;
                self.save_progress(&progress).await?;
                break;
            }

            let feature_name = progress.features[progress.current_feature_index].name.clone();
            let feature_status = progress.features[progress.current_feature_index].status.clone();

            if feature_status == "completed" {
                progress.current_feature_index += 1;
                continue;
            }

            tracing::info!("Ralph Loop: Starting work on feature: {}", feature_name);
            
            let mut checkpoint_summary = String::new();
            if let Some(cp) = &self.agent.checkpointer {
                let thread_id = self.config.thread_id.clone().unwrap_or_else(|| "default".to_string());
                if let Ok(checkpoints) = cp.list_checkpoints(&thread_id).await {
                    checkpoint_summary.push_str("\nRecent Checkpoints (Git Log Orientation):\n");
                    for c in checkpoints.iter().rev().take(5) {
                        checkpoint_summary.push_str(&format!("- [{}] {}\n", c.created_at, c.metadata));
                    }
                }
            }

            // Execute the agent run for this specific feature
            let feature_prompt = format!(
                "You are continuing a long-running task.\nOverall Task: {}\nFeature to implement now: {}{}\nExecute steps to complete this feature, verify it, and then stop.",
                progress.task_description, feature_name, checkpoint_summary
            );

            // We use a fresh config to keep the context window small (compaction/reset)
            let mut feature_config = self.config.clone();
            feature_config.user_instructions = feature_prompt.clone();
            
            let mut on_event = |event: AgentEvent| {
                // In production, we could stream these to a UI or log them
                if let AgentEvent::TaskError { error } = event {
                    tracing::error!("Ralph Loop Feature Error: {}", error);
                }
            };

            match self.agent.run(&feature_config, &feature_prompt, &mut on_event).await {
                Ok(result) => {
                    tracing::info!("Ralph Loop: Feature {} completed. Result: {}", feature_name, result);
                    progress.features[progress.current_feature_index].status = "completed".to_string();
                    progress.current_feature_index += 1;
                    self.save_progress(&progress).await?;

                    if let Some(cp) = &self.agent.checkpointer {
                        let thread_id = self.config.thread_id.clone().unwrap_or_else(|| "default".to_string());
                        let checkpoint = crate::checkpointer::Checkpoint {
                            thread_id,
                            checkpoint_id: uuid::Uuid::new_v4().to_string(),
                            parent_id: None,
                            data: serde_json::json!({}),
                            metadata: serde_json::json!({"ralph_status": format!("Ralph Loop: Completed feature {}", feature_name)}),
                            created_at: chrono::Utc::now(),
                        };
                        let _ = cp.put_checkpoint(checkpoint).await;
                    }
                }
                Err(e) => {
                    tracing::error!("Ralph Loop failed on feature {}: {}", feature_name, e);
                    // For demo purposes, we break on error. A robust system would retry or mark failed.
                    break;
                }
            }
        }

        tracing::info!("Ralph Loop completely finished.");
        Ok(())
    }

    async fn initialize(&self, task: &str) -> Result<RalphProgress, Box<dyn std::error::Error + Send + Sync>> {
        if let Ok(data) = fs::read_to_string(&self.progress_file_path).await {
            if let Ok(progress) = serde_json::from_str::<RalphProgress>(&data) {
                tracing::info!("Ralph Loop: Resuming from existing progress file.");
                return Ok(progress);
            }
        }

        tracing::info!("Ralph Loop: Initializing new progress file.");
        
        // Use the agent itself to break down the task
        let breakdown_prompt = format!("Break down the following task into 3 distinct, manageable features to implement sequentially. Respond strictly with a JSON array of strings representing the feature names. Task: {}", task);
        
        let mut on_event = |_| {};
        let result = self.agent.run(&self.config, &breakdown_prompt, &mut on_event).await?;
        
        // Simplistic extraction (assuming the model output valid JSON)
        let mut features = vec![];
        if let Ok(parsed) = serde_json::from_str::<Vec<String>>(&result) {
            for name in parsed {
                features.push(Feature { name, status: "pending".to_string() });
            }
        } else {
            // Fallback
            features.push(Feature { name: "Step 1".to_string(), status: "pending".to_string() });
            features.push(Feature { name: "Step 2".to_string(), status: "pending".to_string() });
        }

        let progress = RalphProgress {
            task_description: task.to_string(),
            features,
            current_feature_index: 0,
            is_complete: false,
        };

        self.save_progress(&progress).await?;

        if let Some(cp) = &self.agent.checkpointer {
            let thread_id = self.config.thread_id.clone().unwrap_or_else(|| "default".to_string());
            let checkpoint = crate::checkpointer::Checkpoint {
                thread_id,
                checkpoint_id: uuid::Uuid::new_v4().to_string(),
                parent_id: None,
                data: serde_json::json!({}),
                metadata: serde_json::json!({"ralph_status": "Ralph Initializer: Task broken down"}),
                created_at: chrono::Utc::now(),
            };
            if let Err(e) = cp.put_checkpoint(checkpoint).await {
                tracing::error!("Failed to save initial ralph checkpoint: {}", e);
            }
        }

        Ok(progress)
    }

    async fn save_progress(&self, progress: &RalphProgress) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let json = serde_json::to_string_pretty(progress)?;
        fs::write(&self.progress_file_path, json).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkpointer::{Checkpoint, CheckpointSaver};
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockCheckpointer {
        checkpoints: Mutex<Vec<Checkpoint>>,
    }

    impl MockCheckpointer {
        fn new() -> Self {
            Self {
                checkpoints: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl CheckpointSaver for MockCheckpointer {
        async fn get_checkpoint(&self, _thread_id: &str, _checkpoint_id: &str) -> Result<Option<Checkpoint>, String> {
            Ok(None)
        }

        async fn put_checkpoint(&self, checkpoint: Checkpoint) -> Result<(), String> {
            self.checkpoints.lock().unwrap().push(checkpoint);
            Ok(())
        }

        async fn list_checkpoints(&self, thread_id: &str) -> Result<Vec<Checkpoint>, String> {
            let cps = self.checkpoints.lock().unwrap();
            Ok(cps.iter().filter(|c| c.thread_id == thread_id).cloned().collect())
        }
    }

    #[tokio::test]
    async fn test_ralph_loop_mechanic() {
        use crate::llm::LlmClient;
        use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, Usage};

        struct MockLlm;
        #[async_trait]
        impl LlmClient for MockLlm {
            async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                // Mock responses based on the phase
                let content = if req.messages.last().unwrap().content.contains("Break down the following task") {
                    r#"["Phase 1 Setup", "Phase 2 Core"]"#.to_string()
                } else {
                    "Feature executed successfully.".to_string()
                };

                Ok(ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content,
                        tool_calls: vec![],
                        tool_results: vec![],
                        response_id: None,
                    },
                    usage: Usage { input_tokens: 0, output_tokens: 0 },
                    stop_reason: "stop".to_string(),
                    response_id: None,
                })
            }
        }

        let cp = Arc::new(MockCheckpointer::new());
        let llm: Arc<dyn LlmClient> = Arc::new(MockLlm);
        let agent = Arc::new(Agent::new(llm, vec![]).with_checkpointer(cp.clone()));

        let mut config = AgentRunConfig::default();
        config.thread_id = Some("test_ralph_thread".to_string());

        let progress_file = format!(".ralph_test_{}.json", uuid::Uuid::new_v4());
        let ralph = RalphLoop::new(agent, config, &progress_file);

        let res = ralph.run("Build a small web server").await;
        assert!(res.is_ok());

        // Validate checkpoints were saved correctly
        let checkpoints = cp.checkpoints.lock().unwrap();
        // Should have 1 for initializer and 2 for features (as mocked above)
        assert_eq!(checkpoints.len(), 3);

        assert!(checkpoints[0].metadata.to_string().contains("Ralph Initializer"));
        assert!(checkpoints[1].metadata.to_string().contains("Completed feature Phase 1 Setup"));
        assert!(checkpoints[2].metadata.to_string().contains("Completed feature Phase 2 Core"));

        // Cleanup
        let _ = fs::remove_file(&progress_file).await;
    }
}
