use super::{
    Tool,
    pydantic::{PydanticAdapter, PydanticToolExecutor},
};
use omnisolo_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use server_integrations_shippo::provider::ShippoProvider;

#[derive(Deserialize)]
struct ShippingLabelArgs {
    order_id: String,
    rate_id: String,
}

struct ShippingLabelExecutor {
    provider: Arc<ShippoProvider>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<ShippingLabelArgs> for ShippingLabelExecutor {
    async fn execute_typed(&self, args: ShippingLabelArgs) -> Result<String, ToolError> {
        if args.order_id.is_empty() || args.rate_id.is_empty() {
            return Err(ToolError::LlmRecoverable("order_id and rate_id are required".to_string()));
        }

        let response = self.provider.purchase_label(&args.rate_id).await.map_err(|e: String| ToolError::LlmRecoverable(e))?;
        Ok(json!(response).to_string())
    }
}

pub fn shipping_label_tool(provider: Arc<ShippoProvider>) -> Tool {
    Tool {
        name: "shipping_label".to_string(),
        description: "Purchases a shipping label for a given order using Shippo.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "order_id": {
                    "type": "string",
                    "description": "The ID of the order"
                },
                "rate_id": {
                    "type": "string",
                    "description": "The selected rate ID to purchase"
                }
            },
            "required": ["order_id", "rate_id"]
        }),
        execute: Arc::new(PydanticAdapter::new(ShippingLabelExecutor { provider })),
    }
}
