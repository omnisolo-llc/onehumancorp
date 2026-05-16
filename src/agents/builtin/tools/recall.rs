use ohc_builtin_agent_core::types::ToolError;
use ohc_builtin_agent_core::context::ContextManager;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

pub struct RecallObservationExecutor {
    pub context_manager: Arc<tokio::sync::Mutex<ContextManager>>,
}

#[async_trait::async_trait]
impl ToolExecutor for RecallObservationExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let tool_call_id = args["tool_call_id"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("recall_observation: tool_call_id is required".to_string()))?;

        let cm = self.context_manager.lock().await;
        match cm.recall_observation(tool_call_id) {
            Some(content) => Ok(content),
            None => Err(ToolError::LlmRecoverable(format!("recall_observation: Tool call ID '{}' not found in observation store. It may have expired or was never stored.", tool_call_id))),
        }
    }
}

pub fn recall_observation_tool(context_manager: Arc<tokio::sync::Mutex<ContextManager>>) -> Tool {
    Tool {
        name: "RecallObservation".to_string(),
        description: "Retrieves the full original output of a previously masked tool observation. \
            Use this when you need the detailed results of a tool call that has been truncated or masked in the conversation history."
            .to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "tool_call_id": {
                    "type": "string",
                    "description": "The unique ID of the tool call to recall. This can be found in the masked observation message."
                }
            },
            "required": ["tool_call_id"]
        }),
        execute: Arc::new(RecallObservationExecutor { context_manager }),
    }
}
