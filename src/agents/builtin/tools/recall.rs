use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use dashmap::DashMap;

use super::{Tool, ToolExecutor};

pub struct RecallObservationExecutor {
    pub observation_store: Arc<DashMap<String, String>>,
}

#[async_trait::async_trait]
impl ToolExecutor for RecallObservationExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let tool_call_id = args["tool_call_id"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("recall_observation: tool_call_id is required".to_string()))?;

        match self.observation_store.get(tool_call_id) {
            Some(observation) => Ok(observation.clone()),
            None => Err(ToolError::LlmRecoverable(format!(
                "recall_observation: no observation found for tool_call_id '{}'",
                tool_call_id
            ))),
        }
    }
}

pub fn recall_observation_tool(observation_store: Arc<DashMap<String, String>>) -> Tool {
    Tool {
        name: "RecallObservation".to_string(),
        description: "Retrieve the full, unmasked output of a previous tool call by its tool_call_id. \
            Use this when a previous observation was masked due to its size.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "tool_call_id": {
                    "type": "string",
                    "description": "The unique ID of the tool call to recall."
                }
            },
            "required": ["tool_call_id"]
        }),
        execute: Arc::new(RecallObservationExecutor { observation_store }),
    }
}
