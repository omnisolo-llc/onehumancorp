use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    domain::loyalty::{LoyaltyProgram, LoyaltyTransaction},
    services::loyalty::LoyaltyService,

};

#[derive(Clone)]
pub struct LoyaltyApiState {
    pub loyalty_service: Arc<LoyaltyService>,
}

pub fn routes<S: Clone + Send + Sync + 'static>(state: LoyaltyApiState) -> Router<S> {
    Router::new()
        .route("/programs", get(list_programs).post(create_program))
        .route("/programs/:program_id", get(get_program))
        .route("/programs/:program_id/earn", post(earn_points))
        .route("/programs/:program_id/redeem", post(redeem_reward))
        .route("/programs/:program_id/customers/:customer_id", get(get_customer_account))
        .with_state(state)
}

async fn list_programs(
    State(state): State<LoyaltyApiState>,
    claims: axum::extract::Extension<::server_common::Claims>,
) -> Result<Json<Vec<LoyaltyProgram>>, axum::http::StatusCode> {
    state.loyalty_service.list_programs(&claims.organization_id.clone().unwrap_or_default())
        .await
        .map(Json)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Deserialize)]
struct CreateProgramReq {
    name: String,
    program_type: String,
    config: serde_json::Value,
    is_active: bool,
}

async fn create_program(
    State(state): State<LoyaltyApiState>,
    claims: axum::extract::Extension<::server_common::Claims>,
    Json(payload): Json<CreateProgramReq>,
) -> Result<Json<LoyaltyProgram>, axum::http::StatusCode> {
    let program = LoyaltyProgram {
        id: "".to_string(), // generated in service
        tenant_id: claims.organization_id.clone().unwrap_or_default().clone(),
        name: payload.name,
        program_type: payload.program_type,
        config: payload.config,
        is_active: payload.is_active,
        created_at: None,
        updated_at: None,
    };

    state.loyalty_service.create_program(&claims.organization_id.clone().unwrap_or_default(), program)
        .await
        .map(Json)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_program(
    State(state): State<LoyaltyApiState>,
    Path(program_id): Path<String>,
    claims: axum::extract::Extension<::server_common::Claims>,
) -> Result<Json<LoyaltyProgram>, axum::http::StatusCode> {
    state.loyalty_service.get_program(&claims.organization_id.clone().unwrap_or_default(), &program_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        .and_then(|p| p.map(Json).ok_or(axum::http::StatusCode::NOT_FOUND))
}

#[derive(Deserialize)]
struct EarnReq {
    customer_id: String,
    amount: i32,
    reason: Option<String>,
    order_id: Option<String>,
}

async fn earn_points(
    State(state): State<LoyaltyApiState>,
    Path(program_id): Path<String>,
    claims: axum::extract::Extension<::server_common::Claims>,
    Json(payload): Json<EarnReq>,
) -> Result<Json<LoyaltyTransaction>, axum::http::StatusCode> {
    state.loyalty_service.earn_points(&claims.organization_id.clone().unwrap_or_default(), &program_id, &payload.customer_id, payload.amount, payload.reason, payload.order_id)
        .await
        .map(Json)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Deserialize)]
struct RedeemReq {
    customer_id: String,
    reward_id: String,
    order_id: Option<String>,
}

async fn redeem_reward(
    State(state): State<LoyaltyApiState>,
    Path(program_id): Path<String>,
    claims: axum::extract::Extension<::server_common::Claims>,
    Json(payload): Json<RedeemReq>,
) -> Result<Json<LoyaltyTransaction>, axum::http::StatusCode> {
    state.loyalty_service.redeem_reward(&claims.organization_id.clone().unwrap_or_default(), &program_id, &payload.customer_id, &payload.reward_id, payload.order_id)
        .await
        .map(Json)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_customer_account(
    State(state): State<LoyaltyApiState>,
    Path((program_id, customer_id)): Path<(String, String)>,
    claims: axum::extract::Extension<::server_common::Claims>,
) -> Result<Json<crate::domain::loyalty::CustomerLoyaltyAccount>, axum::http::StatusCode> {
    state.loyalty_service.get_or_create_account(&claims.organization_id.clone().unwrap_or_default(), &program_id, &customer_id)
        .await
        .map(Json)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}
