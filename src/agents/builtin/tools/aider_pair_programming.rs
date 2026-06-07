use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

/// SOTA Harness Pattern: Aider: human-in-loop pair programming.
/// Simulates a pair-programming prompt interaction with a human in the terminal.
pub struct AiderPairProgrammingExecutor;

impl AiderPairProgrammingExecutor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ToolExecutor for AiderPairProgrammingExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let prompt = args.get("prompt").and_then(|v| v.as_str()).unwrap_or("Review this code");
        let context = args.get("context").and_then(|v| v.as_str()).unwrap_or("");

        let msg = format!("Pair Programming Request: {}\nContext: {}", prompt, context);
        Err(ToolError::UserFixable(msg))
    }
}

pub fn aider_pair_programming_tool() -> Tool {
    Tool {
        name: "AiderPairProgramming".to_string(),
        description: "Engages the user in human-in-loop pair programming. Pauses execution and asks the user to review code, provide ideas, or co-author a complex feature. (Aider Mechanic)".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "prompt": {
                    "type": "string",
                    "description": "The question or context you want to pair-program with the human on."
                },
                "context": {
                    "type": "string",
                    "description": "Code snippet or architecture context."
                }
            },
            "required": ["prompt"]
        }),
        execute: Arc::new(AiderPairProgrammingExecutor::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_aider_pair_programming() {
        let executor = AiderPairProgrammingExecutor::new();
        let res = executor.execute(json!({"prompt": "How does this look?", "context": "fn foo() {}"})).await;

        assert!(res.is_err());
        if let Err(ToolError::UserFixable(msg)) = res {
            assert!(msg.contains("Pair Programming Request"));
            assert!(msg.contains("How does this look?"));
        } else {
            panic!("Expected UserFixable error");
        }
    }
}
