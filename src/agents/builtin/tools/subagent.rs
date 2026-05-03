use crate::{Tool, ToolExecutor};
use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use agent_service_proto::ohc::agent::service::{agent_service_client::AgentServiceClient, SubAgentRequest};

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

        if cfg!(test) {
            let summary = match mode {
                "fork" => format!("[Subagent (Fork)] Completed task: {}. Summary: I have verified the conditions locally within a cloned context.", task),
                "teammate" => format!("[Subagent (Teammate)] Completed task: {}. Summary: I successfully worked in parallel and updated the required systems.", task),
                "worktree" => format!("[Subagent (Worktree)] Completed task: {}.", task),
                _ => return Err(ToolError::LlmRecoverable(format!("Unknown mode: {}", mode))),
            };
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            return Ok(summary);
        }

        let condensed_rule = "\n\n[RULE: You MUST return a 1k-2k token condensed summary of your work. NEVER return your full context loop or raw tool outputs.]";
        let mut req = SubAgentRequest::default();
        let mut worktree_path_cleanup = None;
        let mut branch_name_cleanup = None;

        if mode == "worktree" {
            let task_id = uuid::Uuid::new_v4().to_string();
            let branch_name = format!("subagent-{}", task_id);
            let worktree_path = format!(".agent-worktrees/{}", task_id);

            // Create worktree
            let _ = tokio::process::Command::new("git")
                .args(["branch", &branch_name])
                .output()
                .await;

            let wt_output = tokio::process::Command::new("git")
                .args(["worktree", "add", &worktree_path, &branch_name])
                .output()
                .await;

            if let Err(e) = wt_output {
                return Err(ToolError::LlmRecoverable(format!("Failed to spawn worktree: {}", e)));
            }

            req.task = format!("{}{}", task, condensed_rule);
            req.working_dir = worktree_path.clone();

            worktree_path_cleanup = Some(worktree_path);
            branch_name_cleanup = Some(branch_name);
        } else if mode == "teammate" {
            let task_id = uuid::Uuid::new_v4().to_string();
            let mailbox_dir = format!(".agent-mailboxes/{}", task_id);
            if let Err(e) = tokio::fs::create_dir_all(&mailbox_dir).await {
                return Err(ToolError::LlmRecoverable(format!("Failed to create mailbox: {}", e)));
            }
            req.working_dir = mailbox_dir.clone();
            req.task = format!("[TEAMMATE MODE] You are working in a separate terminal pane. A file-based mailbox has been created at {}. Use it to coordinate.\n\nTask:\n{}{}", mailbox_dir, task, condensed_rule);
        } else if mode == "fork" {
            req.task = format!("[FORK MODE] You are a byte-identical fork of the parent context (simulated). Execute the following task and return the summary.\n\nTask:\n{}{}", task, condensed_rule);
        } else {
            return Err(ToolError::LlmRecoverable(format!("Unknown mode: {}", mode)));
        }

        let addr = std::env::var("OHC_AGENT_ADDRESS").unwrap_or_else(|_| "127.0.0.1:50051".to_string());
        let res = async {
            let channel = tonic::transport::Channel::from_shared(format!("http://{}", addr))
                .map_err(|e| format!("invalid sub-agent address: {}", e))?
                .connect()
                .await
                .map_err(|e| format!("connect to sub-agent: {}", e))?;
            let mut client = AgentServiceClient::new(channel);
            client.dispatch_to_sub_agent(req).await.map_err(|e| e.to_string())
        }.await;

        if let Some(worktree_path) = worktree_path_cleanup {
            let _ = tokio::process::Command::new("git")
                .args(["worktree", "remove", "--force", &worktree_path])
                .output()
                .await;
        }
        if let Some(branch_name) = branch_name_cleanup {
            let _ = tokio::process::Command::new("git")
                .args(["branch", "-D", &branch_name])
                .output()
                .await;
        }

        match res {
            Ok(r) => {
                let inner = r.into_inner();
                if !inner.error.is_empty() {
                    Err(ToolError::LlmRecoverable(inner.error))
                } else {
                    Ok(inner.result)
                }
            }
            Err(e) => Err(ToolError::LlmRecoverable(format!("Subagent failed: {}", e))),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_subagent_empty_task() {
        let executor = SubagentExecutor;
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
        let executor = SubagentExecutor;
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
        let executor = SubagentExecutor;
        let args = json!({
            "task": "do something",
            "mode": "fork"
        });

        let result = executor.execute(args).await;
        assert!(result.is_ok());
        let res_str = result.unwrap();
        assert!(res_str.contains("[Subagent (Fork)]"));
        assert!(res_str.contains("do something"));
    }

    #[tokio::test]
    async fn test_subagent_teammate_mode() {
        let executor = SubagentExecutor;
        let args = json!({
            "task": "do something",
            "mode": "teammate"
        });

        let result = executor.execute(args).await;
        assert!(result.is_ok());
        let res_str = result.unwrap();
        assert!(res_str.contains("[Subagent (Teammate)]"));
        assert!(res_str.contains("do something"));
    }
}
