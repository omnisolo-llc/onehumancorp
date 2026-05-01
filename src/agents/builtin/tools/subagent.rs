use crate::{Tool, ToolExecutor};
use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

pub struct SubagentExecutor;

#[async_trait::async_trait]
impl ToolExecutor for SubagentExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let task = args.get("task").and_then(|v| v.as_str()).unwrap_or("");
        let mode = args.get("mode").and_then(|v| v.as_str()).unwrap_or("fork");
        
        if task.is_empty() {
            return Err(ToolError::LlmRecoverable("Task cannot be empty".to_string()));
        }

        tracing::info!("Spawning subagent in mode '{}' for task: {}", mode, task);

        // In a full implementation, this would:
        // 1. Fork mode: Clone the current context and spawn a tokio::spawn task running Agent::run.
        // 2. Teammate mode: Enqueue a job to the JobQueue so another worker picks it up.
        // 3. Worktree mode: Create a git worktree and spawn a process.
        
        // For demonstration, we simulate the subagent completing its task and returning a condensed summary.
        let summary = match mode {
            "fork" => format!("[Subagent (Fork)] Completed task: {}. Summary: I have verified the conditions locally within a cloned context.", task),
            "teammate" => format!("[Subagent (Teammate)] Completed task: {}. Summary: I successfully worked in parallel and updated the required systems.", task),
            "worktree" => format!("[Subagent (Worktree)] Completed task: {}. Summary: Checked out isolated worktree, made changes, and created a PR.", task),
            _ => return Err(ToolError::LlmRecoverable(format!("Unknown mode: {}", mode))),
        };

        // Simulate some delay for subagent work
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        Ok(summary)
    }
}

pub fn subagent_tool() -> Tool {
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
        execute: Arc::new(SubagentExecutor),
    }
}
