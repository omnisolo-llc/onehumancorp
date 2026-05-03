use crate::{Tool, ToolExecutor};
use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use agent_service_proto::ohc::agent::service::{agent_service_client::AgentServiceClient, SubAgentRequest};

pub struct SubagentExecutor {
    thread_id: Option<String>,
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

            let mut req = SubAgentRequest::default();
            req.task = task.to_string();
            req.working_dir = worktree_path.clone();

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

            // Cleanup
            let _ = tokio::process::Command::new("git")
                .args(["worktree", "remove", "--force", &worktree_path])
                .output()
                .await;
            let _ = tokio::process::Command::new("git")
                .args(["branch", "-D", &branch_name])
                .output()
                .await;

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
        } else if mode == "fork" || mode == "teammate" {
            let mut req = SubAgentRequest::default();
            req.task = task.to_string();
            if mode == "fork" {
                if let Some(tid) = &self.thread_id {
                    req.source_thread_id = tid.clone();
                } else {
                    return Err(ToolError::LlmRecoverable("Fork mode requires a parent thread_id which is missing".to_string()));
                }
            }

            let addr = std::env::var("OHC_AGENT_ADDRESS").unwrap_or_else(|_| "127.0.0.1:50051".to_string());

            if mode == "teammate" {
                // Teammate mode runs completely in parallel via background spawn
                tokio::spawn(async move {
                    let channel = match tonic::transport::Channel::from_shared(format!("http://{}", addr)) {
                        Ok(c) => c,
                        Err(e) => { tracing::error!("teammate mode invalid address: {}", e); return; }
                    };
                    let channel = match channel.connect().await {
                        Ok(c) => c,
                        Err(e) => { tracing::error!("teammate mode connect error: {}", e); return; }
                    };
                    let mut client = AgentServiceClient::new(channel);
                    let _ = client.dispatch_to_sub_agent(req).await;
                });
                return Ok(format!("[Subagent (Teammate)] Spawned teammate successfully for task: {}. It will run in parallel and communicate via mailbox.", task));
            } else {
                // Fork mode awaits the result
                let res = async {
                    let channel = tonic::transport::Channel::from_shared(format!("http://{}", addr))
                        .map_err(|e| format!("invalid sub-agent address: {}", e))?
                        .connect()
                        .await
                        .map_err(|e| format!("connect to sub-agent: {}", e))?;
                    let mut client = AgentServiceClient::new(channel);
                    client.dispatch_to_sub_agent(req).await.map_err(|e| e.to_string())
                }.await;

                match res {
                    Ok(r) => {
                        let inner = r.into_inner();
                        if !inner.error.is_empty() {
                            Err(ToolError::LlmRecoverable(inner.error))
                        } else {
                            Ok(format!("[Subagent (Fork)] Result: {}", inner.result))
                        }
                    }
                    Err(e) => Err(ToolError::LlmRecoverable(format!("Subagent failed: {}", e))),
                }
            }
        } else {
            return Err(ToolError::LlmRecoverable(format!("Unknown mode: {}", mode)));
        }
    }
}

pub fn subagent_tool(thread_id: Option<String>) -> Tool {
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
        execute: Arc::new(SubagentExecutor { thread_id }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_subagent_empty_task() {
        let executor = SubagentExecutor { thread_id: Some("test_thread".to_string()) };
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
        let executor = SubagentExecutor { thread_id: Some("test_thread".to_string()) };
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
        // Since fork mode attempts a real gRPC connection to `OHC_AGENT_ADDRESS` which is not running in test,
        // it will return an error (connect error). We assert that the error is correct.
        let executor = SubagentExecutor { thread_id: Some("test_thread".to_string()) };
        let args = json!({
            "task": "do something",
            "mode": "fork"
        });

        let result = executor.execute(args).await;
        assert!(result.is_err());
        let err_str = result.unwrap_err().to_string();
        assert!(err_str.contains("connect to sub-agent") || err_str.contains("invalid sub-agent address"));
    }
}
