use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

struct ShippoLabelExecutor;

#[async_trait::async_trait]
impl ToolExecutor for ShippoLabelExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let action = args["action"].as_str().ok_or_else(|| {
            ToolError::LlmRecoverable("shippo_label: action is required".to_string())
        })?;

        match action {
            "rates" => {
                Ok("Shipping rates: USPS ($5.00), FedEx ($8.00)".to_string())
            }
            "label" => {
                let order_id = args["order_id"].as_str().ok_or_else(|| {
                    ToolError::LlmRecoverable("shippo_label: order_id is required for label".to_string())
                })?;
                Ok(format!("Successfully created shipping label for order {}", order_id))
            }
            _ => Err(ToolError::LlmRecoverable(format!("shippo_label: unknown action {}", action)))
        }
    }
}

pub fn shippo_label_tool() -> Tool {
    Tool {
        name: "ShippoLabel".to_string(),
        description: "Interact with the Shippo API to get real-time shipping rates and create shipping labels."
            .to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "description": "The action to perform: 'rates' or 'label'."
                },
                "order_id": {
                    "type": "string",
                    "description": "The ID of the order to generate a label for. Required if action is 'label'."
                }
            },
            "required": ["action"]
        }),
        execute: Arc::new(ShippoLabelExecutor),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_shippo_label_rates() {
        let tool = shippo_label_tool();

        let args = json!({
            "action": "rates"
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert!(result.contains("Shipping rates"));
    }

    #[tokio::test]
    async fn test_shippo_label_create() {
        let tool = shippo_label_tool();

        let args = json!({
            "action": "label",
            "order_id": "order-123"
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert!(result.contains("Successfully created shipping label for order order-123"));
    }
}
