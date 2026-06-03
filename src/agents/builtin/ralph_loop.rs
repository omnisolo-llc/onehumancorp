use crate::agent::{Agent, AgentEvent, AgentRunConfig};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::process::Command;
use std::time::Duration;
use std::path::PathBuf;
use tokio::fs;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct RalphProgress {
    pub task_description: String,
    pub features: Vec<Feature>,
    pub current_feature_index: usize,
    #[serde(default)]
    pub notes: Vec<String>,
    pub is_complete: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Feature {
    pub name: String,
    pub status: String, // "pending", "in_progress", "completed"
}

/// The "Ralph Loop": For long-running asynchronous tasks spanning multiple context windows.
/// Phase 1 (Initializer Agent): Sets up environment, writes init script, progress file, feature list, and makes initial git commit.
/// Phase 2 (Coding Agent): In every subsequent session, reads git logs and progress files to orient itself, picks the highest-priority incomplete feature, works, commits, and updates the summary.
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
            let mut cmd = Command::new("git");
            cmd.arg("log")
               .arg("--oneline")
               .arg("-n")
               .arg("5")
               .current_dir(&self.repo_path);

            let git_log_output = match tokio::time::timeout(Duration::from_secs(5), cmd.output()).await {
                Ok(Ok(out)) if out.status.success() => String::from_utf8_lossy(&out.stdout).to_string(),
                _ => "No git history available".to_string(),
            };

            // Execute the agent run for this specific feature
            let feature_prompt = format!(
                "You are continuing a long-running task.\nOverall Task: {}\nRecent Git History:\n{}\nFeature to implement now: {}\nExecute steps to complete this feature, verify it, and then stop.",
                progress.task_description, git_log_output, feature_name
            );

            // We use a fresh config to keep the context window small (compaction/reset)
            let mut feature_config = self.config.clone();
            let scratchpad_context = if !progress.notes.is_empty() {
                format!("\nStructured Scratchpad Notes:\n- {}", progress.notes.join("\n- "))
            } else {
                String::new()
            };
            feature_config.user_instructions = format!("{}{}", feature_prompt, scratchpad_context);

            let mut on_event = |event: AgentEvent| {
                if let AgentEvent::TaskError { error } = event {
                    tracing::error!("Ralph Loop Feature Error: {}", error);
                }
            };

            match self.agent.run(&feature_config, &feature_prompt, &mut on_event).await {
                Ok(result) => {
                    tracing::info!("Ralph Loop: Feature {} completed. Result: {}", feature_name, result);
                    progress.features[progress.current_feature_index].status = "completed".to_string();
                    progress.notes.push(format!("Completed feature {}: {}", feature_name, result));
                    progress.current_feature_index += 1;
                    self.save_progress(&progress).await?;

                    // Phase 2: Commit after completion
                    let mut cmd = Command::new("git");
                    cmd.arg("add").arg(".").current_dir(&self.repo_path);
                    if let Ok(Ok(out)) = tokio::time::timeout(Duration::from_secs(10), cmd.output()).await {
                        if !out.status.success() {
                            tracing::warn!("Phase 2 git add returned non-zero: {:?}", String::from_utf8_lossy(&out.stderr));
                        }
                    } else {
                        tracing::error!("Phase 2 failed to git add or timed out");
                    }

                    let commit_msg = format!("Completed feature: {}", feature_name);

                    let mut cmd1 = Command::new("git");
                    cmd1.arg("config").arg("user.name").arg("Ralph Agent").current_dir(&self.repo_path);
                    let _ = cmd1.output().await;
                    let mut cmd2 = Command::new("git");
                    cmd2.arg("config").arg("user.email").arg("ralph@example.com").current_dir(&self.repo_path);
                    let _ = cmd2.output().await;

                    let mut cmd = Command::new("git");
                    cmd.arg("commit").arg("-m").arg(&commit_msg).current_dir(&self.repo_path);
                    match tokio::time::timeout(Duration::from_secs(10), cmd.output()).await {
                        Ok(Ok(out)) => {
                            if !out.status.success() {
                                let stderr = String::from_utf8_lossy(&out.stderr);
                                let stdout = String::from_utf8_lossy(&out.stdout);
                                if stdout.contains("nothing to commit") || stderr.contains("nothing to commit") {
                                    tracing::info!("Phase 2 git commit skipped: nothing to commit.");
                                } else {
                                    tracing::error!("Phase 2 git commit failed: stdout: {}, stderr: {}", stdout, stderr);
                                }
                            }
                        }
                        Ok(Err(e)) => tracing::error!("Phase 2 failed to execute git commit: {}", e),
                        Err(_) => tracing::error!("Phase 2 git commit timed out"),
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
            notes: vec!["Initialized task and broken down into features.".to_string()],
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
        let mut cmd = Command::new("git");
        cmd.arg("status").current_dir(&self.repo_path);
        let needs_init = match tokio::time::timeout(Duration::from_secs(5), cmd.output()).await {
            Ok(Ok(out)) => !out.status.success(),
            _ => true,
        };

        if needs_init {
            let mut cmd = Command::new("git");
            cmd.arg("init").current_dir(&self.repo_path);
            if let Ok(Ok(out)) = tokio::time::timeout(Duration::from_secs(10), cmd.output()).await {
                if !out.status.success() {
                    tracing::error!("Failed to git init: {:?}", String::from_utf8_lossy(&out.stderr));
                }
            } else {
                tracing::error!("git init timed out or failed to execute");
            }
        }

        let mut cmd1 = Command::new("git");
        cmd1.arg("config").arg("user.name").arg("Ralph Agent").current_dir(&self.repo_path);
        let _ = cmd1.output().await;
        let mut cmd2 = Command::new("git");
        cmd2.arg("config").arg("user.email").arg("ralph@example.com").current_dir(&self.repo_path);
        let _ = cmd2.output().await;

        let mut cmd = Command::new("git");
        cmd.arg("add").arg(".").current_dir(&self.repo_path);
        if let Ok(Ok(out)) = tokio::time::timeout(Duration::from_secs(10), cmd.output()).await {
            if !out.status.success() {
                tracing::warn!("Phase 1 git add returned non-zero: {:?}", String::from_utf8_lossy(&out.stderr));
            }
        }

        let mut cmd = Command::new("git");
        cmd.arg("commit").arg("-m").arg("Ralph Loop Initial Commit").current_dir(&self.repo_path);
        if let Ok(Ok(out)) = tokio::time::timeout(Duration::from_secs(10), cmd.output()).await {
            if !out.status.success() {
                let stdout = String::from_utf8_lossy(&out.stdout);
                if !stdout.contains("nothing to commit") {
                    tracing::error!("Failed to git commit: {:?}", String::from_utf8_lossy(&out.stderr));
                }
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
            notes: vec!["Initialized task and broken down into features.".to_string()],
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

    #[tokio::test]
    async fn test_ralph_loop_git_failures_graceful() {
        // This test ensures that when git fails, the loop doesn't panic
        let dir = tempdir().unwrap();
        let progress_file = dir.path().join("progress.json");

        let initial_progress = RalphProgress {
            task_description: "Build a web server".to_string(),
            features: vec![
                Feature { name: "Step 1".to_string(), status: "pending".to_string() },
            ],
            current_feature_index: 0,
            notes: vec![],
            is_complete: false,
        };
        std::fs::write(&progress_file, serde_json::to_string(&initial_progress).unwrap()).unwrap();

        let llm = Arc::new(TestLlmClient { call_count: tokio::sync::Mutex::new(0) });
        let agent = Arc::new(Agent::new(llm, vec![]));
        let config = AgentRunConfig::default();

        let ralph = RalphLoop::new(agent, config, progress_file.to_str().unwrap());

        // We run the loop in a directory that is not a git repo to trigger failures
        let result = ralph.run("Build a web server").await;
        assert!(result.is_ok());

        let saved_progress_str = std::fs::read_to_string(&progress_file).unwrap();
        let saved_progress: RalphProgress = serde_json::from_str(&saved_progress_str).unwrap();

        // It should still complete the feature even if git failed
        assert_eq!(saved_progress.features[0].status, "completed");
    }

    #[tokio::test]
    async fn test_ralph_loop_initialize_git_failures() {
        let dir = tempdir().unwrap();
        let progress_file = dir.path().join("progress.json");

        let llm = Arc::new(TestLlmClient { call_count: tokio::sync::Mutex::new(0) });
        let agent = Arc::new(Agent::new(llm, vec![]));
        let config = AgentRunConfig::default();

        let ralph = RalphLoop::new(agent, config, progress_file.to_str().unwrap());

        // We run initialize in a directory without proper permissions or just check it doesn't panic
        let result = ralph.initialize("Task with git failures").await;
        assert!(result.is_ok());

        assert!(progress_file.exists());
    }
}
