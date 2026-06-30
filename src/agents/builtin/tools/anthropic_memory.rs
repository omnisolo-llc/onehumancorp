use ohc_builtin_agent_core::types::ToolError;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use super::{
    pydantic::{PydanticAdapter, PydanticToolExecutor},
    Tool,
};


#[async_trait::async_trait]
pub trait MemoryAccessor: Send + Sync {
    async fn retrieve_topic(&self, topic_name: &str) -> Result<String, String>;
    async fn search_transcripts(&self, query: &str, limit: usize) -> Result<Vec<String>, String>;
    async fn search_cross_session_messages(&self, query: &str, limit: usize, summarize: bool) -> Result<Vec<String>, String>;
    async fn write_topic(&self, topic_name: &str, content: &str) -> Result<(), String>;
}

// SOTA Harness Pattern: Pydantic-first tool schema validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicRetrieveArgs {
    /// The exact name of the topic to retrieve.
    pub topic_name: String,
}

struct TopicRetrieveExecutor {
    accessor: Arc<dyn MemoryAccessor>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<TopicRetrieveArgs> for TopicRetrieveExecutor {
    async fn execute_typed(&self, args: TopicRetrieveArgs) -> Result<String, ToolError> {
        self.accessor
            .retrieve_topic(&args.topic_name)
            .await
            .map_err(|e| ToolError::LlmRecoverable(e.to_string()))
    }
}

pub fn topic_retrieve_tool(accessor: Arc<dyn MemoryAccessor>) -> Tool {
    Tool {
        name: "TopicRetrieve".to_string(),
        description: "Pull a detailed memory topic file on demand based on hints from the lightweight index. (SOTA Harness Pattern: Pydantic-first tool schema)".to_string(),
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
        execute: Arc::new(PydanticAdapter::new(TopicRetrieveExecutor { accessor })),
    }
}

// SOTA Harness Pattern: Pydantic-first tool schema validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptSearchArgs {
    /// The search query to match within transcripts.
    pub query: String,

    /// Maximum number of results to return (default 5).
    pub limit: Option<usize>,
}

struct TranscriptSearchExecutor {
    accessor: Arc<dyn MemoryAccessor>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<TranscriptSearchArgs> for TranscriptSearchExecutor {
    async fn execute_typed(&self, args: TranscriptSearchArgs) -> Result<String, ToolError> {
        let limit = args.limit.unwrap_or(5);

        let results = self.accessor
            .search_transcripts(&args.query, limit)
            .await
            .map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        if results.is_empty() {
            Ok(format!("No transcripts found matching query: {}", args.query))
        } else {
            Ok(results.join("\n\n---\n\n"))
        }
    }
}

pub fn transcript_search_tool(accessor: Arc<dyn MemoryAccessor>) -> Tool {
    Tool {
        name: "TranscriptSearch".to_string(),
        description: "Search raw historical conversation transcripts across all past sessions. (SOTA Harness Pattern: Pydantic-first tool schema)".to_string(),
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
        execute: Arc::new(PydanticAdapter::new(TranscriptSearchExecutor { accessor })),
    }
}


// SOTA Harness Pattern: Pydantic-first tool schema validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicWriteArgs {
    /// The exact name of the topic to write or update.
    pub topic_name: String,

    /// The detailed content of the topic.
    pub content: String,
}

struct TopicWriteExecutor {
    accessor: Arc<dyn MemoryAccessor>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<TopicWriteArgs> for TopicWriteExecutor {
    async fn execute_typed(&self, args: TopicWriteArgs) -> Result<String, ToolError> {
        self.accessor
            .write_topic(&args.topic_name, &args.content)
            .await
            .map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        Ok(format!("Successfully wrote topic: {}", args.topic_name))
    }
}

pub fn topic_write_tool(accessor: Arc<dyn MemoryAccessor>) -> Tool {
    Tool {
        name: "TopicWrite".to_string(),
        description: "Write or update a detailed memory topic file. (SOTA Harness Pattern: Pydantic-first tool schema)".to_string(),
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
        execute: Arc::new(PydanticAdapter::new(TopicWriteExecutor { accessor })),
    }
}

// SOTA Harness Pattern: Pydantic-first tool schema validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossSessionSearchArgs {
    /// The search query to match within session messages across all sessions.
    pub query: String,

