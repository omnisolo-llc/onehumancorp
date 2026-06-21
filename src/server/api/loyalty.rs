use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use ohc_rust_protos::loyalty::{
    CreateLoyaltyProgramRequest, CreateRewardRequest, EarnPointsRequest,
    GetCustomerAccountRequest, ListLoyaltyProgramsRequest, ListRewardsRequest,
    RedeemRewardRequest, UpdateLoyaltyProgramRequest, UpdateRewardRequest,
};
use ohc_rust_protos::loyalty::loyalty_service_server::LoyaltyService;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::services::loyalty::service::LoyaltyServiceImpl;
use tonic::Request;
use ::server_common::Claims;

#[derive(Clone)]
pub struct LoyaltyState {
    pub service: Arc<LoyaltyServiceImpl>,
}

pub fn loyalty_routes() -> Router<LoyaltyState> {
    Router::new()
        .route("/programs", post(create_program).get(list_programs))
        .route("/programs/:program_id", put(update_program))
        .route("/programs/:program_id/rewards", post(create_reward).get(list_rewards))
        .route("/rewards/:reward_id", put(update_reward))
        .route("/accounts/:program_id/customer/:customer_id", get(get_account))
        .route("/accounts/:program_id/customer/:customer_id/earn", post(earn_points))
        .route("/accounts/:account_id/redeem", post(redeem_reward))
}

async fn create_program(
    State(state): State<LoyaltyState>,
    claims: Claims,
    Json(payload): Json<CreateLoyaltyProgramRequest>,
) -> impl IntoResponse {
    let mut req = payload;
    req.tenant_id = claims.tenant_id;
    match state.service.create_loyalty_program(Request::new(req)).await {
        Ok(res) => Json(res.into_inner()).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.message().to_string()).into_response(),
    }
}

async fn update_program(
    State(state): State<LoyaltyState>,
    claims: Claims,
    Path(program_id): Path<String>,
    Json(payload): Json<UpdateLoyaltyProgramRequest>,
) -> impl IntoResponse {
    let mut req = payload;
    req.tenant_id = claims.tenant_id;
    req.program_id = program_id;
    match state.service.update_loyalty_program(Request::new(req)).await {
        Ok(res) => Json(res.into_inner()).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.message().to_string()).into_response(),
    }
}

async fn list_programs(
    State(state): State<LoyaltyState>,
    claims: Claims,
) -> impl IntoResponse {
    let req = ListLoyaltyProgramsRequest { tenant_id: claims.tenant_id };
    match state.service.list_loyalty_programs(Request::new(req)).await {
        Ok(res) => Json(res.into_inner().programs).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.message().to_string()).into_response(),
    }
}

async fn create_reward(
    State(state): State<LoyaltyState>,
    claims: Claims,
    Path(program_id): Path<String>,
    Json(payload): Json<CreateRewardRequest>,
) -> impl IntoResponse {
    let mut req = payload;
    req.tenant_id = claims.tenant_id;
    req.program_id = program_id;
    match state.service.create_reward(Request::new(req)).await {
        Ok(res) => Json(res.into_inner()).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.message().to_string()).into_response(),
    }
}

async fn update_reward(
    State(state): State<LoyaltyState>,
    claims: Claims,
    Path(reward_id): Path<String>,
    Json(payload): Json<UpdateRewardRequest>,
) -> impl IntoResponse {
    let mut req = payload;
    req.tenant_id = claims.tenant_id;
    req.reward_id = reward_id;
    match state.service.update_reward(Request::new(req)).await {
        Ok(res) => Json(res.into_inner()).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.message().to_string()).into_response(),
    }
}

async fn list_rewards(
    State(state): State<LoyaltyState>,
    claims: Claims,
    Path(program_id): Path<String>,
) -> impl IntoResponse {
    let req = ListRewardsRequest {
        tenant_id: claims.tenant_id,
        program_id,
        only_active: false,
    };
    match state.service.list_rewards(Request::new(req)).await {
        Ok(res) => Json(res.into_inner().rewards).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.message().to_string()).into_response(),
    }
}

async fn get_account(
    State(state): State<LoyaltyState>,
    claims: Claims,
    Path((program_id, customer_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let req = GetCustomerAccountRequest {
        tenant_id: claims.tenant_id,
        program_id,
        customer_id,
    };
    match state.service.get_customer_account(Request::new(req)).await {
        Ok(res) => Json(res.into_inner()).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.message().to_string()).into_response(),
    }
}

async fn earn_points(
    State(state): State<LoyaltyState>,
    claims: Claims,
    Path((program_id, customer_id)): Path<(String, String)>,
    Json(payload): Json<EarnPointsRequest>,
) -> impl IntoResponse {
    let mut req = payload;
    req.tenant_id = claims.tenant_id;
    req.program_id = program_id;
    req.customer_id = customer_id;
    match state.service.earn_points(Request::new(req)).await {
        Ok(res) => Json(res.into_inner()).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.message().to_string()).into_response(),
    }
}

async fn redeem_reward(
    State(state): State<LoyaltyState>,
    claims: Claims,
    Path(account_id): Path<String>,
    Json(payload): Json<RedeemRewardRequest>,
) -> impl IntoResponse {
    let mut req = payload;
    req.tenant_id = claims.tenant_id;
    req.account_id = account_id;
    match state.service.redeem_reward(Request::new(req)).await {
        Ok(res) => Json(res.into_inner()).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.message().to_string()).into_response(),
    }
}
