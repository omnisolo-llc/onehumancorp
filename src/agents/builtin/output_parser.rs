use crate::types::{ChatRequest, Message, ToolError, ChatResponse};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub trait LlmClientForParser: Send + Sync {
    async fn chat(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>>;
}

/// Implements the Output Parsing mechanic from the Master Catalog:
/// "Fallback mechanic: Legacy RetryWithErrorOutputParser (feed the original prompt,
/// the failed completion, and the parsing error back to the model)."
pub async fn parse_structured_output<T: DeserializeOwned>(
    llm: &Arc<dyn LlmClientForParser>,
    req: ChatRequest,
    schema: serde_json::Value,
    max_retries: usize,
) -> Result<T, ToolError> {
    let mut current_req = req.clone();

    current_req.tools.push(crate::types::ToolDefinition {
        name: "return_structured_output".to_string(),
        description: "You MUST use this tool to return the final structured output.".to_string(),
        parameters: schema,
    });
    current_req.system = format!("{}\n\nYou MUST use the 'return_structured_output' tool to provide your final answer.", current_req.system);

    let mut attempt = 0;

    loop {
        attempt += 1;
        let resp = match llm.chat(current_req.clone()).await {
            Ok(r) => r,
            Err(e) => return Err(ToolError::Transient(format!("LLM Error: {}", e))),
        };

        let tool_calls = &resp.message.tool_calls;
        let completion = resp.message.content.clone();

        let mut extracted_json_str = None;

        if let Some(tc) = tool_calls.iter().find(|t| t.name == "return_structured_output") {
            extracted_json_str = Some(tc.arguments.to_string());
        }

        let mut json_str;

        if let Some(s) = extracted_json_str {
            json_str = s;
        } else {
            // Output Parsing: JSON robust extraction mechanic.
            // Handles cases where LLM wraps response in markdown e.g. ```json ... ```
            json_str = completion.trim().to_string();
            let obj_start = json_str.find('{');
            let arr_start = json_str.find('[');

            let start_idx = match (obj_start, arr_start) {
                (Some(o), Some(a)) => std::cmp::min(o, a),
                (Some(o), None) => o,
                (None, Some(a)) => a,
                (None, None) => 0,
            };

            if start_idx > 0 {
                json_str = json_str[start_idx..].to_string();
            }

            let obj_end = json_str.rfind('}');
            let arr_end = json_str.rfind(']');

            let end_idx = match (obj_end, arr_end) {
                (Some(o), Some(a)) => std::cmp::max(o, a),
                (Some(o), None) => o,
                (None, Some(a)) => a,
                (None, None) => json_str.len().saturating_sub(1),
            };

            if end_idx < json_str.len() {
                json_str = json_str[..=end_idx].to_string();
            }

            if json_str.is_empty() {
                json_str = "null".to_string(); // If empty, fall back to null to trigger serde error
            }
        }

        match serde_json::from_str::<T>(&json_str) {
            Ok(parsed) => return Ok(parsed),
            Err(e) => {
                if attempt >= max_retries {
                    return Err(ToolError::Fatal(format!("Output parsing failed after {} retries. Last error: {}", max_retries, e)));
                }

                // Fallback mechanic: Legacy RetryWithErrorOutputParser
                // Feed the original prompt, the failed completion, and the parsing error back to the model.
                if !tool_calls.is_empty() {
                    let mut msg = Message::assistant(completion.clone());
                    msg.tool_calls = tool_calls.clone();
                    current_req.messages.push(msg);

                    let mut tool_results_msg = Message::user("");
                    tool_results_msg.role = crate::types::Role::Tool;
                    let mut results = Vec::new();
                    for tc in tool_calls {
                        results.push(crate::types::ToolResult {
                            tool_call_id: tc.id.clone(),
                            content: String::new(),
                            error: format!("Failed to parse output as valid JSON matching the schema. Error: {}. Please fix the JSON and use the 'return_structured_output' tool.", e),
                        });
                    }
                    tool_results_msg.tool_results = results;
                    current_req.messages.push(tool_results_msg);
                } else {
                    current_req.messages.push(Message::assistant(completion.clone()));
                    let error_msg = format!("Failed to parse output as valid JSON matching the schema. Error: {}. Please fix the JSON and use the 'return_structured_output' tool.", e);
                    current_req.messages.push(Message::user(error_msg));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChatResponse, Usage};
    use serde::Deserialize;
    use tokio::sync::Mutex;

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestOutput {
        result: String,
    }

    struct MockLlmClient {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl LlmClientForParser for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "default".to_string()
            };

            Ok(ChatResponse {
                message: Message::assistant(content),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_parse_structured_output_markdown_wrapper() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                "```json\n{\n  \"result\": \"success_markdown\"\n}\n```".to_string()
            ]),
        });

        let req = ChatRequest {
            model: "test".to_string(),
            system: "".to_string(),
            messages: vec![],
            tools: vec![],
            max_tokens: 100,
            temperature: 0.0,
        };

    let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, serde_json::json!({}), 3).await.unwrap();
        assert_eq!(result.result, "success_markdown");
    }

    #[tokio::test]
    async fn test_parse_structured_output_success() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![r#"{"result": "success"}"#.to_string()]),
        });

        let req = ChatRequest {
            model: "test".to_string(),
            system: "".to_string(),
            messages: vec![],
            tools: vec![],
            max_tokens: 100,
            temperature: 0.0,
        };

    let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, serde_json::json!({}), 3).await.unwrap();
        assert_eq!(result.result, "success");
    }

    #[tokio::test]
    async fn test_parse_structured_output_retry_success() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                "invalid json".to_string(),
                r#"{"result": "success after retry"}"#.to_string()
            ]),
        });

        let req = ChatRequest {
            model: "test".to_string(),
            system: "".to_string(),
            messages: vec![],
            tools: vec![],
            max_tokens: 100,
            temperature: 0.0,
        };

    let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, serde_json::json!({}), 3).await.unwrap();
        assert_eq!(result.result, "success after retry");
    }

    #[tokio::test]
    async fn test_parse_structured_output_failure() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                "invalid json".to_string(),
                "still invalid".to_string(),
            ]),
        });

        let req = ChatRequest {
            model: "test".to_string(),
            system: "".to_string(),
            messages: vec![],
            tools: vec![],
            max_tokens: 100,
            temperature: 0.0,
        };

    let result: Result<TestOutput, _> = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, serde_json::json!({}), 2).await;
        assert!(result.is_err());
        if let Err(ToolError::Fatal(msg)) = result {
            assert!(msg.contains("Output parsing failed after 2 retries"));
        } else {
            panic!("Expected Fatal error");
        }
    }

