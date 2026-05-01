// Billing module stub - provides Tracker struct used by hub.rs
pub use crate::services::billing::auditor::CostAuditor;
use crate::pricing::rate_limit::{RedisRateLimiter, RateLimitStatus};
use redis::Client;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct UiAgentCost {
    pub name: String,
    pub cost: String,
    pub roi: String,
    pub efficiency: String,
    pub pct: f32,
}

#[derive(Clone)]
pub struct Tracker {
    rate_limiter: Option<Arc<RedisRateLimiter>>,
    auditor: Arc<CostAuditor>,
}

impl Tracker {
    pub fn new() -> Self {
        Tracker {
            rate_limiter: None,
            auditor: Arc::new(CostAuditor::new(crate::pricing::calculator::CostConfig::default())),
        }
    }

    pub fn auditor(&self) -> Arc<CostAuditor> {
        self.auditor.clone()
    }

    pub fn new_with_redis(redis_url: &str) -> Self {
        let auditor = Arc::new(CostAuditor::new(crate::pricing::calculator::CostConfig::default()));
        if let Ok(client) = Client::open(redis_url) {
            Tracker {
                rate_limiter: Some(Arc::new(RedisRateLimiter::new(client))),
                auditor,
            }
        } else {
            Tracker { rate_limiter: None, auditor }
        }
    }

    pub fn get_total_cost(&self) -> String {
        let cost = self.auditor.get_total_cost_value();
        format!("${:.2}", cost)
    }

    pub fn get_total_tokens(&self) -> String {
        self.auditor.get_total_output_tokens().to_string()
    }

    pub fn get_agent_costs_ui(&self) -> Vec<UiAgentCost> {
        let snapshot = self.auditor.get_agent_metrics_snapshot();
        let total_cost = self.auditor.get_total_cost_value();

        let mut ui_costs = Vec::new();
        for (agent_id, cost, revenue, output_tokens) in snapshot {
            let roi = self.auditor.calculate_roi(cost, revenue);
            let efficiency = self.auditor.calculate_efficiency(cost, output_tokens);

            let pct = if total_cost > 0.0 {
                (cost / total_cost) as f32
            } else {
                0.0
            };

            ui_costs.push(UiAgentCost {
                name: agent_id,
                cost: format!("${:.2}", cost),
                roi: format!("{:.0}%", roi),
                efficiency: format!("{:.0} tok/$", efficiency),
                pct,
            });
        }
        ui_costs
    }

    pub async fn check_rate_limit(&self, tenant_id: &str, agent_id: &str) -> Result<RateLimitStatus, String> {
        if let Some(ref limiter) = self.rate_limiter {
            limiter.record_action(tenant_id, agent_id).await
        } else {
            // Default allow if Redis is not configured
            Ok(RateLimitStatus {
                is_allowed: true,
                soft_limit_reached: false,
                user_message: None,
            })
        }
    }

    pub fn summary(&self, _scope: &str) -> TokenSummary {
        TokenSummary::default()
    }
}

#[derive(Default)]
pub struct TokenSummary {
    pub total_tokens: i64,
}

impl Default for Tracker {
    fn default() -> Self {
        Tracker::new()
    }
}
