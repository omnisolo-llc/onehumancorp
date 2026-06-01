use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::LlmClient;
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, ToolCall, Usage};

use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

#[allow(dead_code)]
struct CircuitBreaker {
    failures: Mutex<usize>,
    last_failure: Mutex<Option<Instant>>,
    max_failures: usize,
    reset_timeout: Duration,
}

#[allow(dead_code)]
impl CircuitBreaker {
    fn new(max_failures: usize, reset_timeout: Duration) -> Self {
        CircuitBreaker {
            failures: Mutex::new(0),
            last_failure: Mutex::new(None),
            max_failures,
            reset_timeout,
        }
    }

    fn allow(&self) -> bool {
        let failures = self.failures.lock().unwrap();
        if *failures >= self.max_failures {
            let last_failure = self.last_failure.lock().unwrap();
            if let Some(last) = *last_failure {
                if last.elapsed() > self.reset_timeout {
                    return true;
                }
                return false;
            }
        }
        true
    }

    fn record_success(&self) {
        let mut failures = self.failures.lock().unwrap();
        *failures = 0;
    }

    fn record_failure(&self) {
        let mut failures = self.failures.lock().unwrap();
        *failures += 1;
        let mut last_failure = self.last_failure.lock().unwrap();
        *last_failure = Some(Instant::now());
    }
}

#[allow(dead_code)]
static GLOBAL_CIRCUIT_BREAKER: OnceLock<CircuitBreaker> = OnceLock::new();

#[allow(dead_code)]
fn get_circuit_breaker() -> &'static CircuitBreaker {
    GLOBAL_CIRCUIT_BREAKER.get_or_init(|| CircuitBreaker::new(3, Duration::from_secs(60)))
}

pub struct OpenAIClient {
    api_key: String,
    base_url: String,
    default_model: Option<String>,
    embedding_model: String,
    embedding_format: EmbeddingRequestFormat,
    organization: Option<String>,
    project: Option<String>,
    client: Client,
}

#[derive(Debug, Clone, Copy)]
pub enum EmbeddingRequestFormat {
    OpenAI,
    Minimax,
}

#[derive(Debug, Clone)]
pub struct OpenAIClientConfig {
    pub api_key: String,
    pub base_url: String,
    pub default_model: Option<String>,
    pub embedding_model: String,
    pub embedding_format: EmbeddingRequestFormat,
    pub organization: Option<String>,
    pub project: Option<String>,
    pub timeout: Duration,
}

impl OpenAIClientConfig {
    pub fn openai(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.openai.com/v1".to_string(),
            default_model: None,
            embedding_model: std::env::var("OHC_OPENAI_EMBEDDING_MODEL")
                .or_else(|_| std::env::var("OHC_EMBEDDING_MODEL"))
                .unwrap_or_else(|_| "text-embedding-3-small".to_string()),
            embedding_format: EmbeddingRequestFormat::OpenAI,
            organization: std::env::var("OPENAI_ORGANIZATION")
                .ok()
                .filter(|v| !v.trim().is_empty()),
            project: std::env::var("OPENAI_PROJECT")
                .ok()
                .filter(|v| !v.trim().is_empty()),
            timeout: Duration::from_secs(60),
        }
    }

    pub fn openai_compatible(
        api_key: impl Into<String>,
        base_url: impl Into<String>,
        default_model: Option<String>,
    ) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: base_url.into(),
            default_model,
            embedding_model: std::env::var("OHC_OPENAI_COMPATIBLE_EMBEDDING_MODEL")
                .or_else(|_| std::env::var("OHC_EMBEDDING_MODEL"))
                .unwrap_or_else(|_| "text-embedding-3-small".to_string()),
            embedding_format: EmbeddingRequestFormat::OpenAI,
            organization: None,
            project: None,
            timeout: Duration::from_secs(60),
        }
    }

    pub fn minimax(api_key: impl Into<String>, base_url: Option<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: base_url.unwrap_or_else(|| "https://api.minimax.chat/v1".to_string()),
            default_model: Some(
                std::env::var("MINIMAX_MODEL").unwrap_or_else(|_| "MiniMax-M2.7".to_string()),
            ),
            embedding_model: std::env::var("MINIMAX_EMBEDDING_MODEL")
                .unwrap_or_else(|_| "embo-01".to_string()),
            embedding_format: EmbeddingRequestFormat::Minimax,
            organization: None,
            project: None,
            timeout: Duration::from_secs(60),
        }
    }
}

