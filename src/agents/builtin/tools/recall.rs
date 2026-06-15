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
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let tool_call_id = args["tool_call_id"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("recall_observation: tool_call_id is required".to_string()))?;

        match self.observation_store.get(tool_call_id) {
            Some(content) => Ok(content.clone()),
            None => Err(ToolError::LlmRecoverable(format!("recall_observation: Tool call ID '{}' not found in observation store. It may have expired or was never stored.", tool_call_id))),
        }
    }
}

pub fn recall_observation_tool(observation_store: Arc<DashMap<String, String>>) -> Tool {
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
        execute: Arc::new(RecallObservationExecutor { observation_store }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dashmap::DashMap;

    #[tokio::test]
    async fn test_recall_executor_success() {
        let store = Arc::new(DashMap::new());
        store.insert("tool_1".to_string(), "original_content".to_string());

        let executor = RecallObservationExecutor {
            observation_store: store,
        };

        let args = json!({ "tool_call_id": "tool_1" });
        let result = executor.execute(args).await.unwrap();
        assert_eq!(result, "original_content");
    }

    #[tokio::test]
    async fn test_recall_executor_not_found() {
        let store = Arc::new(DashMap::new());
        let executor = RecallObservationExecutor {
            observation_store: store,
        };

        let args = json!({ "tool_call_id": "missing_id" });
        let result = executor.execute(args).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ToolError::LlmRecoverable(msg) => assert!(msg.contains("not found in observation store")),
            _ => panic!("Expected LlmRecoverable error"),
        }
    }

    #[tokio::test]
    async fn test_recall_executor_missing_arg() {
        let store = Arc::new(DashMap::new());
        let executor = RecallObservationExecutor {
            observation_store: store,
        };

        let args = json!({});
        let result = executor.execute(args).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ToolError::LlmRecoverable(msg) => assert!(msg.contains("tool_call_id is required")),
            _ => panic!("Expected LlmRecoverable error for missing argument"),
        }
    }
}
