use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use reqwest::Client;

use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, Usage};
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

#[derive(Serialize)]
struct GeminiFunctionCall {
    name: String,
    args: serde_json::Value,
}

#[derive(Serialize)]
struct GeminiFunctionResponse {
    name: String,
    response: serde_json::Value,
}

#[derive(Serialize)]
struct GeminiPart {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(rename = "functionCall", skip_serializing_if = "Option::is_none")]
    function_call: Option<GeminiFunctionCall>,
    #[serde(rename = "functionResponse", skip_serializing_if = "Option::is_none")]
    function_response: Option<GeminiFunctionResponse>,
}

#[derive(Serialize)]
struct GeminiContent {
    role: String,
    parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
struct GeminiFunctionDeclaration {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Serialize)]
struct GeminiTool {
    #[serde(rename = "functionDeclarations")]
    function_declarations: Vec<GeminiFunctionDeclaration>,
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
    #[serde(rename = "generationConfig", skip_serializing_if = "Option::is_none")]
    generation_config: Option<GeminiGenerationConfig>,
    #[serde(rename = "systemInstruction", skip_serializing_if = "Option::is_none")]
    system_instruction: Option<GeminiContent>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<GeminiTool>,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
    #[serde(rename = "usageMetadata")]
    usage_metadata: Option<GeminiUsageMetadata>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: GeminiResponseContent,
    #[serde(rename = "finishReason")]
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct GeminiResponseContent {
    parts: Vec<GeminiResponsePart>,
}

#[derive(Deserialize)]
struct GeminiResponseFunctionCall {
    name: String,
    args: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct GeminiResponsePart {
    text: Option<String>,
    #[serde(rename = "functionCall")]
    function_call: Option<GeminiResponseFunctionCall>,
}

#[derive(Deserialize)]
struct GeminiUsageMetadata {
    prompt_token_count: i32,
    candidates_token_count: i32,
}

#[async_trait]
impl LlmClient for GeminiClient {
    async fn chat(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut contents = Vec::new();

        for m in &req.messages {
            if m.role == Role::System {
                continue;
            }

            // Determine role:
            // "user" is used for Role::User and Role::Tool
            // "model" is used for Role::Assistant
            let role = match m.role {
                Role::User | Role::Tool => "user",
                Role::Assistant => "model",
                _ => "user",
            };

            let mut parts = Vec::new();

            // Handle tool results
            for tr in &m.tool_results {
                let response_value = if !tr.error.is_empty() {
                    serde_json::json!({ "error": tr.error })
                } else {
                    serde_json::from_str(&tr.content).unwrap_or_else(|_| serde_json::json!({ "result": tr.content }))
                };

                parts.push(GeminiPart {
                    text: None,
                    function_call: None,
                    function_response: Some(GeminiFunctionResponse {
                        name: tr.tool_call_id.clone(),
                        response: response_value,
                    }),
                });
            }

            // Handle tool calls
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

            // Handle text content
            if !m.content.is_empty() {
                parts.push(GeminiPart {
                    text: Some(m.content.clone()),
                    function_call: None,
                    function_response: None,
                });
            }

            // If empty (e.g., just tool calls/results were pushed, or nothing), ensure we don't push empty parts if possible, but parts shouldn't be empty if we have them.
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

        let mut tools = Vec::new();
        if !req.tools.is_empty() {
            let function_declarations = req
                .tools
                .iter()
                .map(|t| GeminiFunctionDeclaration {
                    name: t.name.clone(),
                    description: t.description.clone(),
                    parameters: t.parameters.clone(),
                })
                .collect();
            tools.push(GeminiTool {
                function_declarations,
            });
        }

        let generation_config = Some(GeminiGenerationConfig {
            temperature: Some(req.temperature),
            max_output_tokens: Some(req.max_tokens),
        });

        let payload = GeminiRequest {
            contents,
            generation_config,
            system_instruction,
            tools,
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

        let candidate = result.candidates.unwrap_or_default().into_iter().next().ok_or("no candidates")?;
        let finish_reason = candidate.finish_reason.unwrap_or_default();

        let mut text = String::new();
        let mut tool_calls = Vec::new();

        for part in candidate.content.parts {
            if let Some(t) = part.text {
                text.push_str(&t);
            }
            if let Some(fc) = part.function_call {
                tool_calls.push(ohc_builtin_agent_core::types::ToolCall {
                    id: fc.name.clone(), // Gemini uses name, core needs ID. Using name as ID for Gemini.
                    name: fc.name,
                    arguments: fc.args.unwrap_or(serde_json::Value::Object(Default::default())),
                });
            }
        }

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
