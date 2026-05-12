use std::collections::HashSet;
use crate::compression::reduce_tokens;

pub struct PromptAuditResult {
    pub original_tokens_estimate: usize,
    pub optimized_tokens_estimate: usize,
    pub redundancy_score: f64, // 0.0 to 1.0
    pub optimization_tips: Vec<String>,
}

pub struct PromptAuditor;

impl PromptAuditor {
    pub fn audit_system_prompt(prompt: &str) -> PromptAuditResult {
        let original_words: Vec<&str> = prompt.split_whitespace().collect();
        let original_count = original_words.len();

        let optimized_prompt = reduce_tokens(prompt);
        let optimized_words: Vec<&str> = optimized_prompt.split_whitespace().collect();
        let optimized_count = optimized_words.len();

        let mut tips = Vec::new();
        let mut redundancy_score = 0.0;

        if original_count > 0 {
            redundancy_score = (original_count - optimized_count) as f64 / original_count as f64;
        }

        if redundancy_score > 0.2 {
            tips.push(format!("Your system prompt is {:.1}% redundant. Use denser language to save tokens.", redundancy_score * 100.0));
        }

        // Detect common patterns like repeated instructions or excessive pleasantries
        let mut seen_words = HashSet::new();
        let mut duplicate_words_count = 0;
        for word in &original_words {
            let low = word.to_lowercase();
            if seen_words.contains(&low) && low.len() > 4 {
                duplicate_words_count += 1;
            }
            seen_words.insert(low);
        }

        if duplicate_words_count > 5 {
            tips.push("Detected high frequency of repeated long words. Consider merging similar instructions.".to_string());
        }

        if prompt.contains("Please") || prompt.contains("Thank you") || prompt.contains("I would like") {
             tips.push("Remove conversational filler (pleasantries) from system instructions to reduce costs.".to_string());
        }

        PromptAuditResult {
            original_tokens_estimate: original_count,
            optimized_tokens_estimate: optimized_count,
            redundancy_score,
            optimization_tips: tips,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_system_prompt() {
        let prompt = "You are a helpful assistant. Please be very kind and help the user with anything they need. I would like you to be precise. You are a helpful assistant.";
        let result = PromptAuditor::audit_system_prompt(prompt);

        assert!(result.redundancy_score > 0.0);
        assert!(result.optimization_tips.len() > 0);
        assert!(result.optimization_tips.iter().any(|t| t.contains("conversational filler")));
    }
}
