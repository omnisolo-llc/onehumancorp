use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use reqwest::Client;

use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, Usage};
use super::LlmClient;

pub struct OllamaClient {
    endpoint: String,
    client: Client,
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
        }
    }
}

#[derive(Serialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
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
        };

        let resp = self
            .client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("ollama api error (status {}): {}", status, body).into());
        }

        let result: OllamaResponse = resp.json().await?;

        Ok(ChatResponse {
            message: Message {
                role: Role::Assistant,
                content: result.message.content,
                tool_calls: vec![],
                tool_results: vec![],
            },
            usage: Usage {
                input_tokens: result.prompt_eval_count,
                output_tokens: result.eval_count,
            },
            stop_reason: result.done_reason,
            response_id: "".to_string(),
        })
    }
}
