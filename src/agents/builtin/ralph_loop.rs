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

            // Phase 2 Mechanics: Read git logs to orient
            let wd = self.config.workspace_path.clone().unwrap_or_else(|| ".".to_string());
            let mut git_log_context = String::new();
            if let Ok(output) = tokio::process::Command::new("git").current_dir(&wd).args(["log", "-n", "5", "--oneline"]).output().await {
                if output.status.success() {
                    git_log_context = String::from_utf8_lossy(&output.stdout).to_string();
                }
            }
            
            // Execute the agent run for this specific feature
            let feature_prompt = format!(
                "You are continuing a long-running task.\nOverall Task: {}\nRecent Git Logs:\n{}\n\nFeature to implement now: {}\nExecute steps to complete this feature, verify it, and then stop.",
                progress.task_description, git_log_context.trim(), feature_name
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

                    // Phase 2 Mechanics: Commit work
                    if let Ok(add_out) = tokio::process::Command::new("git").current_dir(&wd).args(["add", "-A"]).output().await {
                        if !add_out.status.success() {
                            tracing::warn!("Failed to git add in Ralph Loop Phase 2: {}", String::from_utf8_lossy(&add_out.stderr));
                        }
                    }
                    let commit_msg = format!("Completed feature: {}", feature_name);
                    if let Ok(commit_out) = tokio::process::Command::new("git").current_dir(&wd).args(["commit", "-m", &commit_msg]).output().await {
                        if !commit_out.status.success() {
                            tracing::warn!("Failed to git commit in Ralph Loop Phase 2 (might be no changes): {}", String::from_utf8_lossy(&commit_out.stderr));
                        }
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

        // Phase 1 Mechanics: Ensure git repo, init script, and initial commit.
        let wd = self.config.workspace_path.clone().unwrap_or_else(|| ".".to_string());
        if !std::path::Path::new(&wd).join(".git").exists() {
            if let Ok(init_out) = tokio::process::Command::new("git").current_dir(&wd).arg("init").output().await {
                if !init_out.status.success() {
                    tracing::warn!("Failed to git init in Ralph Loop Phase 1: {}", String::from_utf8_lossy(&init_out.stderr));
                }
            }
            // Provide a basic config so git commit works in isolated environments
            let _ = tokio::process::Command::new("git").current_dir(&wd).args(["config", "user.name", "Ralph Initializer"]).output().await;
            let _ = tokio::process::Command::new("git").current_dir(&wd).args(["config", "user.email", "ralph@ohc.local"]).output().await;
        }

        let init_script_path = std::path::Path::new(&wd).join("init.sh");
        if !init_script_path.exists() {
            fs::write(&init_script_path, "#!/bin/bash\n# Initialized by Ralph Loop\necho 'Ready.'\n").await?;
            if let Ok(add_out) = tokio::process::Command::new("git").current_dir(&wd).args(["add", "init.sh"]).output().await {
                if !add_out.status.success() {
                    tracing::warn!("Failed to git add init.sh in Ralph Loop Phase 1: {}", String::from_utf8_lossy(&add_out.stderr));
                }
            }
            if let Ok(commit_out) = tokio::process::Command::new("git").current_dir(&wd).args(["commit", "-m", "Initial commit by Initializer Agent"]).output().await {
                if !commit_out.status.success() {
                    tracing::warn!("Failed to git commit init.sh in Ralph Loop Phase 1: {}", String::from_utf8_lossy(&commit_out.stderr));
                }
            }
        }
        
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
    use crate::agent::{Agent, AgentRunConfig};
    use crate::llm::LlmClient;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, Usage};
    use std::sync::Arc;

    struct MockLlmClient;

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            // Mock LLM returning a JSON array of features
            Ok(ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: r#"["Test Feature 1", "Test Feature 2"]"#.to_string(),
                    tool_calls: vec![],
                    tool_results: vec![],
                },
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
            })
        }

        async fn generate_embedding(&self, _text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(vec![0.0; 1536])
        }
    }

    #[tokio::test]
    async fn test_ralph_loop_initialization() {
        let temp_dir = tempfile::tempdir().unwrap();
        let workspace_path = temp_dir.path().join("workspace");
        std::fs::create_dir_all(&workspace_path).unwrap();

        let progress_file = temp_dir.path().join("progress.json");

        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));

        let mut config = AgentRunConfig::default();
        config.workspace_path = Some(workspace_path.to_string_lossy().to_string());

        let ralph = RalphLoop::new(agent, config, &progress_file.to_string_lossy());

        // Test Phase 1
        let progress = ralph.initialize("Test Task").await.unwrap();

        assert_eq!(progress.task_description, "Test Task");
        assert_eq!(progress.features.len(), 2);
        assert_eq!(progress.features[0].name, "Test Feature 1");
        assert_eq!(progress.features[1].name, "Test Feature 2");
        assert_eq!(progress.current_feature_index, 0);
        assert_eq!(progress.is_complete, false);

        // Verify init.sh was created
        let init_sh = workspace_path.join("init.sh");
        assert!(init_sh.exists());
        let init_content = std::fs::read_to_string(init_sh).unwrap();
        assert!(init_content.contains("Initialized by Ralph Loop"));

        // Verify .git was created
        assert!(workspace_path.join(".git").exists());

        // Verify progress file was written
        assert!(progress_file.exists());
    }

    #[tokio::test]
    async fn test_ralph_loop_execution() {
        let temp_dir = tempfile::tempdir().unwrap();
        let workspace_path = temp_dir.path().join("workspace");
        std::fs::create_dir_all(&workspace_path).unwrap();

        let progress_file = temp_dir.path().join("progress.json");

        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));

        let mut config = AgentRunConfig::default();
        config.workspace_path = Some(workspace_path.to_string_lossy().to_string());

        let ralph = RalphLoop::new(agent, config, &progress_file.to_string_lossy());

        // Run full loop
        ralph.run("Test Task").await.unwrap();

        // Since the mock returns no tool calls and stops, it will just iterate through the features and complete them.

        // Verify progress file says complete
        let progress_data = std::fs::read_to_string(&progress_file).unwrap();
        let progress: RalphProgress = serde_json::from_str(&progress_data).unwrap();

        assert!(progress.is_complete);
        assert_eq!(progress.current_feature_index, 2);
        assert_eq!(progress.features[0].status, "completed");
        assert_eq!(progress.features[1].status, "completed");
    }
}
