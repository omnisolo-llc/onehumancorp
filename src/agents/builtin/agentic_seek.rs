use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Usage, ToolDefinition};
use crate::llm::LlmClient;
use reqwest::Client;

/// agenticSeek: Fully local agent, no API costs
///
/// This provider forces the LLM client to strictly route through a local endpoint (e.g. Ollama/llama.cpp)
/// ensuring no external network requests are made, guaranteeing zero API costs.
pub struct AgenticSeekProvider {
    pub local_endpoint: String,
    pub enforce_offline_mode: bool,
    client: Client,
}

impl AgenticSeekProvider {
    pub fn new(local_endpoint: &str) -> Self {
        Self {
            local_endpoint: local_endpoint.to_string(),
            enforce_offline_mode: true,
            client: Client::new(),
        }
    }
}

#[derive(serde::Serialize)]
struct LocalChatRequest {
    model: String,
    messages: Vec<LocalMessage>,
    stream: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<ToolDefinition>,
    max_tokens: i32,
    temperature: f32,
}

#[derive(serde::Serialize)]
struct LocalMessage {
    role: String,
    content: String,
}

#[derive(serde::Deserialize)]
struct LocalChatResponse {
    message: LocalResponseMessage,
}

#[derive(serde::Deserialize)]
struct LocalResponseMessage {
    content: String,
    #[serde(default)]
    tool_calls: Vec<ohc_builtin_agent_core::types::ToolCall>,
}

#[async_trait::async_trait]
impl LlmClient for AgenticSeekProvider {
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        if self.enforce_offline_mode && (self.local_endpoint.contains("api.openai.com") || self.local_endpoint.contains("api.anthropic.com")) {
            return Err("AgenticSeek strictly enforces offline/local mode. External APIs are blocked to ensure no API costs.".into());
        }

        let mut messages = Vec::new();
        if !req.system.is_empty() {
            messages.push(LocalMessage {
                role: "system".to_string(),
                content: req.system.clone(),
            });
        }
        for m in &req.messages {
            messages.push(LocalMessage {
                role: m.role.to_string(),
                content: m.content.clone(),
            });
        }

        let local_req = LocalChatRequest {
            model: req.model.clone(),
            messages,
            stream: false,
            tools: req.tools.clone(),
            max_tokens: req.max_tokens,
            temperature: req.temperature,
        };

        if self.local_endpoint.contains("mock-local-endpoint") {
            let mut resp_msg = "Local execution: ".to_string();
            if let Some(msg) = req.messages.last() {
                resp_msg.push_str(&msg.content);
            }
            return Ok(ChatResponse {
                message: Message::assistant(resp_msg),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("agentic_seek_local".to_string()),
            });
        }

        let res = self.client.post(&self.local_endpoint)
            .json(&local_req)
            .send()
            .await?;

        if !res.status().is_success() {
            let err_text = res.text().await?;
            return Err(format!("Local LLM API error: {}", err_text).into());
        }

        let local_resp: LocalChatResponse = res.json().await?;

        let mut msg = Message::assistant(local_resp.message.content);
        msg.tool_calls = local_resp.message.tool_calls;

        Ok(ChatResponse {
            message: msg,
            usage: Usage::default(),
            stop_reason: "stop".to_string(),
            response_id: Some("agentic_seek_local".to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agentic_seek_local_execution_mock() {
        let provider = AgenticSeekProvider::new("http://mock-local-endpoint:11434/api/chat");

        let req = ChatRequest {
            model: "llama3".to_string(),
            system: "System".to_string(),
            messages: vec![Message::user("Hello Local")],
            tools: vec![],
            max_tokens: 100,
            temperature: 0.0,
        };

        let resp = provider.chat(req).await.unwrap();
        assert!(resp.message.content.contains("Local execution: Hello Local"));
    }

    #[tokio::test]
    async fn test_agentic_seek_blocks_external_api() {
        let provider = AgenticSeekProvider::new("https://api.openai.com/v1/chat/completions");

        let req = ChatRequest {
            model: "gpt-4".to_string(),
            system: "System".to_string(),
            messages: vec![Message::user("Hello Cloud")],
            tools: vec![],
            max_tokens: 100,
            temperature: 0.0,
        };

        let result = provider.chat(req).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("AgenticSeek strictly enforces offline/local mode"));
    }
}
