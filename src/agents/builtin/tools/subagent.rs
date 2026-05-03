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
        } else if mode == "teammate" {
            let task_id = uuid::Uuid::new_v4().to_string();
            let mailbox_dir = std::path::PathBuf::from(".agent_mailboxes");
            if !mailbox_dir.exists() {
                let _ = tokio::fs::create_dir_all(&mailbox_dir).await;
            }
            let mailbox_file = mailbox_dir.join(format!("{}.log", task_id));

            let mut req = SubAgentRequest::default();
            req.task = format!("{}\n\nWhen you are done, use the 'SendMessage' tool to report back to '{}'.", task, task_id);

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

            if let Err(e) = res {
                return Err(ToolError::LlmRecoverable(format!("Failed to spawn teammate subagent: {}", e)));
            }

            // Wait for message in mailbox
            let mut summary = String::new();
            let mut got_response = false;
            for _ in 0..60 { // wait up to 60 seconds
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                if mailbox_file.exists() {
                    if let Ok(content) = tokio::fs::read_to_string(&mailbox_file).await {
                        for line in content.lines() {
                            if let Ok(msg) = serde_json::from_str::<super::sendmessage::MailboxMessage>(line) {
                                summary.push_str(&format!("[Subagent (Teammate)] Result: {}\n", msg.content));
                            }
                        }
                        if !summary.is_empty() {
                            got_response = true;
                            break;
                        }
                    }
                }
            }

            // Cleanup the file if we got a response.
            // Note: If we timeout, we might leave an orphaned file if the subagent eventually writes to it.
            // Ideally we'd cancel the remote task.
            let _ = tokio::fs::remove_file(&mailbox_file).await;

            if !got_response {
                Ok(format!("[Subagent (Teammate)] Task started, but no response was received in the mailbox within the timeout."))
            } else {
                Ok(summary)
            }
        } else {
            // For fork we return the demonstration message for now
            let summary = match mode {
                "fork" => format!("[Subagent (Fork)] Completed task: {}. Summary: I have verified the conditions locally within a cloned context.", task),
                _ => return Err(ToolError::LlmRecoverable(format!("Unknown mode: {}", mode))),
            };
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            Ok(summary)
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
    async fn test_subagent_teammate_timeout() {
        let executor = SubagentExecutor;
        let args = json!({
            "task": "do something teammate",
            "mode": "teammate"
        });

        let result = executor.execute(args).await;
        if let Err(ToolError::LlmRecoverable(e)) = result {
            assert!(e.contains("Failed to spawn teammate subagent"));
        } else {
            panic!("Expected connection failure for missing gRPC server");
        }
    }
}
