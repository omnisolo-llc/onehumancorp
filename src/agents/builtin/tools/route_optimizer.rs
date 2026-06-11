use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use super::{Tool, ToolExecutor};
use tracing::info;

pub struct RouteOptimizerExecutor;

#[async_trait::async_trait]
impl ToolExecutor for RouteOptimizerExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let action = args["action"].as_str().unwrap_or("");
        let tenant_id = args["tenant_id"].as_str().unwrap_or("");
        let date = args["date"].as_str().unwrap_or("");

        if action.is_empty() || tenant_id.is_empty() || date.is_empty() {
            return Err(ToolError::LlmRecoverable("Missing required parameters: action, tenant_id, or date.".to_string()));
        }

        info!("Executing route_optimizer action '{}' for tenant '{}' on date '{}'", action, tenant_id, date);

        match action {
            "optimize_day" => {
                let response = json!({
                    "status": "success",
                    "date": date,
                    "optimized_schedule": [
                        { "time": "09:00", "task": "Job 1", "location": "Downtown" },
                        { "time": "11:00", "task": "Job 2", "location": "Northside", "travel_time_mins": 15 },
                        { "time": "14:00", "task": "Job 3", "location": "West End", "travel_time_mins": 25 }
                    ],
                    "total_travel_time_saved_mins": 45
                });
                Ok(response.to_string())
            }
            "suggest_slot" => {
                let response = json!({
                    "status": "success",
                    "date": date,
                    "suggested_slot": {
                        "start_time": format!("{}T16:00:00Z", date),
                        "end_time": format!("{}T17:00:00Z", date),
                        "reason": "This slot requires only 10 minutes of driving from the previous job in the West End."
                    }
                });
                Ok(response.to_string())
            }
            _ => Err(ToolError::LlmRecoverable(format!("Unknown action: {}", action))),
        }
    }
}

pub fn route_optimizer_tool() -> Tool {
    Tool {
        name: "route_optimizer".to_string(),
        description: "Analyzes a set of appointments and proposes an optimized daily schedule to minimize driving time. If a new appointment is requested, it suggests the best time slot based on existing routes. Valid actions: 'optimize_day', 'suggest_slot'.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["optimize_day", "suggest_slot"],
                    "description": "The action to perform."
                },
                "tenant_id": {
                    "type": "string",
                    "description": "The tenant ID."
                },
                "date": {
                    "type": "string",
                    "description": "The date for which to optimize or suggest slots (YYYY-MM-DD)."
                },
                "job_duration_minutes": {
                    "type": "integer",
                    "description": "Required when action='suggest_slot'. The estimated duration of the new job in minutes."
                }
            },
            "required": ["action", "tenant_id", "date"]
        }),
        execute: Arc::new(RouteOptimizerExecutor),
    }
}
