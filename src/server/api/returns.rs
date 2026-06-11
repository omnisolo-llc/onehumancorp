use axum::{
    extract::{State, Json},
    response::IntoResponse,
    routing::post,
    Router,
};
use reqwest::StatusCode;
use std::sync::Arc;
use serde_json::json;

#[derive(serde::Deserialize)]
pub struct InitiateReturnRequest {
    pub tenant_id: String,
    pub order_id: String,
    pub product_id: String,
    pub reason: String,
    pub return_type: String, // "Refund" or "Exchange"
    pub amount_cents: i64,
}

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<crate::db::DB>) -> Router<S> {
    Router::new()
        .route("/initiate", post(initiate_return_handler))
        .with_state(db)
}

async fn initiate_return_handler(
    State(db): State<Arc<crate::db::DB>>,
    Json(payload): Json<InitiateReturnRequest>,
) -> impl IntoResponse {
    let pool = &db.pool;

    // Create triage item for owner approval
    let triage_id = format!("triage-{}", uuid::Uuid::new_v4());
    let action_payload = json!({
        "order_id": payload.order_id,
        "product_id": payload.product_id,
        "amount_cents": payload.amount_cents,
        "reason": payload.reason,
        "return_type": payload.return_type
    }).to_string();

    let context = format!(
        "Customer requests a {} for Order #{}. Reason: {}. Amount: ${:.2}",
        payload.return_type, payload.order_id, payload.reason, payload.amount_cents as f64 / 100.0
    );

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response();
        }
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &payload.tenant_id).await {
         return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response();
    }

    if let Err(e) = sqlx::query(
        "INSERT INTO triage_items (id, tenant_id, source, priority, context, status) VALUES ($1, $2, 'Return Portal', 'high', $3, 'pending')"
    )
    .bind(&triage_id)
    .bind(&payload.tenant_id)
    .bind(&context)
    .execute(&mut *tx)
    .await {
        tracing::error!("Failed to insert triage item: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response();
    }

    let action_id = format!("action-{}", uuid::Uuid::new_v4());
    if let Err(e) = sqlx::query(
        "INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload) VALUES ($1, $2, $3, 'ProcessReturn', $4)"
    )
    .bind(&action_id)
    .bind(&triage_id)
    .bind(&payload.tenant_id)
    .bind(&action_payload)
    .execute(&mut *tx)
    .await {
        tracing::error!("Failed to insert triage action: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response();
    }

    if let Err(e) = tx.commit().await {
         return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response();
    }

    (StatusCode::OK, Json(json!({"success": true, "triage_id": triage_id}))).into_response()
}
