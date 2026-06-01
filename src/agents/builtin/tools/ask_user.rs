use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

struct AskUserExecutor;

#[async_trait::async_trait]
impl ToolExecutor for AskUserExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let question = args["question"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("ask_user: question is required".to_string()))?;

        // Human-in-loop as spectrum: Return UserFixable error to bubble up to the orchestrator
        // to interrupt execution and ask the human.
        Err(ToolError::UserFixable(question.to_string()))
    }
}

pub fn ask_user_tool() -> Tool {
    Tool {
        name: "AskUser".to_string(),
        description: "Interrupts execution to ask the human user a question or request manual intervention.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "question": {
                    "type": "string",
                    "description": "The question or instruction to present to the user."
                }
            },
            "required": ["question"]
        }),
        execute: Arc::new(AskUserExecutor),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ask_user_missing_question() {
        let executor = AskUserExecutor;
        let args = json!({});

        let result = executor.execute(args).await;
        assert!(result.is_err());
        match result {
            Err(ToolError::LlmRecoverable(msg)) => {
                assert_eq!(msg, "ask_user: question is required");
            }
            _ => panic!("Expected LlmRecoverable error"),
        }
    }

    #[tokio::test]
    async fn test_ask_user_success() {
        let executor = AskUserExecutor;
        let args = json!({
            "question": "Can you provide the API key?"
        });

        let result = executor.execute(args).await;
        assert!(result.is_err());
        match result {
            Err(ToolError::UserFixable(msg)) => {
                assert_eq!(msg, "Can you provide the API key?");
            }
            _ => panic!("Expected UserFixable error"),
        }
    }
}
