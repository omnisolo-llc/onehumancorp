use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use crate::hub::Hub;
use crate::services::loyalty::LoyaltyService;

#[derive(Deserialize)]
pub struct CreateProgramReq {
    pub name: String,
    pub program_type: String,
    pub config: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct EarnPointsReq {
    pub amount: i32,
    pub reason: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateRewardReq {
    pub name: String,
    pub description: Option<String>,
    pub cost: i32,
    pub reward_type: String,
}

#[derive(Deserialize)]
pub struct RedeemRewardReq {
    pub reward_id: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/v1/tenants/{tenant_id}/loyalty/programs", post(create_program).get(list_programs))
        .route("/api/v1/tenants/{tenant_id}/loyalty/programs/{program_id}", get(get_program))
        .route("/api/v1/tenants/{tenant_id}/loyalty/programs/{program_id}/customers/{customer_id}", get(get_account))
        .route("/api/v1/tenants/{tenant_id}/loyalty/programs/{program_id}/customers/{customer_id}/earn", post(earn_points))
        .route("/api/v1/tenants/{tenant_id}/loyalty/programs/{program_id}/rewards", post(create_reward).get(get_rewards))
        .route("/api/v1/tenants/{tenant_id}/loyalty/programs/{program_id}/customers/{customer_id}/redeem", post(redeem_reward))
}

async fn create_program(
    Path(tenant_id): Path<String>,
    Extension(hub): Extension<Arc<Hub>>,
    Json(payload): Json<CreateProgramReq>,
) -> impl IntoResponse {
    let service = LoyaltyService::new(hub);
    let config = payload.config.unwrap_or_else(|| serde_json::json!({}));
    match service.create_program(&tenant_id, &payload.name, &payload.program_type, config).await {
        Ok(program) => (StatusCode::CREATED, Json(program)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn list_programs(
    Path(tenant_id): Path<String>,
    Extension(hub): Extension<Arc<Hub>>,
) -> impl IntoResponse {
    let service = LoyaltyService::new(hub);
    match service.list_programs(&tenant_id).await {
        Ok(programs) => (StatusCode::OK, Json(programs)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn get_program(
    Path((tenant_id, program_id)): Path<(String, String)>,
    Extension(hub): Extension<Arc<Hub>>,
) -> impl IntoResponse {
    let service = LoyaltyService::new(hub);
    match service.get_program(&tenant_id, &program_id).await {
        Ok(program) => (StatusCode::OK, Json(program)).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, e).into_response(),
    }
}

async fn get_account(
    Path((tenant_id, program_id, customer_id)): Path<(String, String, String)>,
    Extension(hub): Extension<Arc<Hub>>,
) -> impl IntoResponse {
    let service = LoyaltyService::new(hub);
    match service.get_account(&tenant_id, &program_id, &customer_id).await {
        Ok(account) => (StatusCode::OK, Json(account)).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, e).into_response(),
    }
}

async fn earn_points(
    Path((tenant_id, program_id, customer_id)): Path<(String, String, String)>,
    Extension(hub): Extension<Arc<Hub>>,
    Json(payload): Json<EarnPointsReq>,
) -> impl IntoResponse {
    let service = LoyaltyService::new(hub);
    match service.earn_points(&tenant_id, &program_id, &customer_id, payload.amount, payload.reason).await {
        Ok(account) => (StatusCode::OK, Json(account)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn create_reward(
    Path((tenant_id, program_id)): Path<(String, String)>,
    Extension(hub): Extension<Arc<Hub>>,
    Json(payload): Json<CreateRewardReq>,
) -> impl IntoResponse {
    let service = LoyaltyService::new(hub);
    match service.create_reward(&tenant_id, &program_id, &payload.name, payload.description, payload.cost, &payload.reward_type).await {
        Ok(reward) => (StatusCode::CREATED, Json(reward)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn get_rewards(
    Path((tenant_id, program_id)): Path<(String, String)>,
    Extension(hub): Extension<Arc<Hub>>,
) -> impl IntoResponse {
    let service = LoyaltyService::new(hub);
    match service.get_rewards(&tenant_id, &program_id).await {
        Ok(rewards) => (StatusCode::OK, Json(rewards)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn redeem_reward(
    Path((tenant_id, program_id, customer_id)): Path<(String, String, String)>,
    Extension(hub): Extension<Arc<Hub>>,
    Json(payload): Json<RedeemRewardReq>,
) -> impl IntoResponse {
    let service = LoyaltyService::new(hub);
    match service.redeem_reward(&tenant_id, &program_id, &customer_id, &payload.reward_id).await {
        Ok(account) => (StatusCode::OK, Json(account)).into_response(),
        Err(e) => {
            if e == "Insufficient points" {
                (StatusCode::BAD_REQUEST, e).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, e).into_response()
            }
        }
    }
}
