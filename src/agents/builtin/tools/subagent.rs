#![allow(clippy::needless_borrow)]
use crate::Tool;
use ohc_builtin_agent_core::types::ToolError;
use server_ohc::agent::service::SubAgentResponse;
use serde_json::json;
use serde::Deserialize;
use super::pydantic::{PydanticToolExecutor, PydanticAdapter};
use std::sync::Arc;




#[derive(Deserialize)]
struct SubagentArgs {
    task: String,
    mode: String,
    #[serde(default)]
    parent_context_file: Option<String>,
}

pub struct SubagentExecutor {
    pub runner: Arc<dyn crate::runner::CommandRunner>,
    pub llm: Option<Arc<dyn ohc_builtin_agent_core::expert_team::ExpertTeamLlmClient>>,
}

impl SubagentExecutor {

    async fn summarize_output(
        &self,
        raw_output: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        const TARGET_CHARS_MAX: usize = 8000;
        const CHUNK_SIZE_CHARS: usize = 20000;

        let mut current_text = raw_output.to_string();

        let system_prompt = "You are an expert summarizer. Compress the following subagent execution result into a dense 1k-2k token summary. Preserve all key decisions, code changes, and unresolved issues. Do not include raw context loops.";

        while current_text.len() > TARGET_CHARS_MAX {
            let mut next_text_parts = Vec::new();

            let chars: Vec<char> = current_text.chars().collect();
            let mut i = 0;
            while i < chars.len() {
                let end = std::cmp::min(i + CHUNK_SIZE_CHARS, chars.len());
                let chunk: String = chars[i..end].iter().collect();

                let req = ohc_builtin_agent_core::types::ChatRequest {
                    model: "gpt-4o-mini".to_string(), // Default fallback model
                    system: ::server_pricing::compression::reduce_tokens(&system_prompt),
                    messages: vec![ohc_builtin_agent_core::types::Message::user(chunk)],
                    tools: vec![],
                    max_tokens: 2000,
                    temperature: 0.0,
                };
                let resp = if let Some(l) = &self.llm { l.chat(req).await? } else { return Err("LLM client not available for condensation".into()) };
                next_text_parts.push(resp.message.content);

                i += CHUNK_SIZE_CHARS;
            }

            let next_text = next_text_parts.join("

");

            if next_text.len() >= current_text.len() {
                tracing::warn!("Condensation loop failed to reduce text size. Stopping early.");
                current_text = next_text;
                break;
            }

            current_text = next_text;
        }

        if raw_output.len() == current_text.len() && current_text.len() > 1000 {
            let req = ohc_builtin_agent_core::types::ChatRequest {
                model: "gpt-4o-mini".to_string(),
                system: ::server_pricing::compression::reduce_tokens(&system_prompt),
                messages: vec![ohc_builtin_agent_core::types::Message::user(current_text)],
                tools: vec![],
                max_tokens: 2000,
                temperature: 0.0,
            };
            let resp = if let Some(l) = &self.llm { l.chat(req).await? } else { return Err("LLM client not available for condensation".into()) };
            current_text = resp.message.content;
        }

        if current_text.len() > TARGET_CHARS_MAX {
            current_text = format!(
                "{}

[Output truncated. Subagent failed to condense summary.]",
                current_text.chars().take(TARGET_CHARS_MAX).collect::<String>()
            );
        }

        Ok(current_text)
    }
}

#[async_trait::async_trait]
impl PydanticToolExecutor<SubagentArgs> for SubagentExecutor {
    async fn execute_typed(&self, args: SubagentArgs) -> Result<String, ToolError> {
        let raw_task = args.task.clone();
        let mode = args.mode.clone();
        
        if raw_task.is_empty() {
            return Err(ToolError::LlmRecoverable("Task cannot be empty".to_string()));
        }

        let task = format!("{}\n\nCRITICAL INSTRUCTION: You are a subagent. When you finish your work, you MUST return a 1k-2k token condensed summary of your findings and actions. NEVER return your full context loop or raw unsummarized output.", raw_task);

        // Master Catalog B.11. Subagent Orchestration: Worktree execution model. Rule: Subagents return 1k-2k token condensed summaries, never their full context loop.
        tracing::info!("Spawning subagent in mode '{}' for task: {}", mode, raw_task);

        if mode == "fork" {
            let parent_context_file = args.parent_context_file.clone().unwrap_or_default();

            let mut envs = vec![];
            if let Ok(addr) = std::env::var("OHC_AGENT_ADDRESS") {
                envs.push(("OHC_AGENT_ADDRESS".to_string(), addr));
            }

            let output = self.runner.run("ohc_builtin_agent", &["--task", &task, "--parent-context-file", &parent_context_file], None, envs).await;

            // Clean up the temporary context file
            if !parent_context_file.is_empty() {
                let _ = tokio::fs::remove_file(parent_context_file).await;
            }

            let res = match output {
                Ok(out) => {
                    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                    if out.status.success() {
                        Ok(SubAgentResponse {
                            result: stdout,
                            error: String::new(),
                        })
                    } else {
                        Err(format!("Process failed: {}", stderr))
                    }
                }
                Err(e) => Err(format!("Runner failed: {}", e)),
            };

            match res {
                Ok(inner) => {
                    if !inner.error.is_empty() {
                        Err(ToolError::LlmRecoverable(inner.error))
                    } else {
                        let summary = self.summarize_output(&inner.result).await.unwrap_or_else(|e| format!("Failed to summarize: {}

{}", e, inner.result));
                        Ok(format!("[Subagent (Fork)] Completed task: {}. Summary: {}", task, summary))
                    }
                }
                Err(e) => Err(ToolError::LlmRecoverable(format!("Subagent failed: {}", e))),
            }
        } else if mode == "teammate" {
            let task_id = uuid::Uuid::new_v4().to_string();
            let mailbox_dir = format!(".agent-mailboxes/subagent-{}", task_id);
            if let Err(e) = tokio::fs::create_dir_all(&mailbox_dir).await {
                return Err(ToolError::LlmRecoverable(format!("Failed to create mailbox directory: {}", e)));
            }

            let inbox_path = format!("{}/inbox.txt", mailbox_dir);
            let outbox_path = format!("{}/outbox.txt", mailbox_dir);

            // We use the augmented `task` which already includes the 1k-2k token condensed summary rule
            if let Err(e) = tokio::fs::write(&inbox_path, &task).await {
                return Err(ToolError::LlmRecoverable(format!("Failed to write to inbox: {}", e)));
            }

            let teammate_task = format!(
                "You are a teammate subagent. Your task is: {}
When finished or if you need to report progress, write your final summary to {}. To receive further instructions, read from {}.",
                task, outbox_path, inbox_path
            );

            let runner_clone = self.runner.clone();
            let outbox_path_clone = outbox_path.clone();
            let mailbox_dir_clone = mailbox_dir.clone();
            let executor_clone = Self { runner: self.runner.clone(), llm: self.llm.clone() };

            let mut envs = vec![];
            if let Ok(addr) = std::env::var("OHC_AGENT_ADDRESS") {
                envs.push(("OHC_AGENT_ADDRESS".to_string(), addr));
            }

            tokio::spawn(async move {
                let output = runner_clone.run("ohc_builtin_agent", &["--task", &teammate_task, "--mailbox", &mailbox_dir_clone], None, envs).await;

                let res = match output {
                    Ok(out) => {
                        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                        if out.status.success() {
                            executor_clone.summarize_output(&stdout).await.unwrap_or_else(|e| format!("Failed to summarize: {}

{}", e, stdout))
                        } else {
                            format!("Subagent error: {}", stderr)
                        }
                    }
                    Err(e) => format!("Subagent failed: {}", e),
                };

                use tokio::io::AsyncWriteExt;
                if let Ok(mut file) = tokio::fs::OpenOptions::new().create(true).append(true).open(&outbox_path_clone).await {
                    let _ = file.write_all(format!("
[System: Subagent Process Terminated]
Final Result: {}", res).as_bytes()).await;
                }
            });

            Ok(format!("Teammate subagent spawned. Communicate via {} and {}", inbox_path, outbox_path))
        } else if mode == "worktree" {
            let branch_name = format!("subagent-{}", uuid::Uuid::new_v4());
            let worktree_dir = format!(".agent-worktrees/{}", branch_name);

            // Check if there are any uncommitted changes, as creating a worktree might require a clean working directory in some cases
            // but `git worktree add` mostly just needs a new branch name.
            let git_status = self.runner.run("git", &["status", "--porcelain"], None, vec![]).await;
            if let Ok(out) = git_status {
                if !out.stdout.is_empty() {
                    // It's not strictly required to be clean, but Claude Code style worktree isolation often prefers branching off cleanly.
                    // We'll proceed but log it.
                    tracing::warn!("Spawning worktree with dirty parent git status.");
                }
            }

            // Create the git worktree
            let add_worktree = self.runner.run("git", &["worktree", "add", "-b", &branch_name, &worktree_dir], None, vec![]).await;

            match add_worktree {
                Ok(out) => {
                    if !out.status.success() {
                        return Err(ToolError::LlmRecoverable(format!("Failed to create git worktree: {}", String::from_utf8_lossy(&out.stderr))));
                    }
                },
                Err(e) => return Err(ToolError::LlmRecoverable(format!("Command runner failed on git worktree: {}", e))),
            }

            // Spawn the subagent in the new worktree directory
            let mut envs = vec![];
            if let Ok(addr) = std::env::var("OHC_AGENT_ADDRESS") {
                envs.push(("OHC_AGENT_ADDRESS".to_string(), addr));
            }

            let output = self.runner.run("ohc_builtin_agent", &["--task", &task], Some(std::path::Path::new(&worktree_dir)), envs).await;

            let res = match output {
                Ok(out) => {
                    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                    if out.status.success() {
                        Ok(SubAgentResponse {
                            result: stdout,
                            error: String::new(),
                        })
                    } else {
                        Err(format!("Process failed: {}", stderr))
                    }
                }
                Err(e) => Err(format!("Runner failed: {}", e)),
            };

            // Clean up the worktree
            let cleanup = self.runner.run("git", &["worktree", "remove", "--force", &worktree_dir], None, vec![]).await;
            if let Err(e) = cleanup {
                tracing::warn!("Failed to clean up git worktree {}: {}", worktree_dir, e);
            }

            // Optionally we might want to run `git branch -D` if the worktree failed and we want to discard it,
            // but usually the caller will inspect the branch.

            match res {
                Ok(inner) => {
                    if !inner.error.is_empty() {
                        Err(ToolError::LlmRecoverable(inner.error))
                    } else {
                        let summary = self.summarize_output(&inner.result).await.unwrap_or_else(|e| format!("Failed to summarize: {}\n\n{}", e, inner.result));
                        Ok(format!("[Subagent (Worktree)] Completed task: {}. Summary: {}\nBranch: {}", task, summary, branch_name))
                    }
                }
                Err(e) => Err(ToolError::LlmRecoverable(format!("Subagent failed in worktree: {}", e))),
            }
        } else {
            return Err(ToolError::LlmRecoverable(format!("Unknown mode: {}", mode)));
        }
    }
}

pub fn subagent_tool(runner: Arc<dyn crate::runner::CommandRunner>, llm: Option<Arc<dyn ohc_builtin_agent_core::expert_team::ExpertTeamLlmClient>>) -> Tool {
    Tool {
        name: "spawn_subagent".to_string(),
        description: "Spawn a subagent to work on a task in an isolated context (fork, teammate, or worktree) and return a condensed summary.".to_string(),
        is_read_only: false, // It might mutate the world depending on the mode
        parameters: json!({
            "type": "object",
            "properties": {
                "task": {
                    "type": "string",
                    "description": "The explicit instructions for the subagent."
                },
                "mode": {
                    "type": "string",
                    "enum": ["fork", "teammate", "worktree"],
                    "description": "Isolation mode. Fork: exact memory clone. Teammate: parallel worker via queue. Worktree: isolated git branch."
                }
            },
            "required": ["task", "mode"]
        }),
        execute: Arc::new(PydanticAdapter::new(SubagentExecutor { runner, llm })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_subagent_empty_task() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = SubagentExecutor { runner, llm: None };
        let args = json!({
            "task": "",
            "mode": "fork"
        });

        let result = executor.execute_typed(serde_json::from_value(args).unwrap()).await;
        assert!(result.is_err());
        match result {
            Err(ToolError::LlmRecoverable(msg)) => {
                assert_eq!(msg, "Task cannot be empty");
            }
            _ => panic!("Expected LlmRecoverable error"),
        }
    }

    #[tokio::test]
    async fn test_subagent_invalid_mode() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = SubagentExecutor { runner, llm: None };
        let args = json!({
            "task": "do something",
            "mode": "invalid"
        });

        let result = executor.execute_typed(serde_json::from_value(args).unwrap()).await;
        assert!(result.is_err());
        match result {
            Err(ToolError::LlmRecoverable(msg)) => {
                assert_eq!(msg, "Unknown mode: invalid");
            }
            _ => panic!("Expected LlmRecoverable error"),
        }
    }

    #[tokio::test]
    async fn test_subagent_fork_mode() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "I completed the fork task", "")));

        let executor = SubagentExecutor { runner, llm: None };
        let args = json!({
            "task": "Do this fork task",
            "mode": "fork"
        });

        let result = executor.execute_typed(serde_json::from_value(args).unwrap()).await;
        assert!(result.is_ok(), "Expected Ok for fork mode");
        let msg = result.unwrap();

        assert!(msg.contains("[Subagent (Fork)] Completed task: Do this fork task"), "msg was: {}", msg);
        assert!(msg.contains("I completed the fork task"), "Message should contain the agent output");
    }

    #[test]
    fn test_subagent_teammate_mode() {
        temp_env::with_vars(vec![("OHC_AGENT_ADDRESS", Some("127.0.0.1:0"))], || {
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
        // We set the address to something invalid to quickly trigger connection failure for the background task


        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = SubagentExecutor { runner, llm: None };
        let args = json!({
            "task": "Do this teammate task",
            "mode": "teammate"
        });

        let result = executor.execute_typed(serde_json::from_value(args).unwrap()).await;
        assert!(result.is_ok(), "Expected Ok for teammate mode");
        let msg = result.unwrap();

        assert!(msg.contains("Teammate subagent spawned. Communicate via"), "Message should contain success notification");

        let parts: Vec<&str> = msg.split("Communicate via ").collect();
        assert_eq!(parts.len(), 2);

        let path_parts: Vec<&str> = parts[1].split(" and ").collect();
        assert_eq!(path_parts.len(), 2);

        let inbox_path = path_parts[0];
        let outbox_path = path_parts[1];

        assert!(std::path::Path::new(inbox_path).exists(), "Inbox should exist");

        // Mock command runner will return success default, no error.
        let mut attempts = 0;
        let mut found = false;
        while attempts < 20 {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            if let Ok(content) = tokio::fs::read_to_string(outbox_path).await {
                if content.contains("[System: Subagent Process Terminated]") {
                    found = true;
                    break;
                }
            }
            attempts += 1;
        }

        assert!(found, "Background task should have written to outbox");

        let parent_dir = std::path::Path::new(inbox_path).parent().unwrap();
                let _ = tokio::fs::remove_dir_all(parent_dir).await;
            });
        });
    }

