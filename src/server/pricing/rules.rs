use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use server_auth::middleware::user_auth::AuthUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicPricingRule {
    pub id: Uuid,
    pub tenant_id: String,
    pub rule_name: String,
    pub condition_variable: String,
    pub condition_operator: String,
    pub condition_value: String,
    pub adjustment_type: String,
    pub adjustment_amount: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePricingRulePayload {
    pub rule_name: String,
    pub condition_variable: String,
    pub condition_operator: String,
    pub condition_value: String,
    pub adjustment_type: String,
    pub adjustment_amount: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePricingRulePayload {
    pub rule_name: Option<String>,
    pub condition_variable: Option<String>,
    pub condition_operator: Option<String>,
    pub condition_value: Option<String>,
    pub adjustment_type: Option<String>,
    pub adjustment_amount: Option<f64>,
}

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/rules", get(list_rules).post(create_rule))
        .route("/rules/:id", get(get_rule).put(update_rule).delete(delete_rule))
        .with_state(pool)
}

async fn list_rules(
    State(pool): State<PgPool>,
    user: AuthUser,
) -> Result<Json<Vec<DynamicPricingRule>>, StatusCode> {
    let tenant_id = user.current_tenant;
    let _ = sqlx::query!("SELECT set_config('app.current_tenant', $1, true)", tenant_id)
        .execute(&pool)
        .await;

    let rules = sqlx::query_as!(
        DynamicPricingRule,
        r#"
        SELECT id, tenant_id, rule_name, condition_variable, condition_operator, condition_value, adjustment_type, adjustment_amount, created_at, updated_at
        FROM dynamic_pricing_rules
        WHERE tenant_id = $1
        "#,
        tenant_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch rules: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(rules))
}

async fn create_rule(
    State(pool): State<PgPool>,
    user: AuthUser,
    Json(payload): Json<CreatePricingRulePayload>,
) -> Result<(StatusCode, Json<DynamicPricingRule>), StatusCode> {
    let tenant_id = user.current_tenant;
    let _ = sqlx::query!("SELECT set_config('app.current_tenant', $1, true)", tenant_id)
        .execute(&pool)
        .await;

    let id = Uuid::new_v4();
    let rule = sqlx::query_as!(
        DynamicPricingRule,
        r#"
        INSERT INTO dynamic_pricing_rules (id, tenant_id, rule_name, condition_variable, condition_operator, condition_value, adjustment_type, adjustment_amount)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, tenant_id, rule_name, condition_variable, condition_operator, condition_value, adjustment_type, adjustment_amount, created_at, updated_at
        "#,
        id,
        tenant_id,
        payload.rule_name,
        payload.condition_variable,
        payload.condition_operator,
        payload.condition_value,
        payload.adjustment_type,
        payload.adjustment_amount
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create rule: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::CREATED, Json(rule)))
}

async fn get_rule(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    user: AuthUser,
) -> Result<Json<DynamicPricingRule>, StatusCode> {
    let tenant_id = user.current_tenant;
    let _ = sqlx::query!("SELECT set_config('app.current_tenant', $1, true)", tenant_id)
        .execute(&pool)
        .await;

    let rule = sqlx::query_as!(
        DynamicPricingRule,
        r#"
        SELECT id, tenant_id, rule_name, condition_variable, condition_operator, condition_value, adjustment_type, adjustment_amount, created_at, updated_at
        FROM dynamic_pricing_rules
        WHERE id = $1 AND tenant_id = $2
        "#,
        id,
        tenant_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch rule: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(rule))
}

async fn update_rule(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<UpdatePricingRulePayload>,
) -> Result<Json<DynamicPricingRule>, StatusCode> {
    let tenant_id = user.current_tenant;
    let _ = sqlx::query!("SELECT set_config('app.current_tenant', $1, true)", tenant_id)
        .execute(&pool)
        .await;

    let rule = sqlx::query_as!(
        DynamicPricingRule,
        r#"
        UPDATE dynamic_pricing_rules
        SET
            rule_name = COALESCE($1, rule_name),
            condition_variable = COALESCE($2, condition_variable),
            condition_operator = COALESCE($3, condition_operator),
            condition_value = COALESCE($4, condition_value),
            adjustment_type = COALESCE($5, adjustment_type),
            adjustment_amount = COALESCE($6, adjustment_amount),
            updated_at = NOW()
        WHERE id = $7 AND tenant_id = $8
        RETURNING id, tenant_id, rule_name, condition_variable, condition_operator, condition_value, adjustment_type, adjustment_amount, created_at, updated_at
        "#,
        payload.rule_name,
        payload.condition_variable,
        payload.condition_operator,
        payload.condition_value,
        payload.adjustment_type,
        payload.adjustment_amount,
        id,
        tenant_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update rule: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(rule))
}

async fn delete_rule(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    user: AuthUser,
) -> Result<StatusCode, StatusCode> {
    let tenant_id = user.current_tenant;
    let _ = sqlx::query!("SELECT set_config('app.current_tenant', $1, true)", tenant_id)
        .execute(&pool)
        .await;

    let result = sqlx::query!(
        r#"
        DELETE FROM dynamic_pricing_rules
        WHERE id = $1 AND tenant_id = $2
        "#,
        id,
        tenant_id
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete rule: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        Ok(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
