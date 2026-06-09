use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct OpportunitiesQuery {
    pub tenant_id: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateOpportunityStageRequest {
    pub opportunity_id: String,
    pub stage: String,
}

pub async fn list_opportunities_handler(
    State(db): State<Arc<crate::db::DB>>,
    Query(query): Query<OpportunitiesQuery>,
) -> impl IntoResponse {
    let tenant_id = query.tenant_id.unwrap_or_else(|| "default".to_string());

    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {:?}", e);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(Vec::<crate::domain::repository::models::Opportunity>::new()),
            )
                .into_response();
        }
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        tracing::error!("Failed to set org context: {:?}", e);
        return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(Vec::<crate::domain::repository::models::Opportunity>::new()),
        )
            .into_response();
    }

    match sqlx::query_as::<_, crate::domain::repository::models::Opportunity>(
        "SELECT id, tenant_id, lead_id, title, stage, estimated_value, priority, created_at, updated_at FROM opportunities WHERE tenant_id = $1 ORDER BY created_at DESC"
    )
    .bind(&tenant_id)
    .fetch_all(&mut *tx)
    .await
    {
        Ok(rows) => Json(rows).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch opportunities: {:?}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(Vec::<crate::domain::repository::models::Opportunity>::new()),
            )
                .into_response()
        }
    }
}

pub async fn update_opportunity_stage_handler(
    State(db): State<Arc<crate::db::DB>>,
    Json(payload): Json<UpdateOpportunityStageRequest>,
) -> impl IntoResponse {
    match sqlx::query("UPDATE opportunities SET stage = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2")
        .bind(&payload.stage)
        .bind(&payload.opportunity_id)
        .execute(&db.pool)
        .await
    {
        Ok(_) => (axum::http::StatusCode::OK, Json(serde_json::json!({"status": "success"}))).into_response(),
        Err(e) => {
            tracing::error!("Failed to update opportunity stage: {:?}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to update"}))).into_response()
        }
    }
}
