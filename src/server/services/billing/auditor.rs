use std::sync::Mutex;
use std::collections::HashMap;

pub struct CostAuditor {
    tenant_costs: Mutex<HashMap<String, u64>>,
    tenant_savings: Mutex<HashMap<String, u64>>,
    tenant_revenue: Mutex<HashMap<String, u64>>,
}

impl CostAuditor {
    pub fn new() -> Self {
        Self {
            tenant_costs: Mutex::new(HashMap::new()),
            tenant_savings: Mutex::new(HashMap::new()),
            tenant_revenue: Mutex::new(HashMap::new()),
        }
    }

    pub fn record_cost(&self, tenant_id: &str, cents: u64) {
        let mut costs = self.tenant_costs.lock().unwrap();
        *costs.entry(tenant_id.to_string()).or_insert(0) += cents;
    }

    pub fn record_savings(&self, tenant_id: &str, cents: u64) {
        let mut savings = self.tenant_savings.lock().unwrap();
        *savings.entry(tenant_id.to_string()).or_insert(0) += cents;
    }

    pub fn record_revenue(&self, tenant_id: &str, cents: u64) {
        let mut revenue = self.tenant_revenue.lock().unwrap();
        *revenue.entry(tenant_id.to_string()).or_insert(0) += cents;
    }

    pub fn get_total_cost(&self, tenant_id: &str) -> u64 {
        let costs = self.tenant_costs.lock().unwrap();
        *costs.get(tenant_id).unwrap_or(&0)
    }

    pub fn get_total_savings(&self, tenant_id: &str) -> u64 {
        let savings = self.tenant_savings.lock().unwrap();
        *savings.get(tenant_id).unwrap_or(&0)
    }

    pub fn get_total_revenue(&self, tenant_id: &str) -> u64 {
        let revenue = self.tenant_revenue.lock().unwrap();
        *revenue.get(tenant_id).unwrap_or(&0)
    }
}
