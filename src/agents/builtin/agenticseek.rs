use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, Usage, ToolCall};
use crate::llm::LlmClient;
use reqwest::Client;

/// AgenticSeek Unique Harness Innovations: Fully local agent, no API costs.
/// This provider implements a production-grade local execution environment
/// that hits a local LLM inference server (like Ollama or llama.cpp)
/// using an OpenAI-compatible /v1/chat/completions endpoint.
pub struct AgenticSeekProvider {
    endpoint: String,
    client: Client,
    pub call_count: Mutex<usize>,
}

impl AgenticSeekProvider {
    pub fn new() -> Self {
        let endpoint = std::env::var("AGENTICSEEK_LOCAL_ENDPOINT")
            .unwrap_or_else(|_| "http://127.0.0.1:11434/v1/chat/completions".to_string());
        Self {
            endpoint,
            client: Client::new(),
            call_count: Mutex::new(0),
        }
    }
}

#[async_trait]
impl LlmClient for AgenticSeekProvider {
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut count = self.call_count.lock().await;
        *count += 1;

        // Force model validation: AgenticSeek must use local models
        let model_lower = req.model.to_lowercase();
        if !model_lower.contains("llama") && !model_lower.contains("mistral") && !model_lower.contains("local") {
            return Err("AgenticSeek Policy Violation: Attempted to use non-local external API model. Fully local agent requires zero API costs.".into());
        }

        // Map ChatRequest to OpenAI-compatible JSON format
        let mut messages_json = Vec::new();
        if !req.system.is_empty() {
            messages_json.push(serde_json::json!({
                "role": "system",
                "content": req.system,
            }));
        }

        for m in &req.messages {
            let role_str = match m.role {
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::System => "system",
                Role::Tool => "tool",
            };

            let mut msg_obj = serde_json::Map::new();
            msg_obj.insert("role".to_string(), serde_json::json!(role_str));

            if !m.content.is_empty() {
                msg_obj.insert("content".to_string(), serde_json::json!(m.content));
            }

            if !m.tool_calls.is_empty() {
                let tool_calls_json: Vec<serde_json::Value> = m.tool_calls.iter().map(|tc| {
                    serde_json::json!({
                        "id": tc.id,
                        "type": "function",
                        "function": {
                            "name": tc.name,
                            "arguments": tc.arguments.to_string()
                        }
                    })
                }).collect();
                msg_obj.insert("tool_calls".to_string(), serde_json::json!(tool_calls_json));
            }

            if m.role == Role::Tool {
                // For OpenAI format, tool results are individual messages per tool call.
                // Here we assume 1 tool result per message for simplicity, mapping to the first one.
                if let Some(tr) = m.tool_results.first() {
                    msg_obj.insert("tool_call_id".to_string(), serde_json::json!(tr.tool_call_id));
                    let content = if !tr.error.is_empty() {
                        format!("Error: {}", tr.error)
                    } else {
                        tr.content.clone()
                    };
                    msg_obj.insert("content".to_string(), serde_json::json!(content));
                }
            }

            messages_json.push(serde_json::Value::Object(msg_obj));
        }

        let mut body = serde_json::json!({
            "model": req.model,
            "messages": messages_json,
            "temperature": req.temperature,
            "max_tokens": req.max_tokens,
            "stream": false,
        });

        // Add tools if provided
        if !req.tools.is_empty() {
            let tools_json: Vec<serde_json::Value> = req.tools.iter().map(|t| {
                serde_json::json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters
                    }
                })
            }).collect();
            body.as_object_mut().unwrap().insert("tools".to_string(), serde_json::json!(tools_json));
            body.as_object_mut().unwrap().insert("tool_choice".to_string(), serde_json::json!("auto"));
        }

        let resp = self.client.post(&self.endpoint)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to connect to local AgenticSeek endpoint {}: {}", self.endpoint, e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err_text = resp.text().await.unwrap_or_default();
            return Err(format!("Local AgenticSeek API error ({}): {}", status, err_text).into());
        }

        let json_resp: serde_json::Value = resp.json().await?;

        let choice = json_resp["choices"].as_array()
            .and_then(|c| c.first())
            .ok_or("Invalid response format: missing choices")?;

        let message_obj = choice["message"].as_object()
            .ok_or("Invalid response format: missing message")?;

        let content = message_obj.get("content")
            .and_then(|c| c.as_str())
            .unwrap_or("")
            .to_string();

        let mut tool_calls = Vec::new();
        if let Some(tcs) = message_obj.get("tool_calls").and_then(|t| t.as_array()) {
            for tc in tcs {
                let id = tc["id"].as_str().unwrap_or("").to_string();
                if let Some(func) = tc["function"].as_object() {
                    let name = func.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string();
                    let args_str = func.get("arguments").and_then(|a| a.as_str()).unwrap_or("{}");
                    let arguments: serde_json::Value = serde_json::from_str(args_str).unwrap_or(serde_json::json!({}));
                    tool_calls.push(ToolCall { id, name, arguments });
                }
            }
        }

        let finish_reason = choice["finish_reason"].as_str().unwrap_or("stop").to_string();

        let msg = Message {
            role: Role::Assistant,
            content,
            tool_calls,
            tool_results: vec![],
            response_id: Some(format!("local-resp-{}", *count)),
            previous_response_id: None,
        };

        // Extract usage if available
        let mut usage = Usage::default();
        if let Some(u) = json_resp.get("usage") {
            usage.input_tokens = u["prompt_tokens"].as_i64().unwrap_or(0) as i32;
            usage.output_tokens = u["completion_tokens"].as_i64().unwrap_or(0) as i32;
        }

        Ok(ChatResponse {
            message: msg,
            usage,
            stop_reason: finish_reason,
            response_id: Some(format!("local-resp-{}", *count)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agenticseek_local_enforcement() {
        let provider = AgenticSeekProvider::new();

        let req_invalid = ChatRequest {
            model: "gpt-4o".to_string(),
            system: "system".to_string(),
            messages: vec![],
            tools: vec![],
            max_tokens: 100,
            temperature: 0.0,
        };

        let res = provider.chat(req_invalid).await;
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("AgenticSeek Policy Violation"));
    }

    #[tokio::test]
    async fn test_agenticseek_network_failure() {
        // Test that it attempts to connect to the local endpoint and fails gracefully if not running
        unsafe { std::env::set_var("AGENTICSEEK_LOCAL_ENDPOINT", "http://127.0.0.1:9999/v1/chat/completions"); }
        let provider = AgenticSeekProvider::new();

        let req = ChatRequest {
            model: "llama-3-8b-local".to_string(),
            system: "system".to_string(),
            messages: vec![],
            tools: vec![],
            max_tokens: 100,
            temperature: 0.0,
        };

        let res = provider.chat(req).await;
        assert!(res.is_err());
        let err_msg = res.unwrap_err().to_string();
        assert!(err_msg.contains("Failed to connect to local AgenticSeek endpoint"));
    }
}
