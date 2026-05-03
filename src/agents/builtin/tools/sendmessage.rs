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

        // Strict path traversal / security sanitization for mailbox name
        if !to.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            return Err(ToolError::LlmRecoverable("sendmessage: 'to' field contains invalid characters. Only alphanumeric, dashes, and underscores are allowed.".to_string()));
        }

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

        // Send to in-memory mailbox
        self.mailbox.write().await.send(msg.clone());

        // File-based mailbox (Teammate communication mechanic)
        let mailboxes_dir = std::path::PathBuf::from(".agent_mailboxes");
        if !mailboxes_dir.exists() {
            let _ = tokio::fs::create_dir_all(&mailboxes_dir).await;
        }
        let mailbox_file = mailboxes_dir.join(format!("{}.log", to));
        let json_str = serde_json::to_string(&msg).map_err(|e| ToolError::LlmRecoverable(format!("Failed to serialize message: {}", e)))?;

        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&mailbox_file)
            .await
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to open mailbox: {}", e)))?;

        file.write_all(format!("{}\n", json_str).as_bytes())
            .await
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to write mailbox: {}", e)))?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_sendmessage_writes_to_file() {
        let mailbox = Arc::new(RwLock::new(Mailbox::default()));
        let executor = SendMessageExecutor { mailbox: mailbox.clone() };
        let args = json!({
            "to": "test_teammate",
            "message": "Hello from test"
        });

        let res = executor.execute(args).await;
        assert!(res.is_ok());

        let mailbox_file = std::path::PathBuf::from(".agent_mailboxes/test_teammate.log");
        assert!(mailbox_file.exists());

        let content = tokio::fs::read_to_string(&mailbox_file).await.unwrap();
        assert!(content.contains("Hello from test"));

        let _ = tokio::fs::remove_file(mailbox_file).await;
    }
}
