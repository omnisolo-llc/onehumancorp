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

pub struct GeminiClient {
    api_key: String,
    base_url: String,
    client: Client,
}

impl GeminiClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .unwrap(),
        }
    }

    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: base_url.into(),
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .unwrap(),
        }
    }

    pub(crate) fn build_payload(req: &ChatRequest) -> GeminiRequest {
        let mut contents: Vec<GeminiContent> = Vec::new();

        for m in &req.messages {
            if m.role == Role::System {
                continue;
            }
            let role = if m.role == Role::Tool {
                "user".to_string()
            } else {
                if m.role == Role::Assistant {
                    "model".to_string()
                } else {
                    m.role.to_string()
                }
            };

            let mut parts: Vec<GeminiPart> = Vec::new();

            for tr in &m.tool_results {
                let mut content_map = serde_json::Map::new();
                content_map.insert("content".to_string(), serde_json::Value::String(tr.content.clone()));
                if !tr.error.is_empty() {
                    content_map.insert("error".to_string(), serde_json::Value::String(tr.error.clone()));
                }

                parts.push(GeminiPart {
                    text: None,
                    function_call: None,
                    function_response: Some(GeminiFunctionResponse {
                        name: tr.tool_call_id.clone(),
                        response: serde_json::Value::Object(content_map),
                    }),
                });
            }

            for tc in &m.tool_calls {
                parts.push(GeminiPart {
                    text: None,
                    function_call: Some(GeminiFunctionCall {
                        name: tc.name.clone(),
                        args: tc.arguments.clone(),
                    }),
                    function_response: None,
                });
            }

            if !m.content.is_empty() {
                parts.push(GeminiPart {
                    text: Some(m.content.clone()),
                    function_call: None,
                    function_response: None,
                });
            }

            if parts.is_empty() {
                parts.push(GeminiPart {
                    text: Some(String::new()),
                    function_call: None,
                    function_response: None,
                });
            }

            contents.push(GeminiContent { role, parts });
        }

        let system_instruction = if req.system.is_empty() {
            None
        } else {
            Some(GeminiContent {
                role: "system".to_string(),
                parts: vec![GeminiPart {
                    text: Some(req.system.clone()),
                    function_call: None,
                    function_response: None,
                }],
            })
        };

        let tools = if req.tools.is_empty() {
            None
        } else {
            Some(vec![GeminiTool {
                function_declarations: req
                    .tools
                    .iter()
                    .map(|t| GeminiToolFunction {
                        name: t.name.clone(),
                        description: t.description.clone(),
                        parameters: t.parameters.clone(),
                    })
                    .collect(),
            }])
        };

        let max_tokens = if req.max_tokens == 0 {
            2048
        } else if req.max_tokens > 4096 {
            4096
        } else {
            req.max_tokens
        };

        GeminiRequest {
            contents,
            system_instruction,
            tools,
            generation_config: Some(GeminiGenerationConfig {
                temperature: None,
                max_output_tokens: Some(max_tokens),
            }),
        }
    }
}

// ── Wire types ────────────────────────────────────────────────────────────────

#[derive(Serialize, Debug, PartialEq)]
struct GeminiPart {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_call: Option<GeminiFunctionCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_response: Option<GeminiFunctionResponse>,
}

#[derive(Serialize, Debug, PartialEq)]
struct GeminiFunctionCall {
    name: String,
    args: serde_json::Value,
}

#[derive(Serialize, Debug, PartialEq)]
struct GeminiFunctionResponse {
    name: String,
    response: serde_json::Value,
}

#[derive(Serialize, Debug, PartialEq)]
struct GeminiContent {
    role: String,
    parts: Vec<GeminiPart>,
}

#[derive(Serialize, Debug, PartialEq)]
struct GeminiTool {
    function_declarations: Vec<GeminiToolFunction>,
}

#[derive(Serialize, Debug, PartialEq)]
struct GeminiToolFunction {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Serialize, Debug, PartialEq)]
struct GeminiGenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<i32>,
}

