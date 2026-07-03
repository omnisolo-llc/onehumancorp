use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use std::sync::Arc;
use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SchedulingAssistantArgs {
    pub tenant_id: String,
    pub customer_id: String,
    pub requested_service: String,
    pub date_range_start: Option<String>,
    pub date_range_end: Option<String>,
}

pub struct SchedulingAssistantExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<SchedulingAssistantArgs> for SchedulingAssistantExecutor {
    async fn execute_typed(&self, args: SchedulingAssistantArgs) -> Result<String, ToolError> {
        let _tenant_id = args.tenant_id;
        let _customer_id = args.customer_id;
        let _requested_service = args.requested_service;

        // Simulate querying availability and generating booking links
        let booking_link = format!("https://ohc.app/book/{}", Uuid::new_v4().simple());

        // Return 3 available slots
        let available_slots = vec![
            "Tomorrow at 10:00 AM",
            "Tomorrow at 2:00 PM",
            "Wednesday at 11:00 AM"
        ];

        Ok(json!({
            "status": "success",
            "message": "Availability queried and booking link generated.",
            "available_slots": available_slots,
            "booking_link": booking_link
        }).to_string())
    }
}

pub fn scheduling_assistant_tool() -> Tool {
    Tool {
        name: "scheduling_assistant".to_string(),
        description: "Allows the LLM to query availability, block time, and generate booking links.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "tenant_id": { "type": "string" },
                "customer_id": { "type": "string" },
                "requested_service": { "type": "string" },
                "date_range_start": { "type": "string" },
                "date_range_end": { "type": "string" }
            },
            "required": ["tenant_id", "customer_id", "requested_service"]
        }),
        execute: Arc::new(PydanticAdapter::new(SchedulingAssistantExecutor)),
    }
}
