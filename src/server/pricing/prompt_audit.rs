use std::collections::HashSet;

pub struct PromptAuditResult {
    pub original_tokens: usize,
    pub optimized_tokens: usize,
    pub redundancy_score: f32,
    pub suggestions: Vec<String>,
}

pub fn audit_system_prompt(prompt: &str) -> PromptAuditResult {
    let words: Vec<&str> = prompt.split_whitespace().collect();
    let original_count = words.len();

    let mut seen = HashSet::new();
    let mut duplicates = 0;

    for &word in &words {
        if !seen.insert(word.to_lowercase()) {
            duplicates += 1;
        }
    }

    let redundancy_score = if original_count > 0 {
        duplicates as f32 / original_count as f32
    } else {
        0.0
    };

    let mut suggestions = Vec::new();
    if redundancy_score > 0.20 {
        suggestions.push("High word repetition detected.".to_string());
    }

    if original_count > 1000 {
        suggestions.push("Prompt is very long.".to_string());
    }

    PromptAuditResult {
        original_tokens: original_count,
        optimized_tokens: original_count - duplicates,
        redundancy_score,
        suggestions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_logic() {
        let prompt = "Be helpful. Be helpful. Be very helpful.";
        let result = audit_system_prompt(prompt);
        assert!(result.redundancy_score > 0.0);

        let long_repetitive = "repeat ".repeat(1100);
        let result = audit_system_prompt(&long_repetitive);
        assert!(!result.suggestions.is_empty());
    }
}
