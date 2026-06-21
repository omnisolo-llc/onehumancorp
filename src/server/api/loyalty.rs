use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use sqlx::{PgPool, Row};
use ::server_common::Claims;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::DepartmentEvent;

pub fn router(dept_orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Router<PgPool> {
    let app_state = LoyaltyAppState { dept_orchestrator };
    Router::new()
        .route("/programs", post(create_program).get(list_programs))
        .route("/accounts/{customer_id}", get(get_account))
        .route("/accounts/{customer_id}/earn", post(earn_points))
        .route("/accounts/{customer_id}/redeem", post(redeem_points))
        .route("/programs/{program_id}/rewards", post(create_reward).get(list_rewards))
        .layer(Extension(app_state))
}

#[derive(Clone)]
pub struct LoyaltyAppState {
    pub dept_orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

#[derive(Deserialize)]
pub struct CreateProgramRequest {
    pub name: String,
    pub program_type: String, // 'points', 'punch_card', 'tiers'
    pub config: serde_json::Value,
}

#[derive(Serialize)]
pub struct ProgramResponse {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub program_type: String,
    pub config: serde_json::Value,
    pub is_active: bool,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct EarnPointsRequest {
    pub program_id: String,
    pub points: i32,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct RedeemPointsRequest {
    pub program_id: String,
    pub points: i32,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateRewardRequest {
    pub name: String,
    pub description: Option<String>,
    pub points_cost: i32,
}

async fn create_program(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateProgramRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let program_id = Uuid::new_v4().to_string();

    let q = r#"
        INSERT INTO loyalty_programs (id, tenant_id, name, program_type, config, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
        RETURNING id, name, program_type, config, is_active, created_at
    "#;

    let res = sqlx::query(q)
        .bind(&program_id)
        .bind(&tenant_id)
        .bind(&payload.name)
        .bind(&payload.program_type)
        .bind(sqlx::types::Json(&payload.config))
        .fetch_one(&pool)
        .await;

    match res {
        Ok(row) => {
            let created_at: chrono::DateTime<chrono::Utc> = row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now());
            let config: sqlx::types::Json<serde_json::Value> = row.try_get("config").unwrap_or_else(|_| sqlx::types::Json(serde_json::json!({})));
            let response = ProgramResponse {
                id: row.try_get("id").unwrap_or_default(),
                tenant_id,
                name: row.try_get("name").unwrap_or_default(),
                program_type: row.try_get("program_type").unwrap_or_default(),
                config: config.0,
                is_active: row.try_get("is_active").unwrap_or(true),
                created_at: created_at.to_rfc3339(),
            };
            (StatusCode::CREATED, Json(serde_json::json!(response))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create loyalty program: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create program").into_response()
        }
    }
}

async fn list_programs(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let q = "SELECT * FROM loyalty_programs WHERE tenant_id = $1";
    let rows = match sqlx::query(q).bind(&tenant_id).fetch_all(&pool).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Failed to list loyalty programs: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch programs").into_response();
        }
    };

    let mut programs = Vec::new();
    for row in rows {
        let created_at: chrono::DateTime<chrono::Utc> = row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now());
        let config: sqlx::types::Json<serde_json::Value> = row.try_get("config").unwrap_or_else(|_| sqlx::types::Json(serde_json::json!({})));
        programs.push(ProgramResponse {
            id: row.try_get("id").unwrap_or_default(),
            tenant_id: tenant_id.clone(),
            name: row.try_get("name").unwrap_or_default(),
            program_type: row.try_get("program_type").unwrap_or_default(),
            config: config.0,
            is_active: row.try_get("is_active").unwrap_or(true),
            created_at: created_at.to_rfc3339(),
        });
    }

    (StatusCode::OK, Json(serde_json::json!({ "programs": programs }))).into_response()
}

async fn get_account(
    State(pool): State<PgPool>,
    Path(customer_id): Path<String>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let q = "SELECT * FROM customer_loyalty_accounts WHERE tenant_id = $1 AND customer_id = $2";
    let rows = match sqlx::query(q).bind(&tenant_id).bind(&customer_id).fetch_all(&pool).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Failed to get account: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch account").into_response();
        }
    };

    let mut accounts = Vec::new();
    for row in rows {
        let created_at: chrono::DateTime<chrono::Utc> = row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now());
        accounts.push(serde_json::json!({
            "id": row.try_get::<String, _>("id").unwrap_or_default(),
            "customer_id": row.try_get::<String, _>("customer_id").unwrap_or_default(),
            "program_id": row.try_get::<String, _>("program_id").unwrap_or_default(),
            "points_balance": row.try_get::<i32, _>("points_balance").unwrap_or(0),
            "tier_name": row.try_get::<Option<String>, _>("tier_name").unwrap_or_default(),
            "created_at": created_at.to_rfc3339()
        }));
    }

    (StatusCode::OK, Json(serde_json::json!({ "accounts": accounts }))).into_response()
}

