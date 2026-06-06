use axum::{extract::State, Json};
use std::sync::Arc;
use std::sync::OnceLock;
use crate::hub::Hub;
use axum::http::HeaderMap;
use crate::utils::cache::HybridCache;

pub static MY_PLAN_CACHE: OnceLock<HybridCache<MyPlanResponse>> = OnceLock::new();
pub static COST_DASHBOARD_CACHE: OnceLock<HybridCache<CostDashboardResponse>> = OnceLock::new();
pub static BUDGET_ALERT_CACHE: OnceLock<HybridCache<BudgetAlertConfig>> = OnceLock::new();

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct BudgetAlertConfig {
    pub threshold_usd: f64,
    pub notify_at_pct: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct MyPlanResponse {
    pub current_plan: String,
    pub ai_actions_used: i32,
    pub ai_actions_limit: Option<i32>,
    pub storage_used_bytes: i64,
    pub storage_limit_bytes: Option<i64>,
    pub next_bill_estimated: i32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct CostDashboardResponse {
    pub total_revenue: i64,
    pub total_costs: i64,
    pub llm_cost: i64,
    pub storage_cost: i64,
    pub payment_fees: i64,
    pub network_cost: i64,
    pub bandwidth_savings: i64,
    pub cache_hit_rate: f64,
    pub cost_per_1k_tokens: f64,
    pub period_start: String,
    pub period_end: String,
    pub trend: Vec<crate::pricing::cost_aggregator::DailyCost>,
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> axum::Router<S> {
    axum::Router::new()
        .route("/my-plan", axum::routing::get(my_plan_handler))
        .route("/cost-dashboard", axum::routing::get(cost_dashboard_handler))
        .route("/budget-alert", axum::routing::get(budget_alert_get_handler).post(budget_alert_post_handler))
        .with_state(hub)
}

use axum::response::IntoResponse;

pub async fn budget_alert_get_handler(
    _headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
    request: axum::extract::Request,
) -> axum::response::Response {
    let auth = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(a) => a,
        None => return (axum::http::StatusCode::UNAUTHORIZED, "Missing auth").into_response(),
    };
    let tenant_id = if auth.org_id.is_empty() { "default".to_string() } else { auth.org_id.clone() };

    // Fetch from Postgres
    let pool = crate::db::get_pool();
    let row = sqlx::query(
        "SELECT threshold_usd, notify_at_pct FROM tenant_budget_alerts WHERE tenant_id = $1"
    )
    .bind(&tenant_id)
    .fetch_optional(&pool)
    .await;

    let config = match row {
        Ok(Some(r)) => {
            use sqlx::Row;
            let threshold_usd: f64 = r.try_get("threshold_usd").unwrap_or(100.0);
            let notify_at_pct: f64 = r.try_get("notify_at_pct").unwrap_or(80.0);
            BudgetAlertConfig { threshold_usd, notify_at_pct }
        }
        _ => BudgetAlertConfig { threshold_usd: 100.0, notify_at_pct: 80.0 }, // fallback
    };

    Json(config).into_response()
}

pub async fn budget_alert_post_handler(
    State(hub): State<Arc<Hub>>,
    mut request: axum::extract::Request,
) -> axum::response::Response {
    let auth = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(a) => a,
        None => return (axum::http::StatusCode::UNAUTHORIZED, "Missing auth").into_response(),
    };
    let tenant_id = if auth.org_id.is_empty() { "default".to_string() } else { auth.org_id.clone() };

    use axum::extract::FromRequest;
    let payload_res = Json::<BudgetAlertConfig>::from_request(request, &()).await;
    let payload = match payload_res {
        Ok(Json(p)) => p,
        Err(_) => return (axum::http::StatusCode::BAD_REQUEST, "Invalid JSON payload").into_response(),
    };

    // Save to Postgres
    let pool = crate::db::get_pool();
    let _ = sqlx::query(
        r#"
        INSERT INTO tenant_budget_alerts (tenant_id, threshold_usd, notify_at_pct)
        VALUES ($1, $2, $3)
        ON CONFLICT (tenant_id) DO UPDATE SET
            threshold_usd = EXCLUDED.threshold_usd,
            notify_at_pct = EXCLUDED.notify_at_pct,
            updated_at = CURRENT_TIMESTAMP
        "#
    )
    .bind(&tenant_id)
    .bind(payload.threshold_usd)
    .bind(payload.notify_at_pct)
    .execute(&pool)
    .await;

    Json(payload).into_response()
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

    let cache = MY_PLAN_CACHE.get_or_init(|| HybridCache::new(None));
    if let Some(cached_resp) = cache.get(&tenant_id).await {
        return Json(cached_resp);
    }

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

    let resp = MyPlanResponse {
        current_plan: plan_name,
        ai_actions_used: ai_used as i32,
        ai_actions_limit: ai_limit,
        storage_used_bytes,
        storage_limit_bytes: storage_limit,
        next_bill_estimated,
    };
    cache.set(&tenant_id, resp.clone(), std::time::Duration::from_secs(60)).await;
    Json(resp)
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
        None => return Json(CostDashboardResponse { total_revenue: 0, total_costs: 0, llm_cost: 0, storage_cost: 0, payment_fees: 0, network_cost: 0, bandwidth_savings: 0, cache_hit_rate: 0.0, cost_per_1k_tokens: 0.0, period_start: "2024-05-01".to_string(), period_end: "2024-05-31".to_string(), trend: vec![] })
    };

    let cache = COST_DASHBOARD_CACHE.get_or_init(|| HybridCache::new(None));
    if let Some(cached_resp) = cache.get(&tenant_id).await {
        return Json(cached_resp);
    }

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
            auditor.get_tenant_bandwidth_savings(&tenant_id_clone_2),
            auditor.get_tenant_tokens(&tenant_id_clone_2),
            auditor.get_tenant_cached_tokens(&tenant_id_clone_2)
        )
    });

    let storage_future = tokio::task::spawn(async move {
        hub_clone.tracker().get_tenant_storage_used(&tenant_id_clone).await.unwrap_or(0)
    });

    let (storage_res, auditor_res) = tokio::join!(storage_future, auditor_future);

    let storage_bytes = storage_res.unwrap_or(0);
    let (llm_cost_f64, total_revenue_f64, payment_fees_f64, compute_cost_f64, network_cost_f64, bandwidth_savings_f64, total_tokens, cached_tokens) = auditor_res.unwrap_or((0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0, 0));

    let cache_hit_rate = if total_tokens + cached_tokens > 0 {
        (cached_tokens as f64 / (total_tokens as f64 + cached_tokens as f64)) * 100.0
    } else {
        0.0
    };

    let cost_per_1k_tokens = if total_tokens > 0 {
        llm_cost_f64 / (total_tokens as f64 / 1000.0)
    } else {
        0.0
    };

    let storage_gb = storage_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let storage_cost_f64 = storage_gb * 0.10; // $0.10 per GB

    let total_costs_f64 = llm_cost_f64 + storage_cost_f64 + payment_fees_f64 + compute_cost_f64 + network_cost_f64;

    let pool = crate::db::get_pool();
    let trend = crate::pricing::cost_aggregator::aggregate_daily_costs(&pool, &tenant_id).await;

    let resp = CostDashboardResponse {
        total_revenue: (total_revenue_f64 * 100.0).round() as i64,
        total_costs: (total_costs_f64 * 100.0).round() as i64,
        llm_cost: (llm_cost_f64 * 100.0).round() as i64,
        storage_cost: (storage_cost_f64 * 100.0).round() as i64,
        payment_fees: (payment_fees_f64 * 100.0).round() as i64,
        network_cost: (network_cost_f64 * 100.0).round() as i64,
        bandwidth_savings: (bandwidth_savings_f64 * 100.0).round() as i64,
        cache_hit_rate: (cache_hit_rate * 100.0).round() / 100.0,
        cost_per_1k_tokens: (cost_per_1k_tokens * 10000.0).round() / 10000.0,
        period_start,
        period_end,
        trend,
    };
    cache.set(&tenant_id, resp.clone(), std::time::Duration::from_secs(60)).await;
    Json(resp)
}
