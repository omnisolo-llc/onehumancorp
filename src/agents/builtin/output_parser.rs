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
    let mut attempt = 0;

    loop {
        attempt += 1;
        let resp = match llm.chat(current_req.clone()).await {
            Ok(r) => r,
            Err(e) => return Err(ToolError::Transient(format!("LLM Error: {}", e))),
        };
        let completion = resp.message.content.clone();

        // Output Parsing: JSON robust extraction mechanic.
        // Handles cases where LLM wraps response in markdown e.g. ```json ... ```
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

                // Fallback mechanic: Legacy RetryWithErrorOutputParser
                // Feed the original prompt, the failed completion, and the parsing error back to the model.
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
    use crate::types::{ChatResponse, Role, ToolCall, ToolResult, Usage};
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
            previous_response_id: None,
        };

        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
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
            previous_response_id: None,
        };

        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
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
            previous_response_id: None,
        };

        let result: TestOutput = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 3).await.unwrap();
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
            previous_response_id: None,
        };

        let result: Result<TestOutput, _> = parse_structured_output(&(client as Arc<dyn LlmClientForParser>), req, 2).await;
        assert!(result.is_err());
        if let Err(ToolError::Fatal(msg)) = result {
            assert!(msg.contains("Output parsing failed after 2 retries"));
        } else {
            panic!("Expected Fatal error");
        }
    }
}