impl OpenAIClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::from_config(OpenAIClientConfig::openai(api_key))
    }

    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self::from_config(OpenAIClientConfig::openai_compatible(
            api_key,
            base_url,
            None,
        ))
    }

    pub fn from_config(mut config: OpenAIClientConfig) -> Self {
        config.base_url = normalize_api_base_url(&config.base_url);
        Self {
            api_key: config.api_key,
            base_url: config.base_url,
            default_model: config.default_model,
            embedding_model: config.embedding_model,
            embedding_format: config.embedding_format,
            organization: config.organization,
            project: config.project,
            client: Client::builder()
                .timeout(config.timeout)
                .build()
                .unwrap(),
        }
    }

    pub fn minimax(api_key: impl Into<String>, base_url: Option<String>) -> Self {
        Self::from_config(OpenAIClientConfig::minimax(api_key, base_url))
    }

    fn chat_completions_url(&self) -> String {
        endpoint_url(&self.base_url, "chat/completions")
    }

    fn embeddings_url(&self) -> String {
        endpoint_url(&self.base_url, "embeddings")
    }

    fn request_with_auth(&self, url: &str) -> reqwest::RequestBuilder {
        let mut request = self
            .client
            .post(url)
            .header("Content-Type", "application/json");

        if !self.api_key.trim().is_empty() {
            request = request.header("Authorization", format!("Bearer {}", self.api_key));
        }

        if let Some(org) = &self.organization {
            request = request.header("OpenAI-Organization", org);
        }

        if let Some(project) = &self.project {
            request = request.header("OpenAI-Project", project);
        }

        request
    }
}

fn normalize_api_base_url(base_url: &str) -> String {
    let trimmed = base_url.trim().trim_end_matches('/').to_string();
    for suffix in ["/chat/completions", "/embeddings"] {
        if let Some(root) = trimmed.strip_suffix(suffix) {
            return root.trim_end_matches('/').to_string();
        }
    }
    trimmed
}

fn endpoint_url(base_url: &str, endpoint: &str) -> String {
    format!("{}/{}", base_url.trim_end_matches('/'), endpoint)
}

// ── Wire types ────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct OpenAIMessage {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OpenAIToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

#[derive(Serialize)]
struct OpenAIToolCall {
    id: String,
    r#type: &'static str,
    function: OpenAIFunction,
}

#[derive(Serialize)]
struct OpenAIFunction {
    name: String,
    arguments: String,
}

#[derive(Serialize)]
struct OpenAIToolDef {
    r#type: &'static str,
    function: OpenAIFunctionDef,
}

#[derive(Serialize)]
struct OpenAIFunctionDef {
    name: String,
    description: String,
    parameters: Value,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<OpenAIToolDef>,
}

#[derive(Deserialize, Debug)]
struct OpenAIResponse {
    id: Option<String>,
    choices: Vec<OpenAIChoice>,
    usage: Option<OpenAIUsage>,
}

#[derive(Deserialize, Debug)]
struct OpenAIChoice {
    message: OpenAIResponseMessage,
    finish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
struct OpenAIResponseMessage {
    #[allow(dead_code)]
    role: String,
    content: Option<String>,
    tool_calls: Option<Vec<OpenAIResponseToolCall>>,
}

#[derive(Deserialize, Debug)]
struct OpenAIResponseToolCall {
    id: String,
    function: OpenAIResponseFunction,
}

#[derive(Deserialize, Debug)]
struct OpenAIResponseFunction {
    name: String,
    arguments: String,
}

#[derive(Deserialize, Debug)]
struct OpenAIUsage {
    prompt_tokens: i32,
    completion_tokens: i32,
    #[serde(default)]
    prompt_tokens_details: Option<PromptTokensDetails>,
}

#[derive(Deserialize, Debug)]
struct PromptTokensDetails {
    #[serde(default)]
    cached_tokens: i32,
}

#[derive(Serialize)]
struct OpenAIEmbeddingRequest<'a> {
    model: &'a str,
    input: &'a str,
}

