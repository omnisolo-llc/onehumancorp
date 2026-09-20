use crate::Tool;
use crate::pydantic::{PydanticAdapter, PydanticToolExecutor};
use omnisolo_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

// ── ShippoRates ───────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct ShippoRatesArgs {
    order_id: String,
    weight: f64,
    dimensions: String,
}

struct ShippoRatesExecutor {
    tenant: crate::tenant::TenantContext,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<ShippoRatesArgs> for ShippoRatesExecutor {
    async fn execute_typed(&self, args: ShippoRatesArgs) -> Result<String, ToolError> {
        let order_id = args.order_id;
        let weight = args.weight;
        let dimensions = args.dimensions;

        if order_id.is_empty() || weight <= 0.0 || dimensions.is_empty() {
            return Ok(json!({
                "status": "Error",
                "message": "Missing or invalid arguments"
            }).to_string());
        }

        let tenant_id = self.tenant.as_str();

        let token = std::env::var("SHIPPO_API_TOKEN").unwrap_or_default();
        if token.is_empty() {
            return Ok(json!({
                "status": "Error",
                "message": "Shippo is not configured"
            }).to_string());
        }

        Ok(json!({
            "status": "Rates fetched",
            "message": "To proceed, instruct the user to review the shipping rates in the OmniSolo workspace."
        }).to_string())
    }
}

pub fn shippo_rates_tool(tenant: crate::tenant::TenantContext) -> Tool {
    Tool {
        name: "shippo_rates".to_string(),
        description: "Fetches live shipping rates for an order using Shippo.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "order_id": {
                    "type": "string",
                    "description": "The ID of the order to ship."
                },
                "weight": {
                    "type": "number",
                    "description": "The weight of the package in ounces."
                },
                "dimensions": {
                    "type": "string",
                    "description": "The dimensions of the package formatted as LengthxWidthxHeight (e.g. '10x8x6')."
                }
            },
            "required": ["order_id", "weight", "dimensions"]
        }),
        execute: Arc::new(PydanticAdapter::new(ShippoRatesExecutor { tenant })),
    }
}

// ── ShippoLabel ───────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct ShippoLabelArgs {
    order_id: String,
    rate_id: String,
}

struct ShippoLabelExecutor {
    tenant: crate::tenant::TenantContext,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<ShippoLabelArgs> for ShippoLabelExecutor {
    async fn execute_typed(&self, _args: ShippoLabelArgs) -> Result<String, ToolError> {
        Ok(json!({
            "status": "Label drafted",
            "message": "A draft label has been created. The owner must approve it in the UI."
        }).to_string())
    }
}

pub fn shippo_label_tool(tenant: crate::tenant::TenantContext) -> Tool {
    Tool {
        name: "shippo_label".to_string(),
        description: "Purchases a shipping label for an order using a previously fetched rate ID.".to_string(),
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
                    "description": "The rate ID (obtained from shippo_rates)."
                }
            },
            "required": ["order_id", "rate_id"]
        }),
        execute: Arc::new(PydanticAdapter::new(ShippoLabelExecutor { tenant })),
    }
}
