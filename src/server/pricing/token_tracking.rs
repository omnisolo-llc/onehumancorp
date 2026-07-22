use opentelemetry::metrics::Counter;
use opentelemetry::KeyValue;

/// TokenTracking provides robust backend infrastructure for LLM token efficiency tracking.
pub struct TokenTracking {
    pub token_usage_total: Counter<u64>,
    pub token_cost_cents_total: Counter<u64>,
}

impl TokenTracking {
    pub fn new(meter: &opentelemetry::metrics::Meter) -> Self {
        Self {
            token_usage_total: meter.u64_counter("token_usage_total")
                .with_description("Total tokens consumed by LLMs")
                .build(),
            token_cost_cents_total: meter.u64_counter("token_cost_cents_total")
                .with_description("Total cost of tokens consumed by LLMs in cents")
                .build(),
        }
    }

    /// Records token tracking metrics.
    pub fn record_tokens(&self, tenant_id: &str, model: &str, tokens: u64, cost_cents: u64) {
        self.token_usage_total.add(tokens, &[
            KeyValue::new("tenant_id", tenant_id.to_string()),
            KeyValue::new("model", model.to_string()),
        ]);
        self.token_cost_cents_total.add(cost_cents, &[
            KeyValue::new("tenant_id", tenant_id.to_string()),
            KeyValue::new("model", model.to_string()),
        ]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opentelemetry::global;

    #[test]
    fn test_token_tracking_initialization() {
        let meter = global::meter("test_metrics");
        let tracker = TokenTracking::new(&meter);
        tracker.record_tokens("tenant-123", "gpt-4o", 100, 2);
    }
}
