use std::collections::HashMap;
use std::sync::RwLock;
use sha2::{Sha256, Digest};

pub struct Experiment {
    pub id: String,
    pub title: String,
    pub traffic_split: f64,
}

pub struct ExperimentManager {
    experiments: RwLock<HashMap<String, Experiment>>,
}

impl ExperimentManager {
    pub fn new() -> Self {
        ExperimentManager {
            experiments: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_experiment(&self, id: &str, title: &str, split: f64) {
        let mut experiments = self.experiments.write().unwrap();
        experiments.insert(id.to_string(), Experiment {
            id: id.to_string(),
            title: title.to_string(),
            traffic_split: split,
        });
    }

    pub fn get_variant(&self, id: &str, user_id: &str) -> String {
        let experiments = self.experiments.read().unwrap();
        let exp = match experiments.get(id) {
            Some(e) => e,
            None => return "control".to_string(),
        };

        let mut hasher = Sha256::new();
        hasher.update(id.as_bytes());
        hasher.update(user_id.as_bytes());
        let hash = hasher.finalize();

        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&hash[..8]);
        let val = u64::from_be_bytes(bytes) as f64 / (u64::MAX as f64);

        if val < exp.traffic_split {
            "treatment".to_string()
        } else {
            "control".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_experiment_manager() {
        let em = ExperimentManager::new();
        em.add_experiment("exp1", "Test", 1.0);

        let variant = em.get_variant("exp1", "user1");
        assert_eq!(variant, "treatment");

        em.add_experiment("exp2", "Test2", 0.0);
        let variant = em.get_variant("exp2", "user1");
        assert_eq!(variant, "control");

        em.add_experiment("exp3", "Test3", 0.5);

        let var1 = em.get_variant("exp3", "user1");
        let var2 = em.get_variant("exp3", "user1");

        assert_eq!(var1, var2); // Deterministic
    }
}


// Functional extensions for experiments

pub struct ExperimentVariant_0 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_0 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_1 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_1 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_2 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_2 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_3 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_3 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_4 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_4 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_5 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_5 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_6 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_6 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_7 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_7 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_8 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_8 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_9 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_9 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_10 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_10 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_11 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_11 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_12 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_12 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_13 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_13 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_14 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_14 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_15 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_15 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_16 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_16 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_17 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_17 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_18 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_18 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_19 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_19 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_20 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_20 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_21 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_21 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_22 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_22 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_23 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_23 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}

pub struct ExperimentVariant_24 {
    pub variant_id: String,
    pub traffic_allocation: f64,
    pub conversion_rate: f64,
    pub statistical_significance: f64,
}

impl ExperimentVariant_24 {
    pub fn new(id: &str, traffic: f64) -> Self {
        Self {
            variant_id: id.to_string(),
            traffic_allocation: traffic,
            conversion_rate: 0.0,
            statistical_significance: 0.0,
        }
    }

    pub fn update_metrics(&mut self, conversions: i32, visitors: i32) {
        if visitors > 0 {
            self.conversion_rate = conversions as f64 / visitors as f64;
        }
    }

    pub fn calculate_significance(&mut self, control_rate: f64) {
        if control_rate > 0.0 {
            self.statistical_significance = (self.conversion_rate - control_rate) / control_rate;
        }
    }

    pub fn is_winner(&self, threshold: f64) -> bool {
        self.statistical_significance >= threshold
    }
}
