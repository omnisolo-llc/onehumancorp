use async_trait::async_trait;
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse};

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>>;
    async fn generate_embedding(&self, _text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(vec![])
    }
}

pub mod anthropic;
pub mod openai;
pub mod ollama;
pub mod gemini;

use serde_json::Value;

/// MinifyJSONString takes a string, checks if it is valid JSON, and returns
/// the minified version of it (whitespace removed). If it's not valid JSON,
/// it returns the original string.
pub fn minify_json_string(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return input.to_string();
    }

    // Quick check to see if it even looks like JSON
    if !(trimmed.starts_with('{') && trimmed.ends_with('}')) &&
       !(trimmed.starts_with('[') && trimmed.ends_with(']')) {
        return input.to_string();
    }

    match serde_json::from_str::<Value>(trimmed) {
        Ok(v) => serde_json::to_string(&v).unwrap_or_else(|_| input.to_string()),
        Err(_) => input.to_string(),
    }
}

pub fn minify_chat_request(mut req: ChatRequest) -> ChatRequest {
    req.system = minify_json_string(&req.system);
    for m in &mut req.messages {
        m.content = minify_json_string(&m.content);
        for tr in &mut m.tool_results {
            tr.content = minify_json_string(&tr.content);
        }
        for _tc in &mut m.tool_calls {
            // Also minify tool_calls arguments if it's stored as string somehow.
            // But here it is serde_json::Value. Wait, tool_calls arguments are usually Value.
            // The JSON value itself gets minified when serialized.
        }
    }
    req
}


#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{Message, Role, ToolResult};

    #[test]
    fn test_minify_json_string() {
        assert_eq!(minify_json_string("   "), "   ");
        assert_eq!(minify_json_string("not json"), "not json");

        let json_str = r#"{
            "key": "value"
        }"#;
        assert_eq!(minify_json_string(json_str), r#"{"key":"value"}"#);

        let invalid_json = r#"{
            "key": "value",
        }"#;
        assert_eq!(minify_json_string(invalid_json), invalid_json);
    }

    #[test]
    fn test_minify_chat_request() {
        let mut req = ChatRequest {
            model: "test".to_string(),
            system: r#"{
                "system": "instruction"
            }"#.to_string(),
            messages: vec![
                Message {
                    role: Role::User,
                    content: r#"{
                        "user": "input"
                    }"#.to_string(),
                    tool_calls: vec![],
                    tool_results: vec![
                        ToolResult {
                            tool_call_id: "1".to_string(),
                            content: r#"{
                                "tool": "result"
                            }"#.to_string(),
                            error: "".to_string(),
                        }
                    ],
                }
            ],
            tools: vec![],
            max_tokens: 100,
            temperature: 0.0,
        };

        let minified = minify_chat_request(req);

        assert_eq!(minified.system, r#"{"system":"instruction"}"#);
        assert_eq!(minified.messages[0].content, r#"{"user":"input"}"#);
        assert_eq!(minified.messages[0].tool_results[0].content, r#"{"tool":"result"}"#);
    }
}
