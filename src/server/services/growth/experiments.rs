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

}
