use axum::{extract::State, Json};
use std::sync::Arc;
use crate::hub::Hub;
use axum::http::HeaderMap;

#[derive(serde::Serialize)]
pub struct MyPlanResponse {
    pub current_plan: String,
    pub ai_actions_used: i32,
    pub ai_actions_limit: Option<i32>,
    pub storage_used_bytes: i64,
    pub storage_limit_bytes: Option<i64>,
    pub next_bill_estimated: i32,
}

#[derive(serde::Serialize)]
pub struct CostDashboardResponse {
    pub total_revenue: i64,
    pub total_costs: i64,
    pub llm_cost: i64,
    pub storage_cost: i64,
    pub payment_fees: i64,
    pub period_start: String,
    pub period_end: String,
    pub llm_tokens: i64,
}

pub fn router(hub: Arc<Hub>) -> axum::Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new()
        .route("/my-plan", axum::routing::get(my_plan_handler))
        .route("/cost-dashboard", axum::routing::get(cost_dashboard_handler))
        .with_state(hub)
}

pub async fn my_plan_handler(
    _headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
    request: axum::extract::Request,
) -> Json<MyPlanResponse> {
    let tenant_id = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(auth) => {
            if auth.org_id.is_empty() {
                "default".to_string()
            } else {
                auth.org_id.clone()
            }
        },
        None => return Json(MyPlanResponse { current_plan: "Free".to_string(), ai_actions_used: 0, ai_actions_limit: None, storage_used_bytes: 0, storage_limit_bytes: None, next_bill_estimated: 0 })
    };

    let tracker = hub.tracker();
    let tier_future = tracker.get_tenant_tier(&tenant_id);
    let ai_used_future = tracker.get_tenant_actions_used(&tenant_id);
    let storage_used_bytes_future = tracker.get_tenant_storage_used(&tenant_id);

    let (tier_res, ai_used_res, storage_used_bytes_res) = tokio::join!(tier_future, ai_used_future, storage_used_bytes_future);

    let tier = tier_res.unwrap_or(::server_pricing::rate_limit::PlanTier::Free);
    let ai_used = ai_used_res.unwrap_or(0);
    let storage_used_bytes = storage_used_bytes_res.unwrap_or(0);

    let plan_name = match tier {
        ::server_pricing::rate_limit::PlanTier::Free => "Free",
        ::server_pricing::rate_limit::PlanTier::Starter => "Starter",
        ::server_pricing::rate_limit::PlanTier::Pro => "Pro",
        ::server_pricing::rate_limit::PlanTier::Business => "Business",
    }.to_string();

    let ai_limit = tier.monthly_action_limit().map(|v| v as i32);
    let storage_limit = tier.storage_limit_mb().map(|v| (v as i64) * 1024 * 1024);

    let next_bill_estimated = match tier {
        ::server_pricing::rate_limit::PlanTier::Free => 0,
        ::server_pricing::rate_limit::PlanTier::Starter => 9,
        ::server_pricing::rate_limit::PlanTier::Pro => 29,
        ::server_pricing::rate_limit::PlanTier::Business => 79,
    };

    Json(MyPlanResponse {
        current_plan: plan_name,
        ai_actions_used: ai_used as i32,
        ai_actions_limit: ai_limit,
        storage_used_bytes,
        storage_limit_bytes: storage_limit,
        next_bill_estimated,
    })
}

pub async fn cost_dashboard_handler(
    _headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
    request: axum::extract::Request,
) -> Json<CostDashboardResponse> {
    let tenant_id = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(auth) => {
            if auth.org_id.is_empty() {
                "default".to_string()
            } else {
                auth.org_id.clone()
            }
        },
        None => return Json(CostDashboardResponse { total_revenue: 0, total_costs: 0, llm_cost: 0, storage_cost: 0, payment_fees: 0, period_start: "2024-05-01".to_string(), period_end: "2024-05-31".to_string(), llm_tokens: 0 })
    };

    let now = chrono::Utc::now();
    use chrono::Datelike;
    let start_of_month = chrono::NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap().and_hms_opt(0, 0, 0).unwrap().and_utc();
    let period_start = start_of_month.format("%Y-%m-%d").to_string();
    let period_end = now.format("%Y-%m-%d").to_string();
    let auditor = hub.get_cost_auditor();

    let llm_cost_f64 = auditor.get_total_cost();
    let llm_tokens = auditor.get_tenant_tokens(&tenant_id);
    let total_revenue_f64 = auditor.get_total_revenue();

    let storage_bytes = hub.tracker().get_tenant_storage_used(&tenant_id).await.unwrap_or(0);
    let storage_gb = storage_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let storage_cost_f64 = storage_gb * 0.10; // $0.10 per GB

    let payment_fees_f64 = total_revenue_f64 * 0.029;
    let total_costs_f64 = llm_cost_f64 + storage_cost_f64 + payment_fees_f64;

    Json(CostDashboardResponse {
        total_revenue: (total_revenue_f64 * 100.0) as i64,
        total_costs: (total_costs_f64 * 100.0) as i64,
        llm_cost: (llm_cost_f64 * 100.0) as i64,
        storage_cost: (storage_cost_f64 * 100.0) as i64,
        payment_fees: (payment_fees_f64 * 100.0) as i64,
        period_start,
        period_end,
        llm_tokens,
    })
}
