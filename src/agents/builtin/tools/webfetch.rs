use ohc_builtin_agent_core::types::ToolError;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use url::Url;

use super::{
    Tool, network_policy,
    pydantic::{PydanticAdapter, PydanticToolExecutor},
};

const MAX_REDIRECTS: usize = 5;
const MAX_RESPONSE_BYTES: usize = 1_048_576;
const MAX_DISPLAY_CHARS: usize = 10_000;

async fn validated_redirect_target(
    current_url: &Url,
    location: &str,
    allow_private: bool,
) -> Result<(Url, Vec<std::net::SocketAddr>), ToolError> {
    let target = current_url.join(location).map_err(|error| {
        ToolError::LlmRecoverable(format!("webfetch: invalid redirect target: {error}"))
    })?;
    let addresses = network_policy::validate_and_resolve(&target, allow_private).await?;
    Ok((target, addresses))
}

#[derive(Deserialize)]
struct WebFetchArgs {
    url: String,
    #[serde(default)]
    prompt: String,
}

struct WebFetchExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<WebFetchArgs> for WebFetchExecutor {
    async fn execute_typed(&self, args: WebFetchArgs) -> Result<String, ToolError> {
        let mut current_url = Url::parse(&args.url).map_err(|error| {
            ToolError::LlmRecoverable(format!("webfetch: invalid URL: {error}"))
        })?;
        let allow_private = network_policy::private_network_allowed();
        let mut redirect_count = 0;
        let mut resolved_addresses = None;

        let (body, content_type) = loop {
            let addresses = match resolved_addresses.take() {
                Some(addresses) => addresses,
                None => network_policy::validate_and_resolve(&current_url, allow_private).await?,
            };
            let client = network_policy::pin_resolved_addresses(
                Client::builder()
                    .timeout(std::time::Duration::from_secs(30))
                    .no_proxy()
                    .redirect(reqwest::redirect::Policy::none()),
                &current_url,
                &addresses,
            )
            .build()
            .map_err(|error| {
                ToolError::Unexpected(format!("webfetch: failed to build HTTP client: {error}"))
            })?;

            let mut response = client
                .get(current_url.clone())
                .header("User-Agent", "OHC-Agent/1.0")
                .send()
                .await
                .map_err(|error| {
                    ToolError::LlmRecoverable(format!("webfetch: GET {current_url}: {error}"))
                })?;

            if response.status().is_redirection() {
                if redirect_count >= MAX_REDIRECTS {
                    return Err(ToolError::LlmRecoverable(
                        "webfetch: too many redirects (maximum 5)".to_string(),
                    ));
                }
                let location = response
                    .headers()
                    .get(reqwest::header::LOCATION)
                    .ok_or_else(|| {
                        ToolError::LlmRecoverable(
                            "webfetch: redirect response is missing Location".to_string(),
                        )
                    })?
                    .to_str()
                    .map_err(|error| {
                        ToolError::LlmRecoverable(format!(
                            "webfetch: invalid redirect Location: {error}"
                        ))
                    })?;
                let (target, addresses) =
                    validated_redirect_target(&current_url, location, allow_private).await?;
                current_url = target;
                resolved_addresses = Some(addresses);
                redirect_count += 1;
                continue;
            }

            if !response.status().is_success() {
                return Err(ToolError::LlmRecoverable(format!(
                    "webfetch: HTTP {}",
                    response.status()
                )));
            }

            let content_type = response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
                .unwrap_or("")
                .to_string();
            let mut body = Vec::new();
            while let Some(chunk) = response.chunk().await.map_err(|error| {
                ToolError::LlmRecoverable(format!("webfetch: read body: {error}"))
            })? {
                if chunk.len() > MAX_RESPONSE_BYTES.saturating_sub(body.len()) {
                    return Err(ToolError::LlmRecoverable(
                        "webfetch: response exceeds 1 MiB".to_string(),
                    ));
                }
                body.extend_from_slice(&chunk);
            }
            break (body, content_type);
        };

        let body = String::from_utf8(body)
            .unwrap_or_else(|error| String::from_utf8_lossy(error.as_bytes()).into_owned());
        let text = if content_type.contains("html") {
            strip_html(&body)
        } else {
            body
        };

        let result = if let Some((index, _)) = text.char_indices().nth(MAX_DISPLAY_CHARS) {
            format!("{}... (truncated)", &text[..index])
        } else {
            text
        };

        if args.prompt.is_empty() {
            Ok(result)
        } else {
            Ok(format!("URL: {}\n\n{}", args.url, result))
        }
    }
}

fn strip_html(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                result.push(' ');
            }
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }
    // Collapse whitespace
    result.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn webfetch_tool() -> Tool {
    Tool {
        name: "WebFetch".to_string(),
        description: "Fetch the contents of a URL. Returns text content, stripping HTML tags."
            .to_string(),
        is_read_only: true,
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
        execute: Arc::new(PydanticAdapter::new(WebFetchExecutor)),
    }
}

#[cfg(test)]
#[path = "webfetch_test.rs"]
mod webfetch_test;