async fn earn_points(
    State(pool): State<PgPool>,
    Extension(app_state): Extension<LoyaltyAppState>,
    Path(customer_id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<EarnPointsRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let tx_id = Uuid::new_v4().to_string();

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to begin tx: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to earn points").into_response();
        }
    };

    // Ensure account exists
    let account_id = Uuid::new_v4().to_string();
    let q_upsert_account = r#"
        INSERT INTO customer_loyalty_accounts (id, tenant_id, customer_id, program_id, points_balance, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        ON CONFLICT (tenant_id, customer_id, program_id) DO UPDATE
        SET points_balance = customer_loyalty_accounts.points_balance + EXCLUDED.points_balance, updated_at = NOW()
        RETURNING points_balance
    "#;

    let points_balance: i32 = match sqlx::query(q_upsert_account)
        .bind(&account_id)
        .bind(&tenant_id)
        .bind(&customer_id)
        .bind(&payload.program_id)
        .bind(&payload.points)
        .fetch_one(&mut *tx)
        .await {
            Ok(row) => row.try_get("points_balance").unwrap_or(payload.points),
            Err(e) => {
                let _ = tx.rollback().await;
                tracing::error!("Failed to update account balance: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to earn points").into_response();
            }
        };

    // Insert transaction
    let q_tx = r#"
        INSERT INTO loyalty_transactions (id, tenant_id, customer_id, program_id, transaction_type, points, description, created_at)
        VALUES ($1, $2, $3, $4, 'earn', $5, $6, NOW())
    "#;
    if let Err(e) = sqlx::query(q_tx)
        .bind(&tx_id)
        .bind(&tenant_id)
        .bind(&customer_id)
        .bind(&payload.program_id)
        .bind(&payload.points)
        .bind(&payload.description)
        .execute(&mut *tx)
        .await {
            let _ = tx.rollback().await;
            tracing::error!("Failed to record tx: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to earn points").into_response();
        }

    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit tx: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to earn points").into_response();
    }

    // Trigger AI orchestration event
    let event = DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: tenant_id.clone(),
        event_type: "loyalty.points_awarded".to_string(),
        payload: serde_json::json!({
            "customer_id": customer_id,
            "points": payload.points,
            "total_points": points_balance,
            "program_id": payload.program_id
        }),
    };

    // Spawn task so we do not block response
    let orchestrator = app_state.dept_orchestrator.clone();
    tokio::spawn(async move {
        if let Err(e) = orchestrator.dispatch_event(event).await {
            tracing::error!("Failed to handle loyalty event: {}", e);
        }
    });

    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "points_balance": points_balance,
        "transaction_id": tx_id
    }))).into_response()
}

