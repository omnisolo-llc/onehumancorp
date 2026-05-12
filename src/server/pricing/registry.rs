use std::sync::Mutex;
use dashmap::DashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecord {
    pub id: String,
    pub tenant_id: String,
    pub optimization_type: String,
    pub impact_usd: f64,
    pub timestamp: DateTime<Utc>,
}

pub struct MiserRegistry {
    records: DashMap<String, Vec<OptimizationRecord>>,
    total_saved: Mutex<f64>,
}

impl MiserRegistry {
    pub fn new() -> Self {
        Self {
            records: DashMap::new(),
            total_saved: Mutex::new(0.0),
        }
    }

    pub fn record_optimization(&self, tenant_id: &str, opt_type: &str, savings: f64) {
        let record = OptimizationRecord {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            optimization_type: opt_type.to_string(),
            impact_usd: savings,
            timestamp: Utc::now(),
        };

        self.records.entry(tenant_id.to_string()).or_insert_with(Vec::new).push(record);
        let mut total = self.total_saved.lock().unwrap();
        *total += savings;
    }

    pub fn get_tenant_savings(&self, tenant_id: &str) -> f64 {
        self.records.get(tenant_id)
            .map(|r| r.iter().map(|rec| rec.impact_usd).sum())
            .unwrap_or(0.0)
    }

    pub fn get_all_time_savings(&self) -> f64 {
        *self.total_saved.lock().unwrap()
    }

    pub fn generate_savings_report(&self, tenant_id: &str) -> String {
        let savings = self.get_tenant_savings(tenant_id);
        if savings <= 0.0 {
            return "No optimizations applied yet. Let's start saving!".to_string();
        }

        format!("Miser Impact: You've saved a total of ${:.2} through automated optimizations on the OneHumanCorp platform.", savings)
    }
}

use once_cell::sync::Lazy;

pub static GLOBAL_REGISTRY: Lazy<MiserRegistry> = Lazy::new(MiserRegistry::new);
