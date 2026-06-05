use crate::types::Message;

pub struct PromptBuilder;

impl PromptBuilder {
    pub fn apply_lost_in_the_middle_prevention(
        final_messages: &mut Vec<Message>,
        enable: bool,
        developer_instructions: &str,
        user_instructions: &str,
    ) {
        if enable {
            // If it's a long conversation (>3 messages), re-inject the core user instructions
            // and developer instructions at the very end to prevent context rot.
            if final_messages.len() > 3 {
                // Truncate user instructions safely to a reasonable length (e.g. 1000 bytes or first paragraph)
                let core_objective = Self::extract_core_objective(user_instructions);

                // Summarize recent tool errors if any exist near the end of the context
                let recent_errors = Self::summarize_recent_tool_errors(final_messages);

                let reminder = format!(
                    "[System Reminder to combat 'Lost in the Middle' effect: Remember your core objective: {}]{}",
                    core_objective,
                    if recent_errors.is_empty() { String::new() } else { format!(" | Recent Tool Errors: {}", recent_errors) }
                );

                final_messages.push(Message::user(reminder));
                if !developer_instructions.is_empty() {
                    final_messages.push(Message::user(format!("[System Reminder (Developer Instructions): {}]", developer_instructions)));
                }
            } else if !developer_instructions.is_empty() {
                final_messages.push(Message::user(format!("[System Reminder (Developer Instructions): {}]", developer_instructions)));
            }
        } else if !developer_instructions.is_empty() {
            final_messages.push(Message::user(format!("[System Reminder (Developer Instructions): {}]", developer_instructions)));
        }
    }

    fn extract_core_objective(user_instructions: &str) -> String {
        let first_paragraph = user_instructions.split("\n\n").next().unwrap_or(user_instructions);
        if first_paragraph.len() > 1000 {
            format!("{}...", &first_paragraph[..997])
        } else {
            first_paragraph.to_string()
        }
    }

    fn summarize_recent_tool_errors(messages: &[Message]) -> String {
        let mut error_summary = String::new();
        // Look at the last 5 messages for tool errors
        for msg in messages.iter().rev().take(5) {
            if msg.role == crate::types::Role::User && msg.content.to_lowercase().contains("error") {
                if !error_summary.is_empty() {
                    error_summary.push_str(", ");
                }
                let err_msg = if msg.content.len() > 100 {
                    format!("{}...", &msg.content[..97])
                } else {
                    msg.content.clone()
                };
                error_summary.push_str(&err_msg);
            }
        }
        error_summary
    }
}
