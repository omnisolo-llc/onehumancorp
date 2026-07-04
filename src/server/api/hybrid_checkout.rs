use axum::{extract::State, Json};
use std::sync::Arc;
use axum::http::HeaderMap;
use crate::hub::Hub;

#[derive(serde::Deserialize)]
pub struct CreateHybridCheckoutRequest {
    pub amount_cents: i64,
    pub payment_method: String, // "tap_to_pay", "payment_link", "cash"
    pub customer_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct CreateHybridCheckoutResponse {
    pub session_id: String,
    pub checkout_url: Option<String>,
    pub success: bool,
    pub error_message: Option<String>,
}

pub async fn create_hybrid_checkout_handler(
    _headers: HeaderMap,
    State(_hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req: axum::extract::Json<CreateHybridCheckoutRequest>,
) -> Json<CreateHybridCheckoutResponse> {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return Json(CreateHybridCheckoutResponse {
                    session_id: "".to_string(),
                    checkout_url: None,
                    success: false,
                    error_message: Some("Unauthenticated: Missing tenant ID".to_string()),
                });
            }
            auth.org_id.clone()
        }
        None => {
            return Json(CreateHybridCheckoutResponse {
                session_id: "".to_string(),
                checkout_url: None,
                success: false,
                error_message: Some("Unauthenticated".to_string()),
            });
        }
    };

    let session_id = uuid::Uuid::new_v4().to_string();

    // Simulate generation of checkout URL for payment links
    let checkout_url = if req.payment_method == "payment_link" {
        Some(format!("https://checkout.stripe.com/pay/cs_test_{}", session_id.replace("-", "")))
    } else {
        None
    };

    let pool = crate::db::get_pool();
    let res = sqlx::query(
        "INSERT INTO checkout_sessions (id, tenant_id, customer_id, amount_cents, payment_method, checkout_type, status)
         VALUES ($1, $2, $3, $4, $5, 'hybrid', 'PENDING')"
    )
    .bind(&session_id)
    .bind(&tenant_id)
    .bind(&req.customer_id)
    .bind(&req.amount_cents)
    .bind(&req.payment_method)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Json(CreateHybridCheckoutResponse {
            session_id,
            checkout_url,
            success: true,
            error_message: None,
        }),
        Err(e) => {
            tracing::error!("Failed to create hybrid checkout session: {}", e);
            Json(CreateHybridCheckoutResponse {
                session_id: "".to_string(),
                checkout_url: None,
                success: false,
                error_message: Some(e.to_string()),
            })
        }
    }
}
