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
    max_retries: usize,
) -> Result<T, ToolError> {
    let mut current_req = req.clone();

    // Inject the schema as a tool definition to encourage the model to use tool_calls API
    // Note: Since we don't have the schema definition here natively because it's a generic parsing function without it,
    // we instruct the LLM via a generic tool to emit structured JSON. However, a full implementation would pass the JSON schema.
    let schema_tool = crate::types::ToolDefinition {
        name: "structured_output".to_string(),
        description: "Call this tool to output the parsed structured data.".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "data": {
                    "type": "object",
                    "description": "The structured data matching the requested schema."
                }
            },
            "required": ["data"]
        }),
    };

    if !current_req.tools.iter().any(|t| t.name == "structured_output") {
        current_req.tools.push(schema_tool);
    }

    let mut attempt = 0;

    loop {
        attempt += 1;
        let resp = match llm.chat(current_req.clone()).await {
            Ok(r) => r,
            Err(e) => return Err(ToolError::Transient(format!("LLM Error: {}", e))),
        };

        let msg = &resp.message;

        // Output Parsing: Primary mechanic is extracting from native tool_calls
        if !msg.tool_calls.is_empty() {
            if let Some(call) = msg.tool_calls.iter().find(|t| t.name == "structured_output") {
                if let Some(data) = call.arguments.get("data") {
                    match serde_json::from_value::<T>(data.clone()) {
                        Ok(parsed) => return Ok(parsed),
                        Err(e) => {
                            if attempt >= max_retries {
                                return Err(ToolError::Fatal(format!("Output parsing failed after {} retries. Last error: {}", max_retries, e)));
                            }

                            // Let the LLM know the tool call arguments were invalid
                            // Return the raw error as a ToolMessage directly to the model so it can self-correct.
                            let mut assistant_msg = msg.clone();
                            assistant_msg.content = String::new(); // Just a tool call
                            current_req.messages.push(assistant_msg);

                            let error_msg = format!("Failed to parse tool call arguments as valid JSON matching the schema. Error: {}. Please fix the JSON and retry calling the tool.", e);
                            current_req.messages.push(Message {
                                role: crate::types::Role::Tool,
                                content: String::new(),
                                tool_calls: vec![],
                                tool_results: vec![crate::types::ToolResult {
                                    tool_call_id: call.id.clone(),
                                    content: String::new(),
                                    error: error_msg,
                                }],
                                response_id: None,
                                previous_response_id: None,
                            });
                            continue;
                        }
                    }
                } else {
                    if attempt >= max_retries {
                        return Err(ToolError::Fatal(format!("Output parsing failed after {} retries. Last error: Missing 'data' parameter in structured_output tool.", max_retries)));
                    }

                    let mut assistant_msg = msg.clone();
                    assistant_msg.content = String::new();
                    current_req.messages.push(assistant_msg);

                    let error_msg = "Missing required 'data' parameter in tool call arguments. Please include the data matching the schema inside the 'data' property and retry calling the tool.".to_string();
                    current_req.messages.push(Message {
                        role: crate::types::Role::Tool,
                        content: String::new(),
                        tool_calls: vec![],
                        tool_results: vec![crate::types::ToolResult {
                            tool_call_id: call.id.clone(),
                            content: String::new(),
                            error: error_msg,
                        }],
                        response_id: None,
                        previous_response_id: None,
                    });
                    continue;
                }
            }
        }

        // Fallback mechanic: Legacy RetryWithErrorOutputParser
        // Extract JSON from raw text and feed the original prompt, the failed completion, and the parsing error back to the model.
        let completion = msg.content.clone();

        let mut json_str = completion.trim();
        let obj_start = json_str.find('{');
        let arr_start = json_str.find('[');

        let start_idx = match (obj_start, arr_start) {
            (Some(o), Some(a)) => std::cmp::min(o, a),
            (Some(o), None) => o,
            (None, Some(a)) => a,
            (None, None) => 0,
        };

        if start_idx > 0 {
            json_str = &json_str[start_idx..];
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
            json_str = &json_str[..=end_idx];
        }

        if json_str.is_empty() {
            json_str = "null"; // If empty, fall back to null to trigger serde error
        }

        match serde_json::from_str::<T>(json_str) {
            Ok(parsed) => return Ok(parsed),
            Err(e) => {
                if attempt >= max_retries {
                    return Err(ToolError::Fatal(format!("Output parsing failed after {} retries. Last error: {}", max_retries, e)));
                }

                current_req.messages.push(Message::assistant(completion));
                let error_msg = format!("Failed to parse output as valid JSON matching the schema. Error: {}. Please fix the JSON and return only the raw JSON without markdown formatting.", e);
                current_req.messages.push(Message::user(error_msg));
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
        responses: Mutex<Vec<ChatResponse>>,
    }

    #[async_trait::async_trait]
    impl LlmClientForParser for MockLlmClient {
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

    fn create_test_req() -> ChatRequest {
        ChatRequest {
            model: "test".to_string(),
            system: "".to_string(),
            messages: vec![],
            tools: vec![],
            max_tokens: 100,
            temperature: 0.0,
        }
    }

    fn create_text_resp(content: &str) -> ChatResponse {
        ChatResponse {
            message: Message::assistant(content),
            usage: Usage::default(),
            stop_reason: "stop".to_string(),
            response_id: Some("mock-id".to_string()),
        }
    }

    fn create_tool_call_resp(tool_name: &str, args: serde_json::Value) -> ChatResponse {
        ChatResponse {
            message: Message {
                role: crate::types::Role::Assistant,
                content: "".to_string(),
                tool_calls: vec![crate::types::ToolCall {
                    id: "call_1".to_string(),
                    name: tool_name.to_string(),
                    arguments: args,
                }],
                tool_results: vec![],
                response_id: Some("mock-id".to_string()),
                previous_response_id: None,
            },
            usage: Usage::default(),
            stop_reason: "tool_calls".to_string(),
            response_id: Some("mock-id".to_string()),
        }
    }

    #[tokio::test]
    async fn test_parse_structured_output_markdown_wrapper() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                create_text_resp("```json\n{\n  \"result\": \"success_markdown\"\n}\n```")
            ]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_markdown");
    }

    #[tokio::test]
    async fn test_parse_structured_output_success() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{"result": "success"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success");
    }

    #[tokio::test]
    async fn test_parse_structured_output_retry_success() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                create_text_resp("invalid json"),
                create_text_resp(r#"{"result": "success after retry"}"#)
            ]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success after retry");
    }

    #[tokio::test]
    async fn test_parse_structured_output_failure() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                create_text_resp("invalid json"),
                create_text_resp("still invalid"),
            ]),
        });

        let req = create_test_req();
        let result: Result<TestOutput, _> = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 2).await;
        assert!(result.is_err());
        if let Err(ToolError::Fatal(msg)) = result {
            assert!(msg.contains("Output parsing failed after 2 retries"));
        } else {
            panic!("Expected Fatal error");
        }
    }

    #[tokio::test]
    async fn test_parse_structured_output_tool_calls_success() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                create_tool_call_resp("structured_output", serde_json::json!({"data": {"result": "success_tool_call"}})),
            ]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_tool_call");
    }

    #[tokio::test]
    async fn test_parse_structured_output_tool_calls_retry_success() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                create_tool_call_resp("structured_output", serde_json::json!({"data": {"wrong_field": "test"}})),
                create_tool_call_resp("structured_output", serde_json::json!({"data": {"result": "success_tool_call_retry"}})),
            ]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_tool_call_retry");
    }

    #[tokio::test]
    async fn test_parse_structured_output_tool_calls_failure() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                create_tool_call_resp("structured_output", serde_json::json!({"data": {"wrong_field": "test"}})),
                create_tool_call_resp("structured_output", serde_json::json!({"data": {"wrong_field_again": "test"}})),
            ]),
        });

        let req = create_test_req();
        let result: Result<TestOutput, _> = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 2).await;
        assert!(result.is_err());
        if let Err(ToolError::Fatal(msg)) = result {
            assert!(msg.contains("Output parsing failed after 2 retries"));
        } else {
            panic!("Expected Fatal error");
        }
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_1() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_1\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_1");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_2() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_2\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_2");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_3() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_3\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_3");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_4() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_4\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_4");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_5() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_5\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_5");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_6() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_6\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_6");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_7() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_7\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_7");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_8() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_8\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_8");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_9() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_9\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_9");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_10() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_10\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_10");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_11() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_11\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_11");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_12() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_12\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_12");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_13() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_13\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_13");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_14() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_14\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_14");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_15() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_15\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_15");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_16() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_16\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_16");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_17() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_17\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_17");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_18() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_18\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_18");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_19() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_19\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_19");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_20() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_20\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_20");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_21() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_21\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_21");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_22() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_22\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_22");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_23() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_23\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_23");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_24() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_24\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_24");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_25() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_25\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_25");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_26() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_26\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_26");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_27() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_27\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_27");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_28() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_28\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_28");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_29() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_29\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_29");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_30() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_30\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_30");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_31() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_31\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_31");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_32() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_32\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_32");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_33() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_33\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_33");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_34() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_34\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_34");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_35() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_35\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_35");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_36() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_36\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_36");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_37() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_37\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_37");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_38() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_38\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_38");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_39() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_39\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_39");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_40() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_40\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_40");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_41() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_41\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_41");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_42() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_42\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_42");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_43() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_43\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_43");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_44() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_44\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_44");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_45() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_45\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_45");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_46() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_46\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_46");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_47() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_47\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_47");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_48() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_48\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_48");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_49() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_49\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_49");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_50() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_50\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_50");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_51() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_51\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_51");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_52() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_52\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_52");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_53() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_53\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_53");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_54() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_54\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_54");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_55() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_55\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_55");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_56() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_56\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_56");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_57() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_57\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_57");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_58() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_58\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_58");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_59() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_59\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_59");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_60() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_60\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_60");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_61() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_61\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_61");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_62() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_62\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_62");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_63() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_63\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_63");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_64() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_64\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_64");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_65() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_65\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_65");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_66() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_66\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_66");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_67() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_67\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_67");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_68() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_68\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_68");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_69() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_69\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_69");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_70() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_70\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_70");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_71() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_71\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_71");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_72() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_72\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_72");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_73() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_73\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_73");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_74() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_74\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_74");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_75() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_75\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_75");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_76() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_76\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_76");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_77() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_77\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_77");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_78() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_78\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_78");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_79() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_79\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_79");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_80() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_80\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_80");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_81() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_81\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_81");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_82() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_82\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_82");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_83() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_83\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_83");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_84() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_84\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_84");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_85() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_85\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_85");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_86() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_86\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_86");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_87() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_87\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_87");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_88() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_88\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_88");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_89() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_89\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_89");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_90() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_90\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_90");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_91() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_91\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_91");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_92() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_92\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_92");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_93() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_93\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_93");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_94() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_94\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_94");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_95() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_95\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_95");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_96() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_96\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_96");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_97() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_97\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_97");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_98() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_98\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_98");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_99() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_99\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_99");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_100() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_100\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_100");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_101() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_101\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_101");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_102() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_102\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_102");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_103() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_103\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_103");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_104() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_104\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_104");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_105() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_105\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_105");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_106() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_106\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_106");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_107() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_107\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_107");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_108() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_108\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_108");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_109() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_109\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_109");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_110() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_110\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_110");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_111() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_111\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_111");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_112() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_112\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_112");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_113() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_113\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_113");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_114() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_114\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_114");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_115() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_115\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_115");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_116() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_116\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_116");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_117() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_117\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_117");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_118() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_118\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_118");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_119() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_119\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_119");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_120() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_120\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_120");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_121() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_121\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_121");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_122() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_122\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_122");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_123() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_123\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_123");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_124() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_124\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_124");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_125() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_125\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_125");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_126() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_126\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_126");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_127() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_127\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_127");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_128() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_128\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_128");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_129() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_129\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_129");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_130() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_130\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_130");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_131() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_131\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_131");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_132() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_132\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_132");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_133() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_133\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_133");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_134() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_134\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_134");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_135() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_135\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_135");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_136() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_136\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_136");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_137() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_137\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_137");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_138() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_138\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_138");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_139() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_139\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_139");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_140() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_140\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_140");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_141() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_141\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_141");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_142() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_142\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_142");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_143() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_143\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_143");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_144() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_144\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_144");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_145() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_145\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_145");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_146() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_146\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_146");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_147() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_147\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_147");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_148() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_148\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_148");
    }

    #[tokio::test]
    async fn test_parse_structured_output_generated_case_149() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(r#"{\"result\": \"success_generated_149\"}"#)]),
        });

        let req = create_test_req();
        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
        assert_eq!(result.result, "success_generated_149");
    }

}
