#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelTier {
    Economy,
    Standard,
    Premium,
}

pub struct SteeringConfig {
    pub premium_threshold_tokens: usize,
    pub economy_budget_threshold_cents: i64,
}

impl Default for SteeringConfig {
    fn default() -> Self {
        Self {
            premium_threshold_tokens: 2000,
            economy_budget_threshold_cents: 50,
        }
    }
}

pub fn steer_request(prompt: &str, budget_remaining_cents: i64, config: &SteeringConfig) -> ModelTier {
    if budget_remaining_cents < config.economy_budget_threshold_cents {
        return ModelTier::Economy;
    }

    let token_estimate = prompt.split_whitespace().count();

    let complex_keywords = ["analyze", "optimize", "rewrite", "complex", "reasoning", "logic"];
    let is_complex = complex_keywords.iter().any(|&k| prompt.to_lowercase().contains(k));

    if token_estimate > config.premium_threshold_tokens || (is_complex && token_estimate > 500) {
        ModelTier::Premium
    } else if token_estimate < 5 && !is_complex {
        return ModelTier::Economy;
    } else {
        ModelTier::Standard
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steering_logic() {
        let config = SteeringConfig::default();
        assert_eq!(steer_request("Hi", 100, &config), ModelTier::Economy);
        assert_eq!(steer_request("Write a short story about a cat.", 100, &config), ModelTier::Standard);
    }
}
