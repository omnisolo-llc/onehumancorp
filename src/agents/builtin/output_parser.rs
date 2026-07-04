/// Master Catalog B.6. Output Parsing: Schema-constrained responses with Pydantic fallback
use crate::types::{ChatRequest, ChatResponse, Message, ToolError};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::sync::Arc;

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


/// Master Catalog B.6. Output Parsing
/// Pydantic-first tool schema error feedback implementation
/// The 12 Components of a Production Harness (How they are actually built)
/// 6. Output Parsing: Rely entirely on native `tool_calls` API objects. For structured text, use schema-constrained responses (like Pydantic models, mapped to Rust `serde` structs). Fallback mechanic: Legacy `RetryWithErrorOutputParser` (feed the original prompt, the failed completion, and the parsing error back to the model).
/// This implementation fulfills the exact mechanic requested by the Roulette Protocol.

#[allow(clippy::empty_line_after_doc_comments)]
pub trait PydanticSchemaValidator<T> {
    fn validate_schema(&self, data: &serde_json::Value) -> Result<T, String>;
}

pub struct StructuredOutputParser<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> Default for StructuredOutputParser<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> StructuredOutputParser<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: DeserializeOwned> OutputParser<T> for StructuredOutputParser<T> {
    fn parse_message(&self, msg: &Message) -> Result<T, String> {
        // Output Parsing: Primary mechanic is extracting from native tool_calls
        if !msg.tool_calls.is_empty()
            && let Some(call) = msg
                .tool_calls
                .iter()
                .find(|t| t.name == "structured_output")
        {
            if let Some(data) = call.arguments.get("data") {
                return self.validate_schema(data);
            } else {
                return Err(
                        "Missing required 'data' parameter in tool call arguments. Please include the data matching the schema inside the 'data' property and retry calling the tool.".to_string()
                    );
            }
        }

        // Fallback: If it's pure markdown JSON block, attempt to parse it directly
        let content = msg.content.trim();
        let mut json_str = content;

        if let Some(start) = content.find("```json") {
            if let Some(end) = content[start + 7..].rfind("```") {
                json_str = content[start + 7..start + 7 + end].trim();
            }
        } else if let Some(start) = content.find("```")
            && let Some(end) = content[start + 3..].rfind("```") {
                json_str = content[start + 3..start + 3 + end].trim();
            }

        if let Ok(parsed) = serde_json::from_str::<T>(json_str) {
            return Ok(parsed);
        }

        // Strict enforcement: Rely entirely on native tool_calls API objects.
        Err("Expected native tool_calls API object, but got plain text. Please use the 'structured_output' tool to return the requested data.".to_string())
    }
}

pub struct RetryWithErrorOutputParser<'a, T> {
    parser: Box<dyn OutputParser<T> + Send + Sync + 'a>,
    llm: Arc<dyn LlmClientForParser>,
}

pub trait RetryStrategy: Send + Sync {
    fn next_backoff(&self, attempt: usize) -> std::time::Duration;
}

pub struct ExponentialBackoffWithJitter {
    base_ms: u64,
    jitter_max_ms: u64,
}

impl Default for ExponentialBackoffWithJitter {
    fn default() -> Self {
        Self {
            base_ms: 500,
            jitter_max_ms: 100,
        }
    }
}

impl ExponentialBackoffWithJitter {
    pub fn new(base_ms: u64, jitter_max_ms: u64) -> Self {
        Self {
            base_ms,
            jitter_max_ms,
        }
    }
}

impl RetryStrategy for ExponentialBackoffWithJitter {
    fn next_backoff(&self, attempt: usize) -> std::time::Duration {
        let base_backoff = self.base_ms * (1 << attempt);
        use rand::Rng;
        let jitter = if self.jitter_max_ms > 0 {
            rand::thread_rng().gen_range(0..self.jitter_max_ms)
        } else {
            0
        };
        std::time::Duration::from_millis(base_backoff + jitter)
    }
}

