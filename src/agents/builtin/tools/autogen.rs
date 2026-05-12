use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use crate::autogen::AutoGenOrchestrator;

/// A Tool that allows an agent to trigger an AutoGen pattern sub-task.
pub struct AutoGenTool {
    pub orchestrator: Arc<AutoGenOrchestrator>,
}

#[async_trait::async_trait]
impl crate::tools::ToolExecutor for AutoGenTool {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let task = args["sub_task"].as_str().unwrap_or("");
        if task.is_empty() {
            return Err(ToolError::LlmRecoverable("sub_task is required".to_string()));
        }

        match self.orchestrator.run(task).await {
            Ok(res) => {
                Ok(format!("Pattern sub-task completed in {} rounds. Final Result: {}", res.rounds, res.final_response))
            }
            Err(e) => Err(ToolError::LlmRecoverable(format!("Pattern execution failed: {}", e))),
        }
    }
}

pub fn autogen_tool(orchestrator: Arc<AutoGenOrchestrator>) -> crate::tools::Tool {
    crate::tools::Tool {
        name: "autogen_subtask".to_string(),
        description: "Delegate a complex sub-task to a pre-configured team of specialized agents using an AutoGen pattern.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "sub_task": { "type": "string", "description": "The specific sub-task to delegate." }
            },
            "required": ["sub_task"]
        }),
        execute: Arc::new(AutoGenTool { orchestrator }),
    }
}
