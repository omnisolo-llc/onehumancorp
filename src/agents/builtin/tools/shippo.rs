use omnisolo_builtin_agent_core::types::ToolError;
use crate::Tool;
use crate::pydantic::{PydanticAdapter, PydanticToolExecutor};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
pub struct ShippoRatesRequest {
    pub order_id: String,
    pub weight: String,
    pub dimensions: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShippoPurchaseRequest {
    pub order_id: String,
    pub rate_id: String,
}

struct ShippoRatesExecutor {
    #[allow(dead_code)]
    tenant: crate::tenant::TenantContext,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<ShippoRatesRequest> for ShippoRatesExecutor {
    async fn execute_typed(&self, args: ShippoRatesRequest) -> Result<String, ToolError> {
        // Use the backend internal RPC/HTTP since shipping persistence logic is bound to the DB in `src/server/api/shipping.rs`.
        let local_port = std::env::var("OMNISOLO_PORT").unwrap_or_else(|_| "18789".to_string());
        let url = format!("http://127.0.0.1:{}/api/v1/shipping/rates", local_port);
        let client = reqwest::Client::new();

        let req_body = serde_json::json!({
            "orderId": args.order_id,
            "weight": args.weight,
            "dimensions": args.dimensions
        });

        // Use the generic API key format
        let key = std::env::var("OMNISOLO_API_KEY").unwrap_or_default();

        let resp = client.post(&url)
            .header("Authorization", format!("Bearer {}", key))
            .header("Content-Type", "application/json")
            .json(&req_body)
            .send()
            .await
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to reach shipping API: {}", e)))?;

        let status = resp.status();
        let body_text = resp.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(ToolError::LlmRecoverable(format!("Shipping API failed ({}): {}", status, body_text)));
        }

        Ok(body_text)
    }
}

pub fn shippo_rates_tool(tenant: crate::tenant::TenantContext) -> Tool {
    Tool {
        name: "shippo_rates".to_string(),
        description: "Fetch shipping rates from Shippo for a given order, weight, and dimensions. Must provide dimensions exactly as LengthxWidthxHeight.".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "order_id": {
                    "type": "string",
                    "description": "The ID of the order to ship."
                },
                "weight": {
                    "type": "string",
                    "description": "The weight of the package in ounces (e.g. '16' or '2.5')."
                },
                "dimensions": {
                    "type": "string",
                    "description": "The dimensions of the package in inches, formatted exactly as 'LengthxWidthxHeight' (e.g. '10x8x6')."
                }
            },
            "required": ["order_id", "weight", "dimensions"]
        }),
        is_read_only: true,
        execute: Arc::new(PydanticAdapter::new(ShippoRatesExecutor { tenant })),
    }
}

struct ShippoPurchaseExecutor {
    #[allow(dead_code)]
    tenant: crate::tenant::TenantContext,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<ShippoPurchaseRequest> for ShippoPurchaseExecutor {
    async fn execute_typed(&self, args: ShippoPurchaseRequest) -> Result<String, ToolError> {
        let local_port = std::env::var("OMNISOLO_PORT").unwrap_or_else(|_| "18789".to_string());
        let url = format!("http://127.0.0.1:{}/api/v1/shipping/label", local_port);
        let client = reqwest::Client::new();

        let req_body = serde_json::json!({
            "orderId": args.order_id,
            "rateId": args.rate_id
        });

        let key = std::env::var("OMNISOLO_API_KEY").unwrap_or_default();

        let resp = client.post(&url)
            .header("Authorization", format!("Bearer {}", key))
            .header("Content-Type", "application/json")
            .json(&req_body)
            .send()
            .await
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to reach shipping API: {}", e)))?;

        let status = resp.status();
        let body_text = resp.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(ToolError::LlmRecoverable(format!("Shipping API failed ({}): {}", status, body_text)));
        }

        Ok(body_text)
    }
}

pub fn shippo_purchase_tool(tenant: crate::tenant::TenantContext) -> Tool {
    Tool {
        name: "shippo_purchase_label".to_string(),
        description: "Purchase a shipping label from Shippo for a given order and rate ID. The API will persist the tracking number and update order status.".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "order_id": {
                    "type": "string",
                    "description": "The ID of the order."
                },
                "rate_id": {
                    "type": "string",
                    "description": "The ID of the shipping rate to purchase."
                }
            },
            "required": ["order_id", "rate_id"]
        }),
        is_read_only: false,
        execute: Arc::new(PydanticAdapter::new(ShippoPurchaseExecutor { tenant })),
    }
}