async fn redeem_points(
    State(pool): State<PgPool>,
    Path(customer_id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<RedeemPointsRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let tx_id = Uuid::new_v4().to_string();

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to begin tx: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to redeem points").into_response();
        }
    };

    let q_check_balance = "SELECT points_balance FROM customer_loyalty_accounts WHERE tenant_id = $1 AND customer_id = $2 AND program_id = $3";
    let current_balance = match sqlx::query(q_check_balance)
        .bind(&tenant_id)
        .bind(&customer_id)
        .bind(&payload.program_id)
        .fetch_optional(&mut *tx)
        .await {
            Ok(Some(row)) => row.try_get::<i32, _>("points_balance").unwrap_or(0),
            _ => 0
        };

    if current_balance < payload.points {
        let _ = tx.rollback().await;
        return (StatusCode::BAD_REQUEST, "Insufficient points").into_response();
    }

    let q_update = r#"
        UPDATE customer_loyalty_accounts
        SET points_balance = points_balance - $1, updated_at = NOW()
        WHERE tenant_id = $2 AND customer_id = $3 AND program_id = $4
        RETURNING points_balance
    "#;

    let new_balance: i32 = match sqlx::query(q_update)
        .bind(&payload.points)
        .bind(&tenant_id)
        .bind(&customer_id)
        .bind(&payload.program_id)
        .fetch_one(&mut *tx)
        .await {
            Ok(row) => row.try_get("points_balance").unwrap_or(0),
            Err(e) => {
                let _ = tx.rollback().await;
                tracing::error!("Failed to update account balance: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to redeem points").into_response();
            }
        };

    let q_tx = r#"
        INSERT INTO loyalty_transactions (id, tenant_id, customer_id, program_id, transaction_type, points, description, created_at)
        VALUES ($1, $2, $3, $4, 'redeem', $5, $6, NOW())
    "#;
    if let Err(e) = sqlx::query(q_tx)
        .bind(&tx_id)
        .bind(&tenant_id)
        .bind(&customer_id)
        .bind(&payload.program_id)
        .bind(&payload.points)
        .bind(&payload.description)
        .execute(&mut *tx)
        .await {
            let _ = tx.rollback().await;
            tracing::error!("Failed to record tx: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to redeem points").into_response();
        }

    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit tx: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to redeem points").into_response();
    }

    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "points_balance": new_balance,
        "transaction_id": tx_id
    }))).into_response()
}

async fn create_reward(
    State(pool): State<PgPool>,
    Path(program_id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateRewardRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let reward_id = Uuid::new_v4().to_string();

    let q = r#"
        INSERT INTO rewards (id, tenant_id, program_id, name, description, points_cost, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
        RETURNING id, name, description, points_cost, is_active
    "#;

    match sqlx::query(q)
        .bind(&reward_id)
        .bind(&tenant_id)
        .bind(&program_id)
        .bind(&payload.name)
        .bind(&payload.description)
        .bind(&payload.points_cost)
        .fetch_one(&pool)
        .await {
            Ok(row) => {
                let response = serde_json::json!({
                    "id": row.try_get::<String, _>("id").unwrap_or_default(),
                    "program_id": program_id,
                    "name": row.try_get::<String, _>("name").unwrap_or_default(),
                    "description": row.try_get::<Option<String>, _>("description").unwrap_or_default(),
                    "points_cost": row.try_get::<i32, _>("points_cost").unwrap_or(0),
                    "is_active": row.try_get::<bool, _>("is_active").unwrap_or(true)
                });
                (StatusCode::CREATED, Json(response)).into_response()
            }
            Err(e) => {
                tracing::error!("Failed to create reward: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create reward").into_response()
            }
        }
}

async fn list_rewards(
    State(pool): State<PgPool>,
    Path(program_id): Path<String>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let q = "SELECT * FROM rewards WHERE tenant_id = $1 AND program_id = $2";
    let rows = match sqlx::query(q).bind(&tenant_id).bind(&program_id).fetch_all(&pool).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Failed to list rewards: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch rewards").into_response();
        }
    };

    let mut rewards = Vec::new();
    for row in rows {
        rewards.push(serde_json::json!({
            "id": row.try_get::<String, _>("id").unwrap_or_default(),
            "program_id": program_id,
            "name": row.try_get::<String, _>("name").unwrap_or_default(),
            "description": row.try_get::<Option<String>, _>("description").unwrap_or_default(),
            "points_cost": row.try_get::<i32, _>("points_cost").unwrap_or(0),
            "is_active": row.try_get::<bool, _>("is_active").unwrap_or(true)
        }));
    }

    (StatusCode::OK, Json(serde_json::json!({ "rewards": rewards }))).into_response()
}
