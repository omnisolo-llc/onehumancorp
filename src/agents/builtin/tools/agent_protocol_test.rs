use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use super::agent_protocol::agent_protocol_tool;

#[tokio::test]
async fn test_agent_protocol_tool_executor_success() {
    let tool = agent_protocol_tool();
    let args = json!({
        "endpoint": "http://localhost:8000/ap",
        "method": "ap_list_tasks",
        "params": {}
    });
    let result: Result<String, ToolError> = tool.execute.execute(args).await;
    assert!(result.is_ok());
    let msg = result.unwrap();
    assert!(msg.contains("ap_list_tasks executed successfully"));
}

#[tokio::test]
async fn test_agent_protocol_tool_executor_missing_arg() {
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
