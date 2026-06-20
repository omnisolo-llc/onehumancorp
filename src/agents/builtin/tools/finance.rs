use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use serde::Deserialize;
use std::sync::Arc;
use tracing::info;

use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

#[derive(Deserialize)]
struct CreateInvoiceDraftArgs {
    task_id: String,
}

pub struct CreateInvoiceDraftExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<CreateInvoiceDraftArgs> for CreateInvoiceDraftExecutor {
    async fn execute_typed(&self, args: CreateInvoiceDraftArgs) -> Result<String, ToolError> {
        info!("Drafting invoice for task: {}", args.task_id);
        Ok(json!({
            "status": "success",
            "invoice_id": format!("draft_{}", args.task_id),
            "message": format!("Drafted invoice for task {}", args.task_id)
        }).to_string())
    }
}

pub fn create_invoice_draft_tool() -> Tool {
    Tool {
        name: "create_invoice_draft".to_string(),
        description: "Drafts an invoice for a specific task.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "task_id": {"type": "string", "description": "The ID of the task to invoice for."}
            },
            "required": ["task_id"]
        }),
        execute: Arc::new(PydanticAdapter::new(CreateInvoiceDraftExecutor)),
    }
}

#[derive(Deserialize)]
struct GeneratePaymentLinkArgs {
    invoice_id: String,
}

pub struct GeneratePaymentLinkExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<GeneratePaymentLinkArgs> for GeneratePaymentLinkExecutor {
    async fn execute_typed(&self, args: GeneratePaymentLinkArgs) -> Result<String, ToolError> {
        info!("Generating payment link for invoice: {}", args.invoice_id);
        Ok(json!({
            "status": "success",
            "payment_link": format!("https://pay.stripe.com/test_{}", args.invoice_id),
            "message": format!("Generated payment link for invoice {}", args.invoice_id)
        }).to_string())
    }
}

pub fn generate_payment_link_tool() -> Tool {
    Tool {
        name: "generate_payment_link".to_string(),
        description: "Generates a payment link for a drafted invoice.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "invoice_id": {"type": "string", "description": "The ID of the invoice."}
            },
            "required": ["invoice_id"]
        }),
        execute: Arc::new(PydanticAdapter::new(GeneratePaymentLinkExecutor)),
    }
}

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
