use async_trait::async_trait;
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Role};

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

/// Minifies any embedded JSON structures found within a larger text payload.
/// Uses a simple bracket matching algorithm to find potential JSON blocks and
/// attempts to parse them. If valid, replaces the block with its minified version.
pub fn minify_embedded_json(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '{' || c == '[' {
            // Find the matching closing bracket
            let closing_bracket = if c == '{' { '}' } else { ']' };
            let mut depth = 1;
            let mut potential_json = String::new();
            potential_json.push(c);

            let mut in_string = false;
            let mut escape_next = false;

            let mut valid_match = false;

            while let Some(&next_c) = chars.peek() {
                chars.next();
                potential_json.push(next_c);

                if escape_next {
                    escape_next = false;
                    continue;
                }

                if next_c == '\\' {
                    escape_next = true;
                    continue;
                }

                if next_c == '"' {
                    in_string = !in_string;
                    continue;
                }

                if !in_string {
                    if next_c == c {
                        depth += 1;
                    } else if next_c == closing_bracket {
                        depth -= 1;
                        if depth == 0 {
                            valid_match = true;
                            break;
                        }
                    }
                }
            }

            if valid_match {
                // Try parsing as JSON
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&potential_json) {
                    if let Ok(minified) = serde_json::to_string(&v) {
                        result.push_str(&minified);
                    } else {
                        result.push_str(&potential_json);
                    }
                } else {
                    result.push_str(&potential_json);
                }
            } else {
                result.push_str(&potential_json);
            }
        } else {
            result.push(c);
        }
    }

    result
}

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
    req.system = minify_embedded_json(&req.system);
    for m in &mut req.messages {
        m.content = minify_embedded_json(&m.content);
        for tr in &mut m.tool_results {
            tr.content = minify_embedded_json(&tr.content);
        }
    }
    req
}

pub fn truncate_by_word_count(data: &str, max_words: usize) -> String {
    if max_words == 0 {
        return "".to_string();
    }
    let words: Vec<&str> = data.split_whitespace().collect();
    if words.len() <= max_words {
        return data.to_string();
    }
    words[..max_words].join(" ")
}

pub fn truncate_chat_request(mut req: ChatRequest, max_history_words: usize) -> ChatRequest {
    if req.messages.len() <= 1 {
        return req;
    }

    let mut current_words = 0;
    let mut truncated_messages = Vec::new();
    let mut system_messages = Vec::new();
    let mut other_messages = req.messages;

    // Separate system messages to ensure they are preserved
    let mut i = 0;
    while i < other_messages.len() {
        if other_messages[i].role == Role::System {
            system_messages.push(other_messages.remove(i));
        } else {
            i += 1;
        }
    }

    // Always keep the last message
    if let Some(last) = other_messages.pop() {
        current_words += last.content.split_whitespace().count();
        truncated_messages.push(last);
    }

    // Add previous messages until budget is reached
    while let Some(msg) = other_messages.pop() {
        let msg_words = msg.content.split_whitespace().count();
        if current_words + msg_words > max_history_words && !truncated_messages.is_empty() {
            // If this message would put us over budget, truncate it or stop
            let remaining_budget = if max_history_words > current_words {
                max_history_words - current_words
            } else {
                0
            };
            if remaining_budget > 20 {
                let mut truncated_msg = msg;
                truncated_msg.content = truncate_by_word_count(&truncated_msg.content, remaining_budget);
                truncated_messages.push(truncated_msg);
            }
            break;
        }
        current_words += msg_words;
        truncated_messages.push(msg);
    }

    truncated_messages.reverse();

    // Prepend preserved system messages
    let mut final_messages = system_messages;
    final_messages.extend(truncated_messages);

    req.messages = final_messages;
    req
}


#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{Message, Role, ToolResult};


    #[test]
    fn test_minify_embedded_json() {
        let tests = vec![
            (
                "No JSON",
                "Just plain text",
                "Just plain text"
            ),
            (
                "Simple embedded JSON",
                r#"Here is the payload: {
                    "key": "value"
                }"#,
                r#"Here is the payload: {"key":"value"}"#
            ),
            (
                "Multiple embedded JSONs",
                r#"First: [
                    1, 2, 3
                ]. Second: {
                    "a": "b"
                } End."#,
                r#"First: [1,2,3]. Second: {"a":"b"} End."#
            ),
            (
                "JSON with strings containing brackets",
                r#"Text {
                    "key": "value { inside } [ brackets ]"
                }"#,
                r#"Text {"key":"value { inside } [ brackets ]"}"#
            ),
            (
                "Invalid JSON embedded (should be left alone)",
                r#"Text {
                    "key": "value",
                }"#,
                r#"Text {
                    "key": "value",
                }"#
            ),
        ];

        for (name, input, expected) in tests {
            let result = minify_embedded_json(input);
            assert_eq!(result, expected, "Failed on test case: {}", name);
        }
    }

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
