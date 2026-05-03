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
    working_dir: Option<std::path::PathBuf>,
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

        self.mailbox.write().await.send(msg.clone());

        // Teammate mode: File-based Mailboxes
        let wd = self.working_dir.clone().unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")));
        let mailboxes_dir = wd.join(".agent_mailboxes");
        if !mailboxes_dir.exists() {
            let _ = std::fs::create_dir_all(&mailboxes_dir);
        }
        let mailbox_file = mailboxes_dir.join(format!("{}.log", to));
        if let Ok(json_line) = serde_json::to_string(&msg) {
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&mailbox_file)
                .map_err(|e| ToolError::LlmRecoverable(format!("Failed to open mailbox file: {}", e)))?;
            use std::io::Write;
            let _ = writeln!(file, "{}", json_line);
        }

        Ok(format!("Message sent to {}.", to))
    }
}

pub fn sendmessage_tool(mailbox: SharedMailbox, working_dir: Option<std::path::PathBuf>) -> Tool {
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
        execute: Arc::new(SendMessageExecutor { mailbox, working_dir }),
    }
}
