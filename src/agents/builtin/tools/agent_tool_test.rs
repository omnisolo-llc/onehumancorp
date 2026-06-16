use super::*;
use serde_json::json;

#[tokio::test]
async fn test_agent_stop() {
    let tool = agent_stop_tool();
    let res = tool.execute.execute(json!({"task_id": "12345"})).await.unwrap();
    assert_eq!(res, "Stop requested for task 12345.");
}

#[tokio::test]
async fn test_agent_status() {
    let tool = agent_status_tool();
    let res = tool.execute.execute(json!({"task_id": "12345"})).await.unwrap();
    assert!(res.contains("running"));
}
