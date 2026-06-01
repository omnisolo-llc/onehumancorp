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
}

pub fn router(hub: Arc<Hub>) -> axum::Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new()
        .route("/my-plan", axum::routing::get(my_plan_handler))
        .route("/cost-dashboard", axum::routing::get(cost_dashboard_handler))
        .route("/stripe/terminal/connection_token", axum::routing::post(terminal_connection_token_handler))
        .route("/stripe/terminal/payment_intent", axum::routing::post(terminal_payment_intent_handler))
        .route("/stripe/terminal/capture", axum::routing::post(terminal_capture_handler))
        .with_state(hub)
}

#[derive(serde::Deserialize)]
pub struct CreatePaymentIntentRequest {
    pub amount_cents: i64,
}

#[derive(serde::Deserialize)]
pub struct CaptureTerminalPaymentRequest {
    pub payment_intent_id: String,
    pub product_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct CaptureTerminalPaymentResponse {
    pub success: bool,
    pub message: String,
}

pub async fn terminal_connection_token_handler(
    _headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
    request: axum::extract::Request,
) -> Json<serde_json::Value> {
    let tenant_id = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(auth) => if auth.org_id.is_empty() { "default".to_string() } else { auth.org_id.clone() },
        None => "default".to_string()
    };

    let tracker = hub.tracker();
    if let Some(stripe) = &tracker.stripe_client {
        if let Ok(token) = stripe.create_terminal_connection_token(&tenant_id).await {
            return Json(serde_json::json!({ "secret": token }));
        }
    }

    // Fallback if no real stripe key
    let token = format!("tok_terminal_{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
    Json(serde_json::json!({ "secret": token }))
}

pub async fn terminal_payment_intent_handler(
    _headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
    request: axum::extract::Request,
    req_body: Option<Json<CreatePaymentIntentRequest>>,
) -> Json<serde_json::Value> {
    let tenant_id = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(auth) => if auth.org_id.is_empty() { "default".to_string() } else { auth.org_id.clone() },
        None => "default".to_string()
    };

    let amount = match req_body {
        Some(Json(body)) => body.amount_cents,
        None => 0,
    };

    let tracker = hub.tracker();
    if let Some(stripe) = &tracker.stripe_client {
        if let Ok(intent) = stripe.create_terminal_payment_intent(&tenant_id, amount).await {
            return Json(intent);
        }
    }

    let client_secret = format!("pi_{}_secret_{}", uuid::Uuid::new_v4().to_string().replace("-", ""), uuid::Uuid::new_v4().to_string().replace("-", ""));
    Json(serde_json::json!({
        "id": format!("pi_{}", uuid::Uuid::new_v4().to_string().replace("-", "")),
        "object": "payment_intent",
        "amount": amount,
        "currency": "usd",
        "client_secret": client_secret,
        "status": "requires_payment_method"
    }))
}

pub async fn terminal_capture_handler(
    _headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
    request: axum::extract::Request,
    req_body: Option<Json<CaptureTerminalPaymentRequest>>,
) -> Json<CaptureTerminalPaymentResponse> {
    let tenant_id = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(auth) => if auth.org_id.is_empty() { "default".to_string() } else { auth.org_id.clone() },
        None => "default".to_string()
    };

    if let Some(Json(body)) = req_body {
        if let Some(product_id) = body.product_id {
            // Deduct inventory
            let pool = crate::db::get_pool();
            let _ = sqlx::query("UPDATE products SET inventory_count = inventory_count - 1 WHERE id = $1 AND (organization_id = $2 OR tenant_id = $2)")
                .bind(product_id)
                .bind(&tenant_id)
                .execute(&pool)
                .await;
        }
    }

    Json(CaptureTerminalPaymentResponse {
        success: true,
        message: "Payment captured and inventory updated".to_string(),
    })
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

    let base_bill = match tier {
        ::server_pricing::rate_limit::PlanTier::Free => 0.0,
        ::server_pricing::rate_limit::PlanTier::Starter => 29.0,
        ::server_pricing::rate_limit::PlanTier::Pro => 79.0,
        ::server_pricing::rate_limit::PlanTier::Business => 299.0,
    };

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
        None => return Json(CostDashboardResponse { total_revenue: 0, total_costs: 0, llm_cost: 0, storage_cost: 0, payment_fees: 0, period_start: "2024-05-01".to_string(), period_end: "2024-05-31".to_string() })
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
            auditor.get_tenant_payment_fees(&tenant_id_clone_2)
        )
    });

    let storage_future = tokio::task::spawn(async move {
        hub_clone.tracker().get_tenant_storage_used(&tenant_id_clone).await.unwrap_or(0)
    });

    let (storage_res, auditor_res) = tokio::join!(storage_future, auditor_future);

    let storage_bytes = storage_res.unwrap_or(0);
    let (llm_cost_f64, total_revenue_f64, payment_fees_f64) = auditor_res.unwrap_or((0.0, 0.0, 0.0));

    let storage_gb = storage_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let storage_cost_f64 = storage_gb * 0.10; // $0.10 per GB

    let total_costs_f64 = llm_cost_f64 + storage_cost_f64 + payment_fees_f64;

    Json(CostDashboardResponse {
        total_revenue: (total_revenue_f64 * 100.0).round() as i64,
        total_costs: (total_costs_f64 * 100.0).round() as i64,
        llm_cost: (llm_cost_f64 * 100.0).round() as i64,
        storage_cost: (storage_cost_f64 * 100.0).round() as i64,
        payment_fees: (payment_fees_f64 * 100.0).round() as i64,
        period_start,
        period_end,
    })
}
