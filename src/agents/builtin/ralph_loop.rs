use crate::agent::{Agent, AgentEvent, AgentRunConfig};
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
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub architectural_decisions: Vec<String>,
    #[serde(default)]
    pub unresolved_bugs: Vec<String>,
    #[serde(default)]
    pub session_id: String,
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
        // Phase 1: Initializer Agent
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

            tracing::info!("Ralph Loop (Phase 2): Starting work on feature: {}", feature_name);

            // Phase 2 (Coding Agent): Orientation Step
            // In every subsequent session, reads git logs and progress files to orient itself.
            let git_log_output = Command::new("git")
                .arg("log")
                .arg("--oneline")
                .arg("-n")
                .arg("10")
                .current_dir(&self.repo_path)
                .output()
                .ok()
                .and_then(|out| String::from_utf8(out.stdout).ok())
                .unwrap_or_else(|| "No git history available".to_string());

            let orientation_context = format!(
                "[Ralph Loop Orientation]\n\
                Overall Task: {}\n\
                Recent Git History:\n{}\n\
                Architectural Decisions: {:?}\n\
                Unresolved Bugs: {:?}\n\
                Recent Progress Summary:\n{}",
                progress.task_description,
                git_log_output,
                progress.architectural_decisions,
                progress.unresolved_bugs,
                progress.notes.last().cloned().unwrap_or_default()
            );

            // Execute the agent run for this specific feature - Atomic Feature Execution
            // Agent picks the highest-priority incomplete feature, works, commits, and updates the summary.
            let feature_prompt = format!(
                "{}\n\nCURRENT OBJECTIVE: Implement the feature: {}.\n\
                Execute a Gather-Act-Verify cycle to complete this feature, then provide a condensed summary of your changes.",
                orientation_context, feature_name
            );

            // We use a fresh config to keep the context window small (compaction/reset)
            let mut feature_config = self.config.clone();
            feature_config.user_instructions = feature_prompt.clone();
            feature_config.server_system_message.push_str("\nYou are currently operating in Phase 2 (Coding Agent) of a Ralph Loop. Stay focused on the CURRENT OBJECTIVE.");

            let mut on_event = |event: AgentEvent| {
                if let AgentEvent::TaskError { error } = event {
                    tracing::error!("Ralph Loop Feature Error: {}", error);
                }
            };

            match self.agent.run(&feature_config, &feature_prompt, &mut on_event).await {
                Ok(result) => {
                    tracing::info!("Ralph Loop: Feature {} completed. Result: {}", feature_name, result);
                    progress.features[progress.current_feature_index].status = "completed".to_string();

                    // Phase 2: Commit & Summarize
                    progress.notes.push(format!("Completed feature {}: {}", feature_name, result));
                    progress.current_feature_index += 1;

                    // Dynamically extract decisions and bugs from the coding agent's summary
                    if result.to_lowercase().contains("architectural decision:") {
                        progress.architectural_decisions.push(format!("[{}] {}", feature_name, result));
                    }
                    if result.to_lowercase().contains("bug:") || result.to_lowercase().contains("unresolved:") {
                        progress.unresolved_bugs.push(format!("[{}] {}", feature_name, result));
                    }

                    self.save_progress(&progress).await?;

                    // Phase 2: Commit after completion
                    let _ = Command::new("git").arg("add").arg(".").current_dir(&self.repo_path).output();
                    let commit_msg = format!("🤖 Ralph Agent: Completed feature '{}'\n\nSummary: {}", feature_name, result);

                    let _ = Command::new("git").arg("config").arg("user.name").arg("Ralph Agent").current_dir(&self.repo_path).output();
                    let _ = Command::new("git").arg("config").arg("user.email").arg("ralph@onehumancorp.com").current_dir(&self.repo_path).output();

                    if let Err(e) = Command::new("git").arg("commit").arg("-m").arg(&commit_msg).current_dir(&self.repo_path).output() {
                        tracing::error!("Phase 2 failed to git commit: {}", e);
                    }
                }
                Err(e) => {
                    tracing::error!("Ralph Loop failed on feature {}: {}", feature_name, e);
                    progress.notes.push(format!("ERROR on feature {}: {}", feature_name, e));
                    let _ = self.save_progress(&progress).await;
                    break;
                }
            }
        }

        tracing::info!("Ralph Loop completely finished. Task: {}", progress.task_description);
        Ok(())
    }

    async fn initialize(&self, task: &str) -> Result<RalphProgress, Box<dyn std::error::Error + Send + Sync>> {
        if let Ok(data) = fs::read_to_string(&self.progress_file_path).await {
            if let Ok(progress) = serde_json::from_str::<RalphProgress>(&data) {
                tracing::info!("Ralph Loop (Phase 1): Resuming from existing progress file.");
                return Ok(progress);
            }
        }

        tracing::info!("Ralph Loop (Phase 1): Initializing new project environment.");

        // Run git init if not inside a repo
        let git_status = Command::new("git").arg("status").current_dir(&self.repo_path).output();
        if git_status.is_err() || !git_status.unwrap().status.success() {
            let _ = Command::new("git").arg("init").current_dir(&self.repo_path).output();
        }

        let _ = Command::new("git").arg("config").arg("user.name").arg("Ralph Agent").current_dir(&self.repo_path).output();
        let _ = Command::new("git").arg("config").arg("user.email").arg("ralph@onehumancorp.com").current_dir(&self.repo_path).output();

        // 1. Task Breakdown using Initializer Agent (Architect)
        let breakdown_prompt = format!(
            "You are a Project Architect. Break down the following high-level task into a detailed, sequential list of 5-8 atomic features. \
            Each feature must be small enough to implement in a single coding session. \
            Respond strictly with a JSON array of strings representing the feature names.\n\
            Task: {}",
            task
        );

        let mut architect_cfg = self.config.clone();
        architect_cfg.server_system_message.push_str("\nYou are acting as the Project Architect for a Ralph Loop.");

        let mut on_event = |_| {};
        let result = self.agent.run(&architect_cfg, &breakdown_prompt, &mut on_event).await?;

        // Handle JSON array if wrapped in markdown
        let clean_json = result.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
        let mut features = vec![];
        if let Ok(parsed) = serde_json::from_str::<Vec<String>>(clean_json) {
            for name in parsed {
                features.push(Feature { name, status: "pending".to_string() });
            }
        } else {
            tracing::warn!("Ralph Loop: Architect failed to return valid JSON features. Using fallback breakdown.");
            features.push(Feature { name: "Step 1: Scaffolding and Setup".to_string(), status: "pending".to_string() });
            features.push(Feature { name: "Step 2: Core Logic Implementation".to_string(), status: "pending".to_string() });
            features.push(Feature { name: "Step 3: Verification and Polishing".to_string(), status: "pending".to_string() });
        }

        let progress = RalphProgress {
            task_description: task.to_string(),
            features,
            current_feature_index: 0,
            notes: vec![format!("Initialized task breakdown for: {}", task)],
            architectural_decisions: vec!["Project initialized via Ralph Loop architecture.".to_string()],
            unresolved_bugs: vec![],
            session_id: uuid::Uuid::new_v4().to_string(),
            is_complete: false,
        };

        // 2. Automatically generate environment artifacts: init.sh and .gitignore
        let init_script_path = self.repo_path.join("init.sh");
        if !init_script_path.exists() {
            let script_content = format!("#!/bin/bash\n# Ralph Loop Auto-Generated Init Script\n# Task: {}\necho 'Initializing project environment...'\n", task);
            let _ = fs::write(&init_script_path, script_content).await;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(mut perms) = fs::metadata(&init_script_path).await.map(|m| m.permissions()) {
                    perms.set_mode(0o755);
                    let _ = fs::set_permissions(&init_script_path, perms).await;
                }
            }
        }

        let gitignore_path = self.repo_path.join(".gitignore");
        if !gitignore_path.exists() {
            let _ = fs::write(&gitignore_path, ".agent_progress_*.json\n.scratchpad_*.json\nnode_modules/\ntarget/\n.env\n").await;
        }

        self.save_progress(&progress).await?;

        // 3. Create the initial "Baseline" git commit
        let _ = Command::new("git").arg("add").arg(".").current_dir(&self.repo_path).output();
        let _ = Command::new("git").arg("commit").arg("-m").arg("🏁 Ralph Loop: Clean Slate Baseline Commit").current_dir(&self.repo_path).output();

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
}
