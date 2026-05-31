use std::sync::Arc;
use prometheus::{Counter, Registry, Encoder, TextEncoder};

pub struct CostMetrics {
    pub llm_cost_total: Counter,
    pub storage_rw_cost_total: Counter,
    pub outbound_api_cost_total: Counter,
    pub email_send_cost_total: Counter,
    pub registry: Registry,
}

impl CostMetrics {
    pub fn new() -> Self {
        let registry = Registry::new();
        let llm_cost_total = Counter::new("ohc_llm_call_cost_total", "LLM Call Cost by Tenant").unwrap();
        let storage_rw_cost_total = Counter::new("ohc_storage_rw_cost_total", "Storage R/W Cost by Tenant").unwrap();
        let outbound_api_cost_total = Counter::new("ohc_outbound_api_cost_total", "Outbound API Cost by Tenant").unwrap();
        let email_send_cost_total = Counter::new("ohc_email_send_cost_total", "Email Send Cost by Tenant").unwrap();

        registry.register(Box::new(llm_cost_total.clone())).unwrap();
        registry.register(Box::new(storage_rw_cost_total.clone())).unwrap();
        registry.register(Box::new(outbound_api_cost_total.clone())).unwrap();
        registry.register(Box::new(email_send_cost_total.clone())).unwrap();

        Self {
            llm_cost_total,
            storage_rw_cost_total,
            outbound_api_cost_total,
            email_send_cost_total,
            registry,
        }
    }

    pub fn gather(&self) -> String {
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
}
