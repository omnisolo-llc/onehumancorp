use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::Client;

use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, ToolCall, Usage};
use super::LlmClient;

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
    client: Client,
}

impl OpenAIClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.openai.com/v1".to_string(),
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(std::env::var("OHC_LLM_TIMEOUT_SECS").ok().and_then(|v| v.parse().ok()).unwrap_or(60)))
                .build()
                .unwrap(),
        }
    }

    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: base_url.into(),
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(std::env::var("OHC_LLM_TIMEOUT_SECS").ok().and_then(|v| v.parse().ok()).unwrap_or(60)))
                .build()
                .unwrap(),
        }
    }
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
                    content: if m.content.is_empty() { None } else { Some(m.content.clone()) },
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

        let max_tokens = if req.max_tokens == 0 { None } else { Some(req.max_tokens) };

        let payload = OpenAIRequest {
            model: req.model.clone(),
            messages,
            max_tokens,
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

        let url = format!("{}/chat/completions", self.base_url);
        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
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
                let arguments: Value =
                    serde_json::from_str(&tc.function.arguments).unwrap_or(Value::Object(Default::default()));
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
                cache_read_input_tokens: u.prompt_tokens_details.as_ref().map(|d| d.cached_tokens).unwrap_or(0),
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
            },
            usage,
            stop_reason: finish_reason,
            response_id: result.id.clone(),
        })
    }
}
