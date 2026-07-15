use crate::tools::escalate_to_human::escalate_to_human_tool;
use serde_json::json;

#[tokio::test]
async fn test_escalate_to_human_tool_integration() {
    let tool = escalate_to_human_tool();

    let input = json!({
        "tenant_id": "tenant_integration",
        "thread_id": "thread_integration",
        "summary": "Customer issue needs human review.",
        "reason": "Escalation"
    });

    let result_str = tool.execute.execute(input).await.unwrap();
    let result: serde_json::Value = serde_json::from_str(&result_str).unwrap();

    assert_eq!(result["status"], "handoff_requested");
}
