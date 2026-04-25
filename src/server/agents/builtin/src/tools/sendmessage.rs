use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;

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
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let to = args["to"].as_str().unwrap_or("coordinator").to_string();
        let content = args["message"]
            .as_str()
            .ok_or("sendmessage: message is required")?
            .to_string();

        let msg = MailboxMessage {
            from: "agent".to_string(),
            to: to.clone(),
            content,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
        };

        self.mailbox.write().await.send(msg);
        Ok(format!("Message sent to {}.", to))
    }
}

pub fn sendmessage_tool(mailbox: SharedMailbox) -> Tool {
    Tool {
        is_mutating: true,
        name: "SendMessage".to_string(),
        description: "Send a message to the parent agent or coordinator. \
            Used for reporting sub-task results or requesting assistance."
            .to_string(),
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
