use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use super::{
    Tool,
    pydantic::{PydanticAdapter, PydanticToolExecutor},
};

// SOTA Harness Pattern: Pydantic-first tool schema validation.
#[derive(Deserialize)]
struct CreateSkillArgs {
    name: String,
    description: String,
    instruction: String,
}

struct CreateSkillExecutor {
    accessor: Option<Arc<dyn super::anthropic_memory::MemoryAccessor>>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<CreateSkillArgs> for CreateSkillExecutor {
    async fn execute_typed(&self, args: CreateSkillArgs) -> Result<String, ToolError> {
        let skill_name = args.name;
        let description = args.description;
        let instruction = args.instruction;

        let content = format!(
            "Skill: {}\nDescription: {}\nInstruction: {}",
            skill_name, description, instruction
        );
        let _tags = vec![
            "skill".to_string(),
            "autonomous".to_string(),
            skill_name.clone(),
        ];

        if let Some(accessor) = &self.accessor {
            if let Err(e) = accessor.write_topic(&skill_name, &content).await {
                return Err(ToolError::LlmRecoverable(format!(
                    "Failed to save curated skill to memory: {}",
                    e
                )));
            }
            Ok(format!(
                "Successfully created and saved curated skill '{}'. Description: {}. Instruction: {}",
                skill_name, description, instruction
            ))
        } else {
            // For tests or runs without a memory store
            Ok(format!(
                "Successfully created curated skill '{}' (but no persistent memory store is attached). Description: {}. Instruction: {}",
                skill_name, description, instruction
            ))
        }
    }
}

pub fn create_skill_tool(
    accessor: Option<Arc<dyn super::anthropic_memory::MemoryAccessor>>,
) -> Tool {
    Tool {
        name: "CreateSkill".to_string(),
        description: "Curates recent complex trajectory into a reusable autonomous skill."
            .to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "Name of the skill"
                },
                "description": {
                    "type": "string",
                    "description": "Short description of what the skill does"
                },
                "instruction": {
                    "type": "string",
                    "description": "The prompt/instruction for the skill based on your recent successful trajectory"
                }
            },
            "required": ["name", "description", "instruction"]
        }),
        execute: Arc::new(PydanticAdapter::new(CreateSkillExecutor { accessor })),
    }
}

#[cfg(test)]
#[path = "create_skill_test.rs"]
mod create_skill_test;
