use axum::{extract::{State, Path}, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateReturnRequest {
    pub order_id: String,
    pub customer_id: String,
    pub product_id: String,
    pub reason: String,
    pub action_type: String, // 'refund' or 'exchange'
    pub refund_amount_cents: i64,
    pub payment_intent_id: Option<String>,
}

#[derive(Serialize)]
pub struct ReturnRequestResponse {
    pub id: String,
    pub tenant_id: String,
    pub order_id: String,
    pub customer_id: String,
    pub product_id: String,
    pub reason: String,
    pub action_type: String,
    pub status: String,
    pub refund_amount_cents: i64,
    pub payment_intent_id: Option<String>,
    pub stripe_refund_id: Option<String>,
}

pub async fn list_returns(
    axum::extract::Extension(user): axum::extract::Extension<crate::common::Claims>,
) -> Result<Json<Vec<ReturnRequestResponse>>, axum::http::StatusCode> {
    let pool = crate::db::get_pool();
    let tenant_id = user.organization_id;

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    if crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.is_err() {
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    let records = sqlx::query_as!(
        ReturnRequestResponse,
        "SELECT id, tenant_id, order_id, customer_id, product_id, reason, action_type, status, refund_amount_cents, payment_intent_id, stripe_refund_id FROM return_requests WHERE tenant_id = $1 ORDER BY created_at DESC",
        tenant_id
    )
    .fetch_all(&mut *tx)
    .await;

    let _ = tx.commit().await;

    match records {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_return(
    axum::extract::Extension(user): axum::extract::Extension<crate::common::Claims>,
    Json(req): Json<CreateReturnRequest>,
) -> Result<Json<ReturnRequestResponse>, axum::http::StatusCode> {
    let pool = crate::db::get_pool();
    let tenant_id = user.organization_id;
    let id = Uuid::new_v4().to_string();

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    if crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.is_err() {
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    let result = sqlx::query!(
        "INSERT INTO return_requests (id, tenant_id, order_id, customer_id, product_id, reason, action_type, status, refund_amount_cents, payment_intent_id) VALUES ($1, $2, $3, $4, $5, $6, $7, 'pending', $8, $9)",
        id, tenant_id, req.order_id, req.customer_id, req.product_id, req.reason, req.action_type, req.refund_amount_cents, req.payment_intent_id
    ).execute(&mut *tx).await;

    let _ = tx.commit().await;

    if result.is_ok() {
        Ok(Json(ReturnRequestResponse {
            id,
            tenant_id,
            order_id: req.order_id,
            customer_id: req.customer_id,
            product_id: req.product_id,
            reason: req.reason,
            action_type: req.action_type,
            status: "pending".to_string(),
            refund_amount_cents: req.refund_amount_cents,
            payment_intent_id: req.payment_intent_id,
            stripe_refund_id: None,
        }))
    } else {
        Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn approve_return(
    axum::extract::Extension(user): axum::extract::Extension<crate::common::Claims>,
    Path(id): Path<String>,
) -> Result<Json<ReturnRequestResponse>, axum::http::StatusCode> {
    let pool = crate::db::get_pool();
    let tenant_id = user.organization_id;

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    if crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.is_err() {
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    let request = match sqlx::query_as!(
        ReturnRequestResponse,
        "SELECT id, tenant_id, order_id, customer_id, product_id, reason, action_type, status, refund_amount_cents, payment_intent_id, stripe_refund_id FROM return_requests WHERE id = $1 AND tenant_id = $2",
        id, tenant_id
    ).fetch_optional(&mut *tx).await {
        Ok(Some(r)) => r,
        _ => return Err(axum::http::StatusCode::NOT_FOUND),
    };

    if request.status != "pending" {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }

    // Call Finance Agent to refund
    let mut stripe_refund_id = None;
    if let Some(pi) = &request.payment_intent_id {
        let stripe_client = crate::integrations::stripe::client::StripeClient::new(std::env::var("STRIPE_API_KEY").unwrap_or_default());
        if let Ok(refund_id) = stripe_client.create_refund(pi, request.refund_amount_cents).await {
            stripe_refund_id = Some(refund_id);
        } else {
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // Call Operations Agent to restock
    let redis_url = std::env::var("OHC_REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let redis_client_opt = redis::Client::open(redis_url).ok();
    let inventory_service = crate::services::inventory::service::InventoryService::new(redis_client_opt);
    let _ = inventory_service.restock_inventory(&tenant_id, &request.product_id, 1).await;

    // Update status
    let _ = sqlx::query!(
        "UPDATE return_requests SET status = 'processed', stripe_refund_id = $1 WHERE id = $2",
        stripe_refund_id, id
    ).execute(&mut *tx).await;

    let _ = tx.commit().await;

    let updated_request = ReturnRequestResponse {
        status: "processed".to_string(),
        stripe_refund_id,
        ..request
    };

    Ok(Json(updated_request))
}
