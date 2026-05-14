use ::server_ohc::orchestration::Message;

pub struct ContextManager {
    max_tokens_budget: usize,
}

impl ContextManager {
    pub fn new(max_tokens_budget: usize) -> Self {
        Self { max_tokens_budget }
    }

    /// Prunes a list of messages based on signal-to-noise ratio.
    /// Preserves system messages and most recent messages.
    pub fn prune_messages(&self, messages: Vec<Message>) -> Vec<Message> {
        if messages.len() <= 5 {
            return messages;
        }

        let mut preserved = Vec::new();
        let total = messages.len();

        for (i, msg) in messages.into_iter().enumerate() {
            // Preserve system messages
            if msg.from_agent == "SYSTEM" || msg.r#type == "status" {
                preserved.push(msg);
                continue;
            }

            // Preserve last 3 messages always
            if i >= total - 3 {
                preserved.push(msg);
                continue;
            }

            // Heuristic: discard short messages that likely contain low signal (pleasantries)
            if msg.content.split_whitespace().count() < 4 {
                continue;
            }

            preserved.push(msg);
        }

        preserved
    }

    /// Summarizes content to reduce token usage.
    pub fn summarize_content(&self, content: &str) -> String {
        // Use pricing compression utilities
        crate::compression::reduce_tokens(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_prune_messages() {
        let cm = ContextManager::new(1000);
        let messages = vec![
            Message { id: "1".into(), from_agent: "SYSTEM".into(), to_agent: "all".into(), r#type: "status".into(), content: "Start".into(), occurred_at_unix: Utc::now().timestamp(), meeting_id: "m1".into() },
            Message { id: "2".into(), from_agent: "A".into(), to_agent: "B".into(), r#type: "chat".into(), content: "Hi".into(), occurred_at_unix: Utc::now().timestamp(), meeting_id: "m1".into() },
            Message { id: "3".into(), from_agent: "B".into(), to_agent: "A".into(), r#type: "chat".into(), content: "Hello there friend".into(), occurred_at_unix: Utc::now().timestamp(), meeting_id: "m1".into() },
            Message { id: "4".into(), from_agent: "A".into(), to_agent: "B".into(), r#type: "chat".into(), content: "Thanks".into(), occurred_at_unix: Utc::now().timestamp(), meeting_id: "m1".into() },
            Message { id: "5".into(), from_agent: "B".into(), to_agent: "A".into(), r#type: "chat".into(), content: "Important data here".into(), occurred_at_unix: Utc::now().timestamp(), meeting_id: "m1".into() },
            Message { id: "6".into(), from_agent: "A".into(), to_agent: "B".into(), r#type: "chat".into(), content: "Last 1".into(), occurred_at_unix: Utc::now().timestamp(), meeting_id: "m1".into() },
            Message { id: "7".into(), from_agent: "B".into(), to_agent: "A".into(), r#type: "chat".into(), content: "Last 2".into(), occurred_at_unix: Utc::now().timestamp(), meeting_id: "m1".into() },
            Message { id: "8".into(), from_agent: "A".into(), to_agent: "B".into(), r#type: "chat".into(), content: "Last 3".into(), occurred_at_unix: Utc::now().timestamp(), meeting_id: "m1".into() },
        ];

        let pruned = cm.prune_messages(messages);

        let ids: Vec<String> = pruned.iter().map(|m| m.id.clone()).collect();
        assert!(ids.contains(&"1".to_string()));
        assert!(!ids.contains(&"2".to_string()));
        assert!(ids.contains(&"3".to_string()));
        assert!(!ids.contains(&"4".to_string()));
        assert!(ids.contains(&"6".to_string()));
        assert!(ids.contains(&"7".to_string()));
        assert!(ids.contains(&"8".to_string()));
    }
}
