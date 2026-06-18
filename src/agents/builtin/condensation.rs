use crate::types::{ChatRequest, Message};
use crate::expert_team::ExpertTeamLlmClient;

const TARGET_CHARS_MAX: usize = 8000;
const CHUNK_SIZE_CHARS: usize = 20000;

fn get_system_prompt() -> &'static str {
    "You are an expert summarizer. Compress the following subagent execution result into a dense 1k-2k token summary. Preserve all key decisions, code changes, and unresolved issues. Do not include raw context loops."
}

/// SOTA Harness Patterns (2025-2026): 11. Subagent Orchestration -> Subagents return 1k-2k token condensed summaries, never their full context loop.
/// Recursively condenses a large text into a 1k-2k token summary using an ExpertTeamLlmClient.
pub async fn condense_summary_expert(
    raw_output: &str,
    llm: &(impl ExpertTeamLlmClient + ?Sized),
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
            current_text.chars().take(TARGET_CHARS_MAX).collect::<String>()
        );
    }

    Ok(current_text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChatRequest, ChatResponse, Message, Role, Usage};
    use std::sync::Mutex;

    struct MockCondensationLlm {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl ExpertTeamLlmClient for MockCondensationLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().unwrap();
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "Default mock response".to_string()
            };

            Ok(ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content,
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: Usage::default(),
                response_id: None,
                stop_reason: "stop".to_string(),
            })
        }
    }

    #[tokio::test]
    async fn test_condense_summary_short_text() {
        let llm = MockCondensationLlm {
            responses: Mutex::new(vec!["Condensed short text".to_string()]),
        };
        let input = "This is a short text that doesn't need much condensation but triggers the fallback if length > 1000. Let's make it short though.";

        let result = condense_summary_expert(input, &llm, "test-model").await.unwrap();
        // Since input length is < 1000 and < TARGET_CHARS_MAX, it returns the original text untouched.
        assert_eq!(result, input);
    }

    #[tokio::test]
    async fn test_condense_summary_medium_text() {
        let llm = MockCondensationLlm {
            responses: Mutex::new(vec!["Condensed medium text".to_string()]),
        };
        let input = "A".repeat(1500); // Between 1000 and TARGET_CHARS_MAX

        let result = condense_summary_expert(&input, &llm, "test-model").await.unwrap();
        // It triggers the 1000+ length condensation pass
        assert_eq!(result, "Condensed medium text");
    }

    #[tokio::test]
    async fn test_condense_summary_large_text() {
        // Needs multiple passes
        let llm = MockCondensationLlm {
            responses: Mutex::new(vec![
                "Condensed part 1".to_string(),
                "Condensed part 2".to_string(),
                // Note: The intermediate string length of "Condensed part 1\n\nCondensed part 2"
                // is small (< 8000), so the loop terminates and it returns the joined string
                // UNLESS its length is > 1000 and we want to do a final pass. In our case,
                // "Condensed part 1\n\nCondensed part 2" is < 1000 chars and length != original length.
                // Thus the loop exits and returns it directly.
            ]),
        };

        // Make text length > TARGET_CHARS_MAX (8000), e.g., 25000 chars.
        // It's larger than CHUNK_SIZE_CHARS (20000), so it splits into 2 chunks.
        let input = "A".repeat(25000);

        let result = condense_summary_expert(&input, &llm, "test-model").await.unwrap();
        assert_eq!(result, "Condensed part 1\n\nCondensed part 2");
    }

    #[tokio::test]
    async fn test_condense_summary_failed_condensation_loop() {
        // Mock returns responses that are larger than the input, causing the loop to break
        let llm = MockCondensationLlm {
            responses: Mutex::new(vec![
                "B".repeat(25000)
            ]),
        };

        let input = "A".repeat(9000); // 1 chunk
        let result = condense_summary_expert(&input, &llm, "test-model").await.unwrap();

        // Because the mock returns 25000 chars which is > 9000, it breaks early.
        // And then because it's > TARGET_CHARS_MAX (8000), it gets truncated.
        assert!(result.contains("[Output truncated. Subagent failed to condense summary.]"));
        assert_eq!(result.chars().count(), 8000 + "\n\n[Output truncated. Subagent failed to condense summary.]".chars().count());
    }
}
