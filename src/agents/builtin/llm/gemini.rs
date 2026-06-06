use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use reqwest::Client;

use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, Usage};
use super::LlmClient;
use server_pricing::prompt_caching::PromptCache;

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
    cache: PromptCache,
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
            cache: PromptCache::new(Duration::from_secs(600)),
        }
    }
}

#[derive(Serialize)]
struct GeminiPart {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    // Add functionCall and functionResponse if supporting tools
}

#[derive(Serialize)]
struct GeminiContent {
    role: String,
    parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
struct GeminiGenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<i32>,
}

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<GeminiGenerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_instruction: Option<GeminiContent>,
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

        // 💰 Miser: Check Prompt Cache
        let cache_key = format!("{}:{:?}:{}", req.model, req.messages, req.system);
        if let (Some(cached), _) = self.cache.get_with_cost_cents(&cache_key) {
            return Ok(ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: cached.text,
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: Usage {
                    input_tokens: 0,
                    output_tokens: 0,
                    cache_read_input_tokens: cached.token_count as i32,
                    cache_creation_input_tokens: 0,
                },
                stop_reason: "stop".to_string(),
                response_id: None,
            });
        }

        let mut contents = Vec::new();

        for m in &req.messages {
            let role = match m.role {
                Role::User => "user",
                Role::Assistant => "model",
                _ => "user", // Default
            };
            contents.push(GeminiContent {
                role: role.to_string(),
                parts: vec![GeminiPart {
                    text: Some(m.content.clone()),
                }],
            });
        }

        let system_instruction = if !req.system.is_empty() {
            Some(GeminiContent {
                role: "system".to_string(),
                parts: vec![GeminiPart {
                    text: Some(req.system.clone()),
                }],
            })
        } else {
            None
        };

        let generation_config = Some(GeminiGenerationConfig {
            temperature: Some(req.temperature),
            max_output_tokens: Some(req.max_tokens),
        });

        let payload = GeminiRequest {
            contents,
            generation_config,
            system_instruction,
        };

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


        let candidate = result.candidates.into_iter().next().ok_or("no candidates")?;

        let finish_reason = candidate.finish_reason.clone().unwrap_or_default();

        let response_text = candidate
            .content
            .parts
            .iter()
            .filter_map(|p| p.text.as_ref())
            .cloned()
            .collect::<Vec<String>>()
            .join("");

        // 💰 Miser: Update Cache
        self.cache.set(&cache_key, &response_text, result.usage_metadata.as_ref().map(|u| u.prompt_token_count as usize).unwrap_or(0));

        let usage = result
            .usage_metadata
                        .map(|u| Usage {
                input_tokens: u.prompt_token_count,
                output_tokens: u.candidates_token_count,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: u.cached_content_token_count.unwrap_or(0),
            })
            .unwrap_or_default();

        Ok(ChatResponse {
            message: Message {
                role: Role::Assistant,
                content: response_text,
                tool_calls: vec![], // Tools not supported in this simple impl
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
            usage,
            stop_reason: finish_reason,
            response_id: None,
        })
    }
}
