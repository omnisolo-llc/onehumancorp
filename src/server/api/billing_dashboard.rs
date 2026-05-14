use axum::{
    extract::{State, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use ::server_pricing::rate_limit::{PlanTier, RedisRateLimiter};

#[derive(Clone)]
pub struct DashboardState {
    pub rate_limiter: Arc<RedisRateLimiter>,
}

#[derive(Serialize)]
pub struct TenantPlanInfo {
    pub tenant_id: String,
    pub current_tier: String,
    pub ai_actions_used: u32,
    pub ai_actions_limit: Option<u32>,
    pub storage_used_mb: f64,
    pub storage_limit_mb: Option<u32>,
    pub estimated_next_bill: f64,
}

pub async fn get_my_plan(
    State(state): State<DashboardState>,
    Path(tenant_id): Path<String>,
) -> impl IntoResponse {
    // In a real scenario, these values would be fetched from Postgres and Redis.
    // For this demonstration, we'll fetch tier from the rate limiter and mock the usage.

    let tier = state.rate_limiter.get_tenant_tier(&tenant_id).await.unwrap_or(PlanTier::Free);
    let ai_limit = tier.monthly_action_limit();
    let storage_limit = tier.storage_limit_mb();

    // Mock usage data
    let actions_used = 45;
    let storage_used = 120.5;

    // Mock bill calculation based on tier
    let estimated_bill = match tier {
        PlanTier::Free => 0.00,
        PlanTier::Starter => 29.00,
        PlanTier::Pro => 99.00,
        PlanTier::Business => 299.00,
    };

    let info = TenantPlanInfo {
        tenant_id: tenant_id.clone(),
        current_tier: match tier {
            PlanTier::Free => "Free",
            PlanTier::Starter => "Starter",
            PlanTier::Pro => "Pro",
            PlanTier::Business => "Business",
        }.to_string(),
        ai_actions_used: actions_used,
        ai_actions_limit: ai_limit,
        storage_used_mb: storage_used,
        storage_limit_mb: storage_limit,
        estimated_next_bill: estimated_bill,
    };

    (StatusCode::OK, Json(info))
}
