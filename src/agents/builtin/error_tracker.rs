use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tracks errors for tool calls to enforce the "exactly 2 retries" compounding error prevention mechanic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorTracker {
    pub counts: HashMap<String, u64>,
    pub max_retries: u64,
}

impl ErrorTracker {
    pub fn new(max_retries: u64) -> Self {
        Self {
            counts: HashMap::new(),
            max_retries,
        }
    }

    /// Records a success, resetting the count for the tool.
    pub fn record_success(&mut self, tool_name: &str) {
        self.counts.insert(tool_name.to_string(), 0);
    }

    /// Records an error. Returns true if the max retries limit is exceeded.
    pub fn record_error(&mut self, tool_name: &str) -> bool {
        let count = self.counts.entry(tool_name.to_string()).or_insert(0);
        *count += 1;
        *count > self.max_retries
    }

    pub fn get_count(&self, tool_name: &str) -> u64 {
        *self.counts.get(tool_name).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_tracker() {
        let mut tracker = ErrorTracker::new(2);

        assert_eq!(tracker.record_error("tool1"), false); // count 1
        assert_eq!(tracker.record_error("tool1"), false); // count 2
        assert_eq!(tracker.record_error("tool1"), true);  // count 3 -> limit exceeded

        tracker.record_success("tool1");
        assert_eq!(tracker.record_error("tool1"), false); // count 1
    }
}
