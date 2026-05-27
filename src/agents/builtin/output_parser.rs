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



pub trait OutputParser<T> {
    fn parse_message(&self, msg: &Message) -> Result<T, String>;
}

pub struct StructuredOutputParser<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> StructuredOutputParser<T> {
    pub fn new() -> Self {
        Self { _marker: std::marker::PhantomData }
    }
}

impl<T: DeserializeOwned> OutputParser<T> for StructuredOutputParser<T> {
    fn parse_message(&self, msg: &Message) -> Result<T, String> {
        let completion = msg.content.clone();

        // Output Parsing: Primary mechanic is extracting from native tool_calls
        if !msg.tool_calls.is_empty() {
            if let Some(call) = msg.tool_calls.iter().find(|t| t.name == "structured_output") {
                if let Some(data) = call.arguments.get("data") {
                    return match serde_json::from_value::<T>(data.clone()) {
                        Ok(parsed) => Ok(parsed),
                        Err(e) => {
                            Err(format!(
                                "Failed to parse tool call arguments as valid JSON matching the schema. Error: {}. Please fix the JSON and retry calling the tool.", e
                            ))
                        }
                    };
                } else {
                    return Err(
                        "Missing required 'data' parameter in tool call arguments. Please include the data matching the schema inside the 'data' property and retry calling the tool.".to_string()
                    );
                }
            }
        }

        // Fallback: extract JSON from raw text
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
            Ok(parsed) => Ok(parsed),
            Err(e) => {
                Err(format!(
                    "Failed to parse output as valid JSON matching the schema. Error: {}. Please fix the JSON and return only the raw JSON without markdown formatting. Your raw text was: {}", e, completion
                ))
            }
        }
    }
}

pub struct RetryWithErrorOutputParser<'a, T> {
    parser: Box<dyn OutputParser<T> + Send + Sync + 'a>,
    llm: Arc<dyn LlmClientForParser>,
}

impl<'a, T: DeserializeOwned> RetryWithErrorOutputParser<'a, T> {
    pub fn new(parser: Box<dyn OutputParser<T> + Send + Sync + 'a>, llm: Arc<dyn LlmClientForParser>) -> Self {
        Self { parser, llm }
    }

    pub async fn parse_with_prompt(&self, req: ChatRequest, max_retries: usize) -> Result<T, ToolError> {
        let mut current_req = req.clone();

        // Inject the schema as a tool definition to encourage the model to use tool_calls API
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

        let mut attempt = 1;
        loop {
            let resp = match self.llm.chat(current_req.clone()).await {
                Ok(r) => r,
                Err(e) => return Err(ToolError::Transient(format!("LLM Error: {}", e))),
            };

            let msg = &resp.message;
            match self.parser.parse_message(msg) {
                Ok(parsed) => return Ok(parsed),
                Err(parse_error_msg) => {
                    if attempt > max_retries {
                        return Err(ToolError::LlmRecoverable(format!(
                            "Output parsing failed after {} retries. Last error: {}",
                            max_retries, parse_error_msg
                        )));
                    }

                    // Feed the original prompt, the failed completion, and the parsing error back to the model as an LLM-recoverable ToolMessage
                    if !msg.tool_calls.is_empty() {
                        current_req.messages.push(msg.clone());
                        let tool_results = msg.tool_calls.iter().map(|tc| crate::types::ToolResult {
                            tool_call_id: tc.id.clone(),
                            content: String::new(),
                            error: parse_error_msg.clone(),
                        }).collect();

                        current_req.messages.push(Message {
                            role: crate::types::Role::Tool,
                            content: String::new(),
                            tool_calls: vec![],
                            tool_results,
                            response_id: None,
                            previous_response_id: msg.response_id.clone(),
                        });
                    } else {
                        current_req.messages.push(msg.clone());
                        current_req.messages.push(Message::user(parse_error_msg));
                    }
                    attempt += 1;
                }
            }
        }
    }
}

/// Implements the Output Parsing mechanic from the Master Catalog:
/// "Fallback mechanic: Legacy RetryWithErrorOutputParser (feed the original prompt,
/// the failed completion, and the parsing error back to the model)."
pub async fn parse_structured_output<T: DeserializeOwned + Send + Sync>(
    llm: &Arc<dyn LlmClientForParser>,
    req: ChatRequest,
    max_retries: usize,
) -> Result<T, ToolError> {
    let parser = Box::new(StructuredOutputParser::<T>::new());
    let retry_parser = RetryWithErrorOutputParser::new(parser, llm.clone());
    retry_parser.parse_with_prompt(req, max_retries).await
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
        let result: Result<TestOutput, _> = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result, "success after retry");
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
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Failed to parse output as valid JSON"));
        } else {
            panic!("Expected LlmRecoverable error, got {:?}", result);
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
        let result: Result<TestOutput, _> = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result, "success_tool_call_retry");
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
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Failed to parse tool call arguments"));
        } else {
            panic!("Expected LlmRecoverable error, got {:?}", result);
        }
    }
}
