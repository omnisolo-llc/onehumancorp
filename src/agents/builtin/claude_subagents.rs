use crate::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent_core::types::Message;
use std::sync::Arc;
use std::path::{Path, PathBuf};
use tokio::process::Command;
use tokio::fs;

/// Subagent Orchestration: Claude Code Execution Models
/// 1) Fork (byte-identical copy of parent context)
/// 2) Teammate (parallel worker via queue)
/// 3) Worktree (spawns its own git worktree with an isolated branch)
///
/// Rule: Subagents return 1k-2k token condensed summaries, never their full context loop.
#[derive(Debug, Clone)]
pub enum ClaudeSubagentMode {
    Fork,
    Teammate { task_id: String },
    Worktree { base_repo_path: PathBuf, branch_name: String },
}

pub struct ClaudeSubagentSpawner {
    pub parent_agent: Arc<Agent>,
    pub subagent: Arc<Agent>,
    pub mode: ClaudeSubagentMode,
}

impl ClaudeSubagentSpawner {
    pub fn new(parent_agent: Arc<Agent>, subagent: Arc<Agent>, mode: ClaudeSubagentMode) -> Self {
        Self {
            parent_agent,
            subagent,
            mode,
        }
    }

    pub async fn run_subagent(
        &self,
        task: &str,
        parent_context: &[Message],
        config: &AgentRunConfig,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut sub_config = config.clone();

        match &self.mode {
            ClaudeSubagentMode::Fork => {
                // Fork: byte-identical copy of parent context
                sub_config.injected_context = Some(parent_context.to_vec());
                self.execute_and_summarize(task, &sub_config).await
            }
            ClaudeSubagentMode::Teammate { task_id } => {
                // Teammate: Enqueues a job using the sub_agent_jobs database queue.
                // We use the REST API as the transport since we might run externally

                let client = reqwest::Client::new();
                let addr = std::env::var("OHC_AGENT_ADDRESS").unwrap_or_else(|_| "http://localhost:3000".to_string());
                let url = format!("{}/api/mesh/v2/broadcast", addr);

                use base64::Engine;
                let payload = serde_json::json!({
                    "topic": "subagent_jobs",
                    "message": {
                        "agent_id": "system",
                        "action": "enqueue",
                        "status": "QUEUED",
                        "payload": base64::engine::general_purpose::STANDARD.encode(
                            serde_json::to_vec(&serde_json::json!({
                                "job_id": task_id,
                                "task": task,
                                "role": "teammate_subagent"
                            })).unwrap()
                        ),
                        "msg_id": task_id.clone()
                    }
                });

                match client.post(&url).json(&payload).send().await {
                    Ok(res) if res.status().is_success() => {
                        Ok(format!("Teammate subagent spawned. Job ID: {}", task_id))
                    }
                    Ok(res) => {
                        Err(format!("Failed to enqueue teammate task. Status: {}", res.status()).into())
                    }
                    Err(e) => {
                        Err(format!("Failed to enqueue teammate task: {}", e).into())
                    }
                }
            }
            ClaudeSubagentMode::Worktree { base_repo_path, branch_name } => {
                // Worktree: spawns its own git worktree with an isolated branch
                let worktree_dir = base_repo_path.parent().unwrap_or(Path::new("/tmp")).join(format!("worktree_{}", branch_name));

                // Setup worktree
                let output = Command::new("git")
                    .arg("worktree")
                    .arg("add")
                    .arg("-b")
                    .arg(branch_name)
                    .arg(&worktree_dir)
                    .current_dir(base_repo_path)
                    .output()
                    .await?;

                if !output.status.success() {
                    return Err(format!("Failed to create worktree: {}", String::from_utf8_lossy(&output.stderr)).into());
                }

                // Instruct agent to use worktree dir
                let mut worktree_instructions = config.developer_instructions.clone();
                worktree_instructions.push_str(&format!(
                    "\n[Worktree Mode] You are operating in an isolated git worktree at {}. Make your changes and commit them here.",
                    worktree_dir.display()
                ));
                sub_config.developer_instructions = worktree_instructions;

                let result = self.execute_and_summarize(task, &sub_config).await?;

                // Cleanup worktree (optional, but good practice to remove after completion or leave for parent to merge)
                // For now, we leave it for the parent to review/merge, just like real Worktree agents.

                Ok(result)
            }
        }
    }

    async fn execute_and_summarize(
        &self,
        task: &str,
        config: &AgentRunConfig,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut on_event = |_| {};

        let mut retry_count = 0;
        let mut backoff = 1;
        let max_retries = 3;

        let start_time = std::time::Instant::now();
        let raw_output = loop {
            match self.subagent.run(config, task, &mut on_event).await {
                Ok(res) => break res,
                Err(e) => {
                    retry_count += 1;
                    if retry_count >= max_retries {
                        ::server_telemetry::record_ohc_sub_agent_failures_total();
                        return Err(e);
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(backoff)).await;
                    backoff *= 2;
                }
            }
        };
        let duration = start_time.elapsed().as_secs_f64();
        ::server_telemetry::record_ohc_sub_agent_execution_duration_seconds(duration);

        // Rule: Subagents return 1k-2k token condensed summaries, never their full context loop.
        let system_prompt = "You are a subagent synthesizer. Condense this subagent output to 1000-2000 tokens while preserving all key decisions, code changes, and unresolved issues.";
        let req = ohc_builtin_agent_core::types::ChatRequest {
            model: config.model.clone(),
            system: system_prompt.to_string(),
            messages: vec![ohc_builtin_agent_core::types::Message::user(raw_output)],
            tools: vec![],
            max_tokens: 2500,
            temperature: 0.0,
        };

        let resp = self.parent_agent.llm.chat(req).await?;
        Ok(resp.message.content)
    }