    /// Maximum number of snippets to return (default 5).
    pub limit: Option<usize>,

    /// Whether to return a synthesized LLM summary of the matching snippets (default true).
    pub summarize: Option<bool>,
}

struct CrossSessionSearchExecutor {
    accessor: Arc<dyn MemoryAccessor>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<CrossSessionSearchArgs> for CrossSessionSearchExecutor {
    async fn execute_typed(&self, args: CrossSessionSearchArgs) -> Result<String, ToolError> {
        let limit = args.limit.unwrap_or(5);
        let summarize = args.summarize.unwrap_or(true);

        let results = self.accessor
            .search_cross_session_messages(&args.query, limit, summarize)
            .await
            .map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        if results.is_empty() {
            Ok(format!("No cross-session messages found matching query: {}", args.query))
        } else {
            Ok(results.join("\n\n---\n\n"))
        }
    }
}

pub fn cross_session_search_tool(accessor: Arc<dyn MemoryAccessor>) -> Tool {
    Tool {
        name: "CrossSessionSearch".to_string(),
        description: "Searches session messages using FTS5 MATCH across ALL past sessions, returning ranked snippets, and optionally summarizing them to synthesize information. (SOTA Harness Pattern: Pydantic-first tool schema)".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The search query to match within session messages across all sessions."
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of snippets to return (default 5)."
                },
                "summarize": {
                    "type": "boolean",
                    "description": "Whether to return a synthesized LLM summary of the matching snippets (default true)."
                }
            },
            "required": ["query"]
        }),
        execute: Arc::new(PydanticAdapter::new(CrossSessionSearchExecutor { accessor })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    struct MockMemoryAccessor;

    #[async_trait::async_trait]
    impl MemoryAccessor for MockMemoryAccessor {
        async fn retrieve_topic(&self, _topic_name: &str) -> Result<String, String> {
            Ok("".to_string())
        }
        async fn search_transcripts(&self, _query: &str, _limit: usize) -> Result<Vec<String>, String> {
            Ok(vec![])
        }
        async fn search_cross_session_messages(&self, _query: &str, _limit: usize, _summarize: bool) -> Result<Vec<String>, String> {
            Ok(vec![])
        }
        async fn write_topic(&self, _topic_name: &str, _content: &str) -> Result<(), String> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_topic_retrieve_pydantic_validation() {
        let accessor = Arc::new(MockMemoryAccessor);
        let tool = topic_retrieve_tool(accessor);

        let invalid_args = json!({});
        let res = tool.execute.execute(invalid_args).await;

        assert!(res.is_err());
        let err_msg = res.unwrap_err().to_string();
        assert!(err_msg.contains("Validation Error (Pydantic-first tool schema)"));

    }

    #[tokio::test]
    async fn test_transcript_search_pydantic_validation() {
        let accessor = Arc::new(MockMemoryAccessor);
        let tool = transcript_search_tool(accessor);

        let invalid_args = json!({});
        let res = tool.execute.execute(invalid_args).await;

        assert!(res.is_err());
        let err_msg = res.unwrap_err().to_string();
        assert!(err_msg.contains("Validation Error (Pydantic-first tool schema)"));
    }

    #[tokio::test]
    async fn test_topic_write_pydantic_validation() {
        let accessor = Arc::new(MockMemoryAccessor);
        let tool = topic_write_tool(accessor);

        let invalid_args = json!({"content": "hello"});
        let res = tool.execute.execute(invalid_args).await;

        assert!(res.is_err());
        let err_msg = res.unwrap_err().to_string();
        assert!(err_msg.contains("Validation Error (Pydantic-first tool schema)"));
    }

    #[tokio::test]
    async fn test_cross_session_search_pydantic_validation() {
        let accessor = Arc::new(MockMemoryAccessor);
        let tool = cross_session_search_tool(accessor);

        let invalid_args = json!({});
        let res = tool.execute.execute(invalid_args).await;

        assert!(res.is_err());
        let err_msg = res.unwrap_err().to_string();
        assert!(err_msg.contains("Validation Error (Pydantic-first tool schema)"));

        let valid_args = json!({"query": "hello"});
        let res_valid = tool.execute.execute(valid_args).await;
        assert!(res_valid.is_ok());
    }
}
