use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;

use crate::marketing::qr_generate_tool;

#[tokio::test]
async fn test_qr_generate_success() {
    let tool = qr_generate_tool();

    let args = json!({
        "content": "https://example.com",
        "label": "My Test QR"
    });

    let result = tool.execute.execute(args).await.expect("Execution should succeed");

    let json_result: serde_json::Value = serde_json::from_str(&result).expect("Result should be JSON");

    assert_eq!(json_result["status"], "success");
    assert_eq!(json_result["label"], "My Test QR");
    assert_eq!(json_result["message"], "QR code for 'https://example.com' has been generated.");
    assert!(json_result["ascii_art"].as_str().unwrap().contains("██"));
}

#[tokio::test]
async fn test_qr_generate_missing_content() {
    let tool = qr_generate_tool();

    let args = json!({
        "label": "My Test QR"
    });

    let result = tool.execute.execute(args).await;

    assert!(result.is_err());

    if let Err(ToolError::LlmRecoverable(msg)) = result {
        assert!(msg.contains("Validation Error"));
    } else {
        panic!("Expected LlmRecoverable error");
    }
}
