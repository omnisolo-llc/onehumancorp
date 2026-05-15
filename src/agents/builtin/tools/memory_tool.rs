use ohc_builtin_agent_core::types::LongTermMemory;
use ohc_builtin_agent_core::types::EmbeddingRecord;
use crate::tools::{Tool, ToolExecutor};
use ohc_builtin_agent_core::types::{ToolError, EmbeddingRecord, LongTermMemory};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
struct MemoryStoreArgs {
    content: String,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct MemorySearchArgs {
    query: String,
    limit: Option<usize>,
}

#[derive(Serialize, Deserialize)]
struct MemoryDeleteArgs {
    memory_id: String,
}

pub struct MemoryStoreExecutor {
    pub memory: Arc<dyn LongTermMemory>,
}

#[async_trait::async_trait]
impl ToolExecutor for MemoryStoreExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let args: MemoryStoreArgs = serde_json::from_value(args).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
        self.memory.store(&args.content, args.tags).await.map_err(|e| ToolError::Fatal(e))?;
        Ok("Memory stored successfully.".to_string())
    }
}

pub struct MemorySearchExecutor {
    pub memory: Arc<dyn LongTermMemory>,
}

#[async_trait::async_trait]
impl ToolExecutor for MemorySearchExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let args: MemorySearchArgs = serde_json::from_value(args).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
        let limit = args.limit.unwrap_or(5);
        let results = self.memory.retrieve(&args.query, limit).await.map_err(|e| ToolError::Fatal(e))?;

        if results.is_empty() {
            Ok("No relevant memories found.".to_string())
        } else {
            let mut output = String::from("Found relevant memories:\n");
            for (i, res) in results.iter().enumerate() {
                output.push_str(&format!("{}. {}\n", i + 1, res));
            }
            Ok(output)
        }
    }
}

pub fn memory_store_tool(memory: Arc<dyn LongTermMemory>) -> Tool {
    Tool {
        name: "store_memory".to_string(),
        description: "Stores an important fact or observation in long-term memory for future sessions.".to_string(),
        is_read_only: false,
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "content": { "type": "string", "description": "The fact to remember." },
                "tags": { "type": "array", "items": { "type": "string" } }
            },
            "required": ["content", "tags"]
        }),
        execute: Arc::new(MemoryStoreExecutor { memory }),
    }
}

pub fn memory_search_tool(memory: Arc<dyn LongTermMemory>) -> Tool {
    Tool {
        name: "search_memory".to_string(),
        description: "Searches long-term memory for relevant past experiences.".to_string(),
        is_read_only: true,
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "query": { "type": "string", "description": "The search query." },
                "limit": { "type": "integer" }
            },
            "required": ["query"]
        }),
        execute: Arc::new(MemorySearchExecutor { memory }),
    }
}