#[derive(Serialize)]
struct MinimaxEmbeddingRequest<'a> {
    model: &'a str,
    r#type: &'static str,
    texts: [&'a str; 1],
}

#[derive(Deserialize, Debug)]
pub struct MinimaxBaseResp {
    pub status_code: i32,
    pub status_msg: String,
}

#[derive(Deserialize)]
struct OpenAIEmbeddingResponse {
    #[serde(default)]
    base_resp: Option<MinimaxBaseResp>,
    #[serde(default)]
    data: Vec<OpenAIEmbeddingData>,
    #[serde(default)]
    vectors: Vec<Vec<f32>>,
}

#[derive(Deserialize)]
struct OpenAIEmbeddingData {
    embedding: Vec<f32>,
}

#[async_trait]
impl LlmClient for OpenAIClient {
    async fn chat(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let cb = get_circuit_breaker();
        if !cb.allow() {
            return Err("Circuit breaker is open: Too many consecutive LLM failures".into());
        }

        let req = super::minify_chat_request(req);
        let mut messages = Vec::new();

        if !req.system.is_empty() {
            messages.push(OpenAIMessage {
                role: "system".to_string(),
                content: Some(req.system.clone()),
                tool_calls: None,
                tool_call_id: None,
            });
        }

        for m in &req.messages {
            if m.role == Role::System {
                continue;
            }

            // Tool results
            for tr in &m.tool_results {
                let content = if !tr.error.is_empty() {
                    format!("Error: {}", tr.error)
                } else {
                    tr.content.clone()
                };
                messages.push(OpenAIMessage {
                    role: "tool".to_string(),
                    content: Some(content),
                    tool_calls: None,
                    tool_call_id: Some(tr.tool_call_id.clone()),
                });
            }

            // Assistant with tool calls
            if !m.tool_calls.is_empty() {
                let calls: Vec<OpenAIToolCall> = m
                    .tool_calls
                    .iter()
                    .map(|tc| OpenAIToolCall {
                        id: tc.id.clone(),
                        r#type: "function",
                        function: OpenAIFunction {
                            name: tc.name.clone(),
                            arguments: tc.arguments.to_string(),
                        },
                    })
                    .collect();
                messages.push(OpenAIMessage {
                    role: "assistant".to_string(),
                    content: if m.content.is_empty() {
                        None
                    } else {
                        Some(m.content.clone())
                    },
                    tool_calls: Some(calls),
                    tool_call_id: None,
                });
            } else {
                messages.push(OpenAIMessage {
                    role: m.role.to_string(),
                    content: Some(m.content.clone()),
                    tool_calls: None,
                    tool_call_id: None,
                });
            }
        }

        let tools: Vec<OpenAIToolDef> = req
            .tools
            .iter()
            .map(|t| OpenAIToolDef {
                r#type: "function",
                function: OpenAIFunctionDef {
                    name: t.name.clone(),
                    description: t.description.clone(),
                    parameters: t.parameters.clone(),
                },
            })
            .collect();

        let model = if req.model.trim().is_empty() {
            self.default_model
                .clone()
                .ok_or("missing model: set OHC_LLM_MODEL or provider-specific model env var")?
        } else {
            req.model.clone()
        };

        let payload = OpenAIRequest {
            model,
            messages,
            max_tokens: Some(req.max_tokens),
            temperature: Some(req.temperature),
            tools,
        };

        // Enable prompt caching for supported models (gpt-4o, gpt-4o-mini)
        // Note: OpenAI prompt caching is automatic but we can nudge it by including
        // 'user' role messages that are likely to be reused.
        // We also check for OHC_OPENAI_CACHE_BYPASS env var.
        let cache_bypass = std::env::var("OHC_OPENAI_CACHE_BYPASS").unwrap_or_default() == "true";
        if !cache_bypass && (req.model.contains("gpt-4o") || req.model.contains("gpt-4.1")) {
            // In some scenarios we might want to specifically structure the prompt
            // to maximize cache hits (e.g. putting static system instructions first).
            // Our build_hierarchical_system_prompt already does this.
        }

        let url = self.chat_completions_url();
        let resp = self
            .request_with_auth(&url)
            .json(&payload)
            .send()
            .await?;

        if !resp.status().is_success() {
            cb.record_failure();
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("openai api error (status {}): {}", status, body).into());
        }

