use axum::{
    extract::{State, Path},
    response::IntoResponse,
    Json,
    Extension,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::db::DB;
use ::server_common::Claims;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoyaltyProgram {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub program_type: String, // points, punch_card, tiers
    pub config: String, // JSON string
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateLoyaltyProgramRequest {
    pub name: String,
    pub program_type: String,
    pub config: String,
}

pub async fn create_program_handler(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateLoyaltyProgramRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_default();
    if tenant_id.is_empty() {
        return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response();
    }

    let program_id = uuid::Uuid::new_v4().to_string();
    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to begin transaction: {}", e)}))).into_response(),
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to set org context: {}", e)}))).into_response();
    }

    match sqlx::query(
        "INSERT INTO loyalty_programs (id, tenant_id, name, program_type, config, is_active) VALUES ($1, $2, $3, $4, $5, true)"
    )
    .bind(&program_id)
    .bind(&tenant_id)
    .bind(&payload.name)
    .bind(&payload.program_type)
    .bind(&payload.config) // expecting a valid JSON string
    .execute(&mut *tx)
    .await {
        Ok(_) => {
            if let Err(e) = tx.commit().await {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to commit transaction: {}", e)}))).into_response();
            }
            (axum::http::StatusCode::OK, Json(serde_json::json!({"id": program_id}))).into_response()
        },
        Err(e) => {
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to create loyalty program: {}", e)}))).into_response()
        }
    }
}

// Additional handlers like get_programs, get_customer_account, earn_points, redeem_reward, etc.
// should be implemented here.


#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct CustomerLoyaltyAccount {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub program_id: String,
    pub points_balance: i32,
    pub punches: i32,
    pub tier_name: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub async fn get_customer_account_handler(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(customer_id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_default();
    if tenant_id.is_empty() {
        return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response();
    }

    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to begin transaction: {}", e)}))).into_response(),
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to set org context: {}", e)}))).into_response();
    }

    match sqlx::query_as::<_, CustomerLoyaltyAccount>(
        "SELECT * FROM customer_loyalty_accounts WHERE tenant_id = $1 AND customer_id = $2"
    )
    .bind(&tenant_id)
    .bind(&customer_id)
    .fetch_all(&mut *tx)
    .await {
        Ok(accounts) => {
            if let Err(e) = tx.commit().await {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to commit transaction: {}", e)}))).into_response();
            }
            (axum::http::StatusCode::OK, Json(accounts)).into_response()
        },
        Err(e) => {
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to fetch customer account: {}", e)}))).into_response()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EarnPointsRequest {
    pub account_id: String,
    pub points: i32,
    pub punches: i32,
    pub reason: String,
}

pub async fn earn_points_handler(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<EarnPointsRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_default();
    if tenant_id.is_empty() {
        return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response();
    }

    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to begin transaction: {}", e)}))).into_response(),
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to set org context: {}", e)}))).into_response();
    }

    let tx_id = uuid::Uuid::new_v4().to_string();

    // Insert transaction
    if let Err(e) = sqlx::query(
        "INSERT INTO loyalty_transactions (id, tenant_id, account_id, transaction_type, points, punches, reason) VALUES ($1, $2, $3, 'earn', $4, $5, $6)"
    )
    .bind(&tx_id)
    .bind(&tenant_id)
    .bind(&payload.account_id)
    .bind(payload.points)
    .bind(payload.punches)
    .bind(&payload.reason)
    .execute(&mut *tx)
    .await {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to create transaction: {}", e)}))).into_response();
    }

    // Update account balance
    match sqlx::query(
        "UPDATE customer_loyalty_accounts SET points_balance = points_balance + $1, punches = punches + $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND tenant_id = $4"
    )
    .bind(payload.points)
    .bind(payload.punches)
    .bind(&payload.account_id)
    .bind(&tenant_id)
    .execute(&mut *tx)
    .await {
        Ok(_) => {
            if let Err(e) = tx.commit().await {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to commit transaction: {}", e)}))).into_response();
            }

            // Trigger an orchestration event here in a real implementation
            // queue.dispatch(...)

            (axum::http::StatusCode::OK, Json(serde_json::json!({"status": "success"}))).into_response()
        },
        Err(e) => {
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to update account: {}", e)}))).into_response()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RedeemRewardRequest {
    pub account_id: String,
    pub reward_id: String,
    pub points_cost: i32,
    pub punches_cost: i32,
}

pub async fn redeem_reward_handler(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<RedeemRewardRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_default();
    if tenant_id.is_empty() {
        return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response();
    }

    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to begin transaction: {}", e)}))).into_response(),
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to set org context: {}", e)}))).into_response();
    }

    // Update account balance
    match sqlx::query(
        "UPDATE customer_loyalty_accounts SET points_balance = points_balance - $1, punches = punches - $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND tenant_id = $4 AND points_balance >= $1 AND punches >= $2"
    )
    .bind(payload.points_cost)
    .bind(payload.punches_cost)
    .bind(&payload.account_id)
    .bind(&tenant_id)
    .execute(&mut *tx)
    .await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                return (axum::http::StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Insufficient points or punches"}))).into_response();
            }
        },
        Err(e) => {
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to update account: {}", e)}))).into_response();
        }
    }

    let tx_id = uuid::Uuid::new_v4().to_string();

    // Insert transaction
    if let Err(e) = sqlx::query(
        "INSERT INTO loyalty_transactions (id, tenant_id, account_id, transaction_type, points, punches, reason) VALUES ($1, $2, $3, 'redeem', $4, $5, $6)"
    )
    .bind(&tx_id)
    .bind(&tenant_id)
    .bind(&payload.account_id)
    .bind(-payload.points_cost)
    .bind(-payload.punches_cost)
    .bind(format!("Redeemed reward {}", payload.reward_id))
    .execute(&mut *tx)
    .await {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to create transaction: {}", e)}))).into_response();
    }

    if let Err(e) = tx.commit().await {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to commit transaction: {}", e)}))).into_response();
    }

    (axum::http::StatusCode::OK, Json(serde_json::json!({"status": "success"}))).into_response()
}

pub fn router() -> axum::Router<Arc<DB>> {
    axum::Router::new()
        .route("/programs", axum::routing::post(create_program_handler))
        .route("/accounts/:customer_id", axum::routing::get(get_customer_account_handler))
        .route("/points/earn", axum::routing::post(earn_points_handler))
        .route("/points/redeem", axum::routing::post(redeem_reward_handler))
}
