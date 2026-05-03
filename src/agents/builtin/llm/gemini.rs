use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::Client;

use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, ToolCall, Usage};
use super::LlmClient;

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
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiFunctionCall {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiFunctionResponse {
    pub name: String,
    pub response: Value,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiPart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<GeminiFunctionCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_response: Option<GeminiFunctionResponse>,
}

#[derive(Serialize)]
pub struct GeminiContent {
    pub role: String,
    pub parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
pub struct GeminiFunctionDeclaration {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
}

#[derive(Serialize)]
pub struct GeminiTool {
    pub function_declarations: Vec<GeminiFunctionDeclaration>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiGenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiRequest {
    pub contents: Vec<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<GeminiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GeminiGenerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<GeminiContent>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiResponse {
    pub candidates: Vec<GeminiCandidate>,
    pub usage_metadata: Option<GeminiUsageMetadata>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiCandidate {
    pub content: GeminiResponseContent,
    pub finish_reason: Option<String>,
}

#[derive(Deserialize)]
pub struct GeminiResponseContent {
    pub parts: Vec<GeminiResponsePart>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiResponseFunctionCall {
    pub name: String,
    pub args: Option<Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiResponsePart {
    pub text: Option<String>,
    pub function_call: Option<GeminiResponseFunctionCall>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiUsageMetadata {
    pub prompt_token_count: i32,
    pub candidates_token_count: i32,
}

#[async_trait]
impl LlmClient for GeminiClient {
    async fn chat(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut contents = Vec::new();

        for m in &req.messages {
            let role = match m.role {
                Role::User | Role::Tool => "user",
                Role::Assistant => "model",
                Role::System => continue, // Handled separately
            };

            let mut parts = Vec::new();

            if !m.content.is_empty() {
                parts.push(GeminiPart {
                    text: Some(m.content.clone()),
                    function_call: None,
                    function_response: None,
                });
            }

            for tc in &m.tool_calls {
                parts.push(GeminiPart {
                    text: None,
                    function_call: Some(GeminiFunctionCall {
                        name: tc.name.clone(),
                        args: Some(tc.arguments.clone()),
                    }),
                    function_response: None,
                });
            }

            for tr in &m.tool_results {
                let response_val = if !tr.error.is_empty() {
                    serde_json::json!({ "error": tr.error })
                } else if let Ok(parsed) = serde_json::from_str::<Value>(&tr.content) {
                    parsed
                } else {
                    serde_json::json!({ "result": tr.content })
                };

                parts.push(GeminiPart {
                    text: None,
                    function_call: None,
                    function_response: Some(GeminiFunctionResponse {
                        name: tr.tool_call_id.clone(),
                        response: response_val,
                    }),
                });
            }

            if parts.is_empty() {
                parts.push(GeminiPart {
                    text: Some(String::new()),
                    function_call: None,
                    function_response: None,
                });
            }

            contents.push(GeminiContent {
                role: role.to_string(),
                parts,
            });
        }

        let system_instruction = if !req.system.is_empty() {
            Some(GeminiContent {
                role: "system".to_string(),
                parts: vec![GeminiPart {
                    text: Some(req.system.clone()),
                    function_call: None,
                    function_response: None,
                }],
            })
        } else {
            None
        };

        let tools = if !req.tools.is_empty() {
            let declarations = req.tools.into_iter().map(|t| {
                GeminiFunctionDeclaration {
                    name: t.name,
                    description: t.description,
                    parameters: Some(t.parameters),
                }
            }).collect();
            Some(vec![GeminiTool {
                function_declarations: declarations,
            }])
        } else {
            None
        };

        let generation_config = Some(GeminiGenerationConfig {
            temperature: Some(req.temperature),
            max_output_tokens: if req.max_tokens > 0 { Some(req.max_tokens) } else { None },
        });

        let payload = GeminiRequest {
            contents,
            tools,
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
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("gemini api error (status {}): {}", status, body).into());
        }

        let result: GeminiResponse = resp.json().await?;

        let candidate = result.candidates.into_iter().next().ok_or("no candidates")?;
        let finish_reason = candidate.finish_reason.unwrap_or_default();

        let mut text_parts = Vec::new();
        let mut tool_calls = Vec::new();

        for part in candidate.content.parts {
            if let Some(text) = part.text {
                text_parts.push(text);
            }
            if let Some(fc) = part.function_call {
                tool_calls.push(ToolCall {
                    id: fc.name.clone(), // Gemini does not have unique call IDs, so use name as ID
                    name: fc.name,
                    arguments: fc.args.unwrap_or_else(|| serde_json::Value::Object(Default::default())),
                });
            }
        }

        let text = text_parts.join("");

        let usage = result
            .usage_metadata
            .map(|u| Usage {
                input_tokens: u.prompt_token_count,
                output_tokens: u.candidates_token_count,
            })
            .unwrap_or_default();

        Ok(ChatResponse {
            message: Message {
                role: Role::Assistant,
                content: text,
                tool_calls,
                tool_results: vec![],
            },
            usage,
            stop_reason: finish_reason,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ToolDefinition, ToolResult};
    use serde_json::json;

    #[test]
    fn test_gemini_request_serialization() {
        let req = ChatRequest {
            model: "gemini-1.5-pro".to_string(),
            system: "System prompt".to_string(),
            messages: vec![
                Message {
                    role: Role::User,
                    content: "Hello".to_string(),
                    tool_calls: vec![],
                    tool_results: vec![],
                },
                Message {
                    role: Role::Assistant,
                    content: "".to_string(),
                    tool_calls: vec![ToolCall {
                        id: "get_weather".to_string(),
                        name: "get_weather".to_string(),
                        arguments: json!({"location": "London"}),
                    }],
                    tool_results: vec![],
                },
                Message {
                    role: Role::Tool,
                    content: "".to_string(),
                    tool_calls: vec![],
                    tool_results: vec![ToolResult {
                        tool_call_id: "get_weather".to_string(),
                        content: "{\"temp\": 20}".to_string(),
                        error: "".to_string(),
                    }],
                },
            ],
            tools: vec![ToolDefinition {
                name: "get_weather".to_string(),
                description: "Get weather".to_string(),
                parameters: json!({"type": "object"}),
            }],
            max_tokens: 100,
            temperature: 0.5,
        };

        let fc = GeminiFunctionCall {
            name: "test".to_string(),
            args: Some(json!({"a": 1})),
        };
        let s = serde_json::to_string(&fc).unwrap();
        assert!(s.contains("\"name\":\"test\""));
        assert!(s.contains("\"args\":{\"a\":1}"));

        let fr = GeminiFunctionResponse {
            name: "test".to_string(),
            response: json!({"res": "ok"}),
        };
        let s2 = serde_json::to_string(&fr).unwrap();
        assert!(s2.contains("\"name\":\"test\""));
        assert!(s2.contains("\"response\":{\"res\":\"ok\"}"));
    }

    #[test]
    fn test_gemini_response_deserialization() {
        let json_resp = r#"{
            "candidates": [
                {
                    "content": {
                        "parts": [
                            { "text": "Sure, I will check the weather." },
                            { "functionCall": { "name": "get_weather", "args": { "location": "Paris" } } }
                        ]
                    },
                    "finishReason": "STOP"
                }
            ],
            "usageMetadata": {
                "promptTokenCount": 10,
                "candidatesTokenCount": 20
            }
        }"#;

        let resp: GeminiResponse = serde_json::from_str(json_resp).unwrap();
        assert_eq!(resp.candidates.len(), 1);
        let parts = &resp.candidates[0].content.parts;
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].text.as_deref(), Some("Sure, I will check the weather."));
        assert!(parts[0].function_call.is_none());
        assert!(parts[1].text.is_none());
        assert_eq!(parts[1].function_call.as_ref().unwrap().name, "get_weather");
        assert_eq!(parts[1].function_call.as_ref().unwrap().args.as_ref().unwrap()["location"], "Paris");
    }
}
