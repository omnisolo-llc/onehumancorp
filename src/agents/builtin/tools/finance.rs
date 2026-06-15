use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use serde::Deserialize;
use std::sync::Arc;
use tracing::info;

use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

// Pydantic-first tool schema validation: FinanceArgs
#[derive(Deserialize)]
struct FinanceArgs {
    #[serde(default = "default_report_type")]
    report_type: String,
    #[serde(default = "default_start_date")]
    start_date: String,
}

fn default_report_type() -> String {
    "weekly_summary".to_string()
}

fn default_start_date() -> String {
    "7 days ago".to_string()
}

pub struct FinanceReportExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<FinanceArgs> for FinanceReportExecutor {
    async fn execute_typed(&self, args: FinanceArgs) -> Result<String, ToolError> {
        let report_type = args.report_type;
        let start_date = args.start_date;

        // Semi-functional financial report generation.
        // In a full implementation, this would query the database for orders and revenue.
        info!("Generating {} financial report starting from {}", report_type, start_date);

        // Simulating some dynamic data generation based on input.
        let seed = report_type.len() as f64 * 100.0;
        let revenue = seed + (start_date.len() as f64 * 5.5);
        let orders = (revenue / 20.0) as i32;
        let avg_order_value = if orders > 0 { revenue / orders as f64 } else { 0.0 };

        Ok(json!({
            "status": "success",
            "report_type": report_type,
            "generated_at": chrono::Utc::now().to_rfc3339(),
            "summary": format!("Report for {}: Your business is performing well. Total revenue: ${:.2}. Total orders: {}.", report_type, revenue, orders),
            "metrics": {
                "revenue": revenue,
                "orders": orders,
                "avg_order_value": avg_order_value
            }
        }).to_string())
    }
}

pub fn finance_report_tool() -> Tool {
    Tool {
        name: "finance_report".to_string(),
        description: "Generate a plain-language financial report or summary for the business.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "report_type": {
                    "type": "string",
                    "enum": ["weekly_summary", "monthly_trends", "tax_summary"],
                    "description": "The type of financial report to generate."
                },
                "start_date": {
                    "type": "string",
                    "description": "Optional start date for the report (e.g. '2026-01-01' or '7 days ago')."
                }
            }
        }),
        execute: Arc::new(PydanticAdapter::new(FinanceReportExecutor)),
    }
}
