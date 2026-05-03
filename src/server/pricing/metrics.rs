use opentelemetry::{global, KeyValue};
use opentelemetry::metrics::Counter;

#[derive(Clone, Debug)]
pub struct PricingMetrics {
    pub llm_tokens_total: Counter<u64>,
    pub storage_bytes_total: Counter<u64>,
    pub emails_sent_total: Counter<u64>,
    pub outbound_api_calls_total: Counter<u64>,
}

impl Default for PricingMetrics {
    fn default() -> Self { Self::new() }
}

impl PricingMetrics {
    pub fn new() -> Self {
        let meter = global::meter("ohc.pricing");
        PricingMetrics {
            llm_tokens_total: meter.u64_counter("ohc_token_usage_total").build(),
            storage_bytes_total: meter.u64_counter("ohc_storage_bytes_total").build(),
            emails_sent_total: meter.u64_counter("ohc_emails_sent_total").build(),
            outbound_api_calls_total: meter.u64_counter("ohc_outbound_api_calls_total").build(),
        }
    }

    pub fn record_llm_tokens(&self, tenant_id: &str, model: &str, tokens: u64) {
        self.llm_tokens_total.add(tokens, &[KeyValue::new("organization_id", tenant_id.to_string()), KeyValue::new("model", model.to_string())]);
    }

    pub fn record_storage_bytes(&self, tenant_id: &str, bytes: u64) {
        self.storage_bytes_total.add(bytes, &[KeyValue::new("organization_id", tenant_id.to_string())]);
    }

    pub fn record_email_sent(&self, tenant_id: &str) {
        self.emails_sent_total.add(1, &[KeyValue::new("organization_id", tenant_id.to_string())]);
    }

    pub fn record_outbound_api_call(&self, tenant_id: &str, api_name: &str) {
        self.outbound_api_calls_total.add(1, &[KeyValue::new("organization_id", tenant_id.to_string()), KeyValue::new("api_name", api_name.to_string())]);
    }
}
