use crate::agent::{Agent, AgentEvent, AgentRunConfig};
#[allow(unused_imports)]
use ohc_builtin_agent_core::types::{Message, Role};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::process::Command;
use std::path::PathBuf;
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
    pub repo_path: PathBuf,
}

impl RalphLoop {
    pub fn new(agent: Arc<Agent>, config: AgentRunConfig, progress_file_path: &str) -> Self {
        let mut p = std::path::Path::new(progress_file_path).parent().unwrap_or(std::path::Path::new("."));
        if p.as_os_str().is_empty() {
            p = std::path::Path::new(".");
        }
        let repo_path = p.to_path_buf();
        Self {
            agent,
            config,
            progress_file_path: progress_file_path.to_string(),
            repo_path,
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
            if progress.features[progress.current_feature_index].status == "completed" {
                progress.current_feature_index += 1;
                continue;
            }

            tracing::info!("Ralph Loop: Starting work on feature: {}", feature_name);
            
            // Phase 2 (Coding Agent): Read git logs to orient itself
            let git_log_output = Command::new("git")
                .arg("log")
                .arg("--oneline")
                .arg("-n")
                .arg("5")
                .current_dir(&self.repo_path)
                .output()
                .ok()
                .and_then(|out| String::from_utf8(out.stdout).ok())
                .unwrap_or_else(|| "No git history available".to_string());

            // Execute the agent run for this specific feature
            let feature_prompt = format!(
                "You are continuing a long-running task.\nOverall Task: {}\nRecent Git History:\n{}\nFeature to implement now: {}\nExecute steps to complete this feature, verify it, and then stop.",
                progress.task_description, git_log_output, feature_name
            );

            // We use a fresh config to keep the context window small (compaction/reset)
            let mut feature_config = self.config.clone();
            feature_config.user_instructions = feature_prompt.clone();
            
            let mut on_event = |event: AgentEvent| {
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

                    // Phase 2: Commit after completion
                    if let Err(e) = Command::new("git").arg("add").arg(".").current_dir(&self.repo_path).output() {
                        tracing::error!("Phase 2 failed to git add: {}", e);
                    }
                    let commit_msg = format!("Completed feature: {}", feature_name);

                    let _ = Command::new("git").arg("config").arg("user.name").arg("Ralph Agent").current_dir(&self.repo_path).output();
                    let _ = Command::new("git").arg("config").arg("user.email").arg("ralph@example.com").current_dir(&self.repo_path).output();

                    if let Err(e) = Command::new("git").arg("commit").arg("-m").arg(&commit_msg).current_dir(&self.repo_path).output() {
                        tracing::error!("Phase 2 failed to git commit: {}", e);
                    }
                }
                Err(e) => {
                    tracing::error!("Ralph Loop failed on feature {}: {}", feature_name, e);
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
        
        let breakdown_prompt = format!("Break down the following task into 3 distinct, manageable features to implement sequentially. Respond strictly with a JSON array of strings representing the feature names. Task: {}", task);
        
        let mut on_event = |_| {};
        let result = self.agent.run(&self.config, &breakdown_prompt, &mut on_event).await?;
        
        let mut features = vec![];
        if let Ok(parsed) = serde_json::from_str::<Vec<String>>(&result) {
            for name in parsed {
                features.push(Feature { name, status: "pending".to_string() });
            }
        } else {
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

        // Phase 1 (Initializer Agent): Setup environment, write init script, initial git commit.
        let init_script_path = self.repo_path.join("init.sh");
        if !init_script_path.exists() {
            let _ = fs::write(&init_script_path, "#!/bin/bash\n# Ralph Loop Init Script\n").await;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(mut perms) = fs::metadata(&init_script_path).await.map(|m| m.permissions()) {
                    perms.set_mode(0o755);
                    let _ = fs::set_permissions(&init_script_path, perms).await;
                }
            }
        }

        // Run git init if not inside a repo
        let git_status = Command::new("git").arg("status").current_dir(&self.repo_path).output();
        if git_status.is_err() || !git_status.unwrap().status.success() {
            if let Err(e) = Command::new("git").arg("init").current_dir(&self.repo_path).output() {
                tracing::error!("Failed to git init: {}", e);
            }
        }

        let _ = Command::new("git").arg("config").arg("user.name").arg("Ralph Agent").current_dir(&self.repo_path).output();
        let _ = Command::new("git").arg("config").arg("user.email").arg("ralph@example.com").current_dir(&self.repo_path).output();

        if let Err(e) = Command::new("git").arg("add").arg(".").current_dir(&self.repo_path).output() {
            tracing::error!("Failed to git add: {}", e);
        }
        if let Err(e) = Command::new("git").arg("commit").arg("-m").arg("Ralph Loop Initial Commit").current_dir(&self.repo_path).output() {
            tracing::error!("Failed to git commit: {}", e);
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
    use crate::agent::{Agent, AgentRunConfig};
    use crate::llm::LlmClient;
    use crate::types::{ChatRequest, ChatResponse, Message, Usage};
    use std::sync::Arc;
    use tempfile::tempdir;

    struct TestLlmClient {
        call_count: tokio::sync::Mutex<usize>,
    }

    #[async_trait::async_trait]
    impl LlmClient for TestLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut count = self.call_count.lock().await;
            *count += 1;

            if *count == 1 {
                Ok(ChatResponse {
                    message: Message::assistant(r#"["Feat1", "Feat2"]"#),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id1".to_string()),
                })
            } else {
                Ok(ChatResponse {
                    message: Message::assistant("Feature implemented"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id2".to_string()),
                })
            }
        }
    }

    #[tokio::test]
    async fn test_ralph_loop() {
        let dir = tempdir().unwrap();
        let progress_file = dir.path().join("progress.json");
        let progress_file_str = progress_file.to_str().unwrap();

        let llm = Arc::new(TestLlmClient { call_count: tokio::sync::Mutex::new(0) });
        let agent = Arc::new(Agent::new(llm, vec![]));
        let config = AgentRunConfig::default();

        let ralph = RalphLoop::new(agent, config, progress_file_str);

        let result = ralph.run("Build a web server").await;
        assert!(result.is_ok());

        assert!(progress_file.exists());

        let saved_progress_str = std::fs::read_to_string(&progress_file).unwrap();
        let saved_progress: RalphProgress = serde_json::from_str(&saved_progress_str).unwrap();
        assert!(saved_progress.is_complete);
        assert_eq!(saved_progress.features.len(), 2);
        assert_eq!(saved_progress.features[0].status, "completed");

        let git_dir = dir.path().join(".git");
        assert!(git_dir.exists());
    }

    #[tokio::test]
    async fn test_ralph_loop_with_completed_features() {
        let dir = tempdir().unwrap();
        let progress_file = dir.path().join("progress.json");

        let initial_progress = RalphProgress {
            task_description: "Build a web server".to_string(),
            features: vec![
                Feature { name: "Step 1".to_string(), status: "completed".to_string() },
                Feature { name: "Step 2".to_string(), status: "pending".to_string() },
            ],
            current_feature_index: 0,
            is_complete: false,
        };
        std::fs::write(&progress_file, serde_json::to_string(&initial_progress).unwrap()).unwrap();

        let llm = Arc::new(TestLlmClient { call_count: tokio::sync::Mutex::new(0) });
        let agent = Arc::new(Agent::new(llm, vec![]));
        let config = AgentRunConfig::default();

        let ralph = RalphLoop::new(agent, config, progress_file.to_str().unwrap());

        let result = ralph.run("Build a web server").await;
        assert!(result.is_ok());

        let saved_progress_str = std::fs::read_to_string(&progress_file).unwrap();
        let saved_progress: RalphProgress = serde_json::from_str(&saved_progress_str).unwrap();
        assert!(saved_progress.is_complete);
    }
}