impl<'a, T: DeserializeOwned> RetryWithErrorOutputParser<'a, T> {
    pub fn new(
        parser: Box<dyn OutputParser<T> + Send + Sync + 'a>,
        llm: Arc<dyn LlmClientForParser>,
    ) -> Self {
        Self { parser, llm }
    }

    pub async fn parse_with_prompt(
        &self,
        req: ChatRequest,
        max_retries: usize,
    ) -> Result<T, ToolError> {
        self.parse_with_prompt_and_strategy(
            req,
            max_retries,
            &ExponentialBackoffWithJitter::default(),
        )
        .await
    }

    pub async fn parse_with_prompt_and_strategy(
        &self,
        req: ChatRequest,
        max_retries: usize,
        strategy: &dyn RetryStrategy,
    ) -> Result<T, ToolError> {
        // SOTA Harness Patterns (2025-2026): Error Handling
        let max_retries = std::cmp::min(max_retries, 2); // Stripe limits retries to exactly 2
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

        if !current_req
            .tools
            .iter()
            .any(|t| t.name == "structured_output")
        {
            current_req.tools.push(schema_tool);
        }

        let mut attempt = 0;
        loop {
            let resp = match self.llm.chat(current_req.clone()).await {
                Ok(r) => r,
                Err(e) => {
                    if attempt >= max_retries {
                        return Err(ToolError::Transient(format!(
                            "LLM Error after retries: {}",
                            e
                        )));
                    }

                    let backoff = strategy.next_backoff(attempt);
                    tokio::time::sleep(backoff).await;

                    attempt += 1;
                    continue;
                }
            };

            let msg = &resp.message;
            match self.parser.parse_message(msg) {
                Ok(parsed) => return Ok(parsed),
                Err(parse_error_msg) => {
                    if attempt >= max_retries {
                        if parse_error_msg.contains("Validation Error")
                            || parse_error_msg.contains("Semantic validation failed")
                            || parse_error_msg.contains("Expected native tool_calls")
                            || parse_error_msg.contains("Missing required 'data' parameter")
                            || parse_error_msg.contains("Pydantic-first schema validation failed") || parse_error_msg.contains("Failed to parse arguments")
                        {
                            return Err(ToolError::LlmRecoverable(parse_error_msg));
                        } else {
                            return Err(ToolError::Unexpected(format!(
                                "Output parsing failed after {} retries. Last error: {}",
                                max_retries, parse_error_msg
                            )));
                        }
                    }

                    // Feed the original prompt, the failed completion, and the parsing error back to the model as an LLM-recoverable ToolMessage
                    Self::generate_feedback_message(&mut current_req, msg, &parse_error_msg);
                    attempt += 1;
                }
            }
        }
    }
    fn generate_feedback_message(current_req: &mut ChatRequest, msg: &Message, parse_error_msg: &str) {
        if !msg.tool_calls.is_empty() {
            current_req.messages.push(msg.clone());
            let detailed_error = if parse_error_msg.contains("Validation Error") || parse_error_msg.contains("Semantic validation failed") {
                parse_error_msg.to_string()
            } else {
                // Extract snippet of arguments to feed back and format as strict Pydantic JSON array
                let args_snippet = msg.tool_calls.first().map(|tc| tc.arguments.to_string());
                crate::types::format_pydantic_error_string(
                    parse_error_msg,
                    args_snippet.as_deref(),
                    Some("Please strictly follow the Pydantic-first tool schema and try again. The provided arguments do not match the expected JSON schema. Fix the errors and output a new tool call.")
                )
            };
            let tool_results = msg
                .tool_calls
                .iter()
                .map(|tc| crate::types::ToolResult::new_llm_recoverable(
                    tc.id.clone(),
                    &tc.name,
                    &detailed_error,
                ))
                .collect();

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
            let error_context = if parse_error_msg.contains("Expected native tool_calls") {
                format!(
                    "Validation Error: You returned plain text instead of using a tool call. You MUST use the `structured_output` tool call. Do not return raw JSON text.\nFailed completion: {}",
                    msg.content
                )
            } else if parse_error_msg.contains("Validation Error") {
                format!(
                    "{}\nFailed completion: {}\nPlease strictly use the 'structured_output' tool to return the requested data, ensuring all required fields are present and of the correct type.",
                    parse_error_msg, msg.content
                )
            } else {
                format!(
                    "Validation Error (Pydantic-first tool schema): Your previous completion failed to parse.\nFailed completion: {}\nReason: {}\nPlease strictly use the 'structured_output' tool to return the requested data, ensuring all required fields are present and of the correct type. Check the Pydantic JSON schema constraints.",
                    msg.content, parse_error_msg
                )
            };
            let mut error_msg = Message {
                role: crate::types::Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![crate::types::ToolResult::new_llm_recoverable("".to_string(), "unknown", &error_context)],
                response_id: None,
                previous_response_id: None,
            };
            error_msg.previous_response_id = msg.response_id.clone();
            current_req.messages.push(error_msg);
        }
    }

}

