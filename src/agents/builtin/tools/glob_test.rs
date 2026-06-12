use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;

use crate::glob::glob_tool;

#[tokio::test]
async fn test_glob_success() {
    let tool = glob_tool(None);

    let args = json!({
        "pattern": "**/*.rs",
<<<<<<< HEAD
        "path": "."
=======
        "path": "src/agents/builtin/tools"
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
    });

    let result = tool.execute.execute(args).await.expect("Execution should succeed");

<<<<<<< HEAD
    assert!(result.contains("src/agents/builtin/tools/glob.rs") || result.contains("glob.rs"));
=======
    assert!(result.contains("src/agents/builtin/tools/glob.rs"));
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
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
