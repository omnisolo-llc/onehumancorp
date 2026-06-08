use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use std::path::PathBuf;

use super::{Tool, ToolExecutor};


struct CreateSkillExecutor {
    working_dir: Option<PathBuf>,
}

#[async_trait::async_trait]
impl ToolExecutor for CreateSkillExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let skill_name = args["name"].as_str().unwrap_or("UnnamedSkill");
        let description = args["description"].as_str().unwrap_or("");
        let instruction = args["instruction"].as_str().unwrap_or("");

        let content = json!({
            "name": skill_name,
            "description": description,
            "instruction": instruction,
            "allowed_tools": [], // Could be expanded later
        });

        // We persist it locally in .ohc_skills directory
        if let Some(wd) = &self.working_dir {
            let skills_dir = wd.join(".ohc_skills");
            if !skills_dir.exists() {
                if let Err(e) = std::fs::create_dir_all(&skills_dir) {
                    return Err(ToolError::Fatal(format!("Failed to create .ohc_skills dir: {}", e)));
                }
            }
            let file_path = skills_dir.join(format!("{}.json", skill_name));
            if let Err(e) = std::fs::write(&file_path, serde_json::to_string_pretty(&content).unwrap()) {
                return Err(ToolError::Fatal(format!("Failed to write skill file: {}", e)));
            }
            Ok(format!("Successfully created and saved curated skill '{}' to {}. Description: {}. Instruction: {}", skill_name, file_path.display(), description, instruction))
        } else {
            // For tests or runs without a working dir
            Ok(format!("Successfully created curated skill '{}' (but no persistent memory store or working_dir is attached). Description: {}. Instruction: {}", skill_name, description, instruction))
        }
    }
}

pub fn create_skill_tool(working_dir: Option<PathBuf>) -> Tool {
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
        execute: Arc::new(CreateSkillExecutor { working_dir }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_create_skill_executor_with_wd() {
        let dir = tempdir().unwrap();
        let wd = dir.path().to_path_buf();

        let tool = create_skill_tool(Some(wd.clone()));
        let args = json!({
            "name": "MyCoolSkill",
            "description": "It does cool things",
            "instruction": "Do cool things always",
        });

        let res = tool.execute.execute(args).await;
        assert!(res.is_ok());

        let skill_file = wd.join(".ohc_skills/MyCoolSkill.json");
        assert!(skill_file.exists());

        let file_content = std::fs::read_to_string(skill_file).unwrap();
        let json_val: serde_json::Value = serde_json::from_str(&file_content).unwrap();
        assert_eq!(json_val["name"], "MyCoolSkill");
        assert_eq!(json_val["description"], "It does cool things");
    }
}
