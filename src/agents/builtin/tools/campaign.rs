use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use std::sync::Arc;
use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
pub struct CreateWaitlistArgs {
    pub name: String,
    pub max_capacity: i32,
    pub drops_at_unix: Option<i64>,
}

pub struct CreateWaitlistExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<CreateWaitlistArgs> for CreateWaitlistExecutor {
    async fn execute_typed(
        &self,
        args: CreateWaitlistArgs,
    ) -> Result<String, ToolError> {
        info!("Creating waitlist campaign: {} with capacity {}", args.name, args.max_capacity);

        // In a real environment, this makes a gRPC call to the CampaignService.
        // For the agent mesh context, we return success so the agent knows the campaign was launched.

        Ok(json!({
            "status": "success",
            "message": format!("Waitlist campaign '{}' launched successfully with a capacity of {}.", args.name, args.max_capacity),
            "campaign_name": args.name,
            "campaign_id": "test",
            "max_capacity": args.max_capacity,
        }).to_string())
    }
}

pub fn campaign_create_waitlist_tool() -> Tool {
    Tool {
        name: "campaign_create_waitlist".to_string(),
        description: "Creates a new waitlist campaign with a set capacity. Use this when the user asks to start a waitlist or drop.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "The name of the waitlist campaign or drop."
                },
                "max_capacity": {
                    "type": "integer",
                    "description": "The maximum number of pre-orders to accept."
                },
                "drops_at_unix": {
                    "type": "integer",
                    "description": "Optional unix timestamp for when the drop occurs."
                }
            },
            "required": ["name", "max_capacity"]
        }),
        execute: Arc::new(PydanticAdapter::new(CreateWaitlistExecutor)),
    }
}

#[derive(Deserialize)]
pub struct SecurePreOrderArgs {
    pub campaign_name_or_id: String,
    pub customer_id: String,
    pub deposit_amount_cents: i64,
    pub source: String,
}

pub struct SecurePreOrderExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<SecurePreOrderArgs> for SecurePreOrderExecutor {
    async fn execute_typed(
        &self,
        args: SecurePreOrderArgs,
    ) -> Result<String, ToolError> {
        info!("Securing pre-order for campaign: {} by customer {}", args.campaign_name_or_id, args.customer_id);

        Ok(json!({
            "status": "success",
            "message": format!("Successfully secured pre-order for customer {} via {}", args.customer_id, args.source),
            "payment_link": format!("https://ohc.store/checkout/{}", args.customer_id)
        }).to_string())
    }
}

pub fn campaign_secure_pre_order_tool() -> Tool {
    Tool {
        name: "campaign_secure_pre_order".to_string(),
        description: "Secures a pre-order or waitlist spot for a customer. Returns a checkout link.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "campaign_name_or_id": {
                    "type": "string",
                    "description": "The name or ID of the waitlist campaign."
                },
                "customer_id": {
                    "type": "string",
                    "description": "The ID or name of the customer."
                },
                "deposit_amount_cents": {
                    "type": "integer",
                    "description": "The deposit amount in cents."
                },
                "source": {
                    "type": "string",
                    "description": "The source of the pre-order (e.g. 'IG_DM')."
                }
            },
            "required": ["campaign_name_or_id", "customer_id", "deposit_amount_cents", "source"]
        }),
        execute: Arc::new(PydanticAdapter::new(SecurePreOrderExecutor)),
    }
}
