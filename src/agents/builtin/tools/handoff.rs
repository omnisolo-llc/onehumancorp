use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

struct HandoffExecutor;

#[async_trait::async_trait]
impl ToolExecutor for HandoffExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let target_agent = args["target_agent"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("handoff: 'target_agent' is required".to_string()))?
            .to_string();

        let context = args["context"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("handoff: 'context' is required".to_string()))?
            .to_string();

        Err(ToolError::HandoffRequested {
            target_agent,
            context,
        })
    }
}

pub fn handoff_tool() -> Tool {
    Tool {
        name: "Handoff".to_string(),
        description: "Yields execution to another specialized agent. Use this when the user's request is outside of your specialized domain and should be handled by a different department. Execution halts immediately after this tool is called.".to_string(),
        is_read_only: false, // Mutates the execution state by halting
        parameters: json!({
            "type": "object",
            "properties": {
                "target_agent": {
                    "type": "string",
                    "description": "The exact name of the agent to hand off to (e.g., 'Operations', 'Marketing & Advertising', 'Sales & Acquisition', 'Customer Success', 'Finance & Payments', 'Legal & Compliance', 'Business Advisory')."
                },
                "context": {
                    "type": "string",
                    "description": "A detailed summary of what the user is trying to accomplish, what has been done so far, and what the target agent needs to do."
                }
            },
            "required": ["target_agent", "context"]
        }),
        execute: Arc::new(HandoffExecutor),
    }
}
