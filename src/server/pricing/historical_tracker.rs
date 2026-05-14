use std::collections::HashMap;
use std::sync::Mutex;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageEntry {
    pub timestamp: DateTime<Utc>,
    pub tokens: usize,
    pub cost_cents: i64,
    pub model: String,
}

pub struct HistoricalTracker {
    data: Mutex<HashMap<String, Vec<UsageEntry>>>,
}

impl HistoricalTracker {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }

    pub fn record_usage(&self, tenant_id: &str, tokens: usize, cost_cents: i64, model: &str) {
        let mut data = self.data.lock().unwrap();
        let entries = data.entry(tenant_id.to_string()).or_insert_with(Vec::new);
        entries.push(UsageEntry {
            timestamp: Utc::now(),
            tokens,
            cost_cents,
            model: model.to_string(),
        });

        // Keep only last 1000 entries per tenant to avoid memory leak
        if entries.len() > 1000 {
            entries.remove(0);
        }
    }

    pub fn get_history(&self, tenant_id: &str) -> Vec<UsageEntry> {
        let data = self.data.lock().unwrap();
        data.get(tenant_id).cloned().unwrap_or_default()
    }

    pub fn calculate_daily_burn_rate(&self, tenant_id: &str) -> i64 {
        let history = self.get_history(tenant_id);
        if history.is_empty() {
            return 0;
        }

        let now = Utc::now();
        let hours_ago_24 = now - chrono::Duration::hours(24);

        history.iter()
            .filter(|e| e.timestamp > hours_ago_24)
            .map(|e| e.cost_cents)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_historical_tracking() {
        let tracker = HistoricalTracker::new();
        tracker.record_usage("t1", 100, 10, "gpt-4");

        let history = tracker.get_history("t1");
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].cost_cents, 10);
    }
}