    #[tokio::test]
    async fn test_subagent_worktree_mode() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        // Mock successful git status/add check
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "M some_file", "")));
        // Mock successful git worktree add
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "Preparing worktree", "")));
        // Mock successful ohc_builtin_agent run
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "I completed the worktree task", "")));
        // Mock successful git cleanup
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "Removed worktree", "")));

        let executor = SubagentExecutor { runner, llm: None };
        let args = json!({
            "task": "Do this worktree task",
            "mode": "worktree"
        });

        let result = executor.execute_typed(serde_json::from_value(args).unwrap()).await;
        assert!(result.is_ok(), "Expected Ok for worktree mode");
        let msg = result.unwrap();

        assert!(msg.contains("[Subagent (Worktree)"), "msg was: {}", msg);
        assert!(msg.contains("I completed the worktree task"), "Message should contain the agent output");
    }


    #[tokio::test]
    async fn test_subagent_output_truncation() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let long_string = "a".repeat(9000);

        runner.push_response(Ok(crate::runner::mock::mock_output(0, &long_string, "")));

        // Mock LLM client that returns large string
        struct BadLlmClient;

        #[async_trait::async_trait]
        impl ohc_builtin_agent_core::expert_team::ExpertTeamLlmClient for BadLlmClient {
            async fn chat(&self, _req: ohc_builtin_agent_core::types::ChatRequest) -> Result<ohc_builtin_agent_core::types::ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                let message = ohc_builtin_agent_core::types::Message {
                    role: ohc_builtin_agent_core::types::Role::Assistant,
                    content: "a".repeat(9000), // always returns > 8000
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

        let llm = Arc::new(BadLlmClient);

        let executor = SubagentExecutor { runner, llm: Some(llm) };
        let args = json!({
            "task": "Test truncation",
            "mode": "fork"
        });

        let result = executor.execute_typed(serde_json::from_value(args).unwrap()).await;
        assert!(result.is_ok(), "Expected Ok");
        let msg = result.unwrap();
        assert!(msg.contains("[Output truncated. Subagent failed to condense summary.]"), "Expected output to be truncated");
        assert!(msg.len() < 9000, "Expected output length to be less than 9000 after truncation");
    }

}
