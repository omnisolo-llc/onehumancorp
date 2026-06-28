use ohc_builtin_agent_core::types::{ToolCall, ToolError};
use ohc_builtin_agent_tools::Tool;

pub struct ToolExecutionEngine;

impl ToolExecutionEngine {
    /// Executes a single tool using the LangGraph 4-tier Error Handling Mechanic (Compounding Error Prevention).
    #[tracing::instrument(skip(tool, tc), fields(tool_name = %tc.name, tool_call_id = %tc.id))]
    pub async fn execute_tool_with_langgraph_mechanics(
        tool: &Tool,
        tc: &ToolCall,
        max_retries: usize,
    ) -> Result<String, ToolError> {
        crate::error_handling::LangGraphErrorHandlingMechanic::execute(tool, tc, max_retries).await
    }
}
