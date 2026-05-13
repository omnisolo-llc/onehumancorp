use ohc_builtin_agent_core::types::{ToolCall, ToolResult, ToolError};

// LangGraph Mechanic (4-types):
// 1) Transient: retry with backoff
// 2) LLM-recoverable: return raw error as ToolMessage
// 3) User-fixable: interrupt execution
// 4) Unexpected/Fatal: bubble up

pub struct LangGraphErrorMechanic;

impl LangGraphErrorMechanic {
    pub fn handle_transient(msg: &str, retry_count: &mut usize, max_retries: usize) -> Result<(), ToolError> {
        if *retry_count < max_retries {
            *retry_count += 1;
            // Backoff logic handled natively by caller, but state tracks retries
            Ok(())
        } else {
            Err(ToolError::Transient(msg.to_string()))
        }
    }

    pub fn handle_llm_recoverable(
        tc: &ToolCall,
        msg: &str,
        tool_error_counts: &mut std::collections::HashMap<String, usize>,
        max_retries: usize,
    ) -> Result<ToolResult, ToolError> {
        let count = tool_error_counts.entry(tc.name.clone()).or_insert(0);
        *count += 1;
        if *count > max_retries {
            let fatal_msg = format!(
                "Tool '{}' failed consecutively beyond max_retries limit with recoverable errors. Escalating to Fatal to prevent compounding error loops. Last error: {}",
                tc.name, msg
            );
            return Err(ToolError::Fatal(fatal_msg));
        }

        // Return the raw error as a ToolMessage directly to the model so it can self-correct
        Ok(ToolResult {
            tool_call_id: tc.id.clone(),
            content: String::new(),
            error: msg.to_string(),
        })
    }

    pub fn handle_user_fixable(msg: &str) -> ToolError {
        ToolError::UserFixable(msg.to_string())
    }

    pub fn handle_fatal(msg: &str) -> ToolError {
        ToolError::Fatal(msg.to_string())
    }

    pub fn handle_unexpected(msg: &str) -> ToolError {
        ToolError::Unexpected(msg.to_string())
    }
}