    async fn summarize_output(
        &self,
        raw_output: &str,
        config: &AgentRunConfig,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let system_prompt = "You are an expert summarizer. Compress the following subagent execution result into a dense 1k-2k token summary. Do not include raw context loops.";
        let req = ohc_builtin_agent_core::types::ChatRequest {
            model: config.model.clone(),
            system: system_prompt.to_string(),
            messages: vec![ohc_builtin_agent_core::types::Message::user(raw_output.to_string())],
            tools: vec![],
            max_tokens: 2000,
            temperature: 0.0,
        };
        let resp = self.parent_agent.llm.chat(req).await?;
        Ok(resp.message.content)
    }

    #[tokio::test]
    async fn test_claude_subagent_fork() {
        let parent_client = Arc::new(MockLlmClient {
            responses: std::sync::Mutex::new(vec![
                "Condensed summary of fork".to_string()
            ]),
        });
        let parent_agent = Arc::new(Agent::new(parent_client, vec![]));

        let sub_client = Arc::new(MockLlmClient {
            responses: std::sync::Mutex::new(vec![
                "Long raw output from subagent in fork mode...".to_string()
            ]),
        });
        let subagent = Arc::new(Agent::new(sub_client, vec![]));

        let spawner = ClaudeSubagentSpawner::new(
            parent_agent,
            subagent,
            ClaudeSubagentMode::Fork,
        );

        let parent_context = vec![Message::user("Parent context message")];
        let config = AgentRunConfig::default();

        let result = spawner.run_subagent("Do task", &parent_context, &config).await.unwrap();
        assert_eq!(result, "Condensed summary of fork");
    }

    #[test]
    fn test_claude_subagent_teammate() {
        // Mock server to test success path
        let mock_server = httpmock::MockServer::start();
        let mock = mock_server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/api/mesh/v2/broadcast");
            then.status(200);
        });

        temp_env::with_vars(vec![("OHC_AGENT_ADDRESS", Some(mock_server.base_url()))], || {
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
                let parent_client = Arc::new(MockLlmClient {
                    responses: std::sync::Mutex::new(vec![
                        "Condensed summary of teammate".to_string()
                    ]),
                });
                let parent_agent = Arc::new(Agent::new(parent_client, vec![]));

                let sub_client = Arc::new(MockLlmClient {
                    responses: std::sync::Mutex::new(vec![
                        "Long raw output from teammate...".to_string()
                    ]),
                });
                let subagent = Arc::new(Agent::new(sub_client, vec![]));

                let spawner = ClaudeSubagentSpawner::new(
                    parent_agent,
                    subagent,
                    ClaudeSubagentMode::Teammate { task_id: "test-id".to_string() },
                );

                let config = AgentRunConfig::default();
                let result = spawner.run_subagent("Do task", &[], &config).await;

                assert!(result.is_ok());
                let msg = result.unwrap();
                assert!(msg.contains("Teammate subagent spawned. Job ID: test-id"));
            });
        });

        mock.assert();
    }

    #[tokio::test]
    async fn test_claude_subagent_worktree() {
        let parent_client = Arc::new(MockLlmClient {
            responses: std::sync::Mutex::new(vec![
                "Condensed summary of worktree".to_string()
            ]),
        });
        let parent_agent = Arc::new(Agent::new(parent_client, vec![]));

        let sub_client = Arc::new(MockLlmClient {
            responses: std::sync::Mutex::new(vec![
                "Long raw output from worktree...".to_string()
            ]),
        });
        let subagent = Arc::new(Agent::new(sub_client, vec![]));

        // Create a dummy git repo
        let dir = tempdir().unwrap();
        let repo_dir = dir.path().join("test_repo");
        fs::create_dir_all(&repo_dir).await.unwrap();

        Command::new("git").arg("init").current_dir(&repo_dir).output().await.unwrap();
        fs::write(repo_dir.join("test.txt"), "hello").await.unwrap();
        Command::new("git").arg("add").arg(".").current_dir(&repo_dir).output().await.unwrap();
        Command::new("git").arg("config").arg("user.name").arg("Test").current_dir(&repo_dir).output().await.unwrap();
        Command::new("git").arg("config").arg("user.email").arg("test@test.com").current_dir(&repo_dir).output().await.unwrap();
        Command::new("git").arg("commit").arg("-m").arg("init").current_dir(&repo_dir).output().await.unwrap();

        let spawner = ClaudeSubagentSpawner::new(
            parent_agent,
            subagent,
            ClaudeSubagentMode::Worktree {
                base_repo_path: repo_dir.clone(),
                branch_name: "subagent-branch".to_string(),
            },
        );

        let config = AgentRunConfig::default();
        let result = spawner.run_subagent("Do task", &[], &config).await.unwrap();

        assert_eq!(result, "Condensed summary of worktree");

        // Verify worktree was created
        let worktree_dir = dir.path().join("worktree_subagent-branch");
        assert!(worktree_dir.exists());
        assert!(worktree_dir.join("test.txt").exists());
    }
}
