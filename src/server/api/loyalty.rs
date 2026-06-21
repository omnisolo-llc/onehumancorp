use axum::{extract::{Extension, Path, State}, http::StatusCode, routing::{get, post, put}, Json, Router};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct LoyaltyState {
    pub pool: sqlx::PgPool,
    pub hub: Arc<crate::hub::Hub>,
}

pub fn router<S: Clone + Send + Sync + 'static>(pool: sqlx::PgPool, hub: Arc<crate::hub::Hub>) -> Router<S> {
    let state = LoyaltyState { pool, hub };
    Router::new()
        .route("/programs", post(create_program).get(list_programs))
        .route("/programs/:id", get(get_program).put(update_program))
        .route("/programs/:program_id/rewards", post(create_reward).get(get_rewards))
        .route("/rewards/:id", put(update_reward))
        .route("/customers/:customer_id/status", get(get_status))
        .route("/customers/:customer_id/earn", post(earn_points))
        .route("/customers/:customer_id/redeem", post(redeem_reward))
        .with_state(state)
}

#[derive(Debug, Deserialize)]
pub struct CreateProgramReq {
    pub name: String,
    pub program_type: String,
    pub config_json: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct LoyaltyProgram {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub program_type: String,
    pub config: serde_json::Value,
    pub is_active: bool,
}

async fn create_program(
    State(state): State<LoyaltyState>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Json(req): Json<CreateProgramReq>,
) -> Result<Json<LoyaltyProgram>, StatusCode> {
    let tenant_id = auth_info.org_id;
    let id = uuid::Uuid::new_v4().to_string();
    let config = req.config_json.unwrap_or_else(|| serde_json::json!({}));

    let mut tx = state.pool.begin().await.map_err(|e| { tracing::error!("Tx err: {:?}", e); StatusCode::INTERNAL_SERVER_ERROR})?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let program = sqlx::query_as::<_, LoyaltyProgram>(
        "INSERT INTO loyalty_programs (id, tenant_id, name, program_type, config, is_active) VALUES ($1, $2, $3, $4, $5, true) RETURNING id, tenant_id, name, program_type, config, is_active"
    )
    .bind(&id)
    .bind(&tenant_id)
    .bind(&req.name)
    .bind(&req.program_type)
    .bind(&config)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Error creating loyalty program: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(program))
}

