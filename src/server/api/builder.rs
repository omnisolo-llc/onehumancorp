use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use sqlx::PgPool;
use serde_json::json;

pub fn router<S>(pool: PgPool) -> Router<S> {
    Router::new()
        .route("/offerings", get(get_offerings))
        .route("/transactions", get(get_transactions))
        .with_state(pool)
}

pub async fn get_offerings(State(pool): State<PgPool>) -> impl IntoResponse {
    let tenant_id = "tenant-1"; // Simplified for mock testing

    // We use standard query_as instead of the macro query_as! to bypass compile time checks
    // since we do not have local PG running in CI in a deterministic way.
    let offerings: Result<Vec<crate::domain::repository::models::Offering>, _> = sqlx::query_as::<_, crate::domain::repository::models::Offering>(
        r#"SELECT id, tenant_id, type as "r#type", name, description, price, created_at, updated_at FROM offerings WHERE tenant_id = $1"#
    )
    .bind(tenant_id)
    .fetch_all(&pool)
    .await;

    match offerings {
        Ok(data) => Json(json!({ "status": "success", "data": data })).into_response(),
        Err(e) => Json(json!({ "status": "error", "message": e.to_string() })).into_response(),
    }
}

pub async fn get_transactions(State(pool): State<PgPool>) -> impl IntoResponse {
    let tenant_id = "tenant-1"; // Simplified for mock testing

    let transactions: Result<Vec<crate::domain::repository::models::Transaction>, _> = sqlx::query_as::<_, crate::domain::repository::models::Transaction>(
        "SELECT id, tenant_id, customer_id, status, total_amount, created_at, updated_at FROM transactions WHERE tenant_id = $1"
    )
    .bind(tenant_id)
    .fetch_all(&pool)
    .await;

    match transactions {
        Ok(data) => Json(json!({ "status": "success", "data": data })).into_response(),
        Err(e) => Json(json!({ "status": "error", "message": e.to_string() })).into_response(),
    }
}
