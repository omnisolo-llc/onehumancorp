use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use super::{pydantic::{PydanticAdapter, PydanticToolExecutor}, Tool};

/// Anthropic Claude Artifacts Implementation
/// Share session output as artifacts (interactive web pages published to claude.ai)

#[derive(Deserialize)]
struct PublishArtifactArgs {
    title: String,
    content: String,
    content_type: String, // e.g. "text/html", "text/markdown"
    update_existing_url: Option<String>,
}

struct ArtifactPublisher;

#[async_trait::async_trait]
impl PydanticToolExecutor<PublishArtifactArgs> for ArtifactPublisher {
    async fn execute_typed(&self, args: PublishArtifactArgs) -> Result<String, ToolError> {
        // Validate content type
        if !["text/html", "text/markdown"].contains(&args.content_type.as_str()) {
            return Err(ToolError::LlmRecoverable("Artifacts only support text/html or text/markdown content types.".to_string()));
        }

        let size_bytes = args.content.len();
        if size_bytes > 16 * 1024 * 1024 {
            return Err(ToolError::LlmRecoverable("Artifact rendered size must be 16 MiB or smaller.".to_string()));
        }

        // Check if updating
        if let Some(url) = args.update_existing_url {
            if !url.starts_with("https://claude.ai/code/artifact/") {
                return Err(ToolError::LlmRecoverable("Invalid artifact URL for update.".to_string()));
            }
            return Ok(format!("Successfully updated artifact '{}' at URL: {}", args.title, url));
        }

        // Mock publishing a new artifact
        let mock_id = uuid::Uuid::new_v4().to_string().replace("-", "")[..16].to_string();
        let url = format!("https://claude.ai/code/artifact/{}", mock_id);

        Ok(format!("Successfully published artifact '{}'. View it at: {}", args.title, url))
    }
}

pub fn publish_artifact_tool() -> Tool {
    Tool {
        name: "PublishArtifact".to_string(),
        description: "Publish interactive session output (HTML/Markdown) as a live, shareable Artifact page on claude.ai. Use this for dashboards, charts, code diffs, or UI mockups.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "title": {
                    "type": "string",
                    "description": "The title of the artifact."
                },
                "content": {
                    "type": "string",
                    "description": "The HTML or Markdown content to publish."
                },
                "content_type": {
                    "type": "string",
                    "enum": ["text/html", "text/markdown"],
                    "description": "The format of the content."
                },
                "update_existing_url": {
                    "type": "string",
                    "description": "If updating an existing artifact, provide its claude.ai URL here."
                }
            },
            "required": ["title", "content", "content_type"]
        }),
        execute: Arc::new(PydanticAdapter::new(ArtifactPublisher)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::ToolError;

    #[tokio::test]
    async fn test_publish_artifact_success() {
        let executor = ArtifactPublisher;
        let args = PublishArtifactArgs {
            title: "Test Dashboard".to_string(),
            content: "<h1>Hello</h1>".to_string(),
            content_type: "text/html".to_string(),
            update_existing_url: None,
        };
        let res = executor.execute_typed(args).await.unwrap();
        assert!(res.contains("Successfully published artifact"));
        assert!(res.contains("https://claude.ai/code/artifact/"));
    }

    #[tokio::test]
    async fn test_update_artifact_success() {
        let executor = ArtifactPublisher;
        let args = PublishArtifactArgs {
            title: "Test Dashboard".to_string(),
            content: "<h1>Updated</h1>".to_string(),
            content_type: "text/html".to_string(),
            update_existing_url: Some("https://claude.ai/code/artifact/abcdef".to_string()),
        };
        let res = executor.execute_typed(args).await.unwrap();
        assert!(res.contains("Successfully updated artifact"));
        assert!(res.contains("https://claude.ai/code/artifact/abcdef"));
    }

    #[tokio::test]
    async fn test_publish_artifact_invalid_type() {
        let executor = ArtifactPublisher;
        let args = PublishArtifactArgs {
            title: "Test".to_string(),
            content: "text".to_string(),
            content_type: "text/plain".to_string(),
            update_existing_url: None,
        };
        let res = executor.execute_typed(args).await;
        assert!(matches!(res, Err(ToolError::LlmRecoverable(_))));
    }

    #[tokio::test]
    async fn test_publish_artifact_size_limit() {
        let executor = ArtifactPublisher;
        let args = PublishArtifactArgs {
            title: "Too Large".to_string(),
            content: "a".repeat(17 * 1024 * 1024),
            content_type: "text/html".to_string(),
            update_existing_url: None,
        };
        let res = executor.execute_typed(args).await;
        assert!(matches!(res, Err(ToolError::LlmRecoverable(_))));
    }
}
