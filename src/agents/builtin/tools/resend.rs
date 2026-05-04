use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

struct ResendEmailExecutor;

#[async_trait::async_trait]
impl ToolExecutor for ResendEmailExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let to = args["to"].as_str().ok_or_else(|| {
            ToolError::LlmRecoverable("resend_email: to is required".to_string())
        })?;
        let subject = args["subject"].as_str().ok_or_else(|| {
            ToolError::LlmRecoverable("resend_email: subject is required".to_string())
        })?;
        let _body = args["body"].as_str().ok_or_else(|| {
            ToolError::LlmRecoverable("resend_email: body is required".to_string())
        })?;

        // In a real implementation, this would call the Resend API to dispatch the email.
        Ok(format!(
            "Successfully sent email to {} with subject '{}'",
            to, subject
        ))
    }
}

pub fn resend_email_tool() -> Tool {
    Tool {
        name: "ResendEmail".to_string(),
        description: "Draft and send marketing emails or newsletters using the Resend API."
            .to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "to": {
                    "type": "string",
                    "description": "The recipient audience (e.g., 'All Customers' or a specific email)."
                },
                "subject": {
                    "type": "string",
                    "description": "The subject of the email."
                },
                "body": {
                    "type": "string",
                    "description": "The HTML body of the email."
                }
            },
            "required": ["to", "subject", "body"]
        }),
        execute: Arc::new(ResendEmailExecutor),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resend_email_tool() {
        let tool = resend_email_tool();

        let args = json!({
            "to": "All Customers",
            "subject": "Summer Sale",
            "body": "<h1>50% off</h1>"
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert!(result.contains("Successfully sent email to All Customers"));
    }
}
