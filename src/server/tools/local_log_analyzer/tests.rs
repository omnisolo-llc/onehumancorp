use super::LogAnalyzerTool;
use std::fs;
use std::env;
use chrono::{Utc, Duration};

#[test]
fn test_log_analyzer_tool() {
    let dir = env::temp_dir();
    let file_path = dir.join("agent_harness.log");

    let now = Utc::now();
    let recent = now - Duration::minutes(5);
    let old = now - Duration::minutes(100);

    let recent_str = recent.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let old_str = old.format("%Y-%m-%dT%H:%M:%SZ").to_string();

    let log_data = format!("{} INFO something\n{} ERROR error 1\n{} ERROR error 2\n", old_str, recent_str, recent_str);
    fs::write(&file_path, log_data).unwrap();

    let tool = LogAnalyzerTool {
        log_dir: dir.to_path_buf(),
    };

    let result = tool.execute("ERROR", 60).unwrap();
    assert!(result.contains("Found 2 logs"));
    assert!(result.contains("error 1"));
    assert!(result.contains("error 2"));

    let result2 = tool.execute("INFO", 60).unwrap();
    assert!(result2.contains("Found 0 logs"));

    let result3 = tool.execute("INFO", 120).unwrap();
    assert!(result3.contains("Found 1 logs"));
}
