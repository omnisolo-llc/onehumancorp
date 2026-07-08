/// Master Catalog B.11. Subagent Orchestration: Worktree execution model
use crate::agent::{Agent, AgentRunConfig};
use crate::types::Message;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::process::Command;

/// Subagent Orchestration: Claude Code Execution Models
/// 1) Fork (byte-identical copy of parent context)
/// 2) Teammate (separate terminal pane communicating via file-based mailboxes)
/// 3) Worktree (spawns its own git worktree with an isolated branch)
///
/// Rule: Subagents return 1k-2k token condensed summaries, never their full context loop.
#[derive(Debug, Clone)]
pub enum ClaudeSubagentMode {
    Fork,
    Teammate {
        mailbox_dir: PathBuf,
    },
    // Subagent Orchestration: Worktree branch
    Worktree {
        base_repo_path: PathBuf,
        branch_name: String,
        auto_cleanup: bool,
        auto_merge_on_success: bool,
    },
}

pub struct ClaudeSubagentSpawner {
    pub parent_llm: Arc<dyn crate::llm::LlmClient>,
    pub subagent: Arc<Agent>,
    pub mode: ClaudeSubagentMode,
}

impl ClaudeSubagentSpawner {
    pub fn new(
        parent_llm: Arc<dyn crate::llm::LlmClient>,
        subagent: Arc<Agent>,
        mode: ClaudeSubagentMode,
    ) -> Self {
        Self {
            parent_llm,
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
            ClaudeSubagentMode::Teammate { mailbox_dir } => {
                // Teammate: communicates via file-based mailboxes (separate terminal pane)
                if !mailbox_dir.exists() {
                    fs::create_dir_all(mailbox_dir).await?;
                }
                let in_mbox = mailbox_dir.join("inbox.txt");
                let out_mbox = mailbox_dir.join("outbox.txt");

                fs::write(&in_mbox, task).await?;
                // Ensure outbox is clear before starting
                if out_mbox.exists() {
                    let _ = fs::remove_file(&out_mbox).await;
                }

                // Inject mailbox instruction
                let mut mailbox_instructions = config.developer_instructions.clone();
                mailbox_instructions.push_str(&format!(
                    "\n[Teammate Mode] You have an inbox at {} and outbox at {}. Read inbox for task details and write final results to outbox.",
                    in_mbox.display(), out_mbox.display()
                ));
                sub_config.developer_instructions = mailbox_instructions;

                // Clone spawner properties for the background task
                let self_clone = Self {
                    parent_llm: self.parent_llm.clone(),
                    subagent: self.subagent.clone(),
                    mode: self.mode.clone(),
                };
                let task_clone = task.to_string();
                let sub_config_clone = sub_config.clone();
                let out_mbox_clone = out_mbox.clone();

                // Spawn a background OS thread to represent the separate terminal pane.
                // We use a new current-thread tokio runtime because the subagent.run future is !Send.
                let _handle = std::thread::spawn(move || {
                    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
                    rt.block_on(async {
                        let result = match self_clone.execute_and_summarize(&task_clone, &sub_config_clone).await {
                            Ok(res) => res,
                            Err(e) => format!("Subagent failed: {}", e),
                        };
                        // Save result to outbox safely using an atomic rename
                        let tmp_file = out_mbox_clone.with_extension("tmp");
                        let _ = fs::write(&tmp_file, &result).await;
                        let _ = fs::rename(&tmp_file, &out_mbox_clone).await;
                    });
                });

                // Polling loop in the main thread (acts as the caller waiting for the teammate)
                let timeout_duration = std::time::Duration::from_secs(300); // 5 minutes timeout
                let poll_interval = std::time::Duration::from_millis(100);
                let start_time = std::time::Instant::now();

                loop {
                    if start_time.elapsed() > timeout_duration {
                        return Err("Teammate execution timed out waiting for outbox response".into());
                    }

                    if fs::try_exists(&out_mbox).await.unwrap_or(false) {
                        let result = fs::read_to_string(&out_mbox).await?;
                        return Ok(result);
                    }

                    tokio::time::sleep(poll_interval).await;
                }
            }
            ClaudeSubagentMode::Worktree {
                base_repo_path,
                branch_name,
                auto_cleanup,
                auto_merge_on_success,
            } => {
                // Worktree: spawns its own git worktree with an isolated branch
                let worktree_dir = base_repo_path
                    .parent()
                    .unwrap_or(Path::new("/tmp"))
                    .join(format!("worktree_{}", branch_name));

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
                    return Err(format!(
                        "Failed to create worktree: {}",
                        String::from_utf8_lossy(&output.stderr)
                    )
                    .into());
                }

                // Instruct agent to use worktree dir
                let mut worktree_instructions = config.developer_instructions.clone();
                worktree_instructions.push_str(&format!(
                    "\n[Worktree Mode] You are operating in an isolated git worktree at {}. Make your changes and commit them here.",
                    worktree_dir.display()
                ));
                sub_config.developer_instructions = worktree_instructions;
                sub_config.project_trusted = true; // Typically worktrees require trust to commit.
                sub_config.workspace_path = Some(worktree_dir.to_string_lossy().to_string());

                let result = self.execute_and_summarize(task, &sub_config).await?;

                if *auto_merge_on_success {
                    // Merge branch into main repo
                    let merge_output = Command::new("git")
                        .arg("merge")
                        .arg(branch_name)
                        .current_dir(base_repo_path)
                        .output()
                        .await?;
                    if !merge_output.status.success() {
                        return Err(format!(
                            "Failed to merge worktree branch: {}",
                            String::from_utf8_lossy(&merge_output.stderr)
                        )
                        .into());
                    }
                }

                if *auto_cleanup {
                    // Cleanup worktree
                    let cleanup_output = Command::new("git")
                        .arg("worktree")
                        .arg("remove")
                        .arg("--force")
                        .arg(&worktree_dir)
                        .current_dir(base_repo_path)
                        .output()
                        .await?;
                    if !cleanup_output.status.success() {
                        return Err(format!(
                            "Failed to cleanup worktree: {}",
                            String::from_utf8_lossy(&cleanup_output.stderr)
                        )
                        .into());
                    }

                    if *auto_merge_on_success {
                        let branch_del_output = Command::new("git")
                            .arg("branch")
                            .arg("-D")
                            .arg(branch_name)
                            .current_dir(base_repo_path)
                            .output()
                            .await?;
                        if !branch_del_output.status.success() {
                            tracing::warn!(
                                "Failed to delete worktree branch after cleanup: {}",
                                String::from_utf8_lossy(&branch_del_output.stderr)
                            );
                        }
                    }
                }

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
        let raw_output: String = loop {
            match Box::pin(self.subagent.run(config, task, &mut on_event)).await {
                Ok(res) => break res,
                Err(e) => {
                    retry_count += 1;
                    if retry_count >= max_retries {
                        return Err(e);
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(backoff)).await;
                    backoff *= 2;
                }
            }
        };
        let _duration = start_time.elapsed().as_secs_f64();

        self.summarize_output(&raw_output, config).await
    }

    async fn summarize_output(
        &self,
        raw_output: &str,
        config: &AgentRunConfig,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Implement Condensation Algorithm: Subagent condensed summaries (1k-2k tokens)
        // 1 token ≈ 4 characters. Target 1k-2k tokens = 4000-8000 characters.
        const TARGET_CHARS_MAX: usize = 8000;
        const CHUNK_SIZE_CHARS: usize = 20000; // ~5k tokens per chunk for processing

        let mut current_text = raw_output.to_string();

        let system_prompt = "You are an expert summarizer. Compress the following subagent execution result into a dense 1k-2k token summary. Preserve all key decisions, code changes, and unresolved issues. Do not include raw context loops.";

        while current_text.len() > TARGET_CHARS_MAX {
            let mut next_text_parts = Vec::new();

            // Chunk current_text if it's very large
            let chars: Vec<char> = current_text.chars().collect();
            let mut i = 0;
            while i < chars.len() {
                let end = std::cmp::min(i + CHUNK_SIZE_CHARS, chars.len());
                let chunk: String = chars[i..end].iter().collect();

                let req = ohc_builtin_agent_core::types::ChatRequest {
                    model: config.model.clone(),
                    system: ::server_pricing::compression::reduce_tokens(system_prompt),
                    messages: vec![ohc_builtin_agent_core::types::Message::user(chunk)],
                    tools: vec![],
                    max_tokens: 2000,
                    temperature: 0.0,
                };
                let resp = self.parent_llm.chat(req).await?;
                next_text_parts.push(resp.message.content);

                i += CHUNK_SIZE_CHARS;
            }

            let next_text = next_text_parts.join("\n\n");

            // If condensation didn't reduce size (e.g. LLM ignored instructions or hit a limit),
            // prevent infinite loop by breaking and returning the current best effort.
            if next_text.len() >= current_text.len() {
                tracing::warn!("Condensation loop failed to reduce text size. Stopping early.");
                current_text = next_text;
                break;
            }

            current_text = next_text;
        }

        // If it was small enough to begin with, still ensure it's a summary rather than raw output,
        // UNLESS it's already very small, then we might still want to just return it or summarize it.
        // The instructions say: "Subagents return 1k-2k token condensed summaries".
        // Let's do one final pass if it wasn't condensed yet (i.e. length was <= TARGET_CHARS_MAX but we still want a clean summary).
        if raw_output.len() == current_text.len() && current_text.len() > 1000 {
            let req = ohc_builtin_agent_core::types::ChatRequest {
                model: config.model.clone(),
                system: ::server_pricing::compression::reduce_tokens(system_prompt),
                messages: vec![ohc_builtin_agent_core::types::Message::user(current_text)],
                tools: vec![],
                max_tokens: 2000,
                temperature: 0.0,
            };
            let resp = self.parent_llm.chat(req).await?;
            current_text = resp.message.content;
        }

        if current_text.len() > TARGET_CHARS_MAX {
            current_text = format!(
                "{}

[Output truncated. Subagent failed to condense summary.]",
                current_text
                    .chars()
                    .take(TARGET_CHARS_MAX)
                    .collect::<String>()
            );
        }

        Ok(current_text)
    }
}

#[cfg(test)]
mod tests {

