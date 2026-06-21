use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/programs", post(create_program))
        .route("/programs/{id}", put(update_program))
        .route("/accounts/{customer_id}", get(get_account))
        .route("/accounts/{customer_id}/earn", post(earn_points))
        .route("/accounts/{customer_id}/redeem", post(redeem_reward))
}

#[derive(Deserialize)]
pub struct CreateProgramRequest {
    pub name: String,
    pub program_type: String, // 'POINTS', 'PUNCH_CARD', 'TIERS'
    #[serde(default)]
    pub config: serde_json::Value,
}

#[derive(Deserialize)]
pub struct UpdateProgramRequest {
    pub name: Option<String>,
    pub program_type: Option<String>,
    pub config: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

#[derive(Serialize)]
pub struct ProgramResponse {
    pub id: String,
    pub name: String,
    pub program_type: String,
    pub config: serde_json::Value,
    pub is_active: bool,
}

#[derive(Deserialize)]
pub struct EarnPointsRequest {
    pub points: i32,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct RedeemRewardRequest {
    pub points: i32,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct AccountResponse {
    pub id: String,
    pub customer_id: String,
    pub program_id: String,
    pub points_balance: i32,
    pub punch_count: i32,
    pub tier_name: Option<String>,
}

pub async fn create_program(
    State(pool): State<PgPool>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
    Json(payload): Json<CreateProgramRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let program_id = Uuid::new_v4().to_string();
    let tenant_id = claims.organization_id.unwrap_or_default();

    let config = if payload.config.is_null() {
        serde_json::json!({})
    } else {
        payload.config
    };

    let result = sqlx::query("INSERT INTO loyalty_programs (id, tenant_id, program_type, name, config) VALUES ($1, $2, $3, $4, $5)")
        .bind(program_id.clone())
        .bind(tenant_id.clone())
        .bind(payload.program_type.clone())
        .bind(payload.name.clone())
        .bind(config.clone())
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok((StatusCode::CREATED, Json(ProgramResponse {
            id: program_id,
            name: payload.name,
            program_type: payload.program_type,
            config,
            is_active: true,
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_program(
    Path(id): Path<String>,
    State(pool): State<PgPool>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
    Json(payload): Json<UpdateProgramRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_default();

    let mut query_builder = sqlx::QueryBuilder::new("UPDATE loyalty_programs SET ");
    let mut needs_comma = false;

    if let Some(name) = payload.name {
        query_builder.push("name = ");
        query_builder.push_bind(name);
        needs_comma = true;
    }

    if let Some(program_type) = payload.program_type {
        if needs_comma { query_builder.push(", "); }
        query_builder.push("program_type = ");
        query_builder.push_bind(program_type);
        needs_comma = true;
    }

    if let Some(config) = payload.config {
        if needs_comma { query_builder.push(", "); }
        query_builder.push("config = ");
        query_builder.push_bind(config);
        needs_comma = true;
    }

    if let Some(is_active) = payload.is_active {
        if needs_comma { query_builder.push(", "); }
        query_builder.push("is_active = ");
        query_builder.push_bind(is_active);
    }

    query_builder.push(", updated_at = CURRENT_TIMESTAMP WHERE id = ");
    query_builder.push_bind(id);
    query_builder.push(" AND tenant_id = ");
    query_builder.push_bind(tenant_id);

    let result = query_builder.build().execute(&pool).await;

    match result {
        Ok(res) if res.rows_affected() > 0 => Ok(StatusCode::OK),
        Ok(_) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_account(
    Path(customer_id): Path<String>,
    State(pool): State<PgPool>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_default();

    let result = sqlx::query("SELECT id, customer_id, program_id, points_balance, punch_count, tier_name FROM customer_loyalty_accounts WHERE tenant_id = $1 AND customer_id = $2 LIMIT 1")
        .bind(tenant_id)
        .bind(customer_id)
        .fetch_optional(&pool)
        .await;

    match result {
        Ok(Some(row)) => Ok(Json(AccountResponse {
            id: row.get::<String, _>("id"),
            customer_id: row.get::<String, _>("customer_id"),
            program_id: row.get::<String, _>("program_id"),
            points_balance: row.get::<Option<i32>, _>("points_balance").unwrap_or(0),
            punch_count: row.get::<Option<i32>, _>("punch_count").unwrap_or(0),
            tier_name: row.get::<Option<String>, _>("tier_name"),
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Error fetching loyalty account: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn earn_points(
    Path(customer_id): Path<String>,
    State(pool): State<PgPool>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
    Json(payload): Json<EarnPointsRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_default();
    let tx_id = Uuid::new_v4().to_string();

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let result1 = sqlx::query("INSERT INTO customer_loyalty_accounts (id, tenant_id, customer_id, program_id, points_balance, punch_count, last_updated) VALUES ($2, $3, $4, 'DEFAULT', $1, 1, CURRENT_TIMESTAMP) ON CONFLICT (tenant_id, customer_id, program_id) DO UPDATE SET points_balance = customer_loyalty_accounts.points_balance + EXCLUDED.points_balance, punch_count = customer_loyalty_accounts.punch_count + 1, last_updated = EXCLUDED.last_updated RETURNING id, points_balance")
        .bind(payload.points)
        .bind(Uuid::new_v4().to_string())
        .bind(tenant_id.clone())
        .bind(customer_id.clone())
        .fetch_optional(&mut *tx)
        .await;

    match result1 {
        Ok(Some(row)) => {
            let result2 = sqlx::query("INSERT INTO loyalty_transactions (id, tenant_id, account_id, transaction_type, points, description) VALUES ($1, $2, $3, 'EARN', $4, $5)")
                .bind(tx_id)
                .bind(tenant_id)
                .bind(row.get::<String, _>("id"))
                .bind(payload.points)
                .bind(payload.description.clone())
                .execute(&mut *tx)
                .await;

            if result2.is_ok() && tx.commit().await.is_ok() {
                Ok((StatusCode::OK, Json(serde_json::json!({ "points_balance": row.get::<Option<i32>, _>("points_balance").unwrap_or(0) }))))
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn redeem_reward(
    Path(customer_id): Path<String>,
    State(pool): State<PgPool>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
    Json(payload): Json<RedeemRewardRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_default();
    let tx_id = Uuid::new_v4().to_string();

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let result1 = sqlx::query("UPDATE customer_loyalty_accounts SET points_balance = points_balance - $1, last_updated = CURRENT_TIMESTAMP WHERE customer_id = $2 AND tenant_id = $3 AND program_id = 'DEFAULT' AND points_balance >= $1 RETURNING id, points_balance")
        .bind(payload.points)
        .bind(customer_id.clone())
        .bind(tenant_id.clone())
        .fetch_optional(&mut *tx)
        .await;

    match result1 {
        Ok(Some(row)) => {
            let result2 = sqlx::query("INSERT INTO loyalty_transactions (id, tenant_id, account_id, transaction_type, points, description) VALUES ($1, $2, $3, 'REDEEM', $4, $5)")
                .bind(tx_id)
                .bind(tenant_id)
                .bind(row.get::<String, _>("id"))
                .bind(-payload.points)
                .bind(payload.description.clone())
                .execute(&mut *tx)
                .await;

            if result2.is_ok() && tx.commit().await.is_ok() {
                Ok((StatusCode::OK, Json(serde_json::json!({ "points_balance": row.get::<Option<i32>, _>("points_balance").unwrap_or(0) }))))
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Ok(None) => Err(StatusCode::BAD_REQUEST), // Insufficient points or not found
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
