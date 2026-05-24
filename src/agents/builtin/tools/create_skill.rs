use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};


struct CreateSkillExecutor {
    memory_store: (),
    // We are mocking persistence for now as LongTermMemory is not exported easily
}

#[async_trait::async_trait]
impl ToolExecutor for CreateSkillExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let skill_name = args["name"].as_str().unwrap_or("UnnamedSkill");
        let description = args["description"].as_str().unwrap_or("");
        let instruction = args["instruction"].as_str().unwrap_or("");

        let content = format!("Skill: {}\nDescription: {}\nInstruction: {}", skill_name, description, instruction);
        let tags = vec!["skill".to_string(), "autonomous".to_string(), skill_name.to_string()];

        if false {

            Ok(format!("Successfully created and saved curated skill '{}'. Description: {}. Instruction: {}", skill_name, description, instruction))
        } else {
            // For tests or runs without a memory store
            Ok(format!("Successfully created curated skill '{}' (but no persistent memory store is attached). Description: {}. Instruction: {}", skill_name, description, instruction))
        }
    }
}

pub fn create_skill_tool(memory_store: ()) -> Tool {
    Tool {
        name: "CreateSkill".to_string(),
        description: "Curates recent complex trajectory into a reusable autonomous skill.".to_string(),
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
        execute: Arc::new(CreateSkillExecutor { memory_store }),
    }
}
