use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use serde::Deserialize;
use std::sync::Arc;

use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

#[derive(Deserialize)]
struct AnchorArgs {
    text: String,
    reason: Option<String>,
}

struct AnchorExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<AnchorArgs> for AnchorExecutor {
    async fn execute_typed(&self, args: AnchorArgs) -> Result<String, ToolError> {
        let text = args.text;
        let reason = args.reason.unwrap_or_else(|| "Important context".to_string());

        Ok(format!(
            "[SYSTEM NOTIFICATION: Context Rot Prevention Anchor]\nAnchored Text: {}\nReason: {}\n[This text will be preserved during context compaction.]",
            text, reason
        ))
    }
}

pub fn anchor_tool() -> Tool {
    Tool {
        name: "AnchorContext".to_string(),
        description: "Anchor specific critical information so that it is permanently retained across compactions and context window rotations, bypassing standard decay rules. Use this for crucial IDs, summaries, or constraints. (Self-Reflective Context Anchoring)".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "text": {
                    "type": "string",
                    "description": "The exact text to anchor in context."
                },
                "reason": {
                    "type": "string",
                    "description": "Why this text needs to be anchored."
                }
            },
            "required": ["text"]
        }),
        execute: Arc::new(PydanticAdapter::new(AnchorExecutor)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_anchor_tool_basic() {
        let tool = anchor_tool();
        let args = json!({
            "text": "CRITICAL_ID_12345",
            "reason": "Used for all subsequent API calls"
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert!(result.contains("[SYSTEM NOTIFICATION: Context Rot Prevention Anchor]"));
        assert!(result.contains("CRITICAL_ID_12345"));
        assert!(result.contains("Used for all subsequent API calls"));
    }

    #[tokio::test]
    async fn test_anchor_tool_default_reason() {
        let tool = anchor_tool();
        let args = json!({
            "text": "Only use Python 3.9+"
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert!(result.contains("Important context"));
        assert!(result.contains("Only use Python 3.9+"));
    }
}
