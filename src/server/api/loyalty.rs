use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::{Pool, Postgres};

use crate::services::loyalty::engine;

use std::sync::Arc;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub orchestrator: Option<Arc<DepartmentOrchestrator>>,
}

#[derive(Deserialize)]
pub struct CreateProgramRequest {
    pub tenant_id: String,
    pub name: String,
    pub program_type: String,
    #[serde(default)]
    pub config: JsonValue,
}

#[derive(Serialize)]
pub struct CreateProgramResponse {
    pub id: String,
}

pub async fn create_program_handler(
    State(state): State<AppState>,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
    Json(mut payload): Json<CreateProgramRequest>,
) -> Result<Json<CreateProgramResponse>, axum::http::StatusCode> {
    if !auth_info.spiffe_id.is_empty() {
        payload.tenant_id = auth_info.spiffe_id.clone();
    } else {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }
    match engine::create_loyalty_program(&state.pool, &payload.tenant_id, &payload.name, &payload.program_type, payload.config).await {
        Ok(id) => Ok(Json(CreateProgramResponse { id })),
        Err(e) => {
            tracing::error!("Failed to create program: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct GetProgramsQuery {
    pub tenant_id: String,
}

pub async fn get_programs_handler(
    State(state): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<GetProgramsQuery>,
) -> Result<Json<Vec<JsonValue>>, axum::http::StatusCode> {
    match engine::get_loyalty_programs(&state.pool, &query.tenant_id).await {
        Ok(programs) => Ok(Json(programs)),
        Err(e) => {
            tracing::error!("Failed to fetch programs: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct EnrollCustomerRequest {
    pub tenant_id: String,
    pub program_id: String,
    pub customer_id: String,
}

#[derive(Serialize)]
pub struct EnrollCustomerResponse {
    pub account_id: String,
}

pub async fn enroll_customer_handler(
    State(state): State<AppState>,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
    Json(mut payload): Json<EnrollCustomerRequest>,
) -> Result<Json<EnrollCustomerResponse>, axum::http::StatusCode> {
    if !auth_info.spiffe_id.is_empty() {
        payload.tenant_id = auth_info.spiffe_id.clone();
    } else {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }
    match engine::enroll_customer(&state.pool, &payload.tenant_id, &payload.program_id, &payload.customer_id).await {
        Ok(account_id) => Ok(Json(EnrollCustomerResponse { account_id })),
        Err(e) => {
            tracing::error!("Failed to enroll customer: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct GetAccountQuery {
    pub tenant_id: String,
    pub program_id: String,
    pub customer_id: String,
}

pub async fn get_account_handler(
    State(state): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<GetAccountQuery>,
) -> Result<Json<Option<JsonValue>>, axum::http::StatusCode> {
    match engine::get_customer_account(&state.pool, &query.tenant_id, &query.program_id, &query.customer_id).await {
        Ok(account) => Ok(Json(account)),
        Err(e) => {
            tracing::error!("Failed to fetch account: {}", e); // pii-safe
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct RecordTransactionRequest {
    pub tenant_id: String,
    pub account_id: String,
    pub transaction_type: String,
    pub amount: i32,
    pub reason: Option<String>,
}

#[derive(Serialize)]
pub struct RecordTransactionResponse {
    pub success: bool,
}

pub async fn record_transaction_handler(
    State(state): State<AppState>,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
    Json(mut payload): Json<RecordTransactionRequest>,
) -> Result<Json<RecordTransactionResponse>, axum::http::StatusCode> {
    if !auth_info.spiffe_id.is_empty() {
        payload.tenant_id = auth_info.spiffe_id.clone();
    } else {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }
    match engine::record_transaction(&state.pool, &payload.tenant_id, &payload.account_id, &payload.transaction_type, payload.amount, payload.reason.as_deref(), state.orchestrator.clone()).await {
        Ok(_) => Ok(Json(RecordTransactionResponse { success: true })),
        Err(e) => {
            tracing::error!("Failed to record transaction: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct CreateRewardRequest {
    pub tenant_id: String,
    pub program_id: String,
    pub name: String,
    pub description: Option<String>,
    pub cost_in_points: i32,
    pub reward_type: String,
    #[serde(default)]
    pub reward_value: JsonValue,
}

#[derive(Serialize)]
pub struct CreateRewardResponse {
    pub id: String,
}

pub async fn create_reward_handler(
    State(state): State<AppState>,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
    Json(mut payload): Json<CreateRewardRequest>,
) -> Result<Json<CreateRewardResponse>, axum::http::StatusCode> {
    if !auth_info.spiffe_id.is_empty() {
        payload.tenant_id = auth_info.spiffe_id.clone();
    } else {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }
    match engine::create_reward(&state.pool, &payload.tenant_id, &payload.program_id, &payload.name, payload.description.as_deref(), payload.cost_in_points, &payload.reward_type, payload.reward_value).await {
        Ok(id) => Ok(Json(CreateRewardResponse { id })),
        Err(e) => {
            tracing::error!("Failed to create reward: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct GetRewardsQuery {
    pub tenant_id: String,
    pub program_id: String,
}

pub async fn get_rewards_handler(
    State(state): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<GetRewardsQuery>,
) -> Result<Json<Vec<JsonValue>>, axum::http::StatusCode> {
    match engine::get_rewards(&state.pool, &query.tenant_id, &query.program_id).await {
        Ok(rewards) => Ok(Json(rewards)),
        Err(e) => {
            tracing::error!("Failed to fetch rewards: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub fn router<S: Clone + Send + Sync + 'static>(pool: Pool<Postgres>, orchestrator: Option<Arc<DepartmentOrchestrator>>) -> Router<S> {
    let state = AppState { pool, orchestrator };
    Router::new()
        .route("/programs", post(create_program_handler).get(get_programs_handler))
        .route("/accounts", post(enroll_customer_handler).get(get_account_handler))
        .route("/transactions", post(record_transaction_handler))
        .route("/rewards", post(create_reward_handler).get(get_rewards_handler))
        .with_state(state)
}
