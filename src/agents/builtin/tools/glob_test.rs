use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;

use crate::glob::glob_tool;

#[tokio::test]
async fn test_glob_success() {
    let tool = glob_tool(None);

    let args = json!({
        "pattern": "**/*.rs",
        "path": "."
    });

    let result = tool.execute.execute(args).await.expect("Execution should succeed");

    assert!(result.contains("src/agents/builtin/tools/glob.rs") || result.contains("glob.rs"));
}

#[tokio::test]
async fn test_glob_missing_pattern() {
    let tool = glob_tool(None);

    let args = json!({
        "path": "src/agents/builtin/tools"
    });

    let result = tool.execute.execute(args).await;

    assert!(result.is_err());

    if let Err(ToolError::LlmRecoverable(msg)) = result {
        assert!(msg.contains("Validation Error"));
    } else {
        panic!("Expected LlmRecoverable error");
    }
}