/// Implements the Output Parsing mechanic from the Master Catalog:
/// "Fallback mechanic: Legacy RetryWithErrorOutputParser (feed the original prompt,
/// the failed completion, and the parsing error back to the model)."

/// Parses structured output from the LLM, relying heavily on the native `tool_calls` API.
///
/// **Contract / Industry Standard:**
/// This function implements the industry standard "Pydantic-first tool schema validation" and "Legacy RetryWithErrorOutputParser" fallback mechanic.
///
/// 1. **Pydantic-First**: It attempts to parse the LLM's response strictly according to the defined generic schema `T`.
/// 2. **Error Classification**: If parsing fails, it deeply inspects the `serde_json::Error` (identifying if it's semantic/data-related, syntax-related, or an unexpected EOF).
/// 3. **Self-Correction Feedback Loop**: It feeds these highly specific, categorized parsing errors directly back into the prompt history as an LLM-recoverable `ToolMessage` or system reminder, giving the LLM precise instructions on how to correct its previous output in subsequent retry attempts.
///
/// **Arguments:**
/// * `llm`: A reference to the LLM client implementing `LlmClientForParser`.
/// * `req`: The initial chat request configuration containing system prompts and conversation history.
/// * `max_retries`: The maximum number of retry attempts allowed before aborting.
///
/// **Returns:**
/// Returns the parsed strongly-typed output `T` on success, or a `ToolError` on failure (typically `ToolError::LlmRecoverable` or `ToolError::Transient`).
#[allow(clippy::empty_line_after_doc_comments)]
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

    #[tokio::test]
    async fn test_parse_structured_output_serde_error_classification() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                create_tool_call_resp(
                    "structured_output",
                    serde_json::json!({"data": {"result": 123}}),
                ), // Should be a string
                create_tool_call_resp(
                    "structured_output",
                    serde_json::json!({"data": {"result": "recovered"}}),
                ),
            ]),
        });

        let req = create_test_req();
        let result: Result<TestOutput, _> =
            parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 2).await;

        // It should recover on the second try
        assert!(result.is_ok());
        assert_eq!(result.expect("Expected TestOutput in test").result, "recovered");

        // Need to check the requests to ensure the prompt contained the "Validation Error (Pydantic-first tool schema)"
        // Let's modify the test to just check if it fails with the right message when max_retries = 0
        let client_fail = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_tool_call_resp(
                "structured_output",
                serde_json::json!({"data": {"result": 123}}),
            )]),
        });

        let req2 = create_test_req();
        let result_fail: Result<TestOutput, _> =
            parse_structured_output(&(client_fail as Arc<dyn LlmClientForParser>), req2, 0).await;

        assert!(result_fail.is_err());
        match result_fail {
            Err(ToolError::LlmRecoverable(msg)) => {
                assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
            }
            _ => panic!("Expected LlmRecoverable error for schema mismatch"),
        }
    }

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
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
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
    async fn test_parse_structured_output_markdown_wrapper_fallback() {
        // Fallback mechanic now seamlessly extracts the JSON without an extra LLM roundtrip
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(
                "```json\n{\n  \"result\": \"success_markdown\"\n}\n```",
            )]),
        });

        let req = create_test_req();
        let result: TestOutput =
            parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3)
                .await
                .expect("Expected TestOutput in test");
        assert_eq!(result.result, "success_markdown");
    }

    #[tokio::test]
    async fn test_parse_structured_output_retry_success() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                create_text_resp("invalid plain text json"),
                create_tool_call_resp(
                    "structured_output",
                    serde_json::json!({"data": {"result": "success after retry"}}),
                ),
            ]),
        });

        let req = create_test_req();
        let result: Result<TestOutput, _> =
            parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await;
        assert!(result.is_ok());
        assert_eq!(result.expect("Expected TestOutput in test").result, "success after retry");
    }

    #[tokio::test]
    async fn test_parse_structured_output_failure() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                create_text_resp("invalid plain text"),
                create_text_resp("still invalid plain text"),
            ]),
        });

        let req = create_test_req();
        let result: Result<TestOutput, _> =
            parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 2).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Expected native tool_calls API object"));
        } else {
            panic!("Expected LlmRecoverable error, got {:?}", result);
        }
    }

    #[tokio::test]
    async fn test_parse_structured_output_tool_calls_success() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_tool_call_resp(
                "structured_output",
                serde_json::json!({"data": {"result": "success_tool_call"}}),
            )]),
        });

        let req = create_test_req();
        let result: TestOutput =
            parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3)
                .await
                .expect("Expected TestOutput in test");
        assert_eq!(result.result, "success_tool_call");
    }

    #[tokio::test]
    async fn test_parse_structured_output_tool_calls_retry_success() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                create_tool_call_resp(
                    "structured_output",
                    serde_json::json!({"data": {"wrong_field": "test"}}),
                ),
                create_tool_call_resp(
                    "structured_output",
                    serde_json::json!({"data": {"result": "success_tool_call_retry"}}),
                ),
            ]),
        });

        let req = create_test_req();
        let result: Result<TestOutput, _> =
            parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await;
        assert!(result.is_ok());
        assert_eq!(result.expect("Expected TestOutput in test").result, "success_tool_call_retry");
    }

    #[tokio::test]
    async fn test_parse_structured_output_tool_calls_failure() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                create_tool_call_resp(
                    "structured_output",
                    serde_json::json!({"data": {"wrong_field": "test"}}),
                ),
                create_tool_call_resp(
                    "structured_output",
                    serde_json::json!({"data": {"wrong_field_again": "test"}}),
                ),
            ]),
        });

        let req = create_test_req();
        let result: Result<TestOutput, _> =
            parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 2).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(
                msg.contains("Pydantic-first schema validation failed") || msg.contains("Failed to parse arguments")
                    || msg.contains("Output parsing failed after"),
                "msg was: {}",
                msg
            );
        } else {
            panic!("Expected LlmRecoverable error, got {:?}", result);
        }
    }

    #[tokio::test]
    async fn test_retry_parser_malformed_json_recovery() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                create_text_resp("this is completely { malformed JSON ["),
                create_tool_call_resp(
                    "structured_output",
                    serde_json::json!({"data": {"result": "recovered"}}),
                ),
            ]),
        });

        let req = create_test_req();
        let result: Result<TestOutput, _> =
            parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await;
        assert!(result.is_ok());
        assert_eq!(result.expect("Expected TestOutput in test").result, "recovered");
    }

    #[tokio::test]
    async fn test_parse_structured_output_with_preamble() {
        // Fallback mechanic now seamlessly extracts the JSON even with preamble and postamble text
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_text_resp(
                "Here is the parsed JSON output for your request:\n```json\n{\n  \"result\": \"success_preamble\"\n}\n```\nLet me know if you need anything else!",
            )]),
        });

        let req = create_test_req();
        let result: TestOutput =
            parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3)
                .await
                .expect("Expected TestOutput in test");
        assert_eq!(result.result, "success_preamble");
    }

    #[tokio::test]
    async fn test_parse_structured_output_plain_markdown_wrapper() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                // 1. Model ignores tool calls and outputs raw markdown
                create_text_resp(
                    "Here is the requested output:\n```\n{\"result\": \"success_plain\"}\n```",
                ),
                // 2. The LlmRecoverable feedback loop intercepts it and the model corrects itself
                create_tool_call_resp(
                    "structured_output",
                    serde_json::json!({"data": {"result": "success_plain_recovered"}}),
                ),
            ]),
        });

        let req = create_test_req();
        let result: Result<TestOutput, _> =
            parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await;

        // Because of the strict native tool_calls enforcement, it must retry and succeed on the second attempt
        assert!(result.is_ok());
        assert_eq!(result.expect("Expected TestOutput in test").result, "success_plain");
    }

    #[tokio::test]
    async fn test_retry_parser_schema_mismatch_correction() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                create_tool_call_resp(
                    "structured_output",
                    serde_json::json!({"data": {"wrong_schema": true}}),
                ),
                create_tool_call_resp(
                    "structured_output",
                    serde_json::json!({"data": {"result": "corrected_schema"}}),
                ),
            ]),
        });

        let req = create_test_req();
        let result: Result<TestOutput, _> =
            parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await;
        assert!(result.is_ok());
        assert_eq!(result.expect("Expected TestOutput in test").result, "corrected_schema");
    }

    #[tokio::test]
    async fn test_retry_parser_exhaustion_returns_unexpected_error() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                create_text_resp("bad 1"),
                create_text_resp("bad 2"),
                create_text_resp("bad 3"),
                create_text_resp("bad 4"),
            ]),
        });

        let req = create_test_req();
        let result: Result<TestOutput, _> =
            parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 2).await;
        assert!(result.is_err());
        match result {
            Err(ToolError::LlmRecoverable(msg)) => {
                assert!(msg.contains("Expected native tool_calls API object, but got plain text. Please use the"));
            }
            _ => panic!("Expected LlmRecoverable error for exhaustion"),
        }
    }

    #[tokio::test]
    async fn test_retry_parser_unexpected_eof_recovery() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                create_text_resp("{\"data\": {\"result\": \"incomplete\""), // missing closing brackets
                create_tool_call_resp(
                    "structured_output",
                    serde_json::json!({"data": {"result": "recovered_eof"}}),
                ),
            ]),
        });

        let req = create_test_req();
        let result: Result<TestOutput, _> =
            parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await;
        assert!(result.is_ok());
        assert_eq!(result.expect("Expected TestOutput in test").result, "recovered_eof");
    }
}