    struct MockLlmClient {
        responses: std::sync::Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl crate::llm::LlmClient for MockLlmClient {
        async fn chat(
            &self,
            _req: ohc_builtin_agent_core::types::ChatRequest,
        ) -> Result<
            ohc_builtin_agent_core::types::ChatResponse,
            Box<dyn std::error::Error + Send + Sync>,
        > {
            let mut resps = self.responses.lock().unwrap();
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "default".to_string()
            };

            let message = ohc_builtin_agent_core::types::Message {
                role: ohc_builtin_agent_core::types::Role::Assistant,
                content,
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            };

            Ok(ohc_builtin_agent_core::types::ChatResponse {
                message,
                usage: Default::default(),
                response_id: None,
                stop_reason: "stop".to_string(),
            })
        }
    }

    #[tokio::test]
    async fn test_claude_subagent_summarize_condensation_fails() {
        struct BadLlmClient;

        #[async_trait::async_trait]
        impl crate::llm::LlmClient for BadLlmClient {
            async fn chat(
                &self,
                _req: ohc_builtin_agent_core::types::ChatRequest,
            ) -> Result<
                ohc_builtin_agent_core::types::ChatResponse,
                Box<dyn std::error::Error + Send + Sync>,
            > {
                let message = ohc_builtin_agent_core::types::Message {
                    role: ohc_builtin_agent_core::types::Role::Assistant,
                    content: "A".repeat(9000), // always returns > 8000
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                };

                Ok(ohc_builtin_agent_core::types::ChatResponse {
                    message,
                    usage: Default::default(),
                    response_id: None,
                    stop_reason: "stop".to_string(),
                })
            }
        }

        let parent_client = std::sync::Arc::new(BadLlmClient);
        let _parent_agent = std::sync::Arc::new(Agent::new(parent_client.clone(), vec![]));

        let sub_client = std::sync::Arc::new(MockLlmClient {
            responses: std::sync::Mutex::new(vec![]),
        });
        let subagent = std::sync::Arc::new(Agent::new(sub_client, vec![]));

        let spawner =
            ClaudeSubagentSpawner::new(parent_client.clone(), subagent, ClaudeSubagentMode::Fork);

        let large_input = "A".repeat(25000);
        let config = AgentRunConfig::default();

        let result = spawner
            .summarize_output(&large_input, &config)
            .await
            .unwrap();

        assert!(result.len() > 8000);
        assert!(result.contains("[Output truncated. Subagent failed to condense summary.]"));
    }

