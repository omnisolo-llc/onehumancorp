use ohc_builtin_agent_core::types::ToolError;
use ohc_builtin_agent_tools::ToolExecutor;
use serde_json::Value;
use std::sync::Arc;
use crate::checkpointer::CheckpointSaver;

/// TimeTravelDebugger tool allowing the agent to inspect or restore previous checkpoints.
pub struct TimeTravelDebugger {
    pub checkpointer: Arc<dyn CheckpointSaver>,
    pub thread_id: String,
}

#[async_trait::async_trait]
impl ToolExecutor for TimeTravelDebugger {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let action = args.get("action").and_then(|v| v.as_str()).ok_or_else(|| ToolError::LlmRecoverable("missing 'action' (list, inspect, restore)".to_string()))?;

        match action {
            "list" => {
                let cps = self.checkpointer.list_checkpoints(&self.thread_id).await
                    .map_err(|e| ToolError::Unexpected(format!("Failed to list checkpoints: {}", e)))?;

                let mut res = String::from("Available Super-step Checkpoints:\n");
                for cp in cps {
                    res.push_str(&format!("- ID: {}, Created: {}\n", cp.checkpoint_id, cp.created_at));
                }
                Ok(res)
            }
            "inspect" => {
                let id = args.get("checkpoint_id").and_then(|v| v.as_str()).ok_or_else(|| ToolError::LlmRecoverable("missing 'checkpoint_id'".to_string()))?;
                let cp = self.checkpointer.get_checkpoint(&self.thread_id, id).await
                    .map_err(|e| ToolError::Unexpected(format!("Failed to get checkpoint: {}", e)))?
                    .ok_or_else(|| ToolError::LlmRecoverable(format!("Checkpoint {} not found", id)))?;

                Ok(serde_json::to_string_pretty(&cp.data).unwrap_or_else(|_| "Error serializing checkpoint data".to_string()))
            }
            "restore" => {
                let id = args.get("checkpoint_id").and_then(|v| v.as_str()).ok_or_else(|| ToolError::LlmRecoverable("missing 'checkpoint_id'".to_string()))?;
                self.checkpointer.restore_checkpoint(id).await
                    .map_err(|e| ToolError::Unexpected(format!("Failed to restore workspace: {}", e)))?;

                Ok(format!("Workspace restored to checkpoint {}. Note: Conversation history is NOT automatically rewound by this tool call; use this to recover files or environment state.", id))
            }
            _ => Err(ToolError::LlmRecoverable(format!("Unknown action: {}", action))),
        }
    }
}

pub fn debugger_tool(checkpointer: Arc<dyn CheckpointSaver>, thread_id: String) -> ohc_builtin_agent_tools::Tool {
    ohc_builtin_agent_tools::Tool {
        name: "TimeTravelDebugger".to_string(),
        description: "Inspect or restore previous agent states (super-steps). Use 'list' to see checkpoints, 'inspect' to see data, and 'restore' to revert files/workspace.".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "action": { "type": "string", "enum": ["list", "inspect", "restore"] },
                "checkpoint_id": { "type": "string" }
            },
            "required": ["action"]
        }),
        is_read_only: true,
        execute: Arc::new(TimeTravelDebugger { checkpointer, thread_id }),
    }
}