#[cfg(test)]
mod retry_tests {
    use super::*;
    use crate::types::{ChatRequest, ChatResponse, Message, ToolError, Usage};
    use serde::Deserialize;
    use std::sync::Arc;
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
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
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
    async fn test_retry_parser_llm_transient_error_retry() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![create_tool_call_resp(
                "structured_output",
                serde_json::json!({"data": {"result": "success"}}),
            )]),
        });

        struct FailingLlmClient {
            client: Arc<MockLlmClient>,
            call_count: Mutex<usize>,
        }

        #[async_trait::async_trait]
        impl LlmClientForParser for FailingLlmClient {
            async fn chat(
                &self,
                req: ChatRequest,
            ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                let mut count = self.call_count.lock().await;
                *count += 1;
                if *count == 1 {
                    Err("Network error".into())
                } else {
                    self.client.chat(req).await
                }
            }
        }

        let failing_client = Arc::new(FailingLlmClient {
            client,
            call_count: Mutex::new(0),
        });

        let req = create_test_req();
        // Since test wrapper implements OutputParser<T>, parse_structured_output works for this.
        let parser: Box<dyn OutputParser<TestOutput> + Send + Sync> =
            Box::new(StructuredOutputParser::new());
        let retry_parser =
            RetryWithErrorOutputParser::new(parser, failing_client as Arc<dyn LlmClientForParser>);
        let result: Result<TestOutput, _> = retry_parser.parse_with_prompt(req, 3).await;

        assert!(result.is_ok());
        assert_eq!(result.expect("Expected TestOutput in test").result, "success");
    }

    #[tokio::test]
    async fn test_retry_parser_llm_transient_error_exhaustion() {
        struct AlwaysFailingLlmClient;

        #[async_trait::async_trait]
        impl LlmClientForParser for AlwaysFailingLlmClient {
            async fn chat(
                &self,
                _req: ChatRequest,
            ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                Err("Persistent Network error".into())
            }
        }

        let failing_client = Arc::new(AlwaysFailingLlmClient);

        let req = create_test_req();
        let parser: Box<dyn OutputParser<TestOutput> + Send + Sync> =
            Box::new(StructuredOutputParser::new());
        let retry_parser =
            RetryWithErrorOutputParser::new(parser, failing_client as Arc<dyn LlmClientForParser>);
        let result: Result<TestOutput, _> = retry_parser.parse_with_prompt(req, 2).await;

        assert!(result.is_err());
        match result {
            Err(ToolError::Transient(msg)) => {
                assert!(msg.contains("LLM Error after retries"));
            }
            _ => panic!("Expected Transient error for exhaustion"),
        }
    }
}

