use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use omnisolo_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, Usage};

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
                .connect_timeout(std::time::Duration::from_secs(2))
                .timeout(std::time::Duration::from_secs(300))
                .redirect(reqwest::redirect::Policy::none())
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
            return Err(format!("ollama api error (status {status})").into());
        }

        let result: OllamaResponse = match super::read_provider_json(resp).await {
            Ok(result) => result,
            Err(error) => {
                cb.record_non_failure();
                return Err(error);
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    fn request() -> ChatRequest {
        ChatRequest {
            model: "owned-fixture-model".into(),
            system: "Analyze only the supplied text".into(),
            messages: vec![Message::user("Do not claim you ran bash or echo hello")],
            tools: vec![],
            max_tokens: 123,
            temperature: 0.0,
        }
    }

    fn client(endpoint: String) -> OllamaClient {
        let mut client = OllamaClient::new(endpoint);
        client.client = Client::builder()
            .timeout(Duration::from_secs(2))
            .build()
            .unwrap();
        client
    }

    async fn response_fixture(
        status: &str,
        body: &str,
    ) -> (String, tokio::task::JoinHandle<serde_json::Value>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let response = format!(
            "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut input = Vec::new();
            let mut buffer = [0u8; 1024];
            let (header_end, content_length) = loop {
                let count = stream.read(&mut buffer).await.unwrap();
                assert_ne!(count, 0, "request ended before its headers");
                input.extend_from_slice(&buffer[..count]);
                assert!(input.len() <= 16_384, "fixture request is bounded");
                if let Some(end) = input.windows(4).position(|chunk| chunk == b"\r\n\r\n") {
                    let headers = std::str::from_utf8(&input[..end]).unwrap();
                    let length = headers
                        .lines()
                        .find_map(|line| {
                            let (name, value) = line.split_once(':')?;
                            name.eq_ignore_ascii_case("content-length")
                                .then(|| value.trim().parse::<usize>().unwrap())
                        })
                        .expect("JSON request has a content length");
                    break (end + 4, length);
                }
            };
            while input.len() < header_end + content_length {
                let count = stream.read(&mut buffer).await.unwrap();
                assert_ne!(count, 0, "request ended before its body");
                input.extend_from_slice(&buffer[..count]);
            }
            let request =
                serde_json::from_slice(&input[header_end..header_end + content_length]).unwrap();
            stream.write_all(response.as_bytes()).await.unwrap();
            stream.shutdown().await.unwrap();
            request
        });
        // Exercise the former default-local-endpoint detection while keeping
        // the real listener isolated on an ephemeral port instead of colliding
        // with a developer's actual Ollama instance on port 11434.
        (
            format!("http://{address}/api/chat?fixture=localhost:11434"),
            server,
        )
    }

    #[tokio::test]
    async fn unavailable_local_endpoint_is_an_error_not_invented_work() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        drop(listener);
        let result = client(format!("http://{address}/api/chat?fixture=localhost:11434"))
            .chat(request())
            .await;
        assert!(
            result.is_err(),
            "a refused local connection must never fabricate successful execution: {result:?}"
        );
    }

    #[tokio::test]
    async fn real_local_http_errors_are_preserved() {
        for status in [
            "401 Unauthorized",
            "429 Too Many Requests",
            "503 Service Unavailable",
        ] {
            let (endpoint, server) =
                response_fixture(status, r#"{"error":"owned fixture rejection"}"#).await;
            let result = client(endpoint).chat(request()).await;
            tokio::time::timeout(Duration::from_secs(3), server)
                .await
                .unwrap()
                .unwrap();
            assert!(
                result.is_err(),
                "HTTP {status} must not become a fabricated success: {result:?}"
            );
        }
    }

    #[tokio::test]
    async fn real_local_response_and_request_fields_are_retained() {
        let (endpoint, server) = response_fixture("200 OK", r#"{"message":{"content":"actual local fixture response"},"prompt_eval_count":17,"eval_count":23,"done_reason":"stop"}"#).await;
        let result = client(endpoint).chat(request()).await.unwrap();
        let received = tokio::time::timeout(Duration::from_secs(3), server)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(received["model"], "owned-fixture-model");
        assert_eq!(received["options"]["num_predict"], 123);
        assert_eq!(
            received["messages"][1]["content"],
            "Do not claim you ran bash or echo hello"
        );
        assert_eq!(result.message.content, "actual local fixture response");
        assert_eq!(result.usage.input_tokens, 17);
        assert_eq!(result.usage.output_tokens, 23);
    }

    #[tokio::test]
    async fn malformed_success_body_is_not_accepted() {
        let (endpoint, server) =
            response_fixture("200 OK", r#"{"unexpected":"no message receipt"}"#).await;
        let result = client(endpoint).chat(request()).await;
        tokio::time::timeout(Duration::from_secs(3), server)
            .await
            .unwrap()
            .unwrap();
        assert!(result.is_err());
    }
}
