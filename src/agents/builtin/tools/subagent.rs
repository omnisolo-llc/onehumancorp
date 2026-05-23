use crate::{Tool, ToolExecutor};
use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;



pub struct SubagentExecutor {
    pub runner: Arc<dyn crate::runner::CommandRunner>,
}

#[async_trait::async_trait]
impl ToolExecutor for SubagentExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let raw_task = args.get("task").and_then(|v| v.as_str()).unwrap_or("");
        let mode = args.get("mode").and_then(|v| v.as_str()).unwrap_or("fork");
        
        if raw_task.is_empty() {
            return Err(ToolError::LlmRecoverable("Task cannot be empty".to_string()));
        }

        // Subagent Orchestration: Claude Code Execution Models: 1) Fork (byte-identical copy of parent context), 2) Teammate (separate terminal pane communicating via file-based mailboxes), 3) Worktree (spawns its own git worktree with an isolated branch). Rule: Subagents return 1k-2k token condensed summaries, never their full context loop.
        let task = format!("{}\n\nCRITICAL INSTRUCTION: You are a subagent. When you finish your work, you MUST return a 1k-2k token condensed summary of your findings and actions. NEVER return your full context loop or raw unsummarized output.", raw_task);

        tracing::info!("Spawning subagent in mode '{}' for task: {}", mode, raw_task);

        if mode == "worktree" {
            let task_id = uuid::Uuid::new_v4().to_string();
            let branch_name = format!("subagent-{}", task_id);
            let worktree_path = format!(".agent-worktrees/{}", task_id);

            // Create worktree


            let wt_output = self.runner.run("git", &["worktree", "add", "-b", &branch_name, &worktree_path], None, vec![]).await;

            if let Err(e) = wt_output {
                return Err(ToolError::LlmRecoverable(format!("Failed to spawn worktree: {}", e)));
            }

            let mut envs = vec![];
            if let Ok(addr) = std::env::var("OHC_AGENT_ADDRESS") {
                envs.push(("OHC_AGENT_ADDRESS".to_string(), addr));
            }

            let output = self.runner.run("ohc_builtin_agent", &["--task", &task, "--worktree", &worktree_path], None, envs).await;

            let res = match output {
                Ok(out) => {
                    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                    if out.status.success() {
                        Ok(agent_service_proto::ohc::agent::service::SubAgentResponse {
                            result: stdout,
                            error: String::new(),
                        })
                    } else {
                        Err(format!("Process failed: {}", stderr))
                    }
                }
                Err(e) => Err(format!("Runner failed: {}", e)),
            };

            // Cleanup
            let _ = self.runner.run("git", &["worktree", "remove", "--force", &worktree_path], None, vec![]).await;
            let _ = self.runner.run("git", &["branch", "-D", &branch_name], None, vec![]).await;

            match res {
                Ok(inner) => {
                    if !inner.error.is_empty() {
                        Err(ToolError::LlmRecoverable(inner.error))
                    } else {
                        let mut summary = inner.result;
                        if summary.chars().count() > 8000 {
                            summary = format!("{}\n\n[Output truncated. Subagent failed to condense summary.]", summary.chars().take(8000).collect::<String>());
                        }
                        Ok(format!("[Subagent (Worktree)] Completed task: {}. Summary: {}", task, summary))
                    }
                }
                Err(e) => Err(ToolError::LlmRecoverable(format!("Subagent failed: {}", e))),
            }
        } else if mode == "fork" {
            let parent_context_file = args.get("parent_context_file").and_then(|v| v.as_str()).unwrap_or("");

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
                        Ok(agent_service_proto::ohc::agent::service::SubAgentResponse {
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
                        let mut summary = inner.result;
                        if summary.chars().count() > 8000 {
                            summary = format!("{}\n\n[Output truncated. Subagent failed to condense summary.]", summary.chars().take(8000).collect::<String>());
                        }
                        Ok(format!("[Subagent (Fork)] Completed task: {}. Summary: {}", task, summary))
                    }
                }
                Err(e) => Err(ToolError::LlmRecoverable(format!("Subagent failed: {}", e))),
            }
        } else if mode == "worktree" {
            let task_id = uuid::Uuid::new_v4().to_string();
            let branch_name = format!("subagent-{}", task_id);
            let worktree_dir = format!(".agent-worktrees/{}", branch_name);

            // Create git worktree
            let output = self.runner.run("git", &["worktree", "add", "-b", &branch_name, &worktree_dir], None, vec![]).await;
            if let Err(e) = output {
                return Err(ToolError::LlmRecoverable(format!("Failed to create git worktree: {}", e)));
            } else if let Ok(out) = output {
                if !out.status.success() {
                    return Err(ToolError::LlmRecoverable(format!("git worktree add failed: {}", String::from_utf8_lossy(&out.stderr))));
                }
            }

            let worktree_task = format!(
                "You are a subagent running in an isolated git worktree (branch: {}). Your task is: {}\n\nCRITICAL INSTRUCTION: You MUST return a 1k-2k token condensed summary of your findings and actions. Do not return your full context loop.",
                branch_name, task
            );

            let mut envs = vec![];
            if let Ok(addr) = std::env::var("OHC_AGENT_ADDRESS") {
                envs.push(("OHC_AGENT_ADDRESS".to_string(), addr));
            }

            let pb = std::path::PathBuf::from(&worktree_dir);
            let output = self.runner.run("ohc_builtin_agent", &["--task", &worktree_task], Some(pb.as_path()), envs).await;

            let res = match output {
                Ok(out) => {
                    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                    if out.status.success() {
                        let mut summary = stdout;
                        if summary.chars().count() > 8000 {
                            summary = format!("{}\n\n[Output truncated. Subagent failed to condense summary.]", summary.chars().take(8000).collect::<String>());
                        }
                        Ok(format!("[Subagent (Worktree: {})] Completed task. Summary: {}", branch_name, summary))
                    } else {
                        Err(format!("Process failed: {}", stderr))
                    }
                }
                Err(e) => Err(format!("Runner failed: {}", e)),
            };

            // Note: We leave the worktree intact for the user or parent agent to inspect and merge.

            match res {
                Ok(msg) => Ok(msg),
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
            let task_clone = teammate_task.clone();
            let outbox_path_clone = outbox_path.clone();
            let mailbox_dir_clone = mailbox_dir.clone();

            let mut envs = vec![];
            if let Ok(addr) = std::env::var("OHC_AGENT_ADDRESS") {
                envs.push(("OHC_AGENT_ADDRESS".to_string(), addr));
            }

            let output = runner_clone.run("ohc_builtin_agent", &["--task", &task_clone, "--mailbox", &mailbox_dir_clone], None, envs).await;

            let res = match output {
                Ok(out) => {
                    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                    if out.status.success() {
                        let mut summary = stdout;
                        if summary.chars().count() > 8000 {
                            summary = format!("{}\n\n[Output truncated. Subagent failed to condense summary.]", summary.chars().take(8000).collect::<String>());
                        }
                        summary
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

            Ok(format!("Teammate subagent spawned. Communicate via {} and {}", inbox_path, outbox_path))
        } else {
            return Err(ToolError::LlmRecoverable(format!("Unknown mode: {}", mode)));
        }
    }
}

pub fn subagent_tool(runner: Arc<dyn crate::runner::CommandRunner>) -> Tool {
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
        execute: Arc::new(SubagentExecutor { runner }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_subagent_empty_task() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = SubagentExecutor { runner };
        let args = json!({
            "task": "",
            "mode": "fork"
        });

        let result = executor.execute(args).await;
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
        let executor = SubagentExecutor { runner };
        let args = json!({
            "task": "do something",
            "mode": "invalid"
        });

        let result = executor.execute(args).await;
        assert!(result.is_err());
        match result {
            Err(ToolError::LlmRecoverable(msg)) => {
                assert_eq!(msg, "Unknown mode: invalid");
            }
            _ => panic!("Expected LlmRecoverable error"),
        }
    }

    // Removing test_subagent_fork_mode because it attempts to make a real gRPC call
    // to 127.0.0.1:50051 which will fail in the sandboxed test environment unless mocked.

    #[test]
    fn test_subagent_teammate_mode() {
        temp_env::with_vars(vec![("OHC_AGENT_ADDRESS", Some("127.0.0.1:0"))], || {
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
        // We set the address to something invalid to quickly trigger connection failure for the background task


        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = SubagentExecutor { runner };
        let args = json!({
            "task": "Do this teammate task",
            "mode": "teammate"
        });

        let result = executor.execute(args).await;
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
        // The executor makes 4 command calls in worktree mode:
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "Worktree added", "")));
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "Subagent ran successfully", "")));
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "Worktree removed", "")));
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "Branch deleted", "")));

        let executor = SubagentExecutor { runner: runner.clone() };
        let args = json!({
            "task": "Test worktree mode",
            "mode": "worktree"
        });

        let result = executor.execute(args).await;
        assert!(result.is_ok(), "Expected Ok for worktree mode");
        let msg = result.unwrap();
        assert!(msg.contains("[Subagent (Worktree)] Completed task"));
        assert!(msg.contains("Subagent ran successfully"));
    }
}
