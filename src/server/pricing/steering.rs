#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelTier {
    Economy,
    Standard,
    Premium,
}

pub struct CostSteerer;

impl CostSteerer {
    pub fn steer(instruction: &str, remaining_budget_cents: i64) -> ModelTier {
        let complexity_score = Self::analyze_complexity(instruction);

        if remaining_budget_cents < 50 {
            return ModelTier::Economy;
        }

        if complexity_score > 8 && remaining_budget_cents > 500 {
            ModelTier::Premium
        } else if complexity_score > 4 && remaining_budget_cents > 200 {
            ModelTier::Standard
        } else {
            ModelTier::Economy
        }
    }

    fn analyze_complexity(instruction: &str) -> usize {
        let mut score = 0;
        score += instruction.len() / 200;
        let complex_keywords = ["analyze", "orchestrate", "plan", "complex", "strategy", "reason", "refactor"];
        for word in complex_keywords {
            if instruction.to_lowercase().contains(word) {
                score += 2;
            }
        }
        if instruction.contains("```") || instruction.contains("fn ") || instruction.contains("class ") {
            score += 3;
        }
        score
    }

    pub fn get_model_for_tier(tier: ModelTier) -> &'static str {
        match tier {
            ModelTier::Economy => "gpt-4o-mini",
            ModelTier::Standard => "gpt-4o",
            ModelTier::Premium => "o1",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steer_economy() {
        let tier = CostSteerer::steer("Say hello", 1000);
        assert_eq!(tier, ModelTier::Economy);
    }
}
