use axum::{
    extract::State,
    response::IntoResponse,
    routing::{post, get},
    Json, Router,
};
use reqwest::StatusCode;
use serde_json::json;
use sqlx::PgPool;

use crate::domain::loyalty_ledger::LoyaltyProgram;
use crate::domain::repository::loyalty_repo::LoyaltyRepo;
use crate::auth::extractors::TenantAuth;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/programs", post(create_program).get(get_program))
}

async fn create_program(
    auth: TenantAuth,
    State(pool): State<PgPool>,
    Json(payload): Json<LoyaltyProgram>,
) -> impl IntoResponse {
    let repo = LoyaltyRepo::new(pool);

    match repo.create_program(&payload).await {
        Ok(prog) => (StatusCode::CREATED, Json(prog)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

async fn get_program(
    auth: TenantAuth,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let repo = LoyaltyRepo::new(pool);

    match repo.get_program_by_tenant(&auth.tenant_id).await {
        Ok(Some(prog)) => (StatusCode::OK, Json(prog)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "No active program found"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}
