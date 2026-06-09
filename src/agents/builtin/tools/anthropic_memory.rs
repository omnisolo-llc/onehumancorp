use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use serde::Deserialize;
use std::sync::Arc;

use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};


#[async_trait::async_trait]
pub trait MemoryAccessor: Send + Sync {
    async fn retrieve_topic(&self, topic_name: &str) -> Result<String, String>;
    async fn search_transcripts(&self, query: &str, limit: usize) -> Result<Vec<String>, String>;
}

// Pydantic-first tool schema validation: TopicRetrieveArgs
#[derive(Deserialize)]
struct TopicRetrieveArgs {
    topic_name: String,
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
        execute: Arc::new(PydanticAdapter::new(TopicRetrieveExecutor { accessor })),
    }
}

// Pydantic-first tool schema validation: TranscriptSearchArgs
#[derive(Deserialize)]
struct TranscriptSearchArgs {
    query: String,
    limit: Option<usize>,
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
        execute: Arc::new(PydanticAdapter::new(TranscriptSearchExecutor { accessor })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::ToolError;

    struct MockAccessor;

    #[async_trait::async_trait]
    impl MemoryAccessor for MockAccessor {
        async fn retrieve_topic(&self, topic_name: &str) -> Result<String, String> {
            if topic_name == "missing" {
                Err("Topic not found".to_string())
            } else {
                Ok(format!("Content of {}", topic_name))
            }
        }

        async fn search_transcripts(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
            if query == "error" {
                Err("Search failed".to_string())
            } else if query == "empty" {
                Ok(vec![])
            } else {
                Ok((0..limit).map(|i| format!("Result {} for {}", i, query)).collect())
            }
        }
    }

    #[tokio::test]
    async fn test_topic_retrieve_success() {
        let tool = topic_retrieve_tool(Arc::new(MockAccessor));
        let args = json!({ "topic_name": "architecture" });
        let res = tool.execute.execute(args).await.unwrap();
        assert_eq!(res, "Content of architecture");
    }

    #[tokio::test]
    async fn test_topic_retrieve_accessor_error() {
        let tool = topic_retrieve_tool(Arc::new(MockAccessor));
        let args = json!({ "topic_name": "missing" });
        let res = tool.execute.execute(args).await;
        assert!(res.is_err());
        match res.unwrap_err() {
            ToolError::LlmRecoverable(msg) => assert_eq!(msg, "Topic not found"),
            _ => panic!("Expected LlmRecoverable error"),
        }
    }

    #[tokio::test]
    async fn test_topic_retrieve_pydantic_error() {
        let tool = topic_retrieve_tool(Arc::new(MockAccessor));
        // Missing required field "topic_name"
        let args = json!({});
        let res = tool.execute.execute(args).await;
        assert!(res.is_err());
        match res.unwrap_err() {
            ToolError::LlmRecoverable(msg) => {
                assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
                assert!(msg.contains("missing field `topic_name`"));
            }
            _ => panic!("Expected LlmRecoverable Pydantic error"),
        }
    }

    #[tokio::test]
    async fn test_transcript_search_success() {
        let tool = transcript_search_tool(Arc::new(MockAccessor));
        let args = json!({ "query": "auth", "limit": 2 });
        let res = tool.execute.execute(args).await.unwrap();
        assert_eq!(res, "Result 0 for auth\n\n---\n\nResult 1 for auth");
    }

    #[tokio::test]
    async fn test_transcript_search_empty() {
        let tool = transcript_search_tool(Arc::new(MockAccessor));
        let args = json!({ "query": "empty" });
        let res = tool.execute.execute(args).await.unwrap();
        assert_eq!(res, "No transcripts found matching query: empty");
    }

    #[tokio::test]
    async fn test_transcript_search_accessor_error() {
        let tool = transcript_search_tool(Arc::new(MockAccessor));
        let args = json!({ "query": "error" });
        let res = tool.execute.execute(args).await;
        assert!(res.is_err());
        match res.unwrap_err() {
            ToolError::LlmRecoverable(msg) => assert_eq!(msg, "Search failed"),
            _ => panic!("Expected LlmRecoverable error"),
        }
    }

    #[tokio::test]
    async fn test_transcript_search_pydantic_error() {
        let tool = transcript_search_tool(Arc::new(MockAccessor));
        // Missing required field "query"
        let args = json!({});
        let res = tool.execute.execute(args).await;
        assert!(res.is_err());
        match res.unwrap_err() {
            ToolError::LlmRecoverable(msg) => {
                assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
                assert!(msg.contains("missing field `query`"));
            }
            _ => panic!("Expected LlmRecoverable Pydantic error"),
        }
    }
}
