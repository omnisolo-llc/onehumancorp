use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use ohc_builtin_agent_core::memory_traits::LongTermMemory;

use super::{Tool, ToolExecutor};

struct CreateSkillExecutor {
    memory_store: Option<Arc<dyn LongTermMemory>>,
}

#[async_trait::async_trait]
impl ToolExecutor for CreateSkillExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let skill_name = args["name"].as_str().unwrap_or("UnnamedSkill");
        let description = args["description"].as_str().unwrap_or("");
        let instruction = args["instruction"].as_str().unwrap_or("");

        let content = format!("Skill: {}\nDescription: {}\nInstruction: {}", skill_name, description, instruction);
        let tags = vec!["skill".to_string(), "autonomous".to_string(), skill_name.to_string()];

        if let Some(store) = &self.memory_store {
            store.store(&content, tags).await.map_err(|e| ToolError::Unexpected(format!("Failed to save skill to memory: {}", e)))?;
            Ok(format!("Successfully created and saved curated skill '{}'. Description: {}. Instruction: {}", skill_name, description, instruction))
        } else {
            // For tests or runs without a memory store
            Ok(format!("Successfully created curated skill '{}' (but no persistent memory store is attached). Description: {}. Instruction: {}", skill_name, description, instruction))
        }
    }
}

pub fn create_skill_tool(memory_store: Option<Arc<dyn LongTermMemory>>) -> Tool {
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::Mutex;

    #[derive(Debug)]
    struct MockMemoryStore {
        stored: Mutex<Vec<(String, Vec<String>)>>,
    }

    impl MockMemoryStore {
        fn new() -> Self {
            Self {
                stored: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait::async_trait]
    impl LongTermMemory for MockMemoryStore {
        async fn retrieve(&self, _query: &str, _limit: usize) -> Result<Vec<String>, String> {
            Ok(vec![])
        }

        async fn store(&self, content: &str, tags: Vec<String>) -> Result<(), String> {
            let mut stored = self.stored.lock().await;
            stored.push((content.to_string(), tags));
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_create_skill_without_memory() {
        let tool = create_skill_tool(None);
        let result = tool.execute.execute(json!({
            "name": "TestSkill",
            "description": "A test skill",
            "instruction": "Do something"
        })).await.unwrap();
        assert!(result.contains("but no persistent memory store is attached"));
    }

    #[tokio::test]
    async fn test_create_skill_with_memory() {
        let memory = Arc::new(MockMemoryStore::new());
        let tool = create_skill_tool(Some(memory.clone() as Arc<dyn LongTermMemory>));
        let result = tool.execute.execute(json!({
            "name": "TestSkill",
            "description": "A test skill",
            "instruction": "Do something"
        })).await.unwrap();

        assert!(result.contains("Successfully created and saved curated skill"));

        let stored = memory.stored.lock().await;
        assert_eq!(stored.len(), 1);
        let (content, tags) = &stored[0];
        assert!(content.contains("TestSkill"));
        assert!(tags.contains(&"skill".to_string()));
    }
}
