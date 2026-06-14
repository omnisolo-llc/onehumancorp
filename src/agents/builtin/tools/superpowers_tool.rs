use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

struct SuperpowersSkillExecutor {}

#[async_trait::async_trait]
impl ToolExecutor for SuperpowersSkillExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let skill_name = args
            .get("skill_name")
            .and_then(Value::as_str)
            .ok_or_else(|| ToolError::LlmRecoverable("skill_name is required".to_string()))?;

        let context = args
            .get("context")
            .and_then(Value::as_str)
            .unwrap_or_default();

        Ok(format!(
            "Superpowers skill '{}' executed with context: {}. Please follow the instructions injected in your prompt for this skill.",
            skill_name, context
        ))
    }
}

pub fn superpowers_skill_tool() -> Tool {
    Tool {
        name: "superpowers_execute_skill".to_string(),
        description: "Executes a Superpowers skill by name (e.g. 'writing-plans', 'subagent-driven-development').".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "skill_name": {
                    "type": "string",
                    "description": "The name of the superpowers skill to execute."
                },
                "context": {
                    "type": "string",
                    "description": "Additional context for the skill execution."
                }
            },
            "required": ["skill_name"]
        }),
        execute: Arc::new(SuperpowersSkillExecutor {}),
    }
}
