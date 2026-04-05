use anyhow::{anyhow, Context};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

// ─── Public types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    Text {
        text: String,
    },
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
    ToolResult {
        tool_use_id: String,
        content: Vec<ContentPart>,
        #[serde(default)]
        is_error: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub role: String,
    pub content: Vec<ContentPart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

#[derive(Debug, Clone)]
pub struct CompletionRequest {
    pub system: String,
    pub messages: Vec<ConversationMessage>,
    pub tools: Vec<ToolDefinition>,
    pub max_tokens: u32,
}

#[derive(Debug, Clone)]
pub struct ToolUseRequest {
    pub id: String,
    pub name: String,
    pub input: Value,
}

#[derive(Debug, Clone)]
pub struct AssistantMessage {
    pub text: String,
    pub tool_uses: Vec<ToolUseRequest>,
    pub stop_reason: String,
}

#[async_trait]
pub trait LLMClient: Send + Sync {
    async fn complete(&self, req: CompletionRequest) -> anyhow::Result<AssistantMessage>;
}

// ─── Anthropic client ─────────────────────────────────────────────────────────

const DEFAULT_ANTHROPIC_MODEL: &str = "claude-sonnet-4-5";
const DEFAULT_ANTHROPIC_ENDPOINT: &str = "https://api.anthropic.com/v1/messages";

pub struct AnthropicClient {
    api_key: String,
    model: String,
    endpoint: String,
    http: reqwest::Client,
}

impl AnthropicClient {
    pub fn new(api_key: String, model: Option<String>, endpoint: Option<String>) -> Self {
        let model = model
            .or_else(|| env::var("OHC_LOCAL_AGENT_MODEL").ok())
            .unwrap_or_else(|| DEFAULT_ANTHROPIC_MODEL.to_string());
        let endpoint = endpoint
            .or_else(|| {
                env::var("ANTHROPIC_API_BASE_URL")
                    .ok()
                    .map(|b| format!("{}/v1/messages", b.trim_end_matches('/')))
            })
            .unwrap_or_else(|| DEFAULT_ANTHROPIC_ENDPOINT.to_string());
        Self {
            api_key,
            model,
            endpoint,
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .expect("http client"),
        }
    }
}

// Anthropic wire types
#[derive(Serialize)]
struct AnthropicRequest<'a> {
    model: &'a str,
    max_tokens: u32,
    system: &'a str,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<AnthropicToolDef<'a>>,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: Vec<AnthropicContent>,
}

#[derive(Serialize)]
struct AnthropicContent {
    #[serde(rename = "type")]
    kind: String,
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
}

#[derive(Serialize)]
struct AnthropicToolDef<'a> {
    name: &'a str,
    description: &'a str,
    input_schema: &'a Value,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicResponseContent>,
    stop_reason: Option<String>,
}

#[derive(Deserialize)]
struct AnthropicResponseContent {
    #[serde(rename = "type")]
    kind: String,
    text: Option<String>,
    id: Option<String>,
    name: Option<String>,
    input: Option<Value>,
}

fn content_part_to_anthropic(part: &ContentPart) -> Vec<AnthropicContent> {
    match part {
        ContentPart::Text { text } => vec![AnthropicContent {
            kind: "text".into(),
            text: Some(text.clone()),
            id: None,
            name: None,
            input: None,
            tool_use_id: None,
            content: None,
            is_error: None,
        }],
        ContentPart::ToolUse { id, name, input } => vec![AnthropicContent {
            kind: "tool_use".into(),
            text: None,
            id: Some(id.clone()),
            name: Some(name.clone()),
            input: Some(input.clone()),
            tool_use_id: None,
            content: None,
            is_error: None,
        }],
        ContentPart::ToolResult {
            tool_use_id,
            content,
            is_error,
        } => {
            let inner: Value = if content.len() == 1 {
                if let ContentPart::Text { text } = &content[0] {
                    Value::String(text.clone())
                } else {
                    serde_json::to_value(content).unwrap_or(Value::Null)
                }
            } else {
                serde_json::to_value(content).unwrap_or(Value::Null)
            };
            vec![AnthropicContent {
                kind: "tool_result".into(),
                text: None,
                id: None,
                name: None,
                input: None,
                tool_use_id: Some(tool_use_id.clone()),
                content: Some(inner),
                is_error: if *is_error { Some(true) } else { None },
            }]
        }
    }
}

