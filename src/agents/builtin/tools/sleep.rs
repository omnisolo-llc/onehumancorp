use ohc_builtin_agent_core::types::ToolError;
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
    ) -> Result<String, ToolError> {
        let secs = args["seconds"]
            .as_f64()
            .ok_or_else(|| ToolError::LlmRecoverable("sleep: seconds is required".to_string()))?;
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
        is_read_only: false,
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sleep_executor_success() {
        let executor = SleepExecutor;
        let args = json!({ "seconds": 0.01 });

        let start = std::time::Instant::now();
        let result = executor.execute(args).await.unwrap();
        let elapsed = start.elapsed();

        assert_eq!(result, "Slept for 0.01s.");
        assert!(elapsed.as_secs_f64() >= 0.01);
    }

    #[tokio::test]
    async fn test_sleep_executor_missing_args() {
        let executor = SleepExecutor;
        let args = json!({});

        let result = executor.execute(args).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ToolError::LlmRecoverable(msg) => assert!(msg.contains("seconds is required")),
            _ => panic!("Expected LlmRecoverable error"),
        }
    }

    #[tokio::test]
    async fn test_sleep_executor_capped() {
        let executor = SleepExecutor;
        // The max sleep is capped at 60. We'll pass 100 but test logic will run (this is slow if it really sleeps 60s).
        // Actually, if we pass 100, it'll sleep 60s, so we'll test capping to 0.0 if negative instead.
        let args = json!({ "seconds": -5.0 });

        let start = std::time::Instant::now();
        let result = executor.execute(args).await.unwrap();
        let elapsed = start.elapsed();

        assert_eq!(result, "Slept for 0s.");
        assert!(elapsed.as_secs_f64() < 1.0);
    }
}
