#![allow(clippy::all)]
use crate::llm::LlmClient;
use crate::types::{ChatRequest, Message};

const TARGET_CHARS_MAX: usize = 8000;
const CHUNK_SIZE_CHARS: usize = 20000;

fn get_system_prompt() -> &'static str {
    "You are an expert summarizer. Compress the following subagent execution result into a dense 1k-2k token summary. Preserve all key decisions, code changes, and unresolved issues. Do not include raw context loops."
}

/// Recursively condenses a large text into a 1k-2k token summary using an LlmClient.
pub async fn condense_summary_llm(
    raw_output: &str,
    llm: &(impl LlmClient + ?Sized),
    model: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut current_text = raw_output.to_string();
    let system_prompt = get_system_prompt();

    while current_text.len() > TARGET_CHARS_MAX {
        let mut next_text_parts = Vec::new();
        let chars: Vec<char> = current_text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let end = std::cmp::min(i + CHUNK_SIZE_CHARS, chars.len());
            let chunk: String = chars[i..end].iter().collect();

            let req = ChatRequest {
                model: model.to_string(),
                system: ::server_pricing::compression::reduce_tokens(&system_prompt),
                messages: vec![Message::user(chunk)],
                tools: vec![],
                max_tokens: 2000,
                temperature: 0.0,
            };

            let resp = llm.chat(req).await?;
            next_text_parts.push(resp.message.content);
            i += CHUNK_SIZE_CHARS;
        }

        let next_text = next_text_parts.join("\n\n");
        if next_text.len() >= current_text.len() {
            tracing::warn!("Condensation loop failed to reduce text size. Stopping early.");
            current_text = next_text;
            break;
        }
        current_text = next_text;
    }

    if raw_output.len() == current_text.len() && current_text.len() > 1000 {
        let req = ChatRequest {
            model: model.to_string(),
            system: ::server_pricing::compression::reduce_tokens(&system_prompt),
            messages: vec![Message::user(current_text.clone())],
            tools: vec![],
            max_tokens: 2000,
            temperature: 0.0,
        };
        let resp = llm.chat(req).await?;
        current_text = resp.message.content;
    }

    if current_text.len() > TARGET_CHARS_MAX {
        current_text = format!(
            "{}\n\n[Output truncated. Subagent failed to condense summary.]",
            current_text
                .chars()
                .take(TARGET_CHARS_MAX)
                .collect::<String>()
        );
    }

    Ok(current_text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use ohc_builtin_agent_core::types::{ChatRequest, Message, Role};
use crate::types::{ChatResponse, Usage};
    use crate::llm::LlmClient;

    struct MockLlmClient {
        fail_to_reduce: bool,
    }

    #[async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let msg = &req.messages[0].content;
            let response_content = if self.fail_to_reduce {
                msg.clone()
            } else {
                "condensed".to_string()
            };
            Ok(ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: response_content,
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: Usage { input_tokens: 0, output_tokens: 0, cache_creation_input_tokens: 0, cache_read_input_tokens: 0 },
                response_id: Some("mock_id".to_string()),
                stop_reason: "mock_reason".to_string(),
            })
        }
    }

    #[tokio::test]
    async fn test_condense_summary_short() {
        let llm = MockLlmClient { fail_to_reduce: false };
        let input = "short text".to_string();
        let res = condense_summary_llm(&input, &llm, "test-model").await.unwrap();
        assert_eq!(res, "short text");
    }

    #[tokio::test]
    async fn test_condense_summary_needs_one_pass() {
        let llm = MockLlmClient { fail_to_reduce: false };
        let input = "a".repeat(1500);
        let res = condense_summary_llm(&input, &llm, "test-model").await.unwrap();
        assert_eq!(res, "condensed");
    }

    #[tokio::test]
    async fn test_condense_summary_needs_loop() {
        let llm = MockLlmClient { fail_to_reduce: false };
        let input = "a".repeat(25000);
        let res = condense_summary_llm(&input, &llm, "test-model").await.unwrap();
        assert_eq!(res, "condensed\n\ncondensed");
    }

    #[tokio::test]
    async fn test_condense_summary_fail_to_reduce() {
        let llm = MockLlmClient { fail_to_reduce: true };
        let input = "a".repeat(10000);
        let res = condense_summary_llm(&input, &llm, "test-model").await.unwrap();
        assert!(res.contains("[Output truncated. Subagent failed to condense summary.]"));
        assert!(res.starts_with(&"a".repeat(8000)));
    }
}
