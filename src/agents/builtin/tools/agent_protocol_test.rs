use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use super::agent_protocol::agent_protocol_tool;

#[tokio::test]
async fn test_agent_protocol_tool_executor_missing_arg() {
    unsafe { std::env::set_var("MCPANY_DANGEROUS_ALLOW_LOCAL_IPS", "true"); }
    unsafe { std::env::set_var("MCPANY_DANGEROUS_ALLOW_LOCAL_IPS", "true"); }

    let tool = agent_protocol_tool();
    let args = json!({
        "endpoint": "http://localhost:8000/ap",
        "method": "ap_list_tasks"
    });
    let result: Result<String, ToolError> = tool.execute.execute(args).await;
    assert!(result.is_err());
    if let Err(ToolError::LlmRecoverable(msg)) = result {
        assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
    } else {
        panic!("Expected Pydantic-first validation error");
    }
}

#[tokio::test]
async fn test_agent_protocol_tool_executor_invalid_url() {
    let tool = agent_protocol_tool();
    let args = json!({
        "endpoint": "file:///etc/passwd",
        "method": "ap_list_tasks",
        "params": {}
    });
    let result: Result<String, ToolError> = tool.execute.execute(args).await;
    assert!(result.is_err());
    if let Err(ToolError::LlmRecoverable(msg)) = result {
        assert!(msg.contains("is invalid or points to a blocked local/private IP address (SSRF protection)."));
    } else {
        panic!("Expected SSRF protection error");
    }
}
