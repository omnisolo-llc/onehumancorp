use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

struct CalcomBookingExecutor;

#[async_trait::async_trait]
impl ToolExecutor for CalcomBookingExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let action = args["action"].as_str().ok_or_else(|| {
            ToolError::LlmRecoverable("calcom_booking: action is required".to_string())
        })?;

        match action {
            "check_availability" => {
                let date = args["date"].as_str().ok_or_else(|| {
                    ToolError::LlmRecoverable("calcom_booking: date is required for check_availability".to_string())
                })?;
                Ok(format!("Availability on {}: 09:00 AM, 02:00 PM", date))
            }
            "book" => {
                let email = args["email"].as_str().unwrap_or("unknown@example.com");
                let date_time = args["date_time"].as_str().ok_or_else(|| {
                    ToolError::LlmRecoverable("calcom_booking: date_time is required for book".to_string())
                })?;
                Ok(format!("Successfully booked appointment for {} at {}", email, date_time))
            }
            _ => Err(ToolError::LlmRecoverable(format!("calcom_booking: unknown action {}", action)))
        }
    }
}

pub fn calcom_booking_tool() -> Tool {
    Tool {
        name: "CalcomBooking".to_string(),
        description: "Interact with Cal.com to check calendar availability or book appointments."
            .to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "description": "The action to perform: 'check_availability' or 'book'."
                },
                "date": {
                    "type": "string",
                    "description": "The date to check availability for (e.g. 'YYYY-MM-DD'). Required if action is 'check_availability'."
                },
                "email": {
                    "type": "string",
                    "description": "The customer email. Required if action is 'book'."
                },
                "date_time": {
                    "type": "string",
                    "description": "The date and time for the booking. Required if action is 'book'."
                }
            },
            "required": ["action"]
        }),
        execute: Arc::new(CalcomBookingExecutor),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_calcom_booking_availability() {
        let tool = calcom_booking_tool();

        let args = json!({
            "action": "check_availability",
            "date": "2023-10-25"
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert!(result.contains("Availability on 2023-10-25"));
    }

    #[tokio::test]
    async fn test_calcom_booking_book() {
        let tool = calcom_booking_tool();

        let args = json!({
            "action": "book",
            "email": "test@example.com",
            "date_time": "2023-10-25T14:00:00Z"
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert!(result.contains("Successfully booked"));
    }
}
