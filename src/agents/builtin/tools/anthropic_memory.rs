use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};


#[async_trait::async_trait]
pub trait MemoryAccessor: Send + Sync {
    async fn retrieve_topic(&self, topic_name: &str) -> Result<String, String>;
    async fn search_transcripts(&self, query: &str, limit: usize) -> Result<Vec<String>, String>;
    async fn write_topic(&self, topic_name: &str, content: &str) -> Result<(), String>;
}

struct TopicRetrieveExecutor {
    accessor: Arc<dyn MemoryAccessor>,
}

#[async_trait::async_trait]
impl ToolExecutor for TopicRetrieveExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let topic_name = args["topic_name"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("topic_retrieve: topic_name is required".to_string()))?;

        self.accessor
            .retrieve_topic(topic_name)
            .await
            .map_err(|e| ToolError::LlmRecoverable(e.to_string()))
    }
}

pub fn topic_retrieve_tool(accessor: Arc<dyn MemoryAccessor>) -> Tool {
    Tool {
        name: "TopicRetrieve".to_string(),
        description: "Pull a detailed memory topic file on demand based on hints from the lightweight index.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "topic_name": {
                    "type": "string",
                    "description": "The exact name of the topic to retrieve."
                }
            },
            "required": ["topic_name"]
        }),
        execute: Arc::new(TopicRetrieveExecutor { accessor }),
    }
}

struct TranscriptSearchExecutor {
    accessor: Arc<dyn MemoryAccessor>,
}

#[async_trait::async_trait]
impl ToolExecutor for TranscriptSearchExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let query = args["query"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("transcript_search: query is required".to_string()))?;

        let limit = args["limit"].as_u64().unwrap_or(5) as usize;

        let results = self.accessor
            .search_transcripts(query, limit)
            .await
            .map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        if results.is_empty() {
            Ok(format!("No transcripts found matching query: {}", query))
        } else {
            Ok(results.join("\n\n---\n\n"))
        }
    }
}

pub fn transcript_search_tool(accessor: Arc<dyn MemoryAccessor>) -> Tool {
    Tool {
        name: "TranscriptSearch".to_string(),
        description: "Search raw historical conversation transcripts across all past sessions.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The search query to match within transcripts."
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of results to return (default 5)."
                }
            },
            "required": ["query"]
        }),
        execute: Arc::new(TranscriptSearchExecutor { accessor }),
    }
}


struct TopicWriteExecutor {
    accessor: Arc<dyn MemoryAccessor>,
}

#[async_trait::async_trait]
impl ToolExecutor for TopicWriteExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let topic_name = args["topic_name"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("topic_write: topic_name is required".to_string()))?;

        let content = args["content"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("topic_write: content is required".to_string()))?;

        self.accessor
            .write_topic(topic_name, content)
            .await
            .map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        Ok(format!("Successfully wrote topic: {}", topic_name))
    }
}

pub fn topic_write_tool(accessor: Arc<dyn MemoryAccessor>) -> Tool {
    Tool {
        name: "TopicWrite".to_string(),
        description: "Write or update a detailed memory topic file.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "topic_name": {
                    "type": "string",
                    "description": "The exact name of the topic to write or update."
                },
                "content": {
                    "type": "string",
                    "description": "The detailed content of the topic."
                }
            },
            "required": ["topic_name", "content"]
        }),
        execute: Arc::new(TopicWriteExecutor { accessor }),
    }
}
