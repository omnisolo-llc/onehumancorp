use ohc_builtin_agent_core::types::ToolError;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::VecDeque;
use std::sync::Arc;


use super::{SharedMailbox, Tool, ToolExecutor};

/// A message in the mailbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailboxMessage {
    pub from: String,
    pub to: String,
    pub content: String,
    pub timestamp_ms: i64,
}

/// In-process mailbox for agent messaging.
#[derive(Default)]
pub struct Mailbox {
    messages: VecDeque<MailboxMessage>,
}

impl Mailbox {
    pub fn send(&mut self, msg: MailboxMessage) {
        self.messages.push_back(msg);
    }

    pub fn receive_all(&mut self) -> Vec<MailboxMessage> {
        self.messages.drain(..).collect()
    }
}

struct SendMessageExecutor {
    mailbox: SharedMailbox,
}

#[async_trait::async_trait]
impl ToolExecutor for SendMessageExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let to = args["to"].as_str().unwrap_or("coordinator").to_string();
        let content = args["message"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("sendmessage: message is required".to_string()))?
            .to_string();

        let msg = MailboxMessage {
            from: "agent".to_string(),
            to: to.clone(),
            content,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
        };

        // Sanitize the `to` parameter to prevent path traversal
        let safe_to: String = to.chars()
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
            .collect();

        if safe_to.is_empty() {
            return Err(ToolError::LlmRecoverable("sendmessage: invalid recipient ID".to_string()));
        }

        let dir_path = ".agent_mailboxes";
        if let Err(e) = tokio::fs::create_dir_all(dir_path).await {
            return Err(ToolError::LlmRecoverable(format!("Failed to create mailbox directory: {}", e)));
        }

        let file_path = format!("{}/{}.log", dir_path, safe_to);
        let serialized = serde_json::to_string(&msg).unwrap_or_default();
        let serialized_with_newline = format!("{}\n", serialized);

        // Append to file
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)
            .await
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to open mailbox file: {}", e)))?;

        use tokio::io::AsyncWriteExt;
        file.write_all(serialized_with_newline.as_bytes())
            .await
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to write to mailbox file: {}", e)))?;

        self.mailbox.write().await.send(msg);
        Ok(format!("Message sent to {}.", to))
    }
}

pub fn sendmessage_tool(mailbox: SharedMailbox) -> Tool {
    Tool {
        name: "SendMessage".to_string(),
        description: "Send a message to the parent agent or coordinator. \
            Used for reporting sub-task results or requesting assistance."
            .to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "to": {
                    "type": "string",
                    "description": "Recipient agent ID (default: 'coordinator')."
                },
                "message": {
                    "type": "string",
                    "description": "The message content."
                }
            },
            "required": ["message"]
        }),
        execute: Arc::new(SendMessageExecutor { mailbox }),
    }
}
