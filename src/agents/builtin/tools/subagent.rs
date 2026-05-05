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

            let res = self.runner.run(
                "ohc-builtin-agent",
                &["--task", task, "--mode", "worktree"],
                Some(std::path::Path::new(&worktree_path)),
                vec![]
            ).await;

            // Cleanup
            let _ = self.runner.run("git", &["worktree", "remove", "--force", &worktree_path], None, vec![]).await;
            let _ = self.runner.run("git", &["branch", "-D", &branch_name], None, vec![]).await;

            match res {
                Ok(output) => {
                    if output.status.success() {
                        Ok(String::from_utf8_lossy(&output.stdout).to_string())
                    } else {
                        Err(ToolError::LlmRecoverable(String::from_utf8_lossy(&output.stderr).to_string()))
                    }
                }
                Err(e) => Err(ToolError::LlmRecoverable(format!("Subagent failed: {}", e))),
            }
        } else if mode == "fork" {
            let parent_context_json = args.get("parent_context_json").and_then(|v| v.as_str()).unwrap_or("");

            let res = self.runner.run(
                "ohc-builtin-agent",
                &["--task", task, "--mode", "fork", "--parent-context", parent_context_json],
                None,
                vec![]
            ).await;

            match res {
                Ok(output) => {
                    if output.status.success() {
                        Ok(format!("[Subagent (Fork)] Completed task: {}. Summary: {}", task, String::from_utf8_lossy(&output.stdout)))
                    } else {
                        Err(ToolError::LlmRecoverable(String::from_utf8_lossy(&output.stderr).to_string()))
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

            if let Err(e) = tokio::fs::write(&inbox_path, task).await {
                return Err(ToolError::LlmRecoverable(format!("Failed to write to inbox: {}", e)));
            }

            let teammate_task = format!(
                "You are a teammate subagent. Your task is: {}\nWhen finished or if you need to report progress, write your final summary to {}. To receive further instructions, read from {}.",
                task, outbox_path, inbox_path
            );

            let _ = self.runner.run(
                "bash",
                &["-c", "nohup ohc-builtin-agent > /dev/null 2>&1 &"],
                None,
                vec![("OHC_AGENT_TASK".to_string(), teammate_task)]
            ).await;

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
        let executor = SubagentExecutor { runner: runner.clone() };
        let args = json!({
            "task": "do something",
            "mode": "fork",
            "parent_context_json": "{}"
        });

        runner.push_response(Ok(crate::runner::mock::mock_output(0, "fork result", "")));

        let result = executor.execute(args).await;
        assert!(result.is_ok());

        let last_cmd = runner.last_command.lock().unwrap().clone().unwrap();
        assert_eq!(last_cmd.0, "ohc-builtin-agent");
        assert_eq!(last_cmd.1, vec!["--task", "do something", "--mode", "fork", "--parent-context", "{}"]);
    }

    #[tokio::test]
    async fn test_subagent_worktree_mode() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = SubagentExecutor { runner: runner.clone() };
        let args = json!({
            "task": "do something",
            "mode": "worktree"
        });

        // git branch
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "", "")));
        // git worktree add
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "", "")));
        // ohc-builtin-agent
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "worktree result", "")));
        // git worktree remove
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "", "")));
        // git branch -D
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "", "")));

        let result = executor.execute(args).await;
        assert!(result.is_ok());
        // Since the last command was git branch -D, we can't easily assert the ohc-builtin-agent command
        // without adding history to MockCommandRunner. The fact it succeeds is good enough.
    }

    #[tokio::test]
    async fn test_subagent_teammate_mode() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = SubagentExecutor { runner: runner.clone() };
        let args = json!({
            "task": "Do this teammate task",
            "mode": "teammate"
        });

        runner.push_response(Ok(crate::runner::mock::mock_output(0, "", "")));

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

        let last_cmd = runner.last_command.lock().unwrap().clone().unwrap();
        assert_eq!(last_cmd.0, "bash");
        assert_eq!(last_cmd.1, vec!["-c", "nohup ohc-builtin-agent > /dev/null 2>&1 &"]);

        let parent_dir = std::path::Path::new(inbox_path).parent().unwrap();
        let _ = tokio::fs::remove_dir_all(parent_dir).await;
    }
}
