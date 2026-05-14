pub enum ModelTier {
    Economy,
    Standard,
    Premium,
}

pub struct Steering {
    tier_config: std::collections::HashMap<String, ModelTier>,
}

impl Steering {
    pub fn new() -> Self {
        Self {
            tier_config: std::collections::HashMap::new(),
        }
    }

    pub fn steer(&self, complexity_score: f32) -> ModelTier {
        if complexity_score < 0.3 {
            ModelTier::Economy
        } else if complexity_score < 0.7 {
            ModelTier::Standard
        } else {
            ModelTier::Premium
        }
    }

    pub fn calculate_complexity(&self, prompt: &str) -> f32 {
        let word_count = prompt.split_whitespace().count();
        if word_count < 10 {
            0.1
        } else if word_count < 50 {
            0.4
        } else {
            0.8
        }
    }
}
