use ohc_builtin_agent_core::types::{ToolCall, ToolError};
use crate::agent::AgentRunConfig;

/// ToolGater implements the Anthropic Mechanic: 3-Stage Tool Gating.
/// Trust establishment at project load -> Permission check before each tool call -> Explicit user confirmation for high-risk operations.
pub struct ToolGater;

impl ToolGater {
    pub fn check_gating(tc: &ToolCall, is_read_only: bool, cfg: &AgentRunConfig) -> Result<(), ToolError> {
        // Stage 1: Trust establishment at project load
        if !cfg.project_trusted && !is_read_only {
            return Err(ToolError::Fatal("Project not trusted. Mutating tools are disabled.".to_string()));
        }

        // Stage 2: Permission check before each tool call
        if let Some(allowed) = &cfg.allowed_tools {
            if !allowed.contains(&tc.name) {
                return Err(ToolError::Fatal(format!("Tool '{}' is not in the allowed list.", tc.name)));
            }
        }

        // Stage 3: Explicit user confirmation for high-risk operations
        if cfg.high_risk_tools.contains(&tc.name) && !cfg.approved_tool_calls.contains(&tc.id) {
            return Err(ToolError::UserFixable(format!("High-risk tool '{}' requires explicit user confirmation. Approve this tool call to proceed.", tc.name)));
        }

        Ok(())
    }
}