#[async_trait]
impl LLMClient for AnthropicClient {
    async fn complete(&self, req: CompletionRequest) -> anyhow::Result<AssistantMessage> {
        let messages: Vec<AnthropicMessage> = req
            .messages
            .iter()
            .map(|m| AnthropicMessage {
                role: m.role.clone(),
                content: m.content.iter().flat_map(content_part_to_anthropic).collect(),
            })
            .collect();

        let tools: Vec<AnthropicToolDef> = req
            .tools
            .iter()
            .map(|t| AnthropicToolDef {
                name: &t.name,
                description: &t.description,
                input_schema: &t.input_schema,
            })
            .collect();

        let body = AnthropicRequest {
            model: &self.model,
            max_tokens: if req.max_tokens > 0 { req.max_tokens } else { 8192 },
            system: &req.system,
            messages,
            tools,
        };

        let resp = self
            .http
            .post(&self.endpoint)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .context("anthropic request")?;

        let status = resp.status();
        let text = resp.text().await.context("read body")?;
        if !status.is_success() {
            return Err(anyhow!("anthropic error {}: {}", status, text));
        }

        let ar: AnthropicResponse =
            serde_json::from_str(&text).context("parse anthropic response")?;

        let mut out_text = String::new();
        let mut tool_uses = Vec::new();
        for block in ar.content {
            match block.kind.as_str() {
                "text" => {
                    if let Some(t) = block.text {
                        out_text.push_str(&t);
                    }
                }
                "tool_use" => {
                    tool_uses.push(ToolUseRequest {
                        id: block.id.unwrap_or_default(),
                        name: block.name.unwrap_or_default(),
                        input: block.input.unwrap_or(Value::Object(Default::default())),
                    });
                }
                _ => {}
            }
        }

        Ok(AssistantMessage {
            text: out_text,
            tool_uses,
            stop_reason: ar.stop_reason.unwrap_or_else(|| "end_turn".into()),
        })
    }
}

// ─── OpenAI-compatible client ─────────────────────────────────────────────────

const DEFAULT_OPENAI_MODEL: &str = "gpt-4o";

pub struct OpenAIClient {
    api_key: String,
    model: String,
    base_url: String,
    http: reqwest::Client,
}

impl OpenAIClient {
    pub fn new(api_key: String, model: Option<String>, base_url: Option<String>) -> Self {
        let model = model.unwrap_or_else(|| DEFAULT_OPENAI_MODEL.to_string());
        let base_url =
            base_url.unwrap_or_else(|| "https://api.openai.com/v1".to_string());
        Self {
            api_key,
            model,
            base_url,
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .expect("http client"),
        }
    }
}

#[derive(Serialize)]
struct OAIRequest {
    model: String,
    messages: Vec<OAIMessage>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<OAITool>,
    max_tokens: u32,
}

