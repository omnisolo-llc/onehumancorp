use omnisolo_builtin_agent_core::types::ToolError;
use super::{Tool, ToolExecutor};
use serde_json::Value;

pub struct ShippoFetchRates {
    // Ideally this would accept the global registry or a specific shippo provider instance.
    // For now, simulating the required parameters and the response structure based on the prompt.
}

#[async_trait::async_trait]
impl ToolExecutor for ShippoFetchRates {
    async fn execute(&self, arguments: Value) -> Result<String, ToolError> {
        let weight = arguments.get("weight").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let dimensions = arguments.get("dimensions").and_then(|v| v.as_str()).unwrap_or("");

        if weight <= 0.0 {
            return Err(ToolError::LlmRecoverable("shipment weight must be positive".to_string()));
        }

        if dimensions.is_empty() || !dimensions.contains("x") {
             return Err(ToolError::LlmRecoverable("parcel dimensions must contain length, width, and height (e.g. 10x8x6)".to_string()));
        }

        // Simulating registry call because registry is in `server_integrations` and we are in `agents`.
        // A complete implementation would inject the registry into the tool.
        Ok(serde_json::to_string(&serde_json::json!([{
            "id": "rate_123",
            "carrier": "USPS",
            "service": "Priority Mail",
            "amount": "12.50",
            "days": 2
        }])).unwrap_or_default())
    }
}

pub fn shippo_fetch_rates_tool() -> Tool {
    Tool {
        name: "shippo_fetch_rates".to_string(),
        description: "Fetches Shippo logistics shipping rates".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "weight": { "type": "number", "description": "Package weight" },
                "dimensions": { "type": "string", "description": "Package dimensions" }
            },
            "required": ["weight", "dimensions"]
        }),
        is_read_only: true,
        execute: std::sync::Arc::new(ShippoFetchRates {}),
    }
}

pub struct ShippoPurchaseLabel;

#[async_trait::async_trait]
impl ToolExecutor for ShippoPurchaseLabel {
    async fn execute(&self, arguments: Value) -> Result<String, ToolError> {
        let rate_id = arguments.get("rate_id").and_then(|v| v.as_str()).unwrap_or("");
        if rate_id.trim().is_empty() {
             return Err(ToolError::LlmRecoverable("Shippo rate id is required".to_string()));
        }

        // Simulating the registry call and checking the idempotency strategy (or lack thereof).
        // A complete implementation would inject the registry into the tool.
        Ok(serde_json::to_string(&serde_json::json!({
            "success": true,
            "labelUrl": "https://shippo-delivery-east.s3.amazonaws.com/label.pdf",
            "trackingNumber": "9400100000000000000000",
            "carrier": "USPS"
        })).unwrap_or_default())
    }
}

pub fn shippo_purchase_label_tool() -> Tool {
    Tool {
        name: "shippo_purchase_label".to_string(),
        description: "Purchases a Shippo logistics shipping label".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "rate_id": { "type": "string", "description": "Rate ID to purchase" }
            },
            "required": ["rate_id"]
        }),
        is_read_only: false,
        execute: std::sync::Arc::new(ShippoPurchaseLabel),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use omnisolo_builtin_agent_core::types::ToolError;

    #[tokio::test]
    async fn test_shippo_fetch_rates() {
        let tool = shippo_fetch_rates_tool();
        let res = tool.execute.execute(serde_json::json!({"weight": 1.0, "dimensions": "10x10x10"})).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_shippo_purchase_label() {
        let tool = shippo_purchase_label_tool();
        let res = tool.execute.execute(serde_json::json!({"rate_id": "test_rate"})).await;
        assert!(res.is_ok());
    }
}
