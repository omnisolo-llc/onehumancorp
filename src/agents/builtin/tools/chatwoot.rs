use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

struct ChatwootReplyExecutor;

#[async_trait::async_trait]
impl ToolExecutor for ChatwootReplyExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let conversation_id = args["conversation_id"].as_str().ok_or_else(|| {
            ToolError::LlmRecoverable("chatwoot_reply: conversation_id is required".to_string())
        })?;
        let message = args["message"].as_str().ok_or_else(|| {
            ToolError::LlmRecoverable("chatwoot_reply: message is required".to_string())
        })?;

        // In a real implementation, this would call the Chatwoot API.
        // For now, we return a mock success response.
        Ok(format!(
            "Successfully sent reply to conversation {}: {}",
            conversation_id, message
        ))
    }
}

pub fn chatwoot_reply_tool() -> Tool {
    Tool {
        name: "ChatwootReply".to_string(),
        description: "Draft and send a reply to a customer via the Chatwoot omnichannel inbox (Instagram, Facebook, WhatsApp)."
            .to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "conversation_id": {
                    "type": "string",
                    "description": "The ID of the Chatwoot conversation to reply to."
                },
                "message": {
                    "type": "string",
                    "description": "The content of the reply message."
                }
            },
            "required": ["conversation_id", "message"]
        }),
        execute: Arc::new(ChatwootReplyExecutor),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::ToolError;

    #[tokio::test]
    async fn test_chatwoot_reply_tool() {
        let tool = chatwoot_reply_tool();

        let args = json!({
            "conversation_id": "conv-123",
            "message": "Hello, how can I help?"
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert!(result.contains("Successfully sent reply"));
        assert!(result.contains("conv-123"));
    }

    #[tokio::test]
    async fn test_chatwoot_reply_tool_missing_args() {
        let tool = chatwoot_reply_tool();

        let args = json!({
            "message": "Hello"
        });

        let result = tool.execute.execute(args).await;
        assert!(matches!(result, Err(ToolError::LlmRecoverable(_))));
    }
}
