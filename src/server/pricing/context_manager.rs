pub struct ContextManager;

impl ContextManager {
    pub fn prune_context(messages: Vec<String>, token_limit: usize) -> Vec<String> {
        let mut current_tokens = 0;
        let mut result = Vec::new();

        for msg in messages.into_iter().rev() {
            let tokens = msg.split_whitespace().count();
            if current_tokens + tokens <= token_limit {
                current_tokens += tokens;
                result.push(msg);
            } else {
                break;
            }
        }

        result.reverse();
        result
    }
}
