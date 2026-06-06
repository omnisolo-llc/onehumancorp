#[cfg(ohc_bazel)]
use ohc_builtin_agent_tools::todowrite::{todoread_tool, todowrite_tool};
#[cfg(not(ohc_bazel))]
use crate::todowrite::{todoread_tool, todowrite_tool};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_todo_write_and_read() {
    let todos = Arc::new(RwLock::new(Vec::new()));
    let write_tool = todowrite_tool(todos.clone());
    let read_tool = todoread_tool(todos.clone());

    let write_args = serde_json::json!({
        "todos": [
            {"id": "t1", "content": "Fix bug", "status": "pending"},
            {"id": "t2", "content": "Write test", "status": "completed"}
        ]
    });

    let write_res = write_tool.execute.execute(write_args).await.unwrap();
    assert_eq!(write_res, "Todo list updated with 2 items.");

    let read_res = read_tool.execute.execute(serde_json::json!({})).await.unwrap();
    assert!(read_res.contains("Fix bug"));
    assert!(read_res.contains("Write test"));
    assert!(read_res.contains("t1"));
    assert!(read_res.contains("t2"));
    assert!(read_res.contains("pending"));
    assert!(read_res.contains("completed"));
}

#[tokio::test]
async fn test_todo_write_invalid_args() {
    let todos = Arc::new(RwLock::new(Vec::new()));
    let write_tool = todowrite_tool(todos.clone());

    let write_args = serde_json::json!({
        "wrong_field": "test"
    });

    let write_res = write_tool.execute.execute(write_args).await;
    assert!(write_res.is_err());

    // We expect LlmRecoverable
    let err = write_res.unwrap_err();
    assert_eq!(err.to_string(), "Recoverable error: todowrite: todos must be an array");
}

#[tokio::test]
async fn test_todo_read_empty() {
    let todos = Arc::new(RwLock::new(Vec::new()));
    let read_tool = todoread_tool(todos.clone());

    let read_res = read_tool.execute.execute(serde_json::json!({})).await.unwrap();
    assert_eq!(read_res, "Todo list is empty.");
}
