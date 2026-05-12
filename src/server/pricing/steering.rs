use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelTier {
    Economy, // e.g., Haiku, gpt-4o-mini
    Standard, // e.g., Sonnet, gpt-4o
    Premium, // e.g., Opus, o1
}

pub struct ModelRouter;

impl ModelRouter {
    pub fn route_task(instruction: &str, budget_remaining: f64) -> ModelTier {
        let instruction_lower = instruction.to_lowercase();

        // Critical logic: if budget is low, force economy
        if budget_remaining < 0.50 {
            return ModelTier::Economy;
        }

        // Complexity indicators
        let complexity_keywords = [
            "architect", "comprehensive", "deep dive", "strategic", "analyze",
            "complex", "legal", "compliance", "mathematical", "solve"
        ];

        let mut complexity_score = 0;
        for kw in complexity_keywords {
            if instruction_lower.contains(kw) {
                complexity_score += 1;
            }
        }

        if instruction_lower.len() > 1000 || complexity_score >= 3 {
            if budget_remaining > 5.0 {
                ModelTier::Premium
            } else {
                ModelTier::Standard
            }
        } else if complexity_score >= 1 || instruction_lower.len() > 300 {
            ModelTier::Standard
        } else {
            ModelTier::Economy
        }
    }

    pub fn get_model_for_tier(tier: ModelTier, provider: &str) -> String {
        match (provider, tier) {
            ("anthropic", ModelTier::Economy) => "claude-3-5-haiku-20241022".to_string(),
            ("anthropic", ModelTier::Standard) => "claude-3-5-sonnet-20241022".to_string(),
            ("anthropic", ModelTier::Premium) => "claude-3-opus-20240229".to_string(),
            ("openai", ModelTier::Economy) => "gpt-4o-mini".to_string(),
            ("openai", ModelTier::Standard) => "gpt-4o".to_string(),
            ("openai", ModelTier::Premium) => "o1-preview".to_string(),
            _ => "default-model".to_string(),
        }
    }
}