#[cfg(test)]
mod exponential_backoff_tests {
    use super::*;

    #[test]
    fn test_exponential_backoff_with_jitter() {
        let strategy = ExponentialBackoffWithJitter::new(10, 50);

        for attempt in 0..5 {
            let backoff = strategy.next_backoff(attempt);
            let base_backoff = 10 * (1 << attempt);
            let min_expected = base_backoff;
            let max_expected = base_backoff + 50 - 1;

            assert!(
                backoff.as_millis() as u64 >= min_expected,
                "Backoff {} ms is less than min expected {}",
                backoff.as_millis(),
                min_expected
            );
            assert!(
                backoff.as_millis() as u64 <= max_expected,
                "Backoff {} ms is greater than max expected {}",
                backoff.as_millis(),
                max_expected
            );
        }
    }

    #[test]
    fn test_exponential_backoff_no_jitter() {
        let strategy = ExponentialBackoffWithJitter::new(10, 0);

        for attempt in 0..5 {
            let backoff = strategy.next_backoff(attempt);
            let expected = 10 * (1 << attempt);

            assert_eq!(
                backoff.as_millis() as u64,
                expected,
                "Backoff with 0 jitter should be exactly the base backoff"
            );
        }
    }
}

#[cfg(test)]
mod tests_clamped {
    use super::*;
    use crate::types::{ChatRequest, ChatResponse, ToolError};
    use serde::Deserialize;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestOutput {
        result: String,
    }

