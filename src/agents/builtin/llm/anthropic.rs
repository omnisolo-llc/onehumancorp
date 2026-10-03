use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::LlmClient;
use super::circuit_breaker::CircuitBreaker;
use omnisolo_builtin_agent_core::types::{
    ChatRequest, ChatResponse, Message, Role, ToolCall, Usage,
};
use std::time::Duration;

pub struct AnthropicClient {
    api_key: String,
    client: Client,
    circuit_breaker: CircuitBreaker,
    prompt_caching: bool,
}

impl AnthropicClient {
    /// Text-only budgeted inference has no approved cache-write tariff.
    pub fn for_text_analysis(api_key: impl Into<String>) -> Self {
        let mut client = Self::new(api_key);
        client.prompt_caching = false;
        client
    }
    fn request_payload(&self, req: ChatRequest) -> AnthropicRequest {
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
        if self.prompt_caching
            && let Some(last_user) = messages.iter_mut().rev().find(|m| m.role == "user")
            && let Some(last_content) = last_user.content.last_mut()
        {
            last_content.cache_control = Some(AnthropicCacheControl {
                r#type: "ephemeral",
            });
        }

        let system = if req.system.is_empty() {
            vec![]
        } else {
            vec![AnthropicSystem {
                r#type: "text",
                text: req.system.clone(),
                cache_control: self.prompt_caching.then_some(AnthropicCacheControl {
                    r#type: "ephemeral",
                }),
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
                cache_control: if self.prompt_caching && i == num_tools - 1 {
                    Some(AnthropicCacheControl {
                        r#type: "ephemeral",
                    })
                } else {
                    None
                },
            })
            .collect();

        AnthropicRequest {
            model: req.model.clone(),
            max_tokens: req.max_tokens,
            system,
            messages,
            tools,
        }
    }
    fn request(&self, payload: &AnthropicRequest) -> reqwest::RequestBuilder {
        let request = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(payload);
        if self.prompt_caching {
            request.header("anthropic-beta", "prompt-caching-2024-07-31")
        } else {
            request
        }
    }
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .unwrap(),
            circuit_breaker: CircuitBreaker::new(3, Duration::from_secs(60)),
            prompt_caching: true,
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

#[derive(Deserialize, Debug)]
struct AnthropicResponse {
    id: Option<String>,
    content: Vec<AnthropicResponseContent>,
    usage: AnthropicUsage,
    stop_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
struct AnthropicResponseContent {
    r#type: String,
    text: Option<String>,
    id: Option<String>,
    name: Option<String>,
    input: Option<Value>,
}

#[derive(Deserialize, Debug)]
struct AnthropicUsage {
    input_tokens: i32,
    output_tokens: i32,
    #[serde(default)]
    cache_creation_input_tokens: i32,
    #[serde(default)]
    cache_read_input_tokens: i32,
}

#[async_trait]
impl LlmClient for AnthropicClient {
    async fn chat(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.chat_prepared(super::minify_chat_request(req)).await
    }
    async fn chat_prepared(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let cb = &self.circuit_breaker;
        if !cb.allow() {
            return Err("Circuit breaker is open: Too many consecutive LLM failures".into());
        }

        let payload = self.request_payload(req);
        let resp = match self.request(&payload).send().await {
            Ok(response) => response,
            Err(error) => {
                cb.record_transport_error(&error);
                return Err(error.into());
            }
        };

        if !resp.status().is_success() {
            cb.record_http_status(resp.status());
            let status = resp.status().as_u16();
            return Err(format!("anthropic api error (status {status})").into());
        }

        let result = super::read_provider_json::<AnthropicResponse>(resp).await;
        if let Err(e) = result {
            cb.record_non_failure();
            return Err(format!("api error: failed to parse response: {:?}", e).into());
        }
        let result = result.unwrap();
        cb.record_success();

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
                            arguments: block
                                .input
                                .clone()
                                .unwrap_or(Value::Object(Default::default())),
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
                previous_response_id: None,
            },
            usage: Usage {
                input_tokens: result.usage.input_tokens,
                output_tokens: result.usage.output_tokens,
                cache_creation_input_tokens: result.usage.cache_creation_input_tokens,
                cache_read_input_tokens: result.usage.cache_read_input_tokens,
            },
            stop_reason,
            response_id: result.id.clone(),
        })
    }
}

#[cfg(test)]
mod text_transport_tests {
    use super::*;
    fn request() -> ChatRequest {
        ChatRequest {
            model: "owned-text-fixture".into(),
            system: "Only this fixed system text".into(),
            messages: vec![Message::user("Only the submitted text")],
            tools: vec![],
            max_tokens: 256,
            temperature: 0.0,
        }
    }
    #[test]
    fn tenant_text_constructor_never_adds_unpriced_prompt_cache_writes() {
        let client = AnthropicClient::for_text_analysis("public-local-fixture-key");
        let wire = client
            .request(&client.request_payload(request()))
            .build()
            .unwrap();
        assert!(!wire.headers().contains_key("anthropic-beta"));
        let bytes = wire.body().unwrap().as_bytes().unwrap();
        let body: Value = serde_json::from_slice(bytes).unwrap();
        assert!(!String::from_utf8_lossy(bytes).contains("cache_control"));
        assert_eq!(body["system"].as_array().unwrap().len(), 1);
        assert_eq!(body["system"][0]["text"], "Only this fixed system text");
        assert_eq!(body["messages"].as_array().unwrap().len(), 1);
        assert_eq!(body["messages"][0]["content"].as_array().unwrap().len(), 1);
        assert_eq!(
            body["messages"][0]["content"][0]["text"],
            "Only the submitted text"
        );
        assert!(body.get("tools").is_none());
        assert!(body.get("previous_response_id").is_none());
        assert_eq!(body["max_tokens"], 256);
    }
    #[test]
    fn general_anthropic_clients_keep_their_existing_opted_in_cache_behavior() {
        let client = AnthropicClient::new("public-local-fixture-key");
        let wire = client
            .request(&client.request_payload(request()))
            .build()
            .unwrap();
        assert!(wire.headers().contains_key("anthropic-beta"));
        let body: Value = serde_json::from_slice(wire.body().unwrap().as_bytes().unwrap()).unwrap();
        assert_eq!(body["system"][0]["cache_control"]["type"], "ephemeral");
        assert_eq!(
            body["messages"][0]["content"][0]["cache_control"]["type"],
            "ephemeral"
        );
    }
}
