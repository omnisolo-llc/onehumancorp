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
        let task = args.get("task").and_then(|v| v.as_str()).unwrap_or("");
        let mode = args.get("mode").and_then(|v| v.as_str()).unwrap_or("fork");
        
        if task.is_empty() {
            return Err(ToolError::LlmRecoverable("Task cannot be empty".to_string()));
        }

        tracing::info!("Spawning subagent in mode '{}' for task: {}", mode, task);

        if mode == "worktree" {
            let task_id = uuid::Uuid::new_v4().to_string();
            let branch_name = format!("subagent-{}", task_id);
            let worktree_path = format!(".agent-worktrees/{}", task_id);

            // Create worktree
            let _ = self.runner.run("git", &["branch", &branch_name], None, vec![]).await;

            let wt_output = self.runner.run("git", &["worktree", "add", &worktree_path, &branch_name], None, vec![]).await;

            if let Err(e) = wt_output {
                return Err(ToolError::LlmRecoverable(format!("Failed to spawn worktree: {}", e)));
            }

            let wt_path = std::path::Path::new(&worktree_path);
            if let Err(e) = tokio::fs::create_dir_all(wt_path).await {
                return Err(ToolError::LlmRecoverable(format!("Failed to create worktree directory: {}", e)));
            }
            let task_file = wt_path.join("agent_task.json");

            let task_content = json!({
                "task": task
            });

            if let Err(e) = tokio::fs::write(&task_file, task_content.to_string()).await {
                return Err(ToolError::LlmRecoverable(format!("Failed to write task file: {}", e)));
            }

            // Execute the agent binary in the isolated worktree directory
            let res = self.runner.run(
                "cargo",
                &["run", "--bin", "ohc-builtin-agent", "--", "--task-file", "agent_task.json"],
                Some(wt_path),
                vec![]
            ).await;

            // Cleanup
            let _ = self.runner.run("git", &["worktree", "remove", "--force", &worktree_path], None, vec![]).await;
            let _ = self.runner.run("git", &["branch", "-D", &branch_name], None, vec![]).await;

            match res {
                Ok(output) => {
                    if !output.status.success() {
                        let err_msg = String::from_utf8_lossy(&output.stderr).to_string();
                        Err(ToolError::LlmRecoverable(format!("Subagent failed with error: {}", err_msg)))
                    } else {
                        let result = String::from_utf8_lossy(&output.stdout).to_string();
                        Ok(result)
                    }
                }
                Err(e) => Err(ToolError::LlmRecoverable(format!("Subagent failed to spawn: {}", e))),
            }
        } else if mode == "fork" {
            let parent_context_json = args.get("parent_context_json").and_then(|v| v.as_str()).unwrap_or("");

            let task_id = uuid::Uuid::new_v4().to_string();
            let fork_dir = format!(".agent-forks/{}", task_id);
            if let Err(e) = tokio::fs::create_dir_all(&fork_dir).await {
                return Err(ToolError::LlmRecoverable(format!("Failed to create fork directory: {}", e)));
            }

            let context_path = format!("{}/parent_context.json", fork_dir);
            if let Err(e) = tokio::fs::write(&context_path, parent_context_json).await {
                return Err(ToolError::LlmRecoverable(format!("Failed to write parent context: {}", e)));
            }

            // Execute the agent binary with the exact memory clone (parent context)
            let res = self.runner.run(
                "cargo",
                &["run", "--bin", "ohc-builtin-agent", "--", "--task", task, "--parent-context", &context_path],
                None,
                vec![]
            ).await;

            let _ = tokio::fs::remove_dir_all(&fork_dir).await;

            match res {
                Ok(output) => {
                    if !output.status.success() {
                        let err_msg = String::from_utf8_lossy(&output.stderr).to_string();
                        Err(ToolError::LlmRecoverable(format!("Subagent failed with error: {}", err_msg)))
                    } else {
                        let result = String::from_utf8_lossy(&output.stdout).to_string();
                        Ok(format!("[Subagent (Fork)] Completed task: {}. Summary: {}", task, result))
                    }
                }
                Err(e) => Err(ToolError::LlmRecoverable(format!("Subagent failed to spawn: {}", e))),
            }
        } else if mode == "teammate" {
            let task_id = uuid::Uuid::new_v4().to_string();
            let mailbox_dir = format!(".agent-mailboxes/subagent-{}", task_id);
            if let Err(e) = tokio::fs::create_dir_all(&mailbox_dir).await {
                return Err(ToolError::LlmRecoverable(format!("Failed to create mailbox directory: {}", e)));
            }

            let inbox_path = format!("{}/inbox.txt", mailbox_dir);
            let outbox_path = format!("{}/outbox.txt", mailbox_dir);

            if let Err(e) = tokio::fs::write(&inbox_path, task).await {
                return Err(ToolError::LlmRecoverable(format!("Failed to write to inbox: {}", e)));
            }

            let teammate_task = format!(
                "You are a teammate subagent. Your task is: {}\nWhen finished or if you need to report progress, write your final summary to {}. To receive further instructions, read from {}.",
                task, outbox_path, inbox_path
            );

            // Write the task to a file to avoid command injection via shell interpolation.
            let task_file_path = format!("{}/task.json", mailbox_dir);
            let task_content = json!({
                "task": teammate_task
            });
            if let Err(e) = tokio::fs::write(&task_file_path, task_content.to_string()).await {
                return Err(ToolError::LlmRecoverable(format!("Failed to write teammate task file: {}", e)));
            }

            // Execute the agent binary in the background (nohup ... &)
            // Using a shell script but referencing the safe task file instead of inline task payload
            let bash_cmd = format!(
                "nohup cargo run --bin ohc-builtin-agent -- --task-file \"{}\" --inbox \"{}\" --outbox \"{}\" > {}/teammate.log 2>&1 &",
                task_file_path,
                inbox_path,
                outbox_path,
                mailbox_dir
            );

            let res = self.runner.run(
                "bash",
                &["-c", &bash_cmd],
                None,
                vec![]
            ).await;

            match res {
                Ok(output) => {
                    if !output.status.success() {
                        let err_msg = String::from_utf8_lossy(&output.stderr).to_string();
                        return Err(ToolError::LlmRecoverable(format!("Subagent failed to spawn background teammate: {}", err_msg)));
                    }
                }
                Err(e) => return Err(ToolError::LlmRecoverable(format!("Subagent failed to spawn teammate: {}", e))),
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

    #[tokio::test]
    async fn test_subagent_fork_mode() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "mock fork summary output", "")));

        let executor = SubagentExecutor { runner };
        let args = json!({
            "task": "do fork task",
            "mode": "fork",
            "parent_context_json": "[]"
        });

        let result = executor.execute(args).await;
        assert!(result.is_ok(), "Expected Ok for fork mode");
        let msg = result.unwrap();
        assert!(msg.contains("[Subagent (Fork)] Completed task: do fork task. Summary: mock fork summary output"));
    }

    #[tokio::test]
    async fn test_subagent_worktree_mode() {
        // Need an extra mock output for tokio::fs::write ? No, that doesn't use the runner.
        // Wait, did we use `fs::write`? Yes: tokio::fs::write(&task_file, task_content.to_string()).await
        // That writes to the filesystem! In a test, `.agent-worktrees/...` might not exist.
        // Ah! `std::path::Path::new(&worktree_path)` and `task_file` !
        // In the test, we mock `runner`, but the `tokio::fs::write` will fail if `.agent-worktrees` directory isn't created by `git worktree add` (since it's mocked!).

        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "", ""))); // git branch
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "", ""))); // git worktree add
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "mock worktree summary output", ""))); // cargo run
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "", ""))); // git worktree remove
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "", ""))); // git branch -D

        // Ensure the mock dir exists so write succeeds
        let _ = tokio::fs::create_dir_all(".agent-worktrees").await;

        let executor = SubagentExecutor { runner };
        let args = json!({
            "task": "do worktree task",
            "mode": "worktree"
        });

        let result = executor.execute(args).await;
        assert!(result.is_ok(), "Expected Ok for worktree mode");
        let msg = result.unwrap();
        assert_eq!(msg, "mock worktree summary output");
    }

    #[tokio::test]
    async fn test_subagent_teammate_mode() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "", ""))); // bash -c

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
        let _outbox_path = path_parts[1];

        assert!(std::path::Path::new(inbox_path).exists(), "Inbox should exist");

        let parent_dir = std::path::Path::new(inbox_path).parent().unwrap();
        let _ = tokio::fs::remove_dir_all(parent_dir).await;
    }
}