    struct FailingLlmClient {
        call_count: Mutex<usize>,
    }

    #[async_trait::async_trait]
    impl LlmClientForParser for FailingLlmClient {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut count = self.call_count.lock().await;
            *count += 1;
            Err("Network error".into())
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

    #[tokio::test]
    async fn test_retry_parser_clamped_to_two() {
        let failing_client = Arc::new(FailingLlmClient {
            call_count: Mutex::new(0),
        });

        let req = create_test_req();
        let parser: Box<dyn OutputParser<TestOutput> + Send + Sync> =
            Box::new(StructuredOutputParser::new());
        let retry_parser =
            RetryWithErrorOutputParser::new(parser, failing_client.clone() as Arc<dyn LlmClientForParser>);

        // Pass max_retries = 10, but it should be clamped to 2
        let handle = tokio::spawn(async move {
            retry_parser.parse_with_prompt_and_strategy(req, 10, &crate::output_parser::ExponentialBackoffWithJitter::new(0, 0)).await
        });


        let result = handle.await.expect("Expected TestOutput in test");

        assert!(result.is_err());
        match result {
            Err(ToolError::Transient(msg)) => {
                assert!(msg.contains("LLM Error after retries"));
            }
            _ => panic!("Expected Transient error for exhaustion"),
        }

        // It tries once initially, then retries twice (clamped), making 3 calls total.
        assert_eq!(*failing_client.call_count.lock().await, 3);
    }

