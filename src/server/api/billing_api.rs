use axum::{Json, response::IntoResponse, extract::State};
use std::sync::Arc;
use crate::hub::Hub;

pub async fn get_my_plan(
    State(hub): State<Arc<Hub>>,
) -> impl IntoResponse {
    let tenant_id = "default";
    let tier = hub.tracker().get_tenant_tier(tenant_id).await.unwrap_or(::server_pricing::rate_limit::PlanTier::Free);
    let ai_used = hub.tracker().get_tenant_actions_used(tenant_id).await.unwrap_or(0);
    let storage_used_bytes = hub.tracker().get_tenant_storage_used(tenant_id).await.unwrap_or(0);

    let plan_name = match tier {
        ::server_pricing::rate_limit::PlanTier::Free => "Free",
        ::server_pricing::rate_limit::PlanTier::Starter => "Starter",
        ::server_pricing::rate_limit::PlanTier::Pro => "Pro",
        ::server_pricing::rate_limit::PlanTier::Business => "Business",
    }.to_string();

    let ai_limit = tier.monthly_action_limit().unwrap_or(0);
    let storage_limit = tier.storage_limit_mb().unwrap_or(0) as i64 * 1024 * 1024;

    let next_bill_estimated = match tier {
        ::server_pricing::rate_limit::PlanTier::Free => 0,
        ::server_pricing::rate_limit::PlanTier::Starter => 9,
        ::server_pricing::rate_limit::PlanTier::Pro => 29,
        ::server_pricing::rate_limit::PlanTier::Business => 79,
    };

    Json(serde_json::json!({
        "current_plan": plan_name,
        "ai_actions_used": ai_used,
        "ai_actions_limit": ai_limit,
        "storage_used_bytes": storage_used_bytes,
        "storage_limit_bytes": storage_limit,
        "next_bill_estimated": next_bill_estimated,
    }))
}

pub async fn get_cost_dashboard(
    State(hub): State<Arc<Hub>>,
) -> impl IntoResponse {
    let tenant_id = "default";
    let auditor = hub.get_cost_auditor();
    let llm_cost_f64 = auditor.get_total_cost();
    let total_revenue_f64 = auditor.get_total_revenue();

    let storage_bytes = hub.tracker().get_tenant_storage_used(tenant_id).await.unwrap_or(0);
    let storage_gb = storage_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let storage_cost_f64 = storage_gb * 0.10; // $0.10 per GB

    let payment_fees_f64 = total_revenue_f64 * 0.029;

    let total_costs_f64 = llm_cost_f64 + storage_cost_f64 + payment_fees_f64;

    Json(serde_json::json!({
        "total_revenue": total_revenue_f64,
        "total_costs": total_costs_f64,
        "llm_cost": llm_cost_f64,
        "storage_cost": storage_cost_f64,
        "payment_fees": payment_fees_f64,
        "period_start": "2024-05-01",
        "period_end": "2024-05-31",
    }))
}

pub fn router(hub: Arc<Hub>) -> axum::Router {
    axum::Router::new()
        .route("/plan", axum::routing::get(get_my_plan))
        .route("/cost", axum::routing::get(get_cost_dashboard))
        .with_state(hub)
}
