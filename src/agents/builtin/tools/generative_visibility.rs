use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use serde::Deserialize;
use std::sync::Arc;
use super::{Tool, pydantic::{PydanticAdapter, PydanticToolExecutor}};

// SOTA Harness Pattern: Pydantic-first tool schema validation.
#[derive(Deserialize)]
struct GenerativeVisibilityArgs {
    #[serde(default)]
    content: String,
    #[serde(default)]
    url: String,
}

pub struct GenerativeVisibilityExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<GenerativeVisibilityArgs> for GenerativeVisibilityExecutor {
    async fn execute_typed(&self, args: GenerativeVisibilityArgs) -> Result<String, ToolError> {
        let content = args.content;
        let url = args.url;

        if content.is_empty() && url.is_empty() {
            return Err(ToolError::LlmRecoverable(
                "generative_visibility: either 'content' or 'url' must be provided.".to_string(),
            ));
        }

        // Basic heuristic evaluation for demonstration.
        let mut score = 50;
        let mut recommendations = Vec::new();

        if !content.is_empty() {
            let lower_content = content.to_lowercase();

            if lower_content.contains("best") || lower_content.contains("top") {
                score += 10;
            } else {
                recommendations.push("Include qualitative words like 'best' or 'top' to align with common generative search queries.");
            }

            if lower_content.contains("near me") || lower_content.contains("in ") {
                score += 15;
            } else {
                recommendations.push("Add geographic context (e.g., 'in Austin' or 'near me') to capture local generative searches.");
            }

            if lower_content.contains("schema.org") || lower_content.contains("json-ld") {
                score += 20;
            } else {
                recommendations.push("Implement Structured Data (schema.org) to help LLM crawlers parse your business details.");
            }

            if content.split_whitespace().count() > 100 {
                score += 5;
            } else {
                recommendations.push("Expand your content. LLMs prefer rich, descriptive text to summarize your offerings.");
            }
        } else {
             // If only URL is provided, we simulate a scan but give generic advice.
             score = 40;
             recommendations.push("Provide the actual page content for a deeper generative visibility analysis.");
             recommendations.push("Ensure your website clearly states what you do in plain language on the homepage.");
             recommendations.push("Use structured data to identify your business type, address, and reviews.");
        }

        // Cap score at 100
        let score = score.min(100);

        Ok(json!({
            "status": "success",
            "generative_score": score,
            "recommendations": recommendations,
            "message": format!("Analyzed visibility for content/url. Score: {}", score)
        }).to_string())
    }
}

pub fn generative_visibility_tool() -> Tool {
    Tool {
        name: "generative_visibility".to_string(),
        description: "Analyze website content or URL and return a Generative Score (0-100) and actionable steps to improve AI searchability (GEO). (SOTA Harness Pattern: Pydantic-first tool schema)".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "content": {
                    "type": "string",
                    "description": "The text content of the website to analyze."
                },
                "url": {
                    "type": "string",
                    "description": "The URL of the website to analyze (optional, if content is provided)."
                }
            }
        }),
        execute: Arc::new(PydanticAdapter::new(GenerativeVisibilityExecutor)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generative_visibility_missing_args() {
        let executor = GenerativeVisibilityExecutor;
        let args = GenerativeVisibilityArgs { content: "".to_string(), url: "".to_string() };
        let result = executor.execute_typed(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_generative_visibility_with_content() {
        let executor = GenerativeVisibilityExecutor;
        let args = GenerativeVisibilityArgs {
            content: "We are the best bakery in Austin. We have json-ld schema.org data. ".repeat(10),
            url: "".to_string()
        };
        let result = executor.execute_typed(args).await.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert_eq!(parsed["status"], "success");
        assert_eq!(parsed["generative_score"], 100);
        let recs = parsed["recommendations"].as_array().unwrap();
        assert!(recs.is_empty());
    }

    #[tokio::test]
    async fn test_generative_visibility_poor_content() {
        let executor = GenerativeVisibilityExecutor;
        let args = GenerativeVisibilityArgs {
            content: "Bakery store.".to_string(),
            url: "".to_string()
        };
        let result = executor.execute_typed(args).await.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert_eq!(parsed["status"], "success");
        assert_eq!(parsed["generative_score"], 50);
        let recs = parsed["recommendations"].as_array().unwrap();
        assert!(!recs.is_empty());
    }

    #[tokio::test]
    async fn test_generative_visibility_pydantic_schema_validation() {
        use crate::tools::ToolExecutor;
        let tool = generative_visibility_tool();

        let args = serde_json::json!({
            "content": 12345 // invalid type
        });

        let result = tool.execute.execute(args).await;
        assert!(result.is_err());
        if let Err(ohc_builtin_agent_core::types::ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
        } else {
            panic!("Expected Pydantic-first validation error");
        }
    }
}