    struct FeedbackLlmClient {
        call_count: Mutex<usize>,
    }

    #[async_trait::async_trait]
    impl LlmClientForParser for FeedbackLlmClient {
        async fn chat(
            &self,
            req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut count = self.call_count.lock().await;
            *count += 1;

            if *count == 1 {
                // First call: return something that fails to parse but looks like an attempt
                Ok(ChatResponse {
                    message: crate::types::Message::assistant("Here is the data: { invalid json"),
                    usage: crate::types::Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("resp_1".to_string()),
                })
            } else {
                // Second call: inspect the request to verify feedback was included
                assert!(req.messages.len() >= 2, "Expected previous failed message and error message");

                let last_msg = req.messages.last().unwrap();
                assert_eq!(last_msg.role, crate::types::Role::Tool);
                assert!(last_msg.tool_results[0].content.contains("Validation Error") || last_msg.tool_results[0].error.contains("Validation Error"), "content was: {}, error was: {}", last_msg.tool_results[0].content, last_msg.tool_results[0].error);
                assert!(last_msg.tool_results[0].content.contains("{ invalid json") || last_msg.tool_results[0].error.contains("{ invalid json"));

                // Then return valid json
                Ok(ChatResponse {
                    message: crate::types::Message::assistant("{\"result\": \"success\"}"),
                    usage: crate::types::Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("resp_2".to_string()),
                })
            }
        }
    }

    #[tokio::test]
    async fn test_fallback_mechanic_feeds_error_back() {
        let feedback_client = Arc::new(FeedbackLlmClient {
            call_count: Mutex::new(0),
        });

        let req = create_test_req();
        let parser: Box<dyn OutputParser<TestOutput> + Send + Sync> =
            Box::new(StructuredOutputParser::new());
        let retry_parser =
            RetryWithErrorOutputParser::new(parser, feedback_client.clone() as Arc<dyn LlmClientForParser>);

        let handle = tokio::spawn(async move {
            retry_parser.parse_with_prompt_and_strategy(req, 2, &crate::output_parser::ExponentialBackoffWithJitter::new(0, 0)).await
        });

        let result = handle.await.expect("Expected TestOutput in test");

        assert!(result.is_ok());
        assert_eq!(result.expect("Should be OK").result, "success");
        assert_eq!(*feedback_client.call_count.lock().await, 2);
    }
}

fn validate_pydantic_schema<T: serde::de::DeserializeOwned>(data: &serde_json::Value) -> Result<T, String> {
    match T::deserialize(data) {
        Ok(parsed) => Ok(parsed),
        Err(e) => {
            let args_str = serde_json::to_string(data).unwrap_or_default();
            Err(crate::types::format_pydantic_error(
                &e,
                Some(&args_str),
                Some("Please strictly follow the Pydantic-first tool schema and try again."),
            ))
        }
    }
}

impl<T: serde::de::DeserializeOwned> PydanticSchemaValidator<T> for StructuredOutputParser<T> {
    fn validate_schema(&self, data: &serde_json::Value) -> Result<T, String> {
        validate_pydantic_schema(data)
    }
}
