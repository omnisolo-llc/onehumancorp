use ohc_builtin_agent_core::types::ToolError;
use reqwest::Client;
use serde_json::json;
use serde::Deserialize;
use std::sync::Arc;

use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

#[derive(Deserialize)]
struct WebSearchArgs {
    query: String,
}

struct WebSearchExecutor {
    client: Client,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<WebSearchArgs> for WebSearchExecutor {
    async fn execute_typed(
        &self,
        args: WebSearchArgs,
    ) -> Result<String, ToolError> {
        let query = &args.query;

        // Use DuckDuckGo HTML endpoint (no API key required).
        let url = format!(
            "https://html.duckduckgo.com/html/?q={}",
            urlencoding::encode(query)
        );

        let resp = self
            .client
            .get(&url)
            .header("User-Agent", "OHC-Agent/1.0")
            .send()
            .await
            .map_err(|e| format!("websearch: {}", e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        if !resp.status().is_success() {
            return Ok(format!(
                "Search for '{}' returned HTTP {}",
                query,
                resp.status()
            ));
        }

        let body = resp.text().await.unwrap_or_default();

        // Extract result snippets from DuckDuckGo HTML
        let results = extract_ddg_results(&body);
        if results.is_empty() {
            return Ok(format!("No results found for: {}", query));
        }

        Ok(results.join("\n\n"))
    }
}

fn extract_ddg_results(html: &str) -> Vec<String> {
    let mut results = Vec::new();
    // Find result snippets between <a class="result__snippet"> tags
    let mut pos = 0;
    while pos < html.len() && results.len() < 5 {
        if let Some(idx) = html[pos..].find("result__snippet") {
            let start = pos + idx;
            if let Some(tag_end) = html[start..].find('>') {
                let content_start = start + tag_end + 1;
                if let Some(tag_close) = html[content_start..].find("</a>") {
                    let snippet = &html[content_start..content_start + tag_close];
                    let clean = strip_tags(snippet).trim().to_string();
                    if !clean.is_empty() {
                        results.push(clean);
                    }
                    pos = content_start + tag_close + 4;
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    results
}

fn strip_tags(s: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }
    result
}

pub fn websearch_tool() -> Tool {
    Tool {
        name: "WebSearch".to_string(),
        description: "Search the web for information. Returns a list of result snippets.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The search query."
                }
            },
            "required": ["query"]
        }),
        execute: Arc::new(PydanticAdapter::new(WebSearchExecutor {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(15))
                .build()
                .unwrap(),
        })),
    }
}
