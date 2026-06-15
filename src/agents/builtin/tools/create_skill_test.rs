use super::*;
use serde_json::json;

#[tokio::test]
async fn test_create_skill() {
    let tool = create_skill_tool();

    // Test match
    let res = tool.execute.execute(json!({
        "name": "MySkill",
        "description": "A new skill",
        "instruction": "Do something"
    })).await.unwrap();

    assert!(res.contains("MySkill"));
    assert!(res.contains("A new skill"));
}
