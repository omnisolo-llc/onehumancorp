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

        // Returning the special error type to short-circuit the loop and signal the harness
        Err(ToolError::HandoffRequested { target_agent, context })
    }
}

pub fn handoff_tool() -> Tool {
    Tool {
        name: "Handoff".to_string(),
        description: "Yield execution to another agent. Use this when the required task is out of your scope or explicitly requires a different department.".to_string(),
        is_read_only: true, // It just signals the orchestrator
        parameters: json!({
            "type": "object",
            "properties": {
                "target_agent": {
                    "type": "string",
                    "description": "The target agent's department or identifier (e.g., 'Finance & Payments', 'Marketing')."
                },
                "context": {
                    "type": "string",
                    "description": "A comprehensive summary of the current state and what the target agent needs to do."
                }
            },
            "required": ["target_agent", "context"]
        }),
        execute: Arc::new(HandoffExecutor),
    }
}
