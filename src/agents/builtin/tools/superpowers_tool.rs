use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use serde::Deserialize;
use std::sync::Arc;

use super::Tool;
use super::pydantic::{PydanticAdapter, PydanticToolExecutor};

#[derive(Deserialize)]
struct SuperpowersSkillArgs {
    skill_name: String,
    #[serde(default)]
    context: String,
}

struct SuperpowersSkillExecutor {}

#[async_trait::async_trait]
impl PydanticToolExecutor<SuperpowersSkillArgs> for SuperpowersSkillExecutor {
    async fn execute_typed(&self, args: SuperpowersSkillArgs) -> Result<String, ToolError> {
        Ok(format!(
            "Superpowers skill '{}' executed with context: {}. Please follow the instructions injected in your prompt for this skill.",
            args.skill_name, args.context
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
        execute: Arc::new(PydanticAdapter::new(SuperpowersSkillExecutor {})),
    }
}
