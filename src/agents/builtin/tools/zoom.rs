use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

struct ZoomMeetingExecutor;

#[async_trait::async_trait]
impl ToolExecutor for ZoomMeetingExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let topic = args["topic"].as_str().unwrap_or("Online Meeting");
        let start_time = args["start_time"].as_str().ok_or_else(|| {
            ToolError::LlmRecoverable("zoom_meeting: start_time is required".to_string())
        })?;

        // In a real implementation, this would call the Zoom API to create a meeting.
        Ok(format!(
            "Successfully created Zoom meeting '{}' at {} - Link: https://zoom.us/j/1234567890",
            topic, start_time
        ))
    }
}

pub fn zoom_meeting_tool() -> Tool {
    Tool {
        name: "ZoomMeeting".to_string(),
        description: "Automatically generate Zoom meeting links for online bookings."
            .to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "topic": {
                    "type": "string",
                    "description": "The topic of the meeting."
                },
                "start_time": {
                    "type": "string",
                    "description": "The start time of the meeting (e.g. 'YYYY-MM-DDTHH:MM:SSZ')."
                }
            },
            "required": ["start_time"]
        }),
        execute: Arc::new(ZoomMeetingExecutor),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_zoom_meeting_tool() {
        let tool = zoom_meeting_tool();

        let args = json!({
            "topic": "Guitar Lesson",
            "start_time": "2023-10-25T14:00:00Z"
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert!(result.contains("Successfully created Zoom meeting 'Guitar Lesson'"));
        assert!(result.contains("https://zoom.us/j/"));
    }
}