    use super::*;

    #[tokio::test]
    async fn test_claude_subagent_fork() {
        let parent_client = Arc::new(MockLlmClient {
            responses: std::sync::Mutex::new(vec!["Condensed summary of fork".to_string()]),
        });
        let _parent_agent = Arc::new(Agent::new(parent_client.clone(), vec![]));

        let long_output = "Long raw output from subagent in fork mode...".repeat(100);
        let sub_client = Arc::new(MockLlmClient {
            responses: std::sync::Mutex::new(vec![
                long_output.clone(),
            ]),
        });
        let subagent = Arc::new(Agent::new(sub_client, vec![]));

        let spawner =
            ClaudeSubagentSpawner::new(parent_client.clone(), subagent, ClaudeSubagentMode::Fork);

        let parent_context = vec![Message::user("Parent context message")];
        let config = AgentRunConfig::default();

        let result = spawner
            .run_subagent("Do task", &parent_context, &config)
            .await
            .unwrap();
        assert_eq!(result, "Condensed summary of fork");
    }

    #[tokio::test]
    async fn test_claude_subagent_teammate() {
        let parent_client = Arc::new(MockLlmClient {
            responses: std::sync::Mutex::new(vec!["Condensed summary of teammate".to_string()]),
        });
        let _parent_agent = Arc::new(Agent::new(parent_client.clone(), vec![]));

        let long_output = "Long raw output from teammate...".repeat(100);
        let sub_client = Arc::new(MockLlmClient {
            responses: std::sync::Mutex::new(vec![long_output.clone()]),
        });
        let subagent = Arc::new(Agent::new(sub_client, vec![]));

        let dir = tempfile::tempdir().unwrap();
        let mailbox_dir = dir.path().join("mailboxes");

        let spawner = ClaudeSubagentSpawner::new(
            parent_client.clone(),
            subagent,
            ClaudeSubagentMode::Teammate {
                mailbox_dir: mailbox_dir.clone(),
            },
        );

        let config = AgentRunConfig::default();
        let result = spawner.run_subagent("Do task", &[], &config).await.unwrap();

        assert_eq!(result, "Condensed summary of teammate");
        assert!(mailbox_dir.join("inbox.txt").exists());
        assert!(mailbox_dir.join("outbox.txt").exists());

        let out_content = fs::read_to_string(mailbox_dir.join("outbox.txt"))
            .await
            .unwrap();
        assert_eq!(out_content, "Condensed summary of teammate");
    }

