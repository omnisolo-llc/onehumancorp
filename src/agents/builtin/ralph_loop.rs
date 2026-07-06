/// Master Catalog B.12. The "Ralph Loop"
use crate::agent::{Agent, AgentEvent, AgentRunConfig};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
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
        let mut p = std::path::Path::new(progress_file_path)
            .parent()
            .unwrap_or(std::path::Path::new("."));
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
    pub async fn run(
        &self,
        initial_task: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Phase 1: Initializer
        let mut progress = self.initialize(initial_task).await?;

        // Phase 2: Coding Agent Loop
        while !progress.is_complete {
            if progress.current_feature_index >= progress.features.len() {
                progress.is_complete = true;
                self.save_progress(&progress).await?;
                break;
            }

            let feature_name = progress.features[progress.current_feature_index]
                .name
                .clone();
            if progress.features[progress.current_feature_index].status == "completed" {
                progress.current_feature_index += 1;
                continue;
            }

            tracing::info!("Ralph Loop: Starting work on feature: {}", feature_name);

            // Create a worktree for this specific feature
            let branch_name = format!("ralph-loop-feat-{}", progress.current_feature_index);
            let worktree_dir = self.repo_path.join(format!(".worktree_{}", branch_name));

            // Clean up existing worktree/branch just in case
            let _ = Command::new("git").arg("worktree").arg("remove").arg("-f").arg(&worktree_dir).current_dir(&self.repo_path).output();
            let _ = Command::new("git").arg("branch").arg("-D").arg(&branch_name).current_dir(&self.repo_path).output();

            let wt_output = Command::new("git")
                .arg("worktree")
                .arg("add")
                .arg("-b")
                .arg(&branch_name)
                .arg(&worktree_dir)
                .current_dir(&self.repo_path)
                .output();

            if let Err(e) = wt_output {
                tracing::error!("Failed to create worktree: {}", e);
                break;
            } else if let Ok(out) = wt_output {
                if !out.status.success() {
                    tracing::error!("Git worktree add failed: {}", String::from_utf8_lossy(&out.stderr));
                    break;
                }
            }

            // Phase 2 (Coding Agent): Read git logs to orient itself
            let git_log_output = Command::new("git")
                .arg("log")
                .arg("--oneline")
                .arg("-n")
                .arg("5")
                .current_dir(&worktree_dir)
                .output()
                .ok()
                .and_then(|out| String::from_utf8(out.stdout).ok())
                .unwrap_or_else(|| "No git history available".to_string());

            // Execute the agent run for this specific feature
            let feature_prompt = format!(
                "You are continuing a long-running task.
Overall Task: {}
Recent Git History:
{}
Feature to implement now: {}
Execute steps to complete this feature inside your isolated workspace, verify it, and then stop.",
                progress.task_description, git_log_output, feature_name
            );

            // We use a fresh config to keep the context window small (compaction/reset)
            let mut feature_config = self.config.clone();
            feature_config.workspace_path = Some(worktree_dir.to_string_lossy().to_string());
            let scratchpad_context = if !progress.notes.is_empty() {
                // Fix Gap 2: Bound the context window by only including the last 10 notes.
                let start_idx = progress.notes.len().saturating_sub(10);
                let recent_notes = &progress.notes[start_idx..];
                format!(
                    "\nStructured Scratchpad Notes:\n- {}",
                    recent_notes.join("\n- ")
                )
            } else {
                String::new()
            };
            feature_config.user_instructions = format!("{}{}", feature_prompt, scratchpad_context);

            let mut on_event = |event: AgentEvent| {
                if let AgentEvent::TaskError { error } = event {
                    tracing::error!("Ralph Loop Feature Error: {}", error);
                }
            };

            match self
                .agent
                .run(&feature_config, &feature_prompt, &mut on_event)
                .await
            {
                Ok(result) => {
                    tracing::info!(
                        "Ralph Loop: Feature {} completed. Result: {}",
                        feature_name,
                        result
                    );
                    progress.features[progress.current_feature_index].status =
                        "completed".to_string();
                    progress
                        .notes
                        .push(format!("Completed feature {}: {}", feature_name, result));
                    progress.current_feature_index += 1;
                    self.save_progress(&progress).await?;

                    // Phase 2: Commit after completion IN THE WORKTREE
                    let commit_msg = format!("Completed feature: {}", feature_name);

                    if let Err(e) = Command::new("git").arg("config").arg("user.name").arg("Ralph Agent").current_dir(&worktree_dir).output() { tracing::error!("Failed: {}", e); }
                    if let Err(e) = Command::new("git").arg("config").arg("user.email").arg("ralph@example.com").current_dir(&worktree_dir).output() { tracing::error!("Failed: {}", e); }
                    if let Err(e) = Command::new("git").arg("add").arg(".").current_dir(&worktree_dir).output() { tracing::error!("Failed: {}", e); }
                    if let Err(e) = Command::new("git").arg("commit").arg("-m").arg(&commit_msg).current_dir(&worktree_dir).output() { tracing::error!("Failed: {}", e); }

                    // Merge worktree branch back into main repo
                    let merge_out = Command::new("git").arg("merge").arg(&branch_name).current_dir(&self.repo_path).output();

                    if let Ok(out) = merge_out {
                        if !out.status.success() {
                            tracing::error!("Merge conflict in Ralph Loop! Aborting merge: {}", String::from_utf8_lossy(&out.stderr));
                            if let Err(e) = Command::new("git").arg("merge").arg("--abort").current_dir(&self.repo_path).output() { tracing::error!("Failed: {}", e); }
                            // Cleanup worktree on error
                            if let Err(e) = Command::new("git").arg("worktree").arg("remove").arg("-f").arg(&worktree_dir).current_dir(&self.repo_path).output() { tracing::error!("Failed: {}", e); }
                            if let Err(e) = Command::new("git").arg("branch").arg("-D").arg(&branch_name).current_dir(&self.repo_path).output() { tracing::error!("Failed: {}", e); }
                            break;
                        }
                    } else {
                        tracing::error!("Failed to execute git merge command");
                        break;
                    }

                    // Cleanup worktree
                    if let Err(e) = Command::new("git").arg("worktree").arg("remove").arg("-f").arg(&worktree_dir).current_dir(&self.repo_path).output() { tracing::error!("Failed: {}", e); }
                    if let Err(e) = Command::new("git").arg("branch").arg("-d").arg(&branch_name).current_dir(&self.repo_path).output() { tracing::error!("Failed: {}", e); }
                }
                Err(e) => {
                    tracing::error!("Ralph Loop failed on feature {}: {}", feature_name, e);
                    progress
                        .notes
                        .push(format!("Failed feature {}: {}", feature_name, e));
                    let _ = self.save_progress(&progress).await;

                    // Cleanup worktree on error
                    if let Err(e) = Command::new("git").arg("worktree").arg("remove").arg("-f").arg(&worktree_dir).current_dir(&self.repo_path).output() { tracing::error!("Failed: {}", e); }
                    if let Err(e) = Command::new("git").arg("branch").arg("-D").arg(&branch_name).current_dir(&self.repo_path).output() { tracing::error!("Failed: {}", e); }

                    break;
                }
            }
        }

        tracing::info!("Ralph Loop completely finished.");
        Ok(())
    }

    async fn initialize(
        &self,
        task: &str,
    ) -> Result<RalphProgress, Box<dyn std::error::Error + Send + Sync>> {
        if let Ok(data) = fs::read_to_string(&self.progress_file_path).await
            && let Ok(progress) = serde_json::from_str::<RalphProgress>(&data)
        {
            tracing::info!("Ralph Loop: Resuming from existing progress file.");
            return Ok(progress);
        }

        tracing::info!("Ralph Loop: Initializing new progress file.");

        let breakdown_prompt = format!(
            "Break down the following task into 3 distinct, manageable features to implement sequentially. Respond strictly with a JSON array of strings representing the feature names. Task: {}",
            task
        );

        let mut on_event = |_| {};
        let result = self
            .agent
            .run(&self.config, &breakdown_prompt, &mut on_event)
            .await?;

        // Fix Gap 3: Strip markdown JSON blocks if the model wrapped the output
        let mut clean_result = result.trim();
        if clean_result.starts_with("```json") {
            clean_result = clean_result.trim_start_matches("```json").trim();
        } else if clean_result.starts_with("```") {
            clean_result = clean_result.trim_start_matches("```").trim();
        }
        if clean_result.ends_with("```") {
            clean_result = clean_result.trim_end_matches("```").trim();
        }

        let mut features = vec![];
        if let Ok(parsed) = serde_json::from_str::<Vec<String>>(clean_result) {
            for name in parsed {
                features.push(Feature {
                    name,
                    status: "pending".to_string(),
                });
            }
        } else {
            features.push(Feature {
                name: "Step 1".to_string(),
                status: "pending".to_string(),
            });
            features.push(Feature {
                name: "Step 2".to_string(),
                status: "pending".to_string(),
            });
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
                if let Ok(mut perms) = fs::metadata(&init_script_path)
                    .await
                    .map(|m| m.permissions())
                {
                    perms.set_mode(0o755);
                    let _ = fs::set_permissions(&init_script_path, perms).await;
                }
            }
        }

        // Run git init if not inside a repo
        let git_status = Command::new("git")
            .arg("status")
            .current_dir(&self.repo_path)
            .output();
        if (git_status.is_err() || !git_status.unwrap().status.success())
            && let Err(e) = Command::new("git")
                .arg("init")
                .current_dir(&self.repo_path)
                .output()
        {
            tracing::error!("Failed to git init: {}", e);
        }

        if let Err(e) = Command::new("git")
            .arg("config")
            .arg("user.name")
            .arg("Ralph Agent")
            .current_dir(&self.repo_path)
            .output() { tracing::error!("Failed git config: {}", e); }
        if let Err(e) = Command::new("git")
            .arg("config")
            .arg("user.email")
            .arg("ralph@example.com")
            .current_dir(&self.repo_path)
            .output() { tracing::error!("Failed git config: {}", e); }

        if let Err(e) = Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(&self.repo_path)
            .output()
        {
            tracing::error!("Failed to git add: {}", e);
        }
        if let Err(e) = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("Ralph Loop Initial Commit")
            .current_dir(&self.repo_path)
            .output()
        {
            tracing::error!("Failed to git commit: {}", e);
        }

        Ok(progress)
    }

    async fn save_progress(
        &self,
        progress: &RalphProgress,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
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

        let llm = Arc::new(TestLlmClient {
            call_count: tokio::sync::Mutex::new(0),
        });
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
                Feature {
                    name: "Step 1".to_string(),
                    status: "completed".to_string(),
                },
                Feature {
                    name: "Step 2".to_string(),
                    status: "pending".to_string(),
                },
            ],
            current_feature_index: 0,
            notes: vec!["Initialized task and broken down into features.".to_string()],
            is_complete: false,
        };
        std::fs::write(
            &progress_file,
            serde_json::to_string(&initial_progress).unwrap(),
        )
        .unwrap();

        let _ = std::process::Command::new("git").arg("init").current_dir(dir.path()).output();
        let _ = std::process::Command::new("git").arg("commit").arg("--allow-empty").arg("-m").arg("init").current_dir(dir.path()).output();

        let llm = Arc::new(TestLlmClient {
            call_count: tokio::sync::Mutex::new(0),
        });
        let agent = Arc::new(Agent::new(llm, vec![]));
        let config = AgentRunConfig::default();

        let ralph = RalphLoop::new(agent, config, progress_file.to_str().unwrap());

        let result = ralph.run("Build a web server").await;
        assert!(result.is_ok());

        let saved_progress_str = std::fs::read_to_string(&progress_file).unwrap();
        let saved_progress: RalphProgress = serde_json::from_str(&saved_progress_str).unwrap();
        assert!(saved_progress.is_complete);
    }

    struct InterruptionLlmClient {
        call_count: tokio::sync::Mutex<usize>,
    }

    #[async_trait::async_trait]
    impl LlmClient for InterruptionLlmClient {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut count = self.call_count.lock().await;
            *count += 1;

            if *count == 1 {
                Ok(ChatResponse {
                    message: Message::assistant(r#"["Feature A"]"#),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id1".to_string()),
                })
            } else if *count == 2 {
                // First run fails immediately because of "Fatal"
                Err("Fatal: Simulated failure during feature implementation".into())
            } else {
                Ok(ChatResponse {
                    message: Message::assistant("Feature implemented successfully after retry"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id3".to_string()),
                })
            }
        }
    }

    #[tokio::test]
    async fn test_ralph_loop_interruptions_and_retry() {
        let dir = tempdir().unwrap();
        let progress_file = dir.path().join("progress.json");
        let progress_file_str = progress_file.to_str().unwrap();

        let llm = Arc::new(InterruptionLlmClient {
            call_count: tokio::sync::Mutex::new(0),
        });
        let agent = Arc::new(Agent::new(llm, vec![]));
        let config = AgentRunConfig::default();

        let ralph = RalphLoop::new(agent.clone(), config.clone(), progress_file_str);

        let result1 = ralph.run("Build a reliable feature").await;
        // In the test, the second LLM call (for "Feature A") fails, which breaks the loop.
        // We now correctly return Ok(()) instead of Err from the loop because the run itself didn't crash,
        // it just stopped due to feature failure so it can be resumed later. Wait, run() returns Ok(()).
        assert!(result1.is_ok());

        // Verify that the error was recorded in the progress file before breaking
        let saved_progress_str1 = std::fs::read_to_string(&progress_file).unwrap();
        let saved_progress1: RalphProgress = serde_json::from_str(&saved_progress_str1).unwrap();
        assert!(!saved_progress1.is_complete);
        assert!(
            saved_progress1
                .notes
                .iter()
                .any(|n| n.contains("Failed feature Feature A"))
        );

        let result2 = ralph.run("Build a reliable feature").await;
        assert!(result2.is_ok());

        let saved_progress_str2 = std::fs::read_to_string(&progress_file).unwrap();
        let saved_progress2: RalphProgress = serde_json::from_str(&saved_progress_str2).unwrap();
        assert!(saved_progress2.is_complete);
        assert_eq!(saved_progress2.features[0].status, "completed");
    }

    struct MarkdownJsonLlmClient;

    #[async_trait::async_trait]
    impl LlmClient for MarkdownJsonLlmClient {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant(
                    "```json\n[\"Markdown Feature 1\", \"Markdown Feature 2\"]\n```",
                ),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id-md".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_ralph_loop_markdown_json_parsing() {
        let dir = tempdir().unwrap();
        let progress_file = dir.path().join("progress_md.json");
        let progress_file_str = progress_file.to_str().unwrap();

        let llm = Arc::new(MarkdownJsonLlmClient);
        let agent = Arc::new(Agent::new(llm, vec![]));
        let config = AgentRunConfig::default();

        let ralph = RalphLoop::new(agent, config, progress_file_str);

        let progress = ralph.initialize("Test markdown parsing").await.unwrap();

        // The features should match the markdown block, not the fallback defaults
        assert_eq!(progress.features.len(), 2);
        assert_eq!(progress.features[0].name, "Markdown Feature 1");
        assert_eq!(progress.features[1].name, "Markdown Feature 2");
    }

    #[tokio::test]
    async fn test_ralph_loop_git_status_failure_continues() {
        let dir = tempfile::tempdir().unwrap();
        let progress_file = dir.path().join("progress.json");

        let llm = Arc::new(TestLlmClient {
            call_count: tokio::sync::Mutex::new(0),
        });
        let agent = Arc::new(Agent::new(llm, vec![]));
        let config = AgentRunConfig::default();

        let mut ralph = RalphLoop::new(agent, config, progress_file.to_str().unwrap());
        // Use a non-existent directory to simulate git commands failing without panicking
        ralph.repo_path = dir.path().join("non_existent_repo");

        let result = ralph.run("Build a web server").await;
        // Even if git commands fail, the loop should continue and return Ok(())
        assert!(result.is_ok());

        assert!(progress_file.exists());
        let saved_progress_str = std::fs::read_to_string(&progress_file).unwrap();
        let saved_progress: RalphProgress = serde_json::from_str(&saved_progress_str).unwrap();
        assert!(!saved_progress.is_complete);
    }
}