struct MockLlmClientWithTool {
    responses: Mutex<Vec<ChatResponse>>,
}

#[async_trait::async_trait]
impl LlmClientForParser for MockLlmClientWithTool {
    async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut resps = self.responses.lock().await;
        if !resps.is_empty() {
            Ok(resps.remove(0))
        } else {
            Ok(ChatResponse {
                message: Message::assistant("default"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }
}

#[tokio::test]
async fn test_parse_structured_output_tool_call() {
    let client = Arc::new(MockLlmClientWithTool {
        responses: Mutex::new(vec![
            ChatResponse {
                message: Message {
                    role: crate::types::Role::Assistant,
                    content: "".to_string(),
                    tool_calls: vec![crate::types::ToolCall {
                        id: "1".to_string(),
                        name: "return_structured_output".to_string(),
                        arguments: serde_json::json!({"result": "success_tool"}),
                    }],
                    tool_results: vec![],
                    response_id: Some("mock-id".to_string()),
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "tool_calls".to_string(),
                response_id: Some("mock-id".to_string()),
            }
        ]),
    });

    let req = ChatRequest {
        model: "test".to_string(),
        system: "".to_string(),
        messages: vec![],
        tools: vec![],
        max_tokens: 100,
        temperature: 0.0,
    };

    let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, serde_json::json!({}), 3).await.unwrap();
    assert_eq!(result.result, "success_tool");
}
}