#[derive(Serialize, Debug, PartialEq)]
struct GeminiSystemInstruction {
    role: String,
    parts: Vec<GeminiPart>,
}

#[derive(Serialize, Debug, PartialEq)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<GeminiGenerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_instruction: Option<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<GeminiTool>>,
}

#[derive(Deserialize, Debug)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
    usage_metadata: Option<GeminiUsageMetadata>,
}

#[derive(Deserialize, Debug)]
struct GeminiCandidate {
    content: GeminiResponseContent,
    finish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
struct GeminiResponseContent {
    parts: Vec<GeminiResponsePart>,
}

#[derive(Deserialize, Debug)]
struct GeminiResponsePart {
    text: Option<String>,
    function_call: Option<GeminiResponseFunctionCall>,
}

#[derive(Deserialize, Debug)]
struct GeminiResponseFunctionCall {
    name: String,
    args: serde_json::Value,
}

#[derive(Deserialize, Debug)]
struct GeminiUsageMetadata {
    prompt_token_count: i32,
    candidates_token_count: i32,
    #[serde(default)]
    cached_content_token_count: Option<i32>,
}

#[async_trait]
impl LlmClient for GeminiClient {
    async fn chat(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let cb = get_circuit_breaker();
        if !cb.allow() {
            return Err("Circuit breaker is open: Too many consecutive LLM failures".into());
        }

        let req = super::minify_chat_request(req);
        let payload = Self::build_payload(&req);

        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.base_url, req.model, self.api_key
        );

        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !resp.status().is_success() {
            cb.record_failure();
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("gemini api error (status {}): {}", status, body).into());
        }

        let result = resp.json::<GeminiResponse>().await;
        if result.is_err() {
            cb.record_failure();
            return Err(format!("gemini api error: failed to parse response: {:?}", result.unwrap_err()).into());
        }
        let result = result.unwrap();
        cb.record_success();

        if result.candidates.is_empty() {
            return Err("gemini returned no candidates".into());
        }

        let candidate = result.candidates.into_iter().next().unwrap();
        let mut text_content = String::new();
        let mut tool_calls = Vec::new();

        for part in candidate.content.parts {
            if let Some(text) = part.text {
                text_content.push_str(&text);
            }
            if let Some(fc) = part.function_call {
                tool_calls.push(ToolCall {
                    id: String::from("uuid"), // Gemini doesn't provide tool_call_id natively yet
                    name: fc.name,
                    arguments: fc.args,
                });
            }
        }

        let usage = result
            .usage_metadata
            .map(|u| Usage {
                input_tokens: u.prompt_token_count,
                output_tokens: u.candidates_token_count,
                cache_read_input_tokens: u.cached_content_token_count.unwrap_or(0),
                cache_creation_input_tokens: 0,
            })
            .unwrap_or_default();

        Ok(ChatResponse {
            message: Message {
                role: Role::Assistant,
                content: text_content,
                tool_calls,
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
            usage,
            stop_reason: candidate.finish_reason.unwrap_or_default(),
            response_id: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::ChatRequest;

    #[test]
    fn test_gemini_build_payload_clamps_max_tokens_zero() {
        let mut req = ChatRequest::default();
        req.max_tokens = 0;
        let payload = GeminiClient::build_payload(&req);
        assert_eq!(payload.generation_config.unwrap().max_output_tokens, Some(2048));
    }

    #[test]
    fn test_gemini_build_payload_clamps_max_tokens_large() {
        let mut req = ChatRequest::default();
        req.max_tokens = 10000;
        let payload = GeminiClient::build_payload(&req);
        assert_eq!(payload.generation_config.unwrap().max_output_tokens, Some(4096));
    }

    #[test]
    fn test_gemini_build_payload_preserves_valid_tokens() {
        let mut req = ChatRequest::default();
        req.max_tokens = 3000;
        let payload = GeminiClient::build_payload(&req);
        assert_eq!(payload.generation_config.unwrap().max_output_tokens, Some(3000));
    }
}
