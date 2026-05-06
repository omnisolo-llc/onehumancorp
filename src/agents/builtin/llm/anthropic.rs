use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::Client;

use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, ToolCall, Usage};
use super::LlmClient;


pub struct AnthropicClient {
    api_key: String,
    client: Client,
}

impl AnthropicClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .unwrap(),
        }
    }
}

// ── Wire types ────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct AnthropicCacheControl {
    r#type: &'static str,
}

#[derive(Serialize)]
struct AnthropicSystem {
    r#type: &'static str,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_control: Option<AnthropicCacheControl>,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: Vec<AnthropicContent>,
}

#[derive(Serialize)]
struct AnthropicContent {
    r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_use_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_control: Option<AnthropicCacheControl>,
}

#[derive(Serialize)]
struct AnthropicToolDef {
    name: String,
    description: String,
    input_schema: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_control: Option<AnthropicCacheControl>,
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: i32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    system: Vec<AnthropicSystem>,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<AnthropicToolDef>,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    id: Option<String>,
    content: Vec<AnthropicResponseContent>,
    usage: AnthropicUsage,
    stop_reason: Option<String>,
}

#[derive(Deserialize)]
struct AnthropicResponseContent {
    r#type: String,
    text: Option<String>,
    id: Option<String>,
    name: Option<String>,
    input: Option<Value>,
}

#[derive(Deserialize)]
struct AnthropicUsage {
    input_tokens: i32,
    output_tokens: i32,
    #[allow(dead_code)]
    #[serde(default)]
    cache_creation_input_tokens: i32,
    #[allow(dead_code)]
    #[serde(default)]
    cache_read_input_tokens: i32,
}

#[async_trait]
impl LlmClient for AnthropicClient {
    async fn chat(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let req = super::minify_chat_request(req);
        let mut messages: Vec<AnthropicMessage> = Vec::new();

        for m in &req.messages {
            if m.role == Role::System {
                continue;
            }
            let role = if m.role == Role::Tool {
                "user".to_string()
            } else {
                m.role.to_string()
            };

            // Build content blocks
            let mut content_blocks: Vec<AnthropicContent> = Vec::new();

            // Tool results
            for tr in &m.tool_results {
                let (text, is_error) = if !tr.error.is_empty() {
                    (format!("Error: {}", tr.error), Some(true))
                } else {
                    (tr.content.clone(), None)
                };
                content_blocks.push(AnthropicContent {
                    r#type: "tool_result".to_string(),
                    text: None,
                    id: None,
                    name: None,
                    input: None,
                    tool_use_id: Some(tr.tool_call_id.clone()),
                    content: Some(Value::String(text)),
                    is_error,
                    cache_control: None,
                });
            }

            // Tool calls (from assistant)
            for tc in &m.tool_calls {
                content_blocks.push(AnthropicContent {
                    r#type: "tool_use".to_string(),
                    text: None,
                    id: Some(tc.id.clone()),
                    name: Some(tc.name.clone()),
                    input: Some(tc.arguments.clone()),
                    tool_use_id: None,
                    content: None,
                    is_error: None,
                    cache_control: None,
                });
            }

            // Text content
            if !m.content.is_empty() {
                content_blocks.push(AnthropicContent {
                    r#type: "text".to_string(),
                    text: Some(m.content.clone()),
                    id: None,
                    name: None,
                    input: None,
                    tool_use_id: None,
                    content: None,
                    is_error: None,
                    cache_control: None,
                });
            }

            if content_blocks.is_empty() {
                content_blocks.push(AnthropicContent {
                    r#type: "text".to_string(),
                    text: Some(String::new()),
                    id: None,
                    name: None,
                    input: None,
                    tool_use_id: None,
                    content: None,
                    is_error: None,
                    cache_control: None,
                });
            }

            messages.push(AnthropicMessage {
                role,
                content: content_blocks,
            });
        }

        // Prompt caching: cache the last user message
        if let Some(last_user) = messages.iter_mut().rev().find(|m| m.role == "user") {
            if let Some(last_content) = last_user.content.last_mut() {
                last_content.cache_control = Some(AnthropicCacheControl { r#type: "ephemeral" });
            }
        }

        let system = if req.system.is_empty() {
            vec![]
        } else {
            vec![AnthropicSystem {
                r#type: "text",
                text: req.system.clone(),
                cache_control: Some(AnthropicCacheControl { r#type: "ephemeral" }),
            }]
        };

        let num_tools = req.tools.len();
        let tools: Vec<AnthropicToolDef> = req
            .tools
            .iter()
            .enumerate()
            .map(|(i, t)| AnthropicToolDef {
                name: t.name.clone(),
                description: t.description.clone(),
                input_schema: t.parameters.clone(),
                cache_control: if i == num_tools - 1 {
                    Some(AnthropicCacheControl { r#type: "ephemeral" })
                } else {
                    None
                },
            })
            .collect();

        let max_tokens = if req.max_tokens == 0 { 2048 } else { req.max_tokens };

        let payload = AnthropicRequest {
            model: req.model.clone(),
            max_tokens,
            system,
            messages,
            tools,
        };

        let resp = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .header("anthropic-beta", "prompt-caching-2024-07-31")
            .json(&payload)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("anthropic api error (status {}): {}", status, body).into());
        }

        let result: AnthropicResponse = resp.json().await?;

        // Extract content + tool calls from response
        let mut text_content = String::new();
        let mut tool_calls = Vec::new();

        for block in &result.content {
            match block.r#type.as_str() {
                "text" => {
                    if let Some(t) = &block.text {
                        text_content.push_str(t);
                    }
                }
                "tool_use" => {
                    if let (Some(id), Some(name)) = (&block.id, &block.name) {
                        tool_calls.push(ToolCall {
                            id: id.clone(),
                            name: name.clone(),
                            arguments: block.input.clone().unwrap_or(Value::Object(Default::default())),
                        });
                    }
                }
                _ => {}
            }
        }

        let stop_reason = result.stop_reason.unwrap_or_default();

        Ok(ChatResponse {
            message: Message {
                role: Role::Assistant,
                content: text_content,
                tool_calls,
                tool_results: vec![],
                response_id: result.id.clone(),
            },
            usage: Usage {
                input_tokens: result.usage.input_tokens,
                output_tokens: result.usage.output_tokens,
            },
            stop_reason,
            response_id: result.id.clone(),
        })
    }
}
