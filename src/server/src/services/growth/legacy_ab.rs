use crate::analytics::Tracker;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ABTestService {
    tracker: Arc<Tracker>,
}

impl ABTestService {
    pub fn new(tracker: Arc<Tracker>) -> Self {
        ABTestService { tracker }
    }

    pub fn record_impression(&self, experiment_id: &str, variant: &str) -> Result<(), String> {
        if experiment_id.is_empty() || variant.is_empty() {
            return Err("invalid experiment parameters".to_string());
        }

        let mut props = HashMap::new();
        props.insert("experiment_id".to_string(), experiment_id.to_string());
        props.insert("variant".to_string(), variant.to_string());

        self.tracker.track_event("ab_test_impression", props);
        Ok(())
    }

    pub fn record_conversion(&self, experiment_id: &str, variant: &str) -> Result<(), String> {
        if experiment_id.is_empty() || variant.is_empty() {
            return Err("invalid experiment parameters".to_string());
        }

        let mut props = HashMap::new();
        props.insert("experiment_id".to_string(), experiment_id.to_string());
        props.insert("variant".to_string(), variant.to_string());

        self.tracker.track_event("ab_test_conversion", props);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analytics::Tracker;

    #[test]
    fn test_record_impression() {
        let tracker = Arc::new(Tracker::new());
        let service = ABTestService::new(tracker);

        let res = service.record_impression("exp-123", "variant-a");
        assert!(res.is_ok());

        let res = service.record_impression("", "");
        assert!(res.is_err());
    }

    #[test]
    fn test_record_conversion() {
        let tracker = Arc::new(Tracker::new());
        let service = ABTestService::new(tracker);

        let res = service.record_conversion("exp-123", "variant-a");
        assert!(res.is_ok());

        let res = service.record_conversion("", "");
        assert!(res.is_err());
    }
}
