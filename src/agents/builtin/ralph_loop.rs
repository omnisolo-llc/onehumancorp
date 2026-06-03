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
    pub is_complete: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Feature {
    pub name: String,
    pub status: String, // "pending", "in_progress", "completed"
}

/// Represents the explicitly delineated phases of The Ralph Loop.
#[derive(Debug, PartialEq, Eq)]
pub enum RalphPhase {
    /// Phase 1 (Initializer Agent): Sets up environment, writes init script, progress file, feature list, and makes initial git commit.
    Phase1Initialize,
    /// Phase 2 (Coding Agent): Reads git logs and progress files to orient itself, picks the highest-priority incomplete feature, works, commits.
    Phase2Coding,
}

/// The "Ralph Loop": For long-running asynchronous tasks spanning multiple context windows.
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

    /// Determines the current phase based on the presence and state of the progress file and git history.
    async fn determine_phase(&self) -> RalphPhase {
        if let Ok(data) = fs::read_to_string(&self.progress_file_path).await {
            if serde_json::from_str::<RalphProgress>(&data).is_ok() {
                // If progress file exists and is valid, we are in Phase 2.
                return RalphPhase::Phase2Coding;
            }
        }
        // Fallback or missing progress file means we must initialize.
        RalphPhase::Phase1Initialize
    }

    /// Run the full Ralph Loop
    pub async fn run(&self, initial_task: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let phase = self.determine_phase().await;

        let mut progress = match phase {
            RalphPhase::Phase1Initialize => {
                tracing::info!("Ralph Loop executing Phase 1: Initialize.");
                self.execute_phase1(initial_task).await?
            }
            RalphPhase::Phase2Coding => {
                tracing::info!("Ralph Loop executing Phase 2: Resume Coding.");
                let data = fs::read_to_string(&self.progress_file_path).await?;
                serde_json::from_str::<RalphProgress>(&data)?
            }
        };

        // Phase 2: Coding Agent Loop
        while !progress.is_complete {
            progress = self.execute_phase2(progress).await?;
        }

        tracing::info!("Ralph Loop completely finished.");
        Ok(())
    }

    /// Phase 1: Initialize the repository, breakdown features, and create initial commits.
    async fn execute_phase1(&self, task: &str) -> Result<RalphProgress, Box<dyn std::error::Error + Send + Sync>> {
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

        // Setup environment, write init script, initial git commit.
        let init_script_path = self.repo_path.join("init.sh");
        if !init_script_path.exists() {
            let _ = fs::write(&init_script_path, "#!/bin/bash
# Ralph Loop Init Script
").await;
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

    /// Phase 2: Execute the next available feature, using the repository context for orientation.
    async fn execute_phase2(&self, mut progress: RalphProgress) -> Result<RalphProgress, Box<dyn std::error::Error + Send + Sync>> {
        if progress.current_feature_index >= progress.features.len() {
            progress.is_complete = true;
            self.save_progress(&progress).await?;
            return Ok(progress);
        }

        let feature_name = progress.features[progress.current_feature_index].name.clone();
        if progress.features[progress.current_feature_index].status == "completed" {
            progress.current_feature_index += 1;
            return Ok(progress);
        }

        tracing::info!("Ralph Loop: Starting work on feature: {}", feature_name);

        progress.features[progress.current_feature_index].status = "in_progress".to_string();
        self.save_progress(&progress).await?;

        // Read git logs to orient itself
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

        // Read progress file to orient itself
        let progress_file_content = match fs::read_to_string(&self.progress_file_path).await {
            Ok(content) => content,
            Err(_) => "Progress file content unavailable".to_string(),
        };

        let mut pending_features = Vec::new();
        for (i, f) in progress.features.iter().enumerate() {
            if f.status == "pending" {
                pending_features.push(format!("{}. {}", i + 1, f.name));
            }
        }
        let pending_features_list = pending_features.join("
");

        let feature_prompt = format!(
            "You are executing Phase 2 of a long-running task.
Overall Task: {}
Recent Git History:
{}
Progress File Content:
{}
Remaining Features:
{}
Feature to implement now (Highest Priority): {}
Execute steps to complete this feature, verify it, and then stop.",
            progress.task_description, git_log_output, progress_file_content, pending_features_list, feature_name
        );

        // We use a fresh config to keep the context window small (compaction/reset)
        let mut feature_config = self.config.clone();
        let scratchpad_context = if !progress.notes.is_empty() {
            format!("
Structured Scratchpad Notes:
- {}", progress.notes.join("
- "))
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

                let _ = Command::new("git").arg("config").arg("user.name").arg("Ralph Agent").current_dir(&self.repo_path).output();
                let _ = Command::new("git").arg("config").arg("user.email").arg("ralph@example.com").current_dir(&self.repo_path).output();

                if Command::new("git").arg("add").arg(".").current_dir(&self.repo_path).output().is_ok() {
                    let commit_msg = format!("Completed feature: {}

{}", feature_name, result);
                    if let Err(e) = Command::new("git").arg("commit").arg("-m").arg(&commit_msg).current_dir(&self.repo_path).output() {
                        tracing::error!("Phase 2 failed to git commit: {}", e);
                    }
                }
            }
            Err(e) => {
                tracing::error!("Ralph Loop failed on feature {}: {}", feature_name, e);
                // Return error to bubble it up, stopping loop execution.
                return Err(e.into());
            }
        }

        Ok(progress)
    }

    async fn save_progress(&self, progress: &RalphProgress) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let json = serde_json::to_string_pretty(progress)?;
        let tmp_path = format!("{}.tmp", self.progress_file_path);
        fs::write(&tmp_path, json).await?;
        if let Err(e) = fs::rename(&tmp_path, &self.progress_file_path).await {
            tracing::error!("Failed to rename progress file: {}", e);
        }
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
    async fn test_ralph_loop_updates_progress() {
        let dir = tempdir().unwrap();
        let progress_file = dir.path().join("progress.json");
        let progress_file_str = progress_file.to_str().unwrap();

        let llm = Arc::new(TestLlmClient { call_count: tokio::sync::Mutex::new(0) });
        let agent = Arc::new(Agent::new(llm, vec![]));
        let config = AgentRunConfig::default();

        let ralph = RalphLoop::new(agent, config, progress_file_str);

        let result = ralph.run("Build a small feature").await;
        assert!(result.is_ok());

        assert!(progress_file.exists());
        let saved_progress_str = std::fs::read_to_string(&progress_file).unwrap();
        let saved_progress: RalphProgress = serde_json::from_str(&saved_progress_str).unwrap();

        // "Feat1" is returned by the mock as the first feature.
        assert!(saved_progress.notes.iter().any(|note| note.contains("Completed feature Feat1: Feature implemented")));
    }

    #[tokio::test]
    async fn test_ralph_loop_orientation_prompt_injection() {
        struct OrientationPromptCaptureLlm {
            call_count: tokio::sync::Mutex<usize>,
            captured_prompts: tokio::sync::Mutex<Vec<String>>,
        }

        #[async_trait::async_trait]
        impl LlmClient for OrientationPromptCaptureLlm {
            async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                let mut count = self.call_count.lock().await;
                *count += 1;

                let mut prompts = self.captured_prompts.lock().await;
                if let Some(msg) = req.messages.first() {
                    prompts.push(msg.content.clone());
                }

                if *count == 1 {
                    Ok(ChatResponse {
                        message: Message::assistant(r#"["Feat1"]"#),
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

        let dir = tempdir().unwrap();
        let progress_file = dir.path().join("progress.json");
        let repo_dir = dir.path().to_path_buf();
        let progress_file_str = progress_file.to_str().unwrap();

        // Setup git repo with a commit
        Command::new("git").arg("init").current_dir(&repo_dir).output().unwrap();
        Command::new("git").arg("config").arg("user.name").arg("Test").current_dir(&repo_dir).output().unwrap();
        Command::new("git").arg("config").arg("user.email").arg("test@example.com").current_dir(&repo_dir).output().unwrap();
        std::fs::write(repo_dir.join("test.txt"), "hello").unwrap();
        Command::new("git").arg("add").arg(".").current_dir(&repo_dir).output().unwrap();
        Command::new("git").arg("commit").arg("-m").arg("Test commit msg xyz").current_dir(&repo_dir).output().unwrap();

        let llm = Arc::new(OrientationPromptCaptureLlm {
            call_count: tokio::sync::Mutex::new(0),
            captured_prompts: tokio::sync::Mutex::new(Vec::new()),
        });

        let agent = Arc::new(Agent::new(llm.clone(), vec![]));
        let config = AgentRunConfig::default();

        let ralph = RalphLoop::new(agent, config, progress_file_str);

        let result = ralph.run("Build orientation test feature").await;
        assert!(result.is_ok());

        let prompts = llm.captured_prompts.lock().await;
        // The first prompt is the initialize/breakdown prompt.
        // The second prompt is the Phase 2 orientation prompt.
        assert!(prompts.len() >= 2);

        let phase2_prompt = &prompts[1];

        // Assert git log is injected
        assert!(phase2_prompt.contains("Recent Git History:"));
        assert!(phase2_prompt.contains("Test commit msg xyz"));

        // Assert progress file content is injected
        assert!(phase2_prompt.contains("Progress File Content:"));
        assert!(phase2_prompt.contains(r#""task_description": "Build orientation test feature""#));
        assert!(phase2_prompt.contains(r#""name": "Feat1""#));
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
    async fn test_ralph_loop_multi_session_integration() {
        let dir = tempdir().unwrap();
        let repo_dir = dir.path().join("repo");
        std::fs::create_dir_all(&repo_dir).unwrap();

        // Git init to simulate real multi-session environment
        let _ = std::process::Command::new("git").arg("init").current_dir(&repo_dir).output().unwrap();

        let progress_file = repo_dir.join("progress.json");

        // Setup partially complete progress file
        let initial_progress = RalphProgress {
            task_description: "Build a web server".to_string(),
            features: vec![
                Feature { name: "Phase 1: Setup".to_string(), status: "completed".to_string() },
                Feature { name: "Phase 2: Router".to_string(), status: "pending".to_string() },
                Feature { name: "Phase 3: Database".to_string(), status: "pending".to_string() },
            ],
            current_feature_index: 1, // Skip Phase 1
            notes: vec!["Completed Phase 1".to_string()],
            is_complete: false,
        };
        std::fs::write(&progress_file, serde_json::to_string(&initial_progress).unwrap()).unwrap();

        struct MultiSessionLlmClient {
            call_count: tokio::sync::Mutex<usize>,
        }
        #[async_trait::async_trait]
        impl LlmClient for MultiSessionLlmClient {
            async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                let mut count = self.call_count.lock().await;
                *count += 1;
                Ok(ChatResponse {
                    message: Message::assistant(format!("Implemented feature session {}", count)),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some(format!("id{}", count)),
                })
            }
        }

        let llm = Arc::new(MultiSessionLlmClient { call_count: tokio::sync::Mutex::new(0) });
        let agent = Arc::new(Agent::new(llm, vec![]));
        let config = AgentRunConfig::default();

        let ralph = RalphLoop::new(agent, config, progress_file.to_str().unwrap());

        // Run the ralph loop which should pick up from index 1 and complete index 1 and 2
        let result = ralph.run("Build a web server").await;
        assert!(result.is_ok());

        let saved_progress_str = std::fs::read_to_string(&progress_file).unwrap();
        let saved_progress: RalphProgress = serde_json::from_str(&saved_progress_str).unwrap();

        assert!(saved_progress.is_complete);
        assert_eq!(saved_progress.features.len(), 3);
        assert_eq!(saved_progress.features[0].status, "completed");
        assert_eq!(saved_progress.features[1].status, "completed");
        assert_eq!(saved_progress.features[2].status, "completed");
        assert!(saved_progress.notes.iter().any(|n| n.contains("Implemented feature session 1")));
        assert!(saved_progress.notes.iter().any(|n| n.contains("Implemented feature session 2")));
    }
}
