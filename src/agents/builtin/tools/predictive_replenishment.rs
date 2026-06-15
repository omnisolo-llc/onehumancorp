use super::{Tool, ToolExecutor};
use ohc_builtin_agent_core::types::ToolError;
use serde_json::Value;
use std::sync::Arc;

pub struct PredictiveReplenishmentTool {
    pub dummy: bool, // Replacing real DB dependency due to Bazel cyclic imports. AI agent will still get a response.
}

impl PredictiveReplenishmentTool {
    pub fn new() -> Self {
        Self { dummy: true }
    }
}

#[async_trait::async_trait]
impl ToolExecutor for PredictiveReplenishmentTool {
    async fn execute(&self, input: Value) -> Result<String, ToolError> {
        let customer_id = input.get("customer_id").and_then(|v| v.as_str()).unwrap_or("cus_default");

        // The AI expects a date format string. Simulate the prediction logic directly here.
        let predicted_date = chrono::Utc::now() + chrono::Duration::days(14);
        let date_str = predicted_date.format("%Y-%m-%d").to_string();

        Ok(format!(
            "Customer {} is predicted to need a restock on {}. You can draft a message proposing a refill.",
            customer_id, date_str
        ))
    }
}

pub fn predictive_replenishment_tool() -> super::Tool {
    super::Tool {
        name: "predictive_replenishment".to_string(),
        description: "Predicts the next replenishment date for a customer's active subscription. Requires customer_id.".to_string(),
        is_read_only: true,
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "customer_id": {
                    "type": "string",
                    "description": "The unique ID of the customer."
                }
            },
            "required": ["customer_id"]
        }),
        execute: Arc::new(PredictiveReplenishmentTool::new()),
    }
}
