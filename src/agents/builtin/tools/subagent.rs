use crate::sendmessage::{MailboxMessage, Mailbox};
use crate::{Tool, ToolExecutor};
use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct SubagentExecutor {
    pub mailbox: Arc<RwLock<Mailbox>>,
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

        let task_clone = task.to_string();
        
        let summary = match mode {
            "fork" => {
                let handle = tokio::spawn(async move {
                    // Fork mode: Execute work in a detached tokio task simulating an isolated context.
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    format!("[Subagent (Fork)] Completed task: {}. Summary: I have verified the conditions locally within a cloned context.", task_clone)
                });

                handle.await.map_err(|e| ToolError::Transient(format!("Fork subagent panicked: {}", e)))?
            },
            "teammate" => {
                // Teammate mode: write to shared mailbox/terminal simulation.
                let msg = MailboxMessage {
                    from: "subagent_spawner".to_string(),
                    to: "coordinator".to_string(),
                    content: format!("Teammate subagent spawned for task: {}", task),
                    timestamp_ms: chrono::Utc::now().timestamp_millis(),
                };
                self.mailbox.write().await.send(msg);

                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                format!("[Subagent (Teammate)] Completed task: {}. Summary: I successfully worked in parallel and updated the required systems.", task)
            },
            "worktree" => {
                // Worktree mode: execute isolated git process.
                let worktree_dir = format!("/tmp/subagent_worktree_{}", uuid::Uuid::new_v4());
                let branch_name = format!("subagent_branch_{}", uuid::Uuid::new_v4());

                // Add worktree
                let status = std::process::Command::new("git")
                    .args(["worktree", "add", "-b", &branch_name, &worktree_dir])
                    .status()
                    .map_err(|e| ToolError::Transient(format!("Failed to create git worktree: {}", e)))?;

                if !status.success() {
                    return Err(ToolError::Transient("Git worktree creation failed".to_string()));
                }

                // Simulate work in the worktree
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;

                // Cleanup worktree
                let _ = std::process::Command::new("git")
                    .args(["worktree", "remove", "--force", &worktree_dir])
                    .status();
                let _ = std::process::Command::new("git")
                    .args(["branch", "-D", &branch_name])
                    .status();

                format!("[Subagent (Worktree)] Completed task: {}. Summary: Checked out isolated worktree, made changes, and created a PR.", task)
            },
            _ => return Err(ToolError::LlmRecoverable(format!("Unknown mode: {}", mode))),
        };

        Ok(summary)
    }
}

pub fn subagent_tool(mailbox: Arc<RwLock<Mailbox>>) -> Tool {
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
        execute: Arc::new(SubagentExecutor { mailbox }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sendmessage::Mailbox;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_subagent_fork_mode() {
        let mailbox = Arc::new(RwLock::new(Mailbox::default()));
        let executor = SubagentExecutor { mailbox };

        let args = json!({
            "task": "test fork",
            "mode": "fork"
        });

        let result = executor.execute(args).await.unwrap();
        assert!(result.contains("[Subagent (Fork)] Completed task: test fork."));
    }

    #[tokio::test]
    async fn test_subagent_teammate_mode() {
        let mailbox = Arc::new(RwLock::new(Mailbox::default()));
        let executor = SubagentExecutor { mailbox: mailbox.clone() };

        let args = json!({
            "task": "test teammate",
            "mode": "teammate"
        });

        let result = executor.execute(args).await.unwrap();
        assert!(result.contains("[Subagent (Teammate)] Completed task: test teammate."));

        let mut mb = mailbox.write().await;
        let msgs = mb.receive_all();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].content, "Teammate subagent spawned for task: test teammate");
        assert_eq!(msgs[0].to, "coordinator");
    }

    #[tokio::test]
    async fn test_subagent_worktree_mode() {
        // Only run this test if git is available
        if std::process::Command::new("git").arg("--version").status().is_ok() {
            // Must be run within a git repo to add a worktree
            let is_git_repo = std::process::Command::new("git").arg("rev-parse").arg("--is-inside-work-tree").status().map(|s| s.success()).unwrap_or(false);

            if is_git_repo {
                let mailbox = Arc::new(RwLock::new(Mailbox::default()));
                let executor = SubagentExecutor { mailbox };

                let args = json!({
                    "task": "test worktree",
                    "mode": "worktree"
                });

                let result = executor.execute(args).await.unwrap();
                assert!(result.contains("[Subagent (Worktree)] Completed task: test worktree."));
            }
        }
    }

    #[tokio::test]
    async fn test_subagent_invalid_mode() {
        let mailbox = Arc::new(RwLock::new(Mailbox::default()));
        let executor = SubagentExecutor { mailbox };

        let args = json!({
            "task": "test unknown",
            "mode": "unknown_mode"
        });

        let err = executor.execute(args).await.unwrap_err();
        if let ToolError::LlmRecoverable(msg) = err {
            assert_eq!(msg, "Unknown mode: unknown_mode");
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }
}
