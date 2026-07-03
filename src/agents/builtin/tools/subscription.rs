use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use std::sync::Arc;
use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SubscriptionAssistantArgs {
    pub tenant_id: String,
    pub customer_id: String,
    pub amount_cents: i64,
    pub billing_cycle: String, // "monthly", "weekly", "yearly"
    pub plan_name: String,
}

pub struct SubscriptionAssistantExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<SubscriptionAssistantArgs> for SubscriptionAssistantExecutor {
    async fn execute_typed(&self, args: SubscriptionAssistantArgs) -> Result<String, ToolError> {
        let _tenant_id = args.tenant_id;
        let _customer_id = args.customer_id;
        let amount_cents = args.amount_cents;
        let _billing_cycle = args.billing_cycle;
        let _plan_name = args.plan_name;

        // Simulate creating a subscription and returning a checkout link
        let subscription_id = Uuid::new_v4().to_string();
        let amount_usd = amount_cents as f64 / 100.0;
        let payment_link = format!("https://ohc.app/sub/{}", subscription_id.replace("-", ""));

        Ok(json!({
            "status": "success",
            "message": "Subscription drafted successfully.",
            "subscription_id": subscription_id,
            "payment_link": payment_link,
            "upcoming_revenue": format!("${:.2}", amount_usd)
        }).to_string())
    }
}

pub fn subscription_assistant_tool() -> Tool {
    Tool {
        name: "subscription_assistant".to_string(),
        description: "Allows the LLM to draft recurring payment links, pause subscriptions, and summarize upcoming revenue.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "tenant_id": { "type": "string" },
                "customer_id": { "type": "string" },
                "amount_cents": { "type": "integer" },
                "billing_cycle": { "type": "string", "enum": ["weekly", "monthly", "yearly"] },
                "plan_name": { "type": "string" }
            },
            "required": ["tenant_id", "customer_id", "amount_cents", "billing_cycle", "plan_name"]
        }),
        execute: Arc::new(PydanticAdapter::new(SubscriptionAssistantExecutor)),
    }
}