#[derive(Serialize)]
struct OAIMessage {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OAIToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct OAIToolCall {
    id: String,
    #[serde(rename = "type")]
    kind: String,
    function: OAIFunction,
}

#[derive(Serialize, Deserialize, Clone)]
struct OAIFunction {
    name: String,
    arguments: String,
}

#[derive(Serialize)]
struct OAITool {
    #[serde(rename = "type")]
    kind: String,
    function: OAIFunctionDef,
}

#[derive(Serialize)]
struct OAIFunctionDef {
    name: String,
    description: String,
    parameters: Value,
}

#[derive(Deserialize)]
struct OAIResponse {
    choices: Vec<OAIChoice>,
}

#[derive(Deserialize)]
struct OAIChoice {
    message: OAIResponseMessage,
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct OAIResponseMessage {
    content: Option<String>,
    tool_calls: Option<Vec<OAIToolCall>>,
}

fn content_parts_to_oai(role: &str, parts: &[ContentPart]) -> Vec<OAIMessage> {
    let mut msgs = Vec::new();
    let mut text_parts = Vec::new();
    let mut tool_calls: Vec<OAIToolCall> = Vec::new();

    for part in parts {
        match part {
            ContentPart::Text { text } => text_parts.push(text.clone()),
            ContentPart::ToolUse { id, name, input } => {
                tool_calls.push(OAIToolCall {
                    id: id.clone(),
                    kind: "function".into(),
                    function: OAIFunction {
                        name: name.clone(),
                        arguments: serde_json::to_string(input).unwrap_or_default(),
                    },
                });
            }
            ContentPart::ToolResult {
                tool_use_id,
                content,
                ..
            } => {
                let text = content
                    .iter()
                    .filter_map(|p| {
                        if let ContentPart::Text { text } = p {
                            Some(text.as_str())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                msgs.push(OAIMessage {
                    role: "tool".into(),
                    content: Some(Value::String(text)),
                    tool_call_id: Some(tool_use_id.clone()),
                    tool_calls: None,
                    name: None,
                });
            }
        }
    }

    let combined_text = text_parts.join("\n");
    if !tool_calls.is_empty() {
        msgs.insert(
            0,
            OAIMessage {
                role: role.to_string(),
                content: if combined_text.is_empty() {
                    None
                } else {
                    Some(Value::String(combined_text))
                },
                tool_calls: Some(tool_calls),
                tool_call_id: None,
                name: None,
            },
        );
    } else if !combined_text.is_empty() || role == "user" {
        msgs.insert(
            0,
            OAIMessage {
                role: role.to_string(),
                content: Some(Value::String(combined_text)),
                tool_calls: None,
                tool_call_id: None,
                name: None,
            },
        );
    }

    msgs
}

#[async_trait]
impl LLMClient for OpenAIClient {
    async fn complete(&self, req: CompletionRequest) -> anyhow::Result<AssistantMessage> {
        let mut messages: Vec<OAIMessage> = vec![OAIMessage {
            role: "system".into(),
            content: Some(Value::String(req.system.clone())),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        }];

        for m in &req.messages {
            messages.extend(content_parts_to_oai(&m.role, &m.content));
        }

        let tools: Vec<OAITool> = req
            .tools
            .iter()
            .map(|t| OAITool {
                kind: "function".into(),
                function: OAIFunctionDef {
                    name: t.name.clone(),
                    description: t.description.clone(),
                    parameters: t.input_schema.clone(),
                },
            })
            .collect();

        let body = OAIRequest {
            model: self.model.clone(),
            messages,
            tools,
            max_tokens: if req.max_tokens > 0 { req.max_tokens } else { 8192 },
        };

        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        let resp = self
            .http
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .context("openai request")?;

        let status = resp.status();
        let text = resp.text().await.context("read body")?;
        if !status.is_success() {
            return Err(anyhow!("openai error {}: {}", status, text));
        }

        let or: OAIResponse = serde_json::from_str(&text).context("parse openai response")?;
        let choice = or.choices.into_iter().next().ok_or_else(|| anyhow!("no choices"))?;

        let out_text = choice.message.content.unwrap_or_default();
        let stop_reason = match choice.finish_reason.as_deref() {
            Some("tool_calls") => "tool_use".to_string(),
            Some(r) => r.to_string(),
            None => "end_turn".to_string(),
        };

        let tool_uses = choice
            .message
            .tool_calls
            .unwrap_or_default()
            .into_iter()
            .map(|tc| {
                let input: Value =
                    serde_json::from_str(&tc.function.arguments).unwrap_or(Value::Null);
                ToolUseRequest {
                    id: tc.id,
                    name: tc.function.name,
                    input,
                }
            })
            .collect();

        Ok(AssistantMessage {
            text: out_text,
            tool_uses,
            stop_reason,
        })
    }
}

// ─── Ollama client ────────────────────────────────────────────────────────────

const DEFAULT_OLLAMA_MODEL: &str = "llama3";

pub struct OllamaClient {
    model: String,
    endpoint: String,
    http: reqwest::Client,
}

impl OllamaClient {
    pub fn new(endpoint: String, model: Option<String>) -> Self {
        let model = model.unwrap_or_else(|| DEFAULT_OLLAMA_MODEL.to_string());
        let url = format!("{}/api/chat", endpoint.trim_end_matches('/'));
        Self {
            model,
            endpoint: url,
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .expect("http client"),
        }
    }
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
}

#[derive(Serialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OllamaResponse {
    message: OllamaResponseMessage,
    done: bool,
}

#[derive(Deserialize)]
struct OllamaResponseMessage {
    content: String,
}

#[async_trait]
impl LLMClient for OllamaClient {
    async fn complete(&self, req: CompletionRequest) -> anyhow::Result<AssistantMessage> {
        let mut messages = vec![OllamaMessage {
            role: "system".into(),
            content: req.system.clone(),
        }];
        for m in &req.messages {
            let text = m
                .content
                .iter()
                .filter_map(|p| {
                    if let ContentPart::Text { text } = p {
                        Some(text.as_str())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            messages.push(OllamaMessage {
                role: m.role.clone(),
                content: text,
            });
        }

        let body = OllamaRequest {
            model: self.model.clone(),
            messages,
            stream: false,
        };

        let resp = self
            .http
            .post(&self.endpoint)
            .json(&body)
            .send()
            .await
            .context("ollama request")?;

        let status = resp.status();
        let text = resp.text().await.context("read body")?;
        if !status.is_success() {
            return Err(anyhow!("ollama error {}: {}", status, text));
        }

        let or: OllamaResponse = serde_json::from_str(&text).context("parse ollama response")?;
        Ok(AssistantMessage {
            text: or.message.content,
            tool_uses: vec![],
            stop_reason: if or.done { "end_turn".into() } else { "max_tokens".into() },
        })
    }
}

// ─── Factory ──────────────────────────────────────────────────────────────────

pub fn default_llm_client() -> Box<dyn LLMClient + Send + Sync> {
    if let Ok(key) = env::var("ANTHROPIC_API_KEY") {
        if !key.is_empty() {
            return Box::new(AnthropicClient::new(key, None, None));
        }
    }
    if let Ok(key) = env::var("OPENAI_API_KEY") {
        if !key.is_empty() {
            let base = env::var("OPENAI_API_BASE_URL").ok();
            let model = env::var("OHC_LOCAL_AGENT_MODEL").ok();
            return Box::new(OpenAIClient::new(key, model, base));
        }
    }
    let endpoint = env::var("OHC_LOCAL_LLM_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:11434".to_string());
    let model = env::var("OHC_LOCAL_AGENT_MODEL").ok();
    Box::new(OllamaClient::new(endpoint, model))
}
