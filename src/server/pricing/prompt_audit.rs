pub struct PromptAuditResult {
    pub fluff_count: usize,
    pub markdown_complexity: usize,
    pub optimization_score: f32,
    pub recommendations: Vec<String>,
}

pub struct PromptAuditor;

impl PromptAuditor {
    pub fn audit(prompt: &str) -> PromptAuditResult {
        let fluff_words = ["please", "kindly", "I would appreciate if", "thank you", "hope you are well"];
        let mut fluff_count = 0;
        for word in fluff_words {
            if prompt.to_lowercase().contains(word) {
                fluff_count += 1;
            }
        }

        let markdown_indicators = ["#", "**", "__", "```", "|", "---"];
        let mut markdown_complexity = 0;
        for indicator in markdown_indicators {
            markdown_complexity += prompt.matches(indicator).count();
        }

        let mut recommendations = Vec::new();
        if fluff_count > 0 {
            recommendations.push("Remove conversational filler to save tokens.".to_string());
        }
        if markdown_complexity > 20 {
            recommendations.push("Simplify Markdown structure to improve LLM parsing speed.".to_string());
        }

        let base_score = 1.0;
        let fluff_penalty = (fluff_count as f32 * 0.1).min(0.5);
        let complexity_penalty = (markdown_complexity as f32 * 0.01).min(0.3);
        let optimization_score = (base_score - fluff_penalty - complexity_penalty).max(0.0);

        PromptAuditResult {
            fluff_count,
            markdown_complexity,
            optimization_score,
            recommendations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_auditor() {
        let prompt = "Please kindly do this task. Thank you! # Heading\n**Bold**\n```code```";
        let result = PromptAuditor::audit(prompt);
        assert!(result.fluff_count >= 3);
        assert!(result.markdown_complexity >= 3);
        assert!(result.optimization_score < 1.0);
        assert!(!result.recommendations.is_empty());
    }
}
