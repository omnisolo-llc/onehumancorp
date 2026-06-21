use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::DB;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DB>,
}

#[derive(Deserialize)]
pub struct CreateLoyaltyProgramRequest {
    pub tenant_id: String,
    pub name: String,
    pub program_type: String,
    pub config: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct LoyaltyProgramResponse {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub program_type: String,
    pub config: serde_json::Value,
    pub is_active: bool,
}

#[derive(Deserialize)]
pub struct UpdateLoyaltyProgramRequest {
    pub tenant_id: String,
    pub name: Option<String>,
    pub config: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

#[derive(Deserialize)]
pub struct EarnPointsRequest {
    pub tenant_id: String,
    pub customer_id: String,
    pub program_id: String,
    pub points: i32,
    pub punches: i32,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct RedeemRewardRequest {
    pub tenant_id: String,
    pub customer_id: String,
    pub program_id: String,
    pub reward_id: String,
}

#[derive(Deserialize)]
pub struct CreateRewardRequest {
    pub tenant_id: String,
    pub program_id: String,
    pub name: String,
    pub description: Option<String>,
    pub points_cost: i32,
    pub punches_cost: i32,
    pub reward_type: String,
    pub reward_value: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct RewardResponse {
    pub id: String,
    pub tenant_id: String,
    pub program_id: String,
    pub name: String,
    pub points_cost: i32,
    pub punches_cost: i32,
    pub reward_type: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/loyalty/programs", post(create_loyalty_program))
        .route("/api/v1/loyalty/programs/{id}", put(update_loyalty_program))
        .route("/api/v1/loyalty/earn", post(earn_points))
        .route("/api/v1/loyalty/redeem", post(redeem_reward))
        .route("/api/v1/loyalty/status/{tenant_id}/{customer_id}/{program_id}", get(get_loyalty_status))
        .route("/api/v1/loyalty/rewards", post(create_reward))
}

async fn create_reward(
    State(state): State<AppState>,
    Json(payload): Json<CreateRewardRequest>,
) -> impl IntoResponse {
    let mut tx = match state.db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&payload.tenant_id)
        .execute(&mut *tx)
        .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    let id = Uuid::new_v4().to_string();
    let value = payload.reward_value.unwrap_or_else(|| serde_json::json!({}));

    if let Err(e) = sqlx::query(
        "INSERT INTO rewards (id, tenant_id, program_id, name, description, points_cost, punches_cost, reward_type, reward_value) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
    )
    .bind(&id)
    .bind(&payload.tenant_id)
    .bind(&payload.program_id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(payload.points_cost)
    .bind(payload.punches_cost)
    .bind(&payload.reward_type)
    .bind(&value)
    .execute(&mut *tx)
    .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    if let Err(e) = tx.commit().await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    (
        StatusCode::CREATED,
        Json(RewardResponse {
            id,
            tenant_id: payload.tenant_id,
            program_id: payload.program_id,
            name: payload.name,
            points_cost: payload.points_cost,
            punches_cost: payload.punches_cost,
            reward_type: payload.reward_type,
        }),
    )
        .into_response()
}

async fn create_loyalty_program(
    State(state): State<AppState>,
    Json(payload): Json<CreateLoyaltyProgramRequest>,
) -> impl IntoResponse {
    let mut tx = match state.db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&payload.tenant_id)
        .execute(&mut *tx)
        .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    let id = Uuid::new_v4().to_string();
    let config = payload.config.unwrap_or_else(|| serde_json::json!({}));

    if let Err(e) = sqlx::query(
        "INSERT INTO loyalty_programs (id, tenant_id, name, program_type, config, is_active) VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(&id)
    .bind(&payload.tenant_id)
    .bind(&payload.name)
    .bind(&payload.program_type)
    .bind(&config)
    .bind(true)
    .execute(&mut *tx)
    .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    if let Err(e) = tx.commit().await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    (
        StatusCode::CREATED,
        Json(LoyaltyProgramResponse {
            id,
            tenant_id: payload.tenant_id,
            name: payload.name,
            program_type: payload.program_type,
            config,
            is_active: true,
        }),
    )
        .into_response()
}

async fn update_loyalty_program(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateLoyaltyProgramRequest>,
) -> impl IntoResponse {
    let mut tx = match state.db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&payload.tenant_id)
        .execute(&mut *tx)
        .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    let mut update_query = "UPDATE loyalty_programs SET updated_at = CURRENT_TIMESTAMP".to_string();
    #[allow(unused_assignments)]
    let mut bind_index = 3;

    if payload.name.is_some() {
        update_query.push_str(&format!(", name = ${}", bind_index));
        bind_index += 1;
    }
    if payload.config.is_some() {
        update_query.push_str(&format!(", config = ${}", bind_index));
        bind_index += 1;
    }
    if payload.is_active.is_some() {
        update_query.push_str(&format!(", is_active = ${}", bind_index));
        bind_index += 1;
    }

    update_query.push_str(" WHERE id = $1 AND tenant_id = $2 RETURNING *");

    let mut query = sqlx::query(&update_query)
        .bind(&id)
        .bind(&payload.tenant_id);

    if let Some(ref name) = payload.name {
        query = query.bind(name);
    }
    if let Some(ref config) = payload.config {
        query = query.bind(config);
    }
    if let Some(is_active) = payload.is_active {
        query = query.bind(is_active);
    }

