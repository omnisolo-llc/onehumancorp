pub struct PromptAuditor;

pub struct AuditReport {
    pub total_tokens: usize,
    pub redundancy_score: f64,
    pub optimization_tips: Vec<String>,
}

impl PromptAuditor {
    pub fn audit(prompt: &str) -> AuditReport {
        let total_tokens = prompt.len() / 4;
        let mut tips = Vec::new();

        let fluff_words = ["please", "highly", "actually", "basically", "literally", "very", "really"];
        let mut fluff_count = 0;
        for word in fluff_words {
            if prompt.to_lowercase().contains(word) {
                fluff_count += 1;
            }
        }

        if fluff_count > 2 {
            tips.push("Consider removing conversational filler (e.g., 'please', 'actually') to save tokens.".to_string());
        }

        if prompt.len() > 2000 {
            tips.push("System prompt is very long. Consider summarizing context.".to_string());
        }

        if prompt.contains("\n\n\n") {
            tips.push("Excessive whitespace detected. Minification could save tokens.".to_string());
        }

        let redundancy_score = (fluff_count as f64 * 0.1).min(1.0);

        AuditReport {
            total_tokens,
            redundancy_score,
            optimization_tips: tips,
        }
    }
}
