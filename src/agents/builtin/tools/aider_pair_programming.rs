use ohc_builtin_agent_core::types::ToolError;
<<<<<<< HEAD
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use super::{Tool, pydantic::{PydanticAdapter, PydanticToolExecutor}};

// SOTA Harness Pattern: Pydantic-first tool schema validation.
#[derive(Deserialize)]
struct AiderPairProgrammingArgs {
    prompt: String,
    context: Option<String>,
}
=======
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))

/// SOTA Harness Pattern: Aider: human-in-loop pair programming.
/// Simulates a pair-programming prompt interaction with a human in the terminal.
pub struct AiderPairProgrammingExecutor;

impl AiderPairProgrammingExecutor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
<<<<<<< HEAD
impl PydanticToolExecutor<AiderPairProgrammingArgs> for AiderPairProgrammingExecutor {
    async fn execute_typed(&self, args: AiderPairProgrammingArgs) -> Result<String, ToolError> {
        let prompt = args.prompt;
        let context = args.context.unwrap_or_default();
=======
impl ToolExecutor for AiderPairProgrammingExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let prompt = args.get("prompt").and_then(|v| v.as_str()).unwrap_or("Review this code");
        let context = args.get("context").and_then(|v| v.as_str()).unwrap_or("");
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))

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
<<<<<<< HEAD
        execute: Arc::new(PydanticAdapter::new(AiderPairProgrammingExecutor::new())),
=======
        execute: Arc::new(AiderPairProgrammingExecutor::new()),
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_aider_pair_programming() {
        let executor = AiderPairProgrammingExecutor::new();
<<<<<<< HEAD
        let res = executor.execute_typed(AiderPairProgrammingArgs {
            prompt: "How does this look?".to_string(),
            context: Some("fn foo() {}".to_string()),
        }).await;
=======
        let res = executor.execute(json!({"prompt": "How does this look?", "context": "fn foo() {}"})).await;
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))

        assert!(res.is_err());
        if let Err(ToolError::UserFixable(msg)) = res {
            assert!(msg.contains("Pair Programming Request"));
            assert!(msg.contains("How does this look?"));
        } else {
            panic!("Expected UserFixable error");
        }
    }
}
