use crate::{Tool, ToolExecutor};
use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use agent_service_proto::ohc::agent::service::{agent_service_client::AgentServiceClient, SubAgentRequest};

#[async_trait::async_trait]
pub trait SubagentDispatcher: Send + Sync {
    async fn dispatch(&self, req: SubAgentRequest) -> Result<agent_service_proto::ohc::agent::service::SubAgentResponse, String>;
}

pub struct GrpcSubagentDispatcher {
    addr: String,
}

impl GrpcSubagentDispatcher {
    pub fn new() -> Self {
        GrpcSubagentDispatcher {
            addr: std::env::var("OHC_AGENT_ADDRESS").unwrap_or_else(|_| "127.0.0.1:50051".to_string()),
        }
    }
}

#[async_trait::async_trait]
impl SubagentDispatcher for GrpcSubagentDispatcher {
    async fn dispatch(&self, req: SubAgentRequest) -> Result<agent_service_proto::ohc::agent::service::SubAgentResponse, String> {
        let channel = tonic::transport::Channel::from_shared(format!("http://{}", self.addr))
            .map_err(|e| format!("invalid sub-agent address: {}", e))?
            .connect()
            .await
            .map_err(|e| format!("connect to sub-agent: {}", e))?;
        let mut client = AgentServiceClient::new(channel);
        client.dispatch_to_sub_agent(req).await.map_err(|e| e.to_string()).map(|r| r.into_inner())
    }
}

pub struct SubagentExecutor {
    pub parent_thread_id: Option<String>,
    pub dispatcher: Arc<dyn SubagentDispatcher>,
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

        let mut req = SubAgentRequest::default();
        req.task = task.to_string();

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

            req.working_dir = worktree_path.clone();

            let res = self.dispatcher.dispatch(req).await;

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
                Ok(inner) => {
                    if !inner.error.is_empty() {
                        Err(ToolError::LlmRecoverable(inner.error))
                    } else {
                        Ok(inner.result)
                    }
                }
                Err(e) => Err(ToolError::LlmRecoverable(format!("Subagent failed: {}", e))),
            }
        } else if mode == "fork" {
            if let Some(thread_id) = &self.parent_thread_id {
                req.source_thread_id = thread_id.clone();
            }

            let res = self.dispatcher.dispatch(req).await;

            match res {
                Ok(inner) => {
                    if !inner.error.is_empty() {
                        Err(ToolError::LlmRecoverable(inner.error))
                    } else {
                        Ok(format!("[Subagent (Fork)] Completed task: {}. Summary: {}", task, inner.result))
                    }
                }
                Err(e) => Err(ToolError::LlmRecoverable(format!("Subagent fork failed: {}", e))),
            }
        } else if mode == "teammate" {
            let to = format!("teammate_{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
            let msg = json!({
                "from": "parent",
                "to": to,
                "task": task,
                "timestamp": chrono::Utc::now().timestamp_millis(),
            });

            // Write to file-based mailbox
            let mailboxes_dir = std::path::PathBuf::from(".agent_mailboxes");
            let _ = tokio::fs::create_dir_all(&mailboxes_dir).await;
            let mailbox_file = mailboxes_dir.join(format!("{}.log", to));

            let content = format!("{}\n", msg.to_string());
            use tokio::io::AsyncWriteExt;
            if let Ok(mut file) = tokio::fs::OpenOptions::new().create(true).append(true).open(&mailbox_file).await {
                let _ = file.write_all(content.as_bytes()).await;
            }

            // Also spawn the actual subagent so it does the work
            let res = self.dispatcher.dispatch(req).await;

            match res {
                Ok(inner) => {
                    if !inner.error.is_empty() {
                        Err(ToolError::LlmRecoverable(inner.error))
                    } else {
                        Ok(format!("[Subagent (Teammate)] Completed task: {}. Summary: {}", task, inner.result))
                    }
                }
                Err(e) => Err(ToolError::LlmRecoverable(format!("Subagent teammate failed: {}", e))),
            }
        } else {
            return Err(ToolError::LlmRecoverable(format!("Unknown mode: {}", mode)));
        }
    }
}

pub fn subagent_tool(parent_thread_id: Option<String>) -> Tool {
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
        execute: Arc::new(SubagentExecutor { parent_thread_id, dispatcher: Arc::new(GrpcSubagentDispatcher::new()) }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockSubagentDispatcher;

    #[async_trait::async_trait]
    impl SubagentDispatcher for MockSubagentDispatcher {
        async fn dispatch(&self, _req: SubAgentRequest) -> Result<agent_service_proto::ohc::agent::service::SubAgentResponse, String> {
            Ok(agent_service_proto::ohc::agent::service::SubAgentResponse {
                result: "Mocked success".to_string(),
                error: "".to_string(),
            })
        }
    }

    #[tokio::test]
    async fn test_subagent_empty_task() {
        let executor = SubagentExecutor { parent_thread_id: None, dispatcher: Arc::new(MockSubagentDispatcher) };
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
        let executor = SubagentExecutor { parent_thread_id: None, dispatcher: Arc::new(MockSubagentDispatcher) };
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
        let executor = SubagentExecutor { parent_thread_id: Some("test_thread_id".to_string()), dispatcher: Arc::new(MockSubagentDispatcher) };
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
}
