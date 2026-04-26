use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

// ── Sleep tool ────────────────────────────────────────────────────────────────

struct SleepExecutor;

#[async_trait::async_trait]
impl ToolExecutor for SleepExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let secs = args["seconds"]
            .as_f64()
            .ok_or("sleep: seconds is required")?;
        let secs = secs.max(0.0).min(60.0); // cap at 60s
        tokio::time::sleep(std::time::Duration::from_secs_f64(secs)).await;
        Ok(format!("Slept for {}s.", secs))
    }
}

pub fn sleep_tool() -> Tool {
    Tool {
        name: "Sleep".to_string(),
        description: "Sleep for a number of seconds (max 60). \
            Use when waiting for async operations."
            .to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "seconds": {
                    "type": "number",
                    "description": "Number of seconds to sleep (max 60)."
                }
            },
            "required": ["seconds"]
        }),
        execute: Arc::new(SleepExecutor),
        requires_permission: false,
    }
}
