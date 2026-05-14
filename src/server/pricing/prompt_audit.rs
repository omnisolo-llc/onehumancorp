use serde_json::{json, Value};
use std::collections::HashMap;

pub struct PromptAuditor;

impl PromptAuditor {
    pub fn audit_system_prompt(prompt: &str) -> Vec<String> {
        let mut tips = Vec::new();

        if prompt.len() > 2000 {
            tips.push("System prompt is very long. Consider pruning redundant instructions to save tokens.".to_string());
        }

        let fluff = vec!["please", "thank you", "if you would be so kind", "generously"];
        for word in fluff {
            if prompt.to_lowercase().contains(word) {
                tips.push(format!("Found conversational fluff: '{}'. Removing this can improve efficiency.", word));
            }
        }

        if !prompt.contains("JSON") && !prompt.contains("format") {
             tips.push("Consider specifying an output format (like JSON) for more consistent agent responses.".to_string());
        }

        tips
    }

    pub fn generate_dashboard_metrics(prompts: HashMap<String, String>) -> Value {
        let mut total_tokens = 0;
        let mut optimizations = 0;

        for (_, p) in prompts {
            total_tokens += p.split_whitespace().count();
            if p.to_lowercase().contains("please") {
                optimizations += 1;
            }
        }

        json!({
            "total_system_tokens": total_tokens,
            "potential_optimizations": optimizations,
            "efficiency_score": 100 - (optimizations * 5).min(100)
        })
    }
}
