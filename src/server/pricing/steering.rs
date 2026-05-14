pub enum ModelTier {
    Economy,
    Standard,
    Premium,
}

pub struct Steering;

impl Steering {
    pub fn route_task(complexity: u32) -> ModelTier {
        if complexity < 3 {
            ModelTier::Economy
        } else if complexity < 7 {
            ModelTier::Standard
        } else {
            ModelTier::Premium
        }
    }
}
