use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::LlmClient;
use super::circuit_breaker::CircuitBreaker;
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, Usage};
use std::time::Duration;

pub struct GeminiClient {
    api_key: String,
    base_url: String,
    client: Client,
    circuit_breaker: CircuitBreaker,
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
            circuit_breaker: CircuitBreaker::new(3, Duration::from_secs(60)),
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
        let cb = &self.circuit_breaker;
        if !cb.allow() {
            return Err("Circuit breaker is open: Too many consecutive LLM failures".into());
        }

        let req = super::minify_chat_request(req);
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

        let resp = match self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => {
                cb.record_transport_error(&error);
                return Err(error.into());
            }
        };

        if !resp.status().is_success() {
            cb.record_http_status(resp.status());
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("gemini api error (status {}): {}", status, body).into());
        }

        let result = resp.json::<GeminiResponse>().await;
        if let Err(e) = result {
            cb.record_non_failure();
            return Err(format!("api error: failed to parse response: {:?}", e).into());
        }
        let result = result.unwrap();

        let candidate = match result.candidates.into_iter().next() {
            Some(candidate) => candidate,
            None => {
                cb.record_non_failure();
                return Err("no candidates".into());
            }
        };
        let finish_reason = candidate.finish_reason.unwrap_or_default();

        let text = candidate
            .content
            .parts
            .into_iter()
            .filter_map(|p| p.text)
            .collect::<Vec<String>>()
            .join("");

        cb.record_success();

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
                content: text,
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
