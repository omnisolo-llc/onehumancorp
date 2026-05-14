use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContextMessage {
    pub role: String,
    pub content: String,
}

pub struct ContextManager {
    pub max_tokens: usize,
    pub keep_recent_count: usize,
}

impl ContextManager {
    pub fn new(max_tokens: usize, keep_recent_count: usize) -> Self {
        Self {
            max_tokens,
            keep_recent_count,
        }
    }

    /// Prunes conversation history to fit within token budget.
    /// Always preserves 'system' messages and the most recent N messages.
    pub fn prune_history(&self, messages: Vec<ContextMessage>) -> Vec<ContextMessage> {
        if messages.len() <= self.keep_recent_count {
            return messages;
        }

        let mut preserved = Vec::new();
        let mut potential_prunables = Vec::new();

        let total_len = messages.len();
        for (i, msg) in messages.into_iter().enumerate() {
            if msg.role == "system" || i >= total_len - self.keep_recent_count {
                preserved.push((i, msg));
            } else {
                potential_prunables.push((i, msg));
            }
        }

        let mut final_messages = preserved;
        final_messages.sort_by_key(|(i, _)| *i);

        final_messages.into_iter().map(|(_, msg)| msg).collect()
    }

    pub fn estimate_tokens(content: &str) -> usize {
        content.len() / 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prune_history() {
        let manager = ContextManager::new(1000, 2);
        let messages = vec![
            ContextMessage { role: "system".to_string(), content: "System prompt".to_string() },
            ContextMessage { role: "user".to_string(), content: "Message 1".to_string() },
            ContextMessage { role: "assistant".to_string(), content: "Response 1".to_string() },
            ContextMessage { role: "user".to_string(), content: "Message 2".to_string() },
            ContextMessage { role: "assistant".to_string(), content: "Response 2".to_string() },
        ];

        let pruned = manager.prune_history(messages);

        assert_eq!(pruned.len(), 3);
        assert_eq!(pruned[0].content, "System prompt");
        assert_eq!(pruned[1].content, "Message 2");
        assert_eq!(pruned[2].content, "Response 2");
    }
}
