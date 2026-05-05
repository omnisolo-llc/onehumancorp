use ohc_builtin_agent_core::types::ToolError;
use serde_json::Value;

pub struct HandoffExecutor;

#[async_trait::async_trait]
impl crate::ToolExecutor for HandoffExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let department = args.get("department").and_then(|v| v.as_str()).unwrap_or("");
        let summary = args.get("context_summary").and_then(|v| v.as_str()).unwrap_or("");

        if department.is_empty() {
            return Err(ToolError::LlmRecoverable("handoff: department is required".to_string()));
        }
        if summary.is_empty() {
            return Err(ToolError::LlmRecoverable("handoff: context_summary is required".to_string()));
        }

        // The AutoGen Handoff mechanic: We return a structured string that the Orchestration Loop
        // detects and uses to break the current loop and transfer control.
        Ok(format!("[HANDOFF_TRIGGERED: {}] Context: {}", department, summary))
    }
}
