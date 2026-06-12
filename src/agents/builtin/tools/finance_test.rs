use serde_json::json;

use crate::finance::finance_report_tool;

#[tokio::test]
async fn test_finance_report_default() {
    let tool = finance_report_tool();

    let args = json!({});

    let result = tool.execute.execute(args).await.expect("Execution should succeed");

    let json_result: serde_json::Value = serde_json::from_str(&result).expect("Result should be JSON");

    assert_eq!(json_result["status"], "success");
    assert_eq!(json_result["report_type"], "weekly_summary");
    assert!(json_result["metrics"]["revenue"].as_f64().unwrap() > 0.0);
}

#[tokio::test]
async fn test_finance_report_custom() {
    let tool = finance_report_tool();

    let args = json!({
        "report_type": "monthly_trends",
        "start_date": "2026-01-01"
    });

    let result = tool.execute.execute(args).await.expect("Execution should succeed");

    let json_result: serde_json::Value = serde_json::from_str(&result).expect("Result should be JSON");

    assert_eq!(json_result["status"], "success");
    assert_eq!(json_result["report_type"], "monthly_trends");
}