        let result = resp.json::<OpenAIResponse>().await;
        if result.is_err() {
            cb.record_failure();
            return Err(format!("openai api error: failed to parse response: {:?}", result.unwrap_err()).into());
        }
        let result = result.unwrap();
        cb.record_success();

        let choice = result.choices.into_iter().next().ok_or("no choices")?;
        let finish_reason = choice.finish_reason.unwrap_or_default();

        let text = choice.message.content.unwrap_or_default();
        let tool_calls: Vec<ToolCall> = choice
            .message
            .tool_calls
            .unwrap_or_default()
            .into_iter()
            .map(|tc| {
                let arguments: Value = serde_json::from_str(&tc.function.arguments)
                    .unwrap_or(Value::Object(Default::default()));
                ToolCall {
                    id: tc.id,
                    name: tc.function.name,
                    arguments,
                }
            })
            .collect();

        let usage = result
            .usage
            .map(|u| Usage {
                input_tokens: u.prompt_tokens,
                output_tokens: u.completion_tokens,
                cache_read_input_tokens: u
                    .prompt_tokens_details
                    .as_ref()
                    .map(|d| d.cached_tokens)
                    .unwrap_or(0),
                cache_creation_input_tokens: 0,
            })
            .unwrap_or_default();

        Ok(ChatResponse {
            message: Message {
                role: Role::Assistant,
                content: text,
                tool_calls,
                tool_results: vec![],
                response_id: result.id.clone(),
                previous_response_id: None,
            },
            usage,
            stop_reason: finish_reason,
            response_id: result.id.clone(),
        })
    }

    async fn generate_embedding(
        &self,
        text: &str,
    ) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
        if text.trim().is_empty() {
            return Ok(vec![]);
        }

        let cb = get_circuit_breaker();
        if !cb.allow() {
            return Err("Circuit breaker is open: Too many consecutive LLM failures".into());
        }

        let url = self.embeddings_url();
        let resp = match self.embedding_format {
            EmbeddingRequestFormat::OpenAI => {
                let payload = OpenAIEmbeddingRequest {
                    model: &self.embedding_model,
                    input: text,
                };
                self.request_with_auth(&url).json(&payload).send().await?
            }
            EmbeddingRequestFormat::Minimax => {
                let payload = MinimaxEmbeddingRequest {
                    model: &self.embedding_model,
                    r#type: "db",
                    texts: [text],
                };
                self.request_with_auth(&url).json(&payload).send().await?
            }
        };

        if !resp.status().is_success() {
            cb.record_failure();
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(
                format!("openai-compatible embeddings error (status {}): {}", status, body)
                    .into(),
            );
        }

        let result: OpenAIEmbeddingResponse = resp.json().await?;

        // Handle Minimax base_resp wrapper which always returns HTTP 200 OK
        if let Some(base_resp) = result.base_resp {
            if base_resp.status_code != 0 && base_resp.status_code != 1000 {
                cb.record_failure();
                return Err(format!(
                    "minimax embeddings error (status {}): {}",
                    base_resp.status_code, base_resp.status_msg
                )
                .into());
            }
        }

        cb.record_success();

        if let Some(item) = result.data.into_iter().next() {
            return Ok(item.embedding);
        }

        if let Some(vector) = result.vectors.into_iter().next() {
            return Ok(vector);
        }

        Err("openai-compatible embeddings response did not include a vector".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_chat_completion_url_to_api_root() {
        assert_eq!(
            normalize_api_base_url("https://example.test/v1/chat/completions"),
            "https://example.test/v1"
        );
    }

    #[test]
    fn builds_chat_completion_endpoint_from_base_url() {
        let client = OpenAIClient::with_base_url("key", "https://example.test/v1/");
        assert_eq!(
            client.chat_completions_url(),
            "https://example.test/v1/chat/completions"
        );
    }

    #[test]
    fn minimax_uses_openai_compatible_api_root() {
        let client = OpenAIClient::minimax("key", None);
        assert_eq!(
            client.chat_completions_url(),
            "https://api.minimax.chat/v1/chat/completions"
        );
    }
}