    match query.fetch_one(&mut *tx).await {
        Ok(row) => {
            if let Err(e) = tx.commit().await {
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
            use sqlx::Row;
            (
                StatusCode::OK,
                Json(LoyaltyProgramResponse {
                    id: row.get("id"),
                    tenant_id: row.get("tenant_id"),
                    name: row.get("name"),
                    program_type: row.get("program_type"),
                    config: row.get("config"),
                    is_active: row.get("is_active"),
                }),
            )
                .into_response()
        }
        Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
    }
}

async fn earn_points(
    State(state): State<AppState>,
    Json(payload): Json<EarnPointsRequest>,
) -> impl IntoResponse {
    let mut tx = match state.db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&payload.tenant_id)
        .execute(&mut *tx)
        .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    let tx_id = Uuid::new_v4().to_string();

    if let Err(e) = sqlx::query(
        "INSERT INTO loyalty_transactions (id, tenant_id, customer_id, program_id, transaction_type, points, punches, description) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    )
    .bind(&tx_id)
    .bind(&payload.tenant_id)
    .bind(&payload.customer_id)
    .bind(&payload.program_id)
    .bind("earn")
    .bind(payload.points)
    .bind(payload.punches)
    .bind(&payload.description)
    .execute(&mut *tx)
    .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    let account_id = Uuid::new_v4().to_string();
    if let Err(e) = sqlx::query(
        r#"
        INSERT INTO customer_loyalty_accounts (id, tenant_id, customer_id, program_id, points_balance, punches)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (tenant_id, customer_id, program_id)
        DO UPDATE SET points_balance = customer_loyalty_accounts.points_balance + EXCLUDED.points_balance,
                      punches = customer_loyalty_accounts.punches + EXCLUDED.punches,
                      updated_at = CURRENT_TIMESTAMP
        "#
    )
    .bind(&account_id)
    .bind(&payload.tenant_id)
    .bind(&payload.customer_id)
    .bind(&payload.program_id)
    .bind(payload.points)
    .bind(payload.punches)
    .execute(&mut *tx)
    .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    if let Err(e) = tx.commit().await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    (StatusCode::OK, "Points earned successfully").into_response()
}

async fn redeem_reward(
    State(state): State<AppState>,
    Json(payload): Json<RedeemRewardRequest>,
) -> impl IntoResponse {
    let mut tx = match state.db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&payload.tenant_id)
        .execute(&mut *tx)
        .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    // 1. Fetch the reward
    use sqlx::Row;
    let reward_row = match sqlx::query("SELECT * FROM rewards WHERE id = $1 AND tenant_id = $2 AND program_id = $3")
        .bind(&payload.reward_id)
        .bind(&payload.tenant_id)
        .bind(&payload.program_id)
        .fetch_optional(&mut *tx)
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => return (StatusCode::NOT_FOUND, "Reward not found").into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let points_cost: i32 = reward_row.get("points_cost");
    let punches_cost: i32 = reward_row.get("punches_cost");

    // 2. Fetch current balance
    let account_row = match sqlx::query("SELECT * FROM customer_loyalty_accounts WHERE tenant_id = $1 AND customer_id = $2 AND program_id = $3")
        .bind(&payload.tenant_id)
        .bind(&payload.customer_id)
        .bind(&payload.program_id)
        .fetch_optional(&mut *tx)
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => return (StatusCode::NOT_FOUND, "Customer account not found").into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let current_points: i32 = account_row.get("points_balance");
    let current_punches: i32 = account_row.get("punches");

    if current_points < points_cost || current_punches < punches_cost {
        return (StatusCode::BAD_REQUEST, "Insufficient balance").into_response();
    }

    // 3. Deduct points/punches
    if let Err(e) = sqlx::query(
        "UPDATE customer_loyalty_accounts SET points_balance = points_balance - $1, punches = punches - $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3"
    )
    .bind(points_cost)
    .bind(punches_cost)
    .bind(account_row.get::<String, _>("id"))
    .execute(&mut *tx)
    .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    // 4. Record transaction
    let tx_id = Uuid::new_v4().to_string();
    if let Err(e) = sqlx::query(
        "INSERT INTO loyalty_transactions (id, tenant_id, customer_id, program_id, transaction_type, points, punches, description) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    )
    .bind(&tx_id)
    .bind(&payload.tenant_id)
    .bind(&payload.customer_id)
    .bind(&payload.program_id)
    .bind("redeem")
    .bind(-points_cost)
    .bind(-punches_cost)
    .bind(format!("Redeemed reward: {}", reward_row.get::<String, _>("name")))
    .execute(&mut *tx)
    .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    if let Err(e) = tx.commit().await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    (StatusCode::OK, "Reward redeemed successfully").into_response()
}

async fn get_loyalty_status(
    State(state): State<AppState>,
    Path((tenant_id, customer_id, program_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    let mut tx = match state.db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    match sqlx::query("SELECT * FROM customer_loyalty_accounts WHERE tenant_id = $1 AND customer_id = $2 AND program_id = $3")
        .bind(&tenant_id)
        .bind(&customer_id)
        .bind(&program_id)
        .fetch_optional(&mut *tx)
        .await
    {
        Ok(Some(row)) => {
            use sqlx::Row;
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "id": row.get::<String, _>("id"),
                    "tenant_id": row.get::<String, _>("tenant_id"),
                    "customer_id": row.get::<String, _>("customer_id"),
                    "program_id": row.get::<String, _>("program_id"),
                    "points_balance": row.get::<i32, _>("points_balance"),
                    "punches": row.get::<i32, _>("punches"),
                    "tier_name": row.get::<Option<String>, _>("tier_name"),
                })),
            )
                .into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Customer account not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
