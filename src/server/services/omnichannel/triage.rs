// LLM triage integration
pub fn generate_draft_reply(message_content: &str) -> String {
    println!("Generating draft reply for: {}", message_content);
    format!("Draft reply to: {}", message_content)
}
