use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use std::path::PathBuf;
use tokio::fs;
use ohc_builtin_agent_tools::repo_map::repo_map_tool;

#[tokio::test]
async fn test_repo_map_extraction() {
    let temp_dir = tempfile::tempdir().unwrap();
    let wd = temp_dir.path().to_path_buf();

    // Create some files
    fs::write(wd.join("main.rs"), "pub fn main() {}\nstruct User;\n").await.unwrap();
    fs::write(wd.join("utils.py"), "def helper():\n    pass\nclass Helper:\n    pass").await.unwrap();

    let sub_dir = wd.join("src");
    fs::create_dir(&sub_dir).await.unwrap();
    fs::write(sub_dir.join("lib.ts"), "export function doIt() {}\ninterface Option {}\n").await.unwrap();

    // Ignored directory
    let ignored_dir = wd.join("node_modules");
    fs::create_dir(&ignored_dir).await.unwrap();
    fs::write(ignored_dir.join("bad.js"), "function hack() {}").await.unwrap();

    let tool = repo_map_tool(Some(wd.clone()));

    let args = json!({});
    let result = tool.execute.execute(args).await.unwrap();

    println!("Result: {}", result);

    assert!(result.contains("RepoMap for"));

    assert!(result.contains("📄 main.rs"));
    assert!(result.contains("│ pub fn main() {}"));
    assert!(result.contains("│ struct User;"));

    assert!(result.contains("📄 utils.py"));
    assert!(result.contains("│ def helper():"));
    assert!(result.contains("│ class Helper:"));

    assert!(result.contains("📄 src/lib.ts"));
    assert!(result.contains("│ export function doIt() {}"));
    assert!(result.contains("│ interface Option {}"));

    assert!(!result.contains("bad.js"));
    assert!(!result.contains("node_modules"));
    assert!(result.contains("Total files indexed: 3"));
}

#[tokio::test]
async fn test_repo_map_not_found() {
    let tool = repo_map_tool(Some(PathBuf::from("/does/not/exist/12345")));
    let args = json!({"path": "sub"});
    let result = tool.execute.execute(args).await;
    assert!(result.is_err());
    if let Err(ToolError::LlmRecoverable(msg)) = result {
        assert!(msg.contains("Failed to canonicalize base path") || msg.contains("Directory not found"));
    } else {
        panic!("Expected recoverable error");
    }
}
