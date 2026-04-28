use reqwest::Client;
use serde_json::{json, Value};
use std::sync::Arc;
use crate::types::ToolError;

use super::{Tool, ToolExecutor};

struct WebFetchExecutor {
    client: Client,
}

#[async_trait::async_trait]
impl ToolExecutor for WebFetchExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let url = args["url"].as_str().ok_or("webfetch: url is required")?;
        let prompt = args["prompt"].as_str().unwrap_or("");

        let resp = self
            .client
            .get(url)
            .header("User-Agent", "OHC-Agent/1.0")
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() || e.is_connect() {
                    Box::new(crate::types::ToolError::Transient(format!("webfetch timeout or connection error: {}", e))) as Box<dyn std::error::Error + Send + Sync>
                } else {
                    Box::new(crate::types::ToolError::Unexpected(format!("webfetch: GET {}: {}", url, e))) as Box<dyn std::error::Error + Send + Sync>
                }
            })?;

        if !resp.status().is_success() {
            return Err(Box::new(crate::types::ToolError::Unexpected(format!("webfetch: HTTP {}", resp.status()))) as Box<dyn std::error::Error + Send + Sync>);
        }

        let content_type = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        let body = resp
            .text()
            .await
            .map_err(|e| {
                if e.is_timeout() || e.is_connect() {
                    Box::new(crate::types::ToolError::Transient(format!("webfetch read timeout: {}", e))) as Box<dyn std::error::Error + Send + Sync>
                } else {
                    Box::new(crate::types::ToolError::Unexpected(format!("webfetch: read body: {}", e))) as Box<dyn std::error::Error + Send + Sync>
                }
            })?;

        // Strip HTML tags for HTML content.
        let text = if content_type.contains("html") {
            strip_html(&body)
        } else {
            body
        };

        // Truncate to 10K chars.
        let result = if text.len() > 10_000 {
            format!("{}... (truncated)", &text[..10_000])
        } else {
            text
        };

        if prompt.is_empty() {
            Ok(result)
        } else {
            Ok(format!("URL: {}\n\n{}", url, result))
        }
    }
}

fn strip_html(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }
    // Collapse whitespace
    result
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn webfetch_tool() -> Tool {
    Tool {
        name: "WebFetch".to_string(),
        description: "Fetch the contents of a URL. Returns text content, stripping HTML tags.".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to fetch."
                },
                "prompt": {
                    "type": "string",
                    "description": "Optional description of what to extract from the page."
                }
            },
            "required": ["url"]
        }),
        execute: Arc::new(WebFetchExecutor {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap(),
        }),
    }
}