async fn list_programs(
    State(state): State<LoyaltyState>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<Vec<LoyaltyProgram>>, StatusCode> {
    let tenant_id = auth_info.org_id;
    let mut tx = state.pool.begin().await.map_err(|e| { tracing::error!("Tx err: {:?}", e); StatusCode::INTERNAL_SERVER_ERROR})?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let programs = sqlx::query_as::<_, LoyaltyProgram>(
        "SELECT id, tenant_id, name, program_type, config, is_active FROM loyalty_programs WHERE tenant_id = $1"
    )
    .bind(&tenant_id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Error listing loyalty programs: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(programs))
}

async fn get_program(
    State(state): State<LoyaltyState>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Path(id): Path<String>,
) -> Result<Json<LoyaltyProgram>, StatusCode> {
    let tenant_id = auth_info.org_id;
    let mut tx = state.pool.begin().await.map_err(|e| { tracing::error!("Tx err: {:?}", e); StatusCode::INTERNAL_SERVER_ERROR})?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let program = sqlx::query_as::<_, LoyaltyProgram>(
        "SELECT id, tenant_id, name, program_type, config, is_active FROM loyalty_programs WHERE id = $1 AND tenant_id = $2"
    )
    .bind(&id)
    .bind(&tenant_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Error getting loyalty program: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(program))
}

#[derive(Debug, Deserialize)]
pub struct UpdateProgramReq {
    pub name: Option<String>,
    pub program_type: Option<String>,
    pub config_json: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

async fn update_program(
    State(state): State<LoyaltyState>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Path(id): Path<String>,
    Json(req): Json<UpdateProgramReq>,
) -> Result<Json<LoyaltyProgram>, StatusCode> {
    let tenant_id = auth_info.org_id;
    let mut tx = state.pool.begin().await.map_err(|e| { tracing::error!("Tx err: {:?}", e); StatusCode::INTERNAL_SERVER_ERROR})?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let program = sqlx::query_as::<_, LoyaltyProgram>(
        "UPDATE loyalty_programs SET
         name = COALESCE($1, name),
         program_type = COALESCE($2, program_type),
         config = COALESCE($3, config),
         is_active = COALESCE($4, is_active),
         updated_at = CURRENT_TIMESTAMP
         WHERE id = $5 AND tenant_id = $6
         RETURNING id, tenant_id, name, program_type, config, is_active"
    )
    .bind(&req.name)
    .bind(&req.program_type)
    .bind(&req.config_json)
    .bind(&req.is_active)
    .bind(&id)
    .bind(&tenant_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Error updating loyalty program: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(program))
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CustomerAccount {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub program_id: String,
    pub points_balance: i32,
    pub punches_count: i32,
    pub tier_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetStatusReq {
    pub program_id: String,
}

async fn get_status(
    State(state): State<LoyaltyState>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Path(customer_id): Path<String>,
    axum::extract::Query(req): axum::extract::Query<GetStatusReq>,
) -> Result<Json<CustomerAccount>, StatusCode> {
    let tenant_id = auth_info.org_id;
    let mut tx = state.pool.begin().await.map_err(|e| { tracing::error!("Tx err: {:?}", e); StatusCode::INTERNAL_SERVER_ERROR})?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let account = sqlx::query_as::<_, CustomerAccount>(
        "SELECT id, tenant_id, customer_id, program_id, points_balance, punches_count, tier_name
         FROM customer_loyalty_accounts
         WHERE customer_id = $1 AND program_id = $2 AND tenant_id = $3"
    )
    .bind(&customer_id)
    .bind(&req.program_id)
    .bind(&tenant_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Error getting loyalty status: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match account {
        Some(acc) => Ok(Json(acc)),
        None => {
            // Create empty account if it doesn't exist
            let id = uuid::Uuid::new_v4().to_string();
            let new_acc = sqlx::query_as::<_, CustomerAccount>(
                "INSERT INTO customer_loyalty_accounts (id, tenant_id, customer_id, program_id)
                 VALUES ($1, $2, $3, $4)
                 RETURNING id, tenant_id, customer_id, program_id, points_balance, punches_count, tier_name"
            )
            .bind(&id)
            .bind(&tenant_id)
            .bind(&customer_id)
            .bind(&req.program_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| {
                tracing::error!("Error creating customer loyalty account: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(new_acc))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct EarnPointsReq {
    pub program_id: String,
    pub points: i32,
    pub reason: Option<String>,
    pub order_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EarnPointsRes {
    pub account: CustomerAccount,
    pub transaction_id: String,
}

async fn earn_points(
    State(state): State<LoyaltyState>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Path(customer_id): Path<String>,
    Json(req): Json<EarnPointsReq>,
) -> Result<Json<EarnPointsRes>, StatusCode> {
    let tenant_id = auth_info.org_id;
    let mut tx = state.pool.begin().await.map_err(|e| { tracing::error!("Tx err: {:?}", e); StatusCode::INTERNAL_SERVER_ERROR})?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Upsert account
    let acc_id = uuid::Uuid::new_v4().to_string();
    let account = sqlx::query_as::<_, CustomerAccount>(
        "INSERT INTO customer_loyalty_accounts (id, tenant_id, customer_id, program_id, points_balance)
         VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT (tenant_id, customer_id, program_id)
         DO UPDATE SET points_balance = customer_loyalty_accounts.points_balance + EXCLUDED.points_balance, updated_at = CURRENT_TIMESTAMP
         RETURNING id, tenant_id, customer_id, program_id, points_balance, punches_count, tier_name"
    )
    .bind(&acc_id)
    .bind(&tenant_id)
    .bind(&customer_id)
    .bind(&req.program_id)
    .bind(req.points)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Error upserting customer loyalty account: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Log transaction
    let tx_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO loyalty_transactions (id, tenant_id, account_id, transaction_type, points, reason, order_id)
         VALUES ($1, $2, $3, 'EARN', $4, $5, $6)"
    )
    .bind(&tx_id)
    .bind(&tenant_id)
    .bind(&account.id)
    .bind(req.points)
    .bind(&req.reason)
    .bind(&req.order_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Error inserting loyalty transaction: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Emit event
    let msg = state.hub.sanitize_hub_event(serde_json::json!({
        "type": "loyalty.points_awarded",
        "tenant_id": tenant_id,
        "customer_id": customer_id,
        "program_id": req.program_id,
        "points": req.points,
        "reason": req.reason,
        "order_id": req.order_id,
        "new_balance": account.points_balance
    }));
    state.hub.append_recent_event(msg);

    Ok(Json(EarnPointsRes {
        account,
        transaction_id: tx_id,
    }))
}

#[derive(Debug, Deserialize)]
pub struct CreateRewardReq {
    pub name: String,
    pub description: Option<String>,
    pub cost_in_points: i32,
    pub reward_type: String,
    pub reward_value_json: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Reward {
    pub id: String,
    pub tenant_id: String,
    pub program_id: String,
    pub name: String,
    pub description: Option<String>,
    pub cost_in_points: i32,
    pub reward_type: String,
    pub reward_value: serde_json::Value,
    pub is_active: bool,
}

async fn create_reward(
    State(state): State<LoyaltyState>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Path(program_id): Path<String>,
    Json(req): Json<CreateRewardReq>,
) -> Result<Json<Reward>, StatusCode> {
    let tenant_id = auth_info.org_id;
    let mut tx = state.pool.begin().await.map_err(|e| { tracing::error!("Tx err: {:?}", e); StatusCode::INTERNAL_SERVER_ERROR})?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let id = uuid::Uuid::new_v4().to_string();
    let value = req.reward_value_json.unwrap_or_else(|| serde_json::json!({}));

    let reward = sqlx::query_as::<_, Reward>(
        "INSERT INTO rewards (id, tenant_id, program_id, name, description, cost_in_points, reward_type, reward_value, is_active)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, true)
         RETURNING id, tenant_id, program_id, name, description, cost_in_points, reward_type, reward_value, is_active"
    )
    .bind(&id)
    .bind(&tenant_id)
    .bind(&program_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(req.cost_in_points)
    .bind(&req.reward_type)
    .bind(&value)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Error creating reward: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(reward))
}

async fn get_rewards(
    State(state): State<LoyaltyState>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Path(program_id): Path<String>,
) -> Result<Json<Vec<Reward>>, StatusCode> {
    let tenant_id = auth_info.org_id;
    let mut tx = state.pool.begin().await.map_err(|e| { tracing::error!("Tx err: {:?}", e); StatusCode::INTERNAL_SERVER_ERROR})?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let rewards = sqlx::query_as::<_, Reward>(
        "SELECT id, tenant_id, program_id, name, description, cost_in_points, reward_type, reward_value, is_active
         FROM rewards WHERE program_id = $1 AND tenant_id = $2"
    )
    .bind(&program_id)
    .bind(&tenant_id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Error getting rewards: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(rewards))
}

#[derive(Debug, Deserialize)]
pub struct UpdateRewardReq {
    pub name: Option<String>,
    pub description: Option<String>,
    pub cost_in_points: Option<i32>,
    pub reward_type: Option<String>,
    pub reward_value_json: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

async fn update_reward(
    State(state): State<LoyaltyState>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Path(id): Path<String>,
    Json(req): Json<UpdateRewardReq>,
) -> Result<Json<Reward>, StatusCode> {
    let tenant_id = auth_info.org_id;
    let mut tx = state.pool.begin().await.map_err(|e| { tracing::error!("Tx err: {:?}", e); StatusCode::INTERNAL_SERVER_ERROR})?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let reward = sqlx::query_as::<_, Reward>(
        "UPDATE rewards SET
         name = COALESCE($1, name),
         description = COALESCE($2, description),
         cost_in_points = COALESCE($3, cost_in_points),
         reward_type = COALESCE($4, reward_type),
         reward_value = COALESCE($5, reward_value),
         is_active = COALESCE($6, is_active),
         updated_at = CURRENT_TIMESTAMP
         WHERE id = $7 AND tenant_id = $8
         RETURNING id, tenant_id, program_id, name, description, cost_in_points, reward_type, reward_value, is_active"
    )
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.cost_in_points)
    .bind(&req.reward_type)
    .bind(&req.reward_value_json)
    .bind(&req.is_active)
    .bind(&id)
    .bind(&tenant_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Error updating reward: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(reward))
}

#[derive(Debug, Deserialize)]
pub struct RedeemRewardReq {
    pub program_id: String,
    pub reward_id: String,
}

#[derive(Debug, Serialize)]
pub struct RedeemRewardRes {
    pub account: CustomerAccount,
    pub transaction_id: String,
}

async fn redeem_reward(
    State(state): State<LoyaltyState>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Path(customer_id): Path<String>,
    Json(req): Json<RedeemRewardReq>,
) -> Result<Json<RedeemRewardRes>, StatusCode> {
    let tenant_id = auth_info.org_id;
    let mut tx = state.pool.begin().await.map_err(|e| { tracing::error!("Tx err: {:?}", e); StatusCode::INTERNAL_SERVER_ERROR})?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get reward cost
    let reward = sqlx::query_as::<_, Reward>(
        "SELECT id, tenant_id, program_id, name, description, cost_in_points, reward_type, reward_value, is_active
         FROM rewards WHERE id = $1 AND program_id = $2 AND tenant_id = $3"
    )
    .bind(&req.reward_id)
    .bind(&req.program_id)
    .bind(&tenant_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Error fetching reward: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    if !reward.is_active {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Deduct points
    let account = sqlx::query_as::<_, CustomerAccount>(
        "UPDATE customer_loyalty_accounts
         SET points_balance = points_balance - $1, updated_at = CURRENT_TIMESTAMP
         WHERE customer_id = $2 AND program_id = $3 AND tenant_id = $4 AND points_balance >= $1
         RETURNING id, tenant_id, customer_id, program_id, points_balance, punches_count, tier_name"
    )
    .bind(reward.cost_in_points)
    .bind(&customer_id)
    .bind(&req.program_id)
    .bind(&tenant_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Error deducting points: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::BAD_REQUEST)?; // Returns BAD_REQUEST if not enough points

    // Log transaction
    let tx_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO loyalty_transactions (id, tenant_id, account_id, transaction_type, points, reason)
         VALUES ($1, $2, $3, 'REDEEM', $4, $5)"
    )
    .bind(&tx_id)
    .bind(&tenant_id)
    .bind(&account.id)
    .bind(-reward.cost_in_points)
    .bind(format!("Redeemed reward: {}", reward.name))
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Error logging redemption: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Emit event
    let msg = state.hub.sanitize_hub_event(serde_json::json!({
        "type": "loyalty.reward_redeemed",
        "tenant_id": tenant_id,
        "customer_id": customer_id,
        "program_id": req.program_id,
        "reward_id": req.reward_id,
        "points_spent": reward.cost_in_points,
        "new_balance": account.points_balance
    }));
    state.hub.append_recent_event(msg);

    Ok(Json(RedeemRewardRes {
        account,
        transaction_id: tx_id,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use ::server_auth::orchestration::AuthInfo;
    use axum::extract::Extension;

    #[tokio::test]
    async fn test_create_loyalty_program() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
        let pool = PgPool::connect(&database_url).await.unwrap();

        let tenant_id = format!("test_tenant_{}", uuid::Uuid::new_v4());
        sqlx::query("INSERT INTO tenants (id, name, ceo_name) VALUES ($1, 't', 't') ON CONFLICT DO NOTHING")
            .bind(&tenant_id)
            .execute(&pool).await.unwrap();

        let hub = Arc::new(crate::hub::Hub::new(tokio::sync::mpsc::channel(1).0, pool.clone()));
        let state = LoyaltyState { pool: pool.clone(), hub };

        let auth_info = AuthInfo {
            spiffe_id: "user".to_string(),
            org_id: tenant_id.clone(),
            agent_id: "".to_string(),
        };

        let req = CreateProgramReq {
            name: "VIP Points".to_string(),
            program_type: "points".to_string(),
            config_json: Some(serde_json::json!({"points_per_dollar": 1})),
        };

        let res = create_program(
            axum::extract::State(state),
            Extension(auth_info),
            axum::Json(req),
        ).await;

        assert!(res.is_ok());
        let program = res.unwrap().0;
        assert_eq!(program.name, "VIP Points");
        assert_eq!(program.program_type, "points");
    }

    #[tokio::test]
    async fn test_earn_points() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
        let pool = PgPool::connect(&database_url).await.unwrap();

        let tenant_id = format!("test_tenant_{}", uuid::Uuid::new_v4());
        let customer_id = format!("cust_{}", uuid::Uuid::new_v4());
        let program_id = format!("prog_{}", uuid::Uuid::new_v4());

        sqlx::query("INSERT INTO tenants (id, name, ceo_name) VALUES ($1, 't', 't') ON CONFLICT DO NOTHING")
            .bind(&tenant_id)
            .execute(&pool).await.unwrap();

        sqlx::query("INSERT INTO loyalty_programs (id, tenant_id, name, program_type, config) VALUES ($1, $2, 't', 'points', '{}')")
            .bind(&program_id).bind(&tenant_id).execute(&pool).await.unwrap();

        let hub = Arc::new(crate::hub::Hub::new(tokio::sync::mpsc::channel(1).0, pool.clone()));
        let state = LoyaltyState { pool: pool.clone(), hub };

        let auth_info = AuthInfo {
            spiffe_id: "user".to_string(),
            org_id: tenant_id.clone(),
            agent_id: "".to_string(),
        };

        let req = EarnPointsReq {
            program_id: program_id.clone(),
            points: 100,
            reason: Some("Purchase".to_string()),
            order_id: None,
        };

        let res = earn_points(
            axum::extract::State(state),
            Extension(auth_info),
            axum::extract::Path(customer_id.clone()),
            axum::Json(req),
        ).await;

        assert!(res.is_ok());
        let earn_res = res.unwrap().0;
        assert_eq!(earn_res.account.points_balance, 100);

        // ensure transactions was logged
        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM loyalty_transactions WHERE account_id = $1")
            .bind(earn_res.account.id)
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count.0, 1);
    }
}
