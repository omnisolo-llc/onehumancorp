use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

#[derive(Clone, Debug, serde::Deserialize)]
pub struct LoadedSkill {
    pub name: String,
    pub description: String,
    pub instruction: String,
    pub allowed_tools: Vec<String>,
    pub model: String,
}

impl LoadedSkill {
    pub fn tool_name(&self) -> String {
        format!("Skill_{}", sanitize_tool_suffix(&self.name))
    }
}

pub fn sanitize_tool_suffix(name: &str) -> String {
    let mut out = String::new();
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else if !out.ends_with('_') {
            out.push('_');
        }
    }
    let trimmed = out.trim_matches('_');
    if trimmed.is_empty() {
        "Unnamed".to_string()
    } else {
        trimmed.to_string()
    }
}

struct SkillExecutor {
    skill: LoadedSkill,
}

#[async_trait::async_trait]
impl ToolExecutor for SkillExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let task = args
            .get("task")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                ToolError::LlmRecoverable(format!(
                    "{}: task is required",
                    self.skill.tool_name()
                ))
            })?;
        let context = args
            .get("context")
            .map(|v| v.to_string())
            .unwrap_or_else(|| "{}".to_string());
        let allowed_tools = if self.skill.allowed_tools.is_empty() {
            "inherit parent toolset".to_string()
        } else {
            self.skill.allowed_tools.join(", ")
        };
        let model = if self.skill.model.trim().is_empty() {
            "inherit parent model".to_string()
        } else {
            self.skill.model.clone()
        };

        Ok(format!(
            "[Skill: {}]\nDescription: {}\nModel: {}\nAllowed tools: {}\n\nInstruction:\n{}\n\nTask:\n{}\n\nContext:\n{}",
            self.skill.name,
            self.skill.description,
            model,
            allowed_tools,
            self.skill.instruction,
            task,
            context
        ))
    }
}

pub fn skill_tool(skill: LoadedSkill) -> Tool {
    let tool_name = skill.tool_name();
    let description = if skill.description.trim().is_empty() {
        format!("Invoke the {} skill with a focused task and optional JSON context.", skill.name)
    } else {
        format!("Invoke the {} skill. {}", skill.name, skill.description)
    };

    Tool {
        name: tool_name,
        description,
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "task": {
                    "type": "string",
                    "description": "The focused task to run through this skill."
                },
                "context": {
                    "type": "object",
                    "description": "Optional structured context for the skill."
                }
            },
            "required": ["task"]
        }),
        execute: Arc::new(SkillExecutor { skill }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_tool_suffix() {
        assert_eq!(sanitize_tool_suffix("Code Review++"), "Code_Review");
        assert_eq!(sanitize_tool_suffix(""), "Unnamed");
    }

    #[tokio::test]
    async fn test_skill_tool_returns_instruction_payload() {
        let tool = skill_tool(LoadedSkill {
            name: "Code Review".to_string(),
            description: "Find correctness risks.".to_string(),
            instruction: "Prioritize bugs and missing tests.".to_string(),
            allowed_tools: vec!["Read".to_string(), "Grep".to_string()],
            model: "gpt-test".to_string(),
        });

        assert_eq!(tool.name, "Skill_Code_Review");
        let result = tool
            .execute
            .execute(json!({"task": "review this diff"}))
            .await
            .unwrap();
        assert!(result.contains("Prioritize bugs"));
        assert!(result.contains("Read, Grep"));
    }
}
