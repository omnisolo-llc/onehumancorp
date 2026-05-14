pub struct ContextMessage {
    pub role: String,
    pub content: String,
    pub tokens: usize,
}

pub struct ContextWindow {
    pub messages: Vec<ContextMessage>,
    pub max_tokens: usize,
    pub current_tokens: usize,
}

impl ContextWindow {
    pub fn new(max_tokens: usize) -> Self {
        Self {
            messages: Vec::new(),
            max_tokens,
            current_tokens: 0,
        }
    }

    pub fn add_message(&mut self, role: String, content: String) {
        let tokens = content.split_whitespace().count();
        self.current_tokens += tokens;
        self.messages.push(ContextMessage { role, content, tokens });

        self.prune();
    }

    pub fn prune(&mut self) {
        let has_system = self.messages.get(0).map(|m| m.role == "system").unwrap_or(false);
        let start_index = if has_system { 1 } else { 0 };

        while self.current_tokens > self.max_tokens && self.messages.len() > (start_index + 1) {
            let removed = self.messages.remove(start_index);
            self.current_tokens -= removed.tokens;
        }
    }

    pub fn get_formatted_context(&self) -> String {
        self.messages.iter()
            .map(|m| format!("{}: {}", m.role, m.content))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_pruning() {
        let mut window = ContextWindow::new(10);
        window.add_message("system".to_string(), "sys prompt".to_string()); // 2 tokens

        window.add_message("user".to_string(), "msg 1 2 3 4 5".to_string()); // 6 tokens
        window.add_message("user".to_string(), "msg 6 7 8 9 0".to_string()); // 6 tokens

        // Total 14 tokens > 10 limit. Should prune "msg 1 2 3 4 5"
        assert_eq!(window.messages[0].role, "system");
        assert!(!window.get_formatted_context().contains("1 2 3 4 5"));
        assert!(window.get_formatted_context().contains("6 7 8 9 0"));
    }
}
