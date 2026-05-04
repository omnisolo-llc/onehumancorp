use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

struct TwilioSmsExecutor;

#[async_trait::async_trait]
impl ToolExecutor for TwilioSmsExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let to = args["to"].as_str().ok_or_else(|| {
            ToolError::LlmRecoverable("twilio_sms: to is required".to_string())
        })?;
        let message = args["message"].as_str().ok_or_else(|| {
            ToolError::LlmRecoverable("twilio_sms: message is required".to_string())
        })?;

        // In a real implementation, this would call the Twilio API to send the SMS.
        Ok(format!(
            "Successfully sent SMS to {}: {}",
            to, message
        ))
    }
}

pub fn twilio_sms_tool() -> Tool {
    Tool {
        name: "TwilioSMS".to_string(),
        description: "Send SMS notifications or reminders to customers via Twilio."
            .to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "to": {
                    "type": "string",
                    "description": "The recipient's phone number in E.164 format."
                },
                "message": {
                    "type": "string",
                    "description": "The text message content."
                }
            },
            "required": ["to", "message"]
        }),
        execute: Arc::new(TwilioSmsExecutor),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_twilio_sms_tool() {
        let tool = twilio_sms_tool();

        let args = json!({
            "to": "+1234567890",
            "message": "Your order is ready!"
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert!(result.contains("Successfully sent SMS to +1234567890"));
    }
}
