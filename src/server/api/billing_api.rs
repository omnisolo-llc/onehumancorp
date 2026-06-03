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
    pub network_cost: i64,
    pub bandwidth_savings: i64,
    pub period_start: String,
    pub period_end: String,
}

#[derive(serde::Deserialize)]
pub struct SelectPlanRequest {
    pub plan_id: String,
}

#[derive(serde::Serialize)]
pub struct SelectPlanResponse {
    pub url: String,
}

pub fn router(hub: Arc<Hub>) -> axum::Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new()
        .route("/my-plan", axum::routing::get(my_plan_handler))
        .route("/cost-dashboard", axum::routing::get(cost_dashboard_handler))
        .route("/select-plan", axum::routing::post(select_plan_handler))
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

    let base_bill = tier.base_price();

    let now = chrono::Utc::now();
    use chrono::Datelike;
    let days_elapsed = now.day() as u32;
    // rough total days
    let total_days = 30;

    let projected_cost = ::server_pricing::calculator::calculate_projected_monthly_cost(
        base_bill,
        days_elapsed,
        total_days
    );

    let next_bill_estimated = projected_cost as i32;

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
        None => return Json(CostDashboardResponse { total_revenue: 0, total_costs: 0, llm_cost: 0, storage_cost: 0, payment_fees: 0, network_cost: 0, bandwidth_savings: 0, period_start: "2024-05-01".to_string(), period_end: "2024-05-31".to_string() })
    };

    let now = chrono::Utc::now();
    use chrono::Datelike;
    let start_of_month = chrono::NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap().and_hms_opt(0, 0, 0).unwrap().and_utc();
    let period_start = start_of_month.format("%Y-%m-%d").to_string();
    let period_end = now.format("%Y-%m-%d").to_string();

    let auditor = hub.get_cost_auditor();
    let hub_clone = hub.clone();
    let tenant_id_clone = tenant_id.clone();

    // Proper concurrent execution combining spawn_blocking for CPU/sync methods
    // and tokio::join! to wait on both the async I/O future and the blocking CPU task simultaneously.
    let tenant_id_clone_2 = tenant_id.clone();
    let auditor_future = tokio::task::spawn_blocking(move || {
        (
            auditor.get_tenant_cost(&tenant_id_clone_2),
            auditor.get_tenant_revenue(&tenant_id_clone_2),
            auditor.get_tenant_payment_fees(&tenant_id_clone_2),
            auditor.get_tenant_compute_cost(&tenant_id_clone_2),
            auditor.get_tenant_network_cost(&tenant_id_clone_2),
            auditor.get_tenant_bandwidth_savings(&tenant_id_clone_2)
        )
    });

    let storage_future = tokio::task::spawn(async move {
        hub_clone.tracker().get_tenant_storage_used(&tenant_id_clone).await.unwrap_or(0)
    });

    let (storage_res, auditor_res) = tokio::join!(storage_future, auditor_future);

    let storage_bytes = storage_res.unwrap_or(0);
    let (llm_cost_f64, total_revenue_f64, payment_fees_f64, compute_cost_f64, network_cost_f64, bandwidth_savings_f64) = auditor_res.unwrap_or((0.0, 0.0, 0.0, 0.0, 0.0, 0.0));

    let storage_gb = storage_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let storage_cost_f64 = storage_gb * 0.10; // $0.10 per GB

    let total_costs_f64 = llm_cost_f64 + storage_cost_f64 + payment_fees_f64 + compute_cost_f64 + network_cost_f64;

    Json(CostDashboardResponse {
        total_revenue: (total_revenue_f64 * 100.0).round() as i64,
        total_costs: (total_costs_f64 * 100.0).round() as i64,
        llm_cost: (llm_cost_f64 * 100.0).round() as i64,
        storage_cost: (storage_cost_f64 * 100.0).round() as i64,
        payment_fees: (payment_fees_f64 * 100.0).round() as i64,
        network_cost: (network_cost_f64 * 100.0).round() as i64,
        bandwidth_savings: (bandwidth_savings_f64 * 100.0).round() as i64,
        period_start,
        period_end,
    })
}

pub async fn select_plan_handler(
    State(_hub): State<Arc<Hub>>,
    request: axum::extract::Request,
) -> Result<Json<SelectPlanResponse>, axum::http::StatusCode> {
    let tenant_id = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(auth) => {
            if auth.org_id.is_empty() {
                "default".to_string()
            } else {
                auth.org_id.clone()
            }
        },
        None => return Err(axum::http::StatusCode::UNAUTHORIZED),
    };

    let (_parts, body) = request.into_parts();
    let bytes = axum::body::to_bytes(body, usize::MAX).await.map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    let payload: SelectPlanRequest = serde_json::from_slice(&bytes).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    let stripe_key = std::env::var("STRIPE_API_KEY").unwrap_or_default();
    if stripe_key.is_empty() {
        tracing::error!("STRIPE_API_KEY is not set");
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    let client = crate::integrations::stripe::client::StripeClient::new(stripe_key);
    let mercadopago_client = std::env::var("MERCADOPAGO_ACCESS_TOKEN").ok().map(|token| crate::integrations::mercadopago::client::MercadoPagoClient::new(token));
    let alipay_client = std::env::var("ALIPAY_ACCESS_TOKEN").ok().map(|token| crate::integrations::alipay::client::AlipayClient::new(token));

    let amount = match payload.plan_id.as_str() {
        "Starter" => 9.0,
        "Pro" => 29.0,
        "Business" => 79.0,
        _ => 0.0
    };

    let optimal_pm = crate::integrations::stripe::routing::PaymentRouter::optimize_payment_method(amount);
    let savings = crate::integrations::stripe::routing::PaymentRouter::calculate_fee_savings(amount);
    if savings > 0.0 {
        tracing::info!("Optimized payment method to {:?} to save ${:.2} on transaction fees", optimal_pm, savings);
    }

    let is_china = payload.plan_id.ends_with("_china");
    let is_latam = payload.plan_id.ends_with("_latam");

    let url = if let Some(alipay_client) = alipay_client.filter(|_| is_china) {
        alipay_client.create_checkout_preference(&payload.plan_id, &tenant_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
    } else if let Some(mp_client) = mercadopago_client.filter(|_| is_latam) {
        mp_client.create_checkout_preference(&payload.plan_id, &tenant_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        client.create_checkout_session(&payload.plan_id, &tenant_id, amount).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
    };

    Ok(Json(SelectPlanResponse { url }))
}