    #[tokio::test]
    async fn test_claude_subagent_worktree() {
        let parent_client = Arc::new(MockLlmClient {
            responses: std::sync::Mutex::new(vec!["Condensed summary of worktree".to_string()]),
        });
        let _parent_agent = Arc::new(Agent::new(parent_client.clone(), vec![]));

        let long_output = "Long raw output from worktree...".repeat(100);
        let sub_client = Arc::new(MockLlmClient {
            responses: std::sync::Mutex::new(vec![long_output.clone()]),
        });
        let subagent = Arc::new(Agent::new(sub_client, vec![]));

        // Create a dummy git repo
        let dir = tempfile::tempdir().unwrap();
        let repo_dir = dir.path().join("test_repo");
        fs::create_dir_all(&repo_dir).await.unwrap();

        Command::new("git")
            .arg("init")
            .current_dir(&repo_dir)
            .output()
            .await
            .unwrap();
        fs::write(repo_dir.join("test.txt"), "hello").await.unwrap();
        Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(&repo_dir)
            .output()
            .await
            .unwrap();
        Command::new("git")
            .arg("config")
            .arg("user.name")
            .arg("Test")
            .current_dir(&repo_dir)
            .output()
            .await
            .unwrap();
        Command::new("git")
            .arg("config")
            .arg("user.email")
            .arg("test@test.com")
            .current_dir(&repo_dir)
            .output()
            .await
            .unwrap();
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("init")
            .current_dir(&repo_dir)
            .output()
            .await
            .unwrap();

        let spawner = ClaudeSubagentSpawner::new(
            parent_client.clone(),
            subagent,
            ClaudeSubagentMode::Worktree {
                base_repo_path: repo_dir.clone(),
                branch_name: "subagent-branch".to_string(),
                auto_cleanup: false,
                auto_merge_on_success: false,
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
    #[tokio::test]
    async fn test_claude_subagent_worktree_cleanup_and_merge() {
        let parent_client = Arc::new(MockLlmClient {
            responses: std::sync::Mutex::new(vec!["Condensed summary of worktree".to_string()]),
        });
        let _parent_agent = Arc::new(Agent::new(parent_client.clone(), vec![]));

        let long_output = "Long raw output from worktree...".repeat(100);
        let sub_client = Arc::new(MockLlmClient {
            responses: std::sync::Mutex::new(vec![long_output.clone()]),
        });
        let subagent = Arc::new(Agent::new(sub_client, vec![]));

        // Create a dummy git repo
        let dir = tempfile::tempdir().unwrap();
        let repo_dir = dir.path().join("test_repo");
        fs::create_dir_all(&repo_dir).await.unwrap();

        Command::new("git")
            .arg("init")
            .current_dir(&repo_dir)
            .output()
            .await
            .unwrap();
        fs::write(repo_dir.join("test.txt"), "hello").await.unwrap();
        Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(&repo_dir)
            .output()
            .await
            .unwrap();
        Command::new("git")
            .arg("config")
            .arg("user.name")
            .arg("Test")
            .current_dir(&repo_dir)
            .output()
            .await
            .unwrap();
        Command::new("git")
            .arg("config")
            .arg("user.email")
            .arg("test@test.com")
            .current_dir(&repo_dir)
            .output()
            .await
            .unwrap();
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("init")
            .current_dir(&repo_dir)
            .output()
            .await
            .unwrap();

        let spawner = ClaudeSubagentSpawner::new(
            parent_client.clone(),
            subagent,
            ClaudeSubagentMode::Worktree {
                base_repo_path: repo_dir.clone(),
                branch_name: "subagent-branch-auto".to_string(),
                auto_cleanup: true,
                auto_merge_on_success: true,
            },
        );

        let config = AgentRunConfig::default();
        let result = spawner.run_subagent("Do task", &[], &config).await.unwrap();

        assert_eq!(result, "Condensed summary of worktree");

        // Verify worktree was created and cleaned up
        let worktree_dir = dir.path().join("worktree_subagent-branch-auto");
        assert!(!worktree_dir.exists());

        // Ensure branch is deleted
        let branch_check = Command::new("git")
            .arg("branch")
            .current_dir(&repo_dir)
            .output()
            .await
            .unwrap();
        let branches = String::from_utf8_lossy(&branch_check.stdout);
        assert!(!branches.contains("subagent-branch-auto"));
    }

    #[tokio::test]
    async fn test_claude_subagent_summarize_condensation() {
        // Create an LLM client that returns progressively smaller chunks,
        // simulating a condensation process.
        struct CondensingLlmClient {
            call_count: std::sync::Mutex<usize>,
        }

        #[async_trait::async_trait]
        impl crate::llm::LlmClient for CondensingLlmClient {
            async fn chat(
                &self,
                _req: ohc_builtin_agent_core::types::ChatRequest,
            ) -> Result<
                ohc_builtin_agent_core::types::ChatResponse,
                Box<dyn std::error::Error + Send + Sync>,
            > {
                let mut count = self.call_count.lock().unwrap();
                *count += 1;

                // Return roughly 3000 chars for chunk condensations to force the loop to condense the combined chunks
                let content = if *count <= 2 {
                    "Chunk summary ".repeat(200) // 200 * 14 = 2800 chars
                } else if *count == 3 {
                    "Final condensed summary".to_string() // Small enough to break loop
                } else {
                    "Unexpected extra call".to_string()
                };

                let message = ohc_builtin_agent_core::types::Message {
                    role: ohc_builtin_agent_core::types::Role::Assistant,
                    content,
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                };

                Ok(ohc_builtin_agent_core::types::ChatResponse {
                    message,
                    usage: Default::default(),
                    response_id: None,
                    stop_reason: "stop".to_string(),
                })
            }
        }

        let parent_client = Arc::new(CondensingLlmClient {
            call_count: std::sync::Mutex::new(0),
        });
        let _parent_agent = Arc::new(Agent::new(parent_client.clone(), vec![]));

        // Subagent won't actually be run, we just need it for the struct.
        let sub_client = Arc::new(MockLlmClient {
            responses: std::sync::Mutex::new(vec![]),
        });
        let subagent = Arc::new(Agent::new(sub_client, vec![]));

        let spawner =
            ClaudeSubagentSpawner::new(parent_client.clone(), subagent, ClaudeSubagentMode::Fork);

        // Generate a 25,000 character string to force chunking (CHUNK_SIZE_CHARS is 20000)
        let large_input = "A".repeat(25000);
        let config = AgentRunConfig::default();

        let result = spawner
            .summarize_output(&large_input, &config)
            .await
            .unwrap();

        // Wait, the combined text was 5600 chars. 5600 < 8000, loop ends.
        // The return value will be the combined text, not "Final condensed summary" because we don't call it a 3rd time.
        // Let's assert it starts with "Chunk summary" and the length is ~5600.
        assert!(result.starts_with("Chunk summary"));
        assert!(result.len() > 5000 && result.len() < 6000);
        assert_eq!(*parent_client.call_count.lock().unwrap(), 2);
    }
}
