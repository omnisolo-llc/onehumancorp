use axum::{
    extract::{Extension, Path, State},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ::server_common::Claims;
use crate::hub::Hub;
use crate::db::DB;

#[derive(Serialize)]
pub struct SubscriptionPlan {
    pub id: String,
    pub name: String,
    pub price_cents: i32,
    pub frequency: String,
}

#[derive(Serialize)]
pub struct Subscriber {
    pub id: String,
    pub plan_id: String,
    pub customer_id: String,
    pub status: String,
    pub next_billing_date: Option<String>,
}

#[derive(Serialize)]
pub struct FulfillmentBatch {
    pub id: String,
    pub plan_id: String,
    pub target_date: String,
    pub status: String,
    pub label_url: Option<String>,
    pub subscriber_count: i64,
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/plans", get(get_plans))
        .route("/subscribers", get(get_subscribers))
        .route("/batches", get(get_batches))
        .route("/batches/:id/print", post(print_batch))
        .layer(Extension(hub))
}

async fn get_plans(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());

    // Stub implementation: Return a mock subscription plan
    let plans = vec![SubscriptionPlan {
        id: "plan_test_1".to_string(),
        name: "Monthly Coffee Bean Box".to_string(),
        price_cents: 2900,
        frequency: "monthly".to_string(),
    }];

    (StatusCode::OK, Json(plans)).into_response()
}

async fn get_subscribers(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());

    // Stub implementation: Return mock active subscribers
    let subscribers = vec![
        Subscriber {
            id: "sub_1".to_string(),
            plan_id: "plan_test_1".to_string(),
            customer_id: "cust_1".to_string(),
            status: "ACTIVE".to_string(),
            next_billing_date: Some("2024-01-05T00:00:00Z".to_string()),
        },
        Subscriber {
            id: "sub_2".to_string(),
            plan_id: "plan_test_1".to_string(),
            customer_id: "cust_2".to_string(),
            status: "ACTIVE".to_string(),
            next_billing_date: Some("2024-01-05T00:00:00Z".to_string()),
        }
    ];

    (StatusCode::OK, Json(subscribers)).into_response()
}

async fn get_batches(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());

    // Stub implementation: Return a mock batch
    let batches = vec![
        FulfillmentBatch {
            id: "batch_1".to_string(),
            plan_id: "plan_test_1".to_string(),
            target_date: "2024-01-05T00:00:00Z".to_string(),
            status: "PENDING".to_string(),
            label_url: None,
            subscriber_count: 42,
        }
    ];

    (StatusCode::OK, Json(batches)).into_response()
}

async fn print_batch(
    Path(id): Path<String>,
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());

    // Return mock PDF URL
    let resp = serde_json::json!({
        "success": true,
        "batch_id": id,
        "label_url": "https://api.goshippo.com/v1/mock_batch_labels.pdf",
        "status": "PRINTED"
    });

    (StatusCode::OK, Json(resp)).into_response()
}
