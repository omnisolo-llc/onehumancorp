use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, Usage};

use super::LlmClient;
use super::circuit_breaker::CircuitBreaker;
use std::time::Duration;

pub struct OllamaClient {
    endpoint: String,
    client: Client,
    circuit_breaker: CircuitBreaker,
}

impl OllamaClient {
    pub fn new(endpoint: impl Into<String>) -> Self {
        let endpoint = endpoint.into();
        let endpoint = if endpoint.is_empty() {
            "http://localhost:11434/api/chat".to_string()
        } else {
            endpoint
        };
        Self {
            endpoint,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(300))
                .build()
                .unwrap(),
            circuit_breaker: CircuitBreaker::new(3, Duration::from_secs(60)),
        }
    }
}

#[derive(Serialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OllamaOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<i32>,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaOptions>,
}

#[derive(Deserialize)]
struct OllamaResponse {
    message: OllamaResponseMessage,
    #[serde(default)]
    prompt_eval_count: i32,
    #[serde(default)]
    eval_count: i32,
    #[serde(default)]
    done_reason: String,
}

#[derive(Deserialize)]
struct OllamaResponseMessage {
    content: String,
}

#[async_trait]
impl LlmClient for OllamaClient {
    async fn chat(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let cb = &self.circuit_breaker;
        if !cb.allow() {
            return Err("Circuit breaker is open: Too many consecutive LLM failures".into());
        }

        let req = super::minify_chat_request(req);
        let mut messages = Vec::new();

        if !req.system.is_empty() {
            messages.push(OllamaMessage {
                role: "system".to_string(),
                content: req.system.clone(),
            });
        }

        for m in &req.messages {
            if m.role == Role::System {
                continue;
            }
            messages.push(OllamaMessage {
                role: m.role.to_string(),
                content: m.content.clone(),
            });
        }

        let payload = OllamaRequest {
            model: req.model.clone(),
            messages,
            stream: false,
            options: Some(OllamaOptions {
                num_predict: Some(req.max_tokens),
            }),
        };

        let resp_result = self
            .client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await;

        let resp = match resp_result {
            Ok(r) => r,
            Err(e) => {
                cb.record_transport_error(&e);
                return Err(e.into());
            }
        };

        if !resp.status().is_success() {
            cb.record_http_status(resp.status());
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("ollama api error (status {}): {}", status, body).into());
        }

        let result: OllamaResponse = match resp.json().await {
            Ok(result) => result,
            Err(error) => {
                cb.record_non_failure();
                return Err(error.into());
            }
        };
        cb.record_success();

        Ok(ChatResponse {
            message: Message {
                role: Role::Assistant,
                content: result.message.content,
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
            usage: Usage {
                input_tokens: result.prompt_eval_count,
                output_tokens: result.eval_count,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
            },
            stop_reason: result.done_reason,
            response_id: None,
        })
    }
}
