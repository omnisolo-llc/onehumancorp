use super::*;
use serde_json::json;

#[tokio::test]
async fn test_toolsearch() {
    let tool = toolsearch_tool();

    // Test match
    let res = tool.execute.execute(json!({"query": "fetch"})).await.unwrap();
    assert!(res.contains("WebFetch"));

    // Test no match
    let res = tool.execute.execute(json!({"query": "nonexistenttool123"})).await.unwrap();
    assert!(res.contains("No tools found"));
}
