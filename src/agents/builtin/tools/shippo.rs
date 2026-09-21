use omnisolo_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use crate::pydantic::{PydanticAdapter, PydanticToolExecutor};
use super::Tool;

#[derive(Deserialize)]
struct ShippoFetchRatesArgs {
    order_id: String,
    weight: String,
    dimensions: String,
}

struct ShippoFetchRatesExecutor { tenant: super::tenant::TenantContext, }

#[async_trait::async_trait]
impl PydanticToolExecutor<ShippoFetchRatesArgs> for ShippoFetchRatesExecutor {
    async fn execute_typed(&self, args: ShippoFetchRatesArgs) -> Result<String, ToolError> {
        let mut body = std::collections::HashMap::new();
        body.insert("orderId".to_string(), args.order_id);
        body.insert("weight".to_string(), args.weight);
        body.insert("dimensions".to_string(), args.dimensions);


        let url = std::env::var("OMNISOLO_BACKEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let endpoint = format!("{url}/api/v1/shipping/rates");
        let mut req = reqwest::Client::new().post(&endpoint);
        let tenant_id = self.tenant.as_str();
        req = req.header("x-omnisolo-tenant", tenant_id);
        let res = req
            .json(&body)
            .send()
            .await;

        match res {
            Ok(response) => {
                let status = response.status();
                if status.is_success() {
                    let result_json = response.text().await.unwrap_or_default();
                    Ok(result_json)
                } else {
                    let text = response.text().await.unwrap_or_default();
                    Err(ToolError::LlmRecoverable(format!("Failed with status {}: {}", status.as_u16(), text)))
                }
            },
            Err(e) => {
                Err(ToolError::LlmRecoverable(e.to_string()))
            }
        }
    }
}

pub fn shippo_fetch_rates_tool(tenant: super::tenant::TenantContext) -> Tool {
    Tool {
        name: "shippo_fetch_rates".to_string(),
        description: "Fetches shipping rates from Shippo given an order ID, weight, and dimensions (e.g. 10x8x6).".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "order_id": {
                    "type": "string",
                    "description": "The ID of the order."
                },
                "weight": {
                    "type": "string",
                    "description": "The weight of the package in ounces (e.g., '16')."
                },
                "dimensions": {
                    "type": "string",
                    "description": "The dimensions of the package formatted as LengthxWidthxHeight (e.g., '10x8x6')."
                }
            },
            "required": ["order_id", "weight", "dimensions"]
        }),
        execute: Arc::new(PydanticAdapter::new(ShippoFetchRatesExecutor { tenant })),
    }
}

#[derive(Deserialize)]
struct ShippoPurchaseLabelArgs {
    order_id: String,
    rate_id: String,
}

struct ShippoPurchaseLabelExecutor { tenant: super::tenant::TenantContext, }

#[async_trait::async_trait]
impl PydanticToolExecutor<ShippoPurchaseLabelArgs> for ShippoPurchaseLabelExecutor {
    async fn execute_typed(&self, args: ShippoPurchaseLabelArgs) -> Result<String, ToolError> {
        let mut body = std::collections::HashMap::new();
        body.insert("orderId".to_string(), args.order_id);
        body.insert("rateId".to_string(), args.rate_id);


        let url = std::env::var("OMNISOLO_BACKEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let endpoint = format!("{url}/api/v1/shipping/label");
        let mut req = reqwest::Client::new().post(&endpoint);
        let tenant_id = self.tenant.as_str();
        req = req.header("x-omnisolo-tenant", tenant_id);
        let res = req
            .json(&body)
            .send()
            .await;

        match res {
            Ok(response) => {
                let status = response.status();
                if status.is_success() {
                    let result_json = response.text().await.unwrap_or_default();
                    Ok(result_json)
                } else {
                    let text = response.text().await.unwrap_or_default();
                    Err(ToolError::LlmRecoverable(format!("Failed with status {}: {}", status.as_u16(), text)))
                }
            },
            Err(e) => {
                Err(ToolError::LlmRecoverable(e.to_string()))
            }
        }
    }
}

pub fn shippo_purchase_label_tool(tenant: super::tenant::TenantContext) -> Tool {
    Tool {
        name: "shippo_purchase_label".to_string(),
        description: "Purchases a shipping label from Shippo given an order ID and rate ID.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "order_id": {
                    "type": "string",
                    "description": "The ID of the order."
                },
                "rate_id": {
                    "type": "string",
                    "description": "The selected rate ID."
                }
            },
            "required": ["order_id", "rate_id"]
        }),
        execute: Arc::new(PydanticAdapter::new(ShippoPurchaseLabelExecutor { tenant })),
    }
}
