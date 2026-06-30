use ohc_builtin_agent_core::types::ToolError;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::VecDeque;
use std::sync::Arc;


use super::{SharedMailbox, Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

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

#[derive(Deserialize)]
struct SendMessageArgs {
    to: Option<String>,
    message: String,
}


struct SendMessageExecutor {
    mailbox: SharedMailbox,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<SendMessageArgs> for SendMessageExecutor {
    async fn execute_typed(&self, args: SendMessageArgs) -> Result<String, ToolError> {
        let to = args.to.unwrap_or("coordinator".to_string());
        let content = args.message;

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
        execute: Arc::new(PydanticAdapter::new(SendMessageExecutor { mailbox })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_sendmessage_basic() {
        let mailbox = Arc::new(RwLock::new(Mailbox::default()));
        let tool = sendmessage_tool(mailbox.clone());
        let args = serde_json::json!({
            "to": "boss",
            "message": "hello world"
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert_eq!(result, "Message sent to boss.");

        let msgs = mailbox.write().await.receive_all();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].to, "boss");
        assert_eq!(msgs[0].content, "hello world");
    }

    #[tokio::test]
    async fn test_sendmessage_default_to() {
        let mailbox = Arc::new(RwLock::new(Mailbox::default()));
        let tool = sendmessage_tool(mailbox.clone());
        let args = serde_json::json!({
            "message": "hello world"
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert_eq!(result, "Message sent to coordinator.");

        let msgs = mailbox.write().await.receive_all();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].to, "coordinator");
    }
}
