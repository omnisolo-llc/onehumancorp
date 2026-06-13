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
                system: system_prompt.to_string(),
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
            system: system_prompt.to_string(),
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
