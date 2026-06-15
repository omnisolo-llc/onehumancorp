use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use crate::finance::finance_report_tool;

#[tokio::test]
async fn test_finance_executor_success_default_args() {
    let tool = finance_report_tool();
    let args = json!({});
    let result: Result<String, ToolError> = tool.execute.execute(args).await;
    assert!(result.is_ok());
    let msg = result.unwrap();
    assert!(msg.contains("weekly_summary"));
}

#[tokio::test]
async fn test_finance_executor_success_custom_args() {
    let tool = finance_report_tool();
    let args = json!({
        "report_type": "monthly_trends",
        "start_date": "2026-01-01"
    });
    let result: Result<String, ToolError> = tool.execute.execute(args).await;
    assert!(result.is_ok());
    let msg = result.unwrap();
    assert!(msg.contains("monthly_trends"));
}

#[tokio::test]
async fn test_finance_executor_invalid_arg_type() {
    let tool = finance_report_tool();
    // report_type is a string
    let args = json!({
        "report_type": 123
    });
    let result: Result<String, ToolError> = tool.execute.execute(args).await;
    assert!(result.is_err());
    if let Err(ToolError::LlmRecoverable(msg)) = result {
        assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
    } else {
        panic!("Expected Pydantic-first validation error");
    }
}
