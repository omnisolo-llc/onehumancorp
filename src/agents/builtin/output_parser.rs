use crate::types::{ChatRequest, Message, ToolError, ChatResponse};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use async_trait::async_trait;

/// Implements the Output Parsing mechanic from the Master Catalog:
/// "Fallback mechanic: Legacy RetryWithErrorOutputParser (feed the original prompt,
/// the failed completion, and the parsing error back to the model)."
pub fn parse_structured_output<T: serde::de::DeserializeOwned>(
    msg: &crate::types::Message,
) -> Result<T, crate::types::ToolError> {
    // Output Parsing: Primary mechanic is extracting from native tool_calls
    if !msg.tool_calls.is_empty() {
        if let Some(call) = msg.tool_calls.iter().find(|t| t.name == "return_structured_output" || t.name == "structured_output") {
            // Try extracting from a nested 'data' parameter if it exists
            let target = if let Some(data) = call.arguments.get("data") {
                data.clone()
            } else {
                call.arguments.clone()
            };

            match serde_json::from_value::<T>(target) {
                Ok(parsed) => return Ok(parsed),
                Err(e) => {
                    return Err(crate::types::ToolError::LlmRecoverable(format!(
                        "Failed to parse tool call arguments as valid JSON matching the schema. Error: {}. Please fix the JSON and retry calling the tool.", e
                    )));
                }
            }
        }
    }

    // Fallback mechanic: Legacy RetryWithErrorOutputParser
    // Extract JSON from raw text and feed the parsing error back to the model.
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
        Ok(parsed) => Ok(parsed),
        Err(e) => {
            Err(crate::types::ToolError::LlmRecoverable(format!(
                "Failed to parse output as valid JSON matching the schema. Error: {}. Please fix the JSON and return only the raw JSON without markdown formatting. Your raw text was: {}", e, completion
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Message, ToolCall, Role};
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestOutput {
        result: String,
    }

    fn create_text_msg(content: &str) -> Message {
        Message::assistant(content)
    }

    fn create_tool_call_msg(tool_name: &str, args: serde_json::Value) -> Message {
        Message {
            role: Role::Assistant,
            content: "".to_string(),
            tool_calls: vec![ToolCall {
                id: "call_1".to_string(),
                name: tool_name.to_string(),
                arguments: args,
            }],
            tool_results: vec![],
            response_id: None,
            previous_response_id: None,
        }
    }

    #[test]
    fn test_parse_structured_output_markdown_wrapper() {
        let msg = create_text_msg("```json\n{\n  \"result\": \"success_markdown\"\n}\n```");
        let result: TestOutput = parse_structured_output(&msg).unwrap();
        assert_eq!(result.result, "success_markdown");
    }

    #[test]
    fn test_parse_structured_output_success() {
        let msg = create_text_msg(r#"{"result": "success"}"#);
        let result: TestOutput = parse_structured_output(&msg).unwrap();
        assert_eq!(result.result, "success");
    }

    #[test]
    fn test_parse_structured_output_tool_calls_success() {
        let msg = create_tool_call_msg("structured_output", serde_json::json!({"data": {"result": "success_tool_call"}}));
        let result: TestOutput = parse_structured_output(&msg).unwrap();
        assert_eq!(result.result, "success_tool_call");
    }

    #[test]
    fn test_parse_structured_output_failure() {
        let msg = create_text_msg("invalid json");
        let result: Result<TestOutput, _> = parse_structured_output(&msg);
        assert!(result.is_err());
        if let Err(crate::types::ToolError::LlmRecoverable(err_msg)) = result {
            assert!(err_msg.contains("Failed to parse output as valid JSON"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }

    #[test]
    fn test_parse_structured_output_tool_calls_failure() {
        let msg = create_tool_call_msg("structured_output", serde_json::json!({"data": {"wrong_field": "test"}}));
        let result: Result<TestOutput, _> = parse_structured_output(&msg);
        assert!(result.is_err());
        if let Err(crate::types::ToolError::LlmRecoverable(err_msg)) = result {
            assert!(err_msg.contains("Failed to parse tool call arguments"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }
}
