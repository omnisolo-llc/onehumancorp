use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ::server_common::Claims;

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
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());

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

    let result = sqlx::query_as::<_, crate::domain::repository::models::Opportunity>(
        "SELECT id, tenant_id, lead_id, title, stage, estimated_value, priority, created_at, updated_at FROM opportunities WHERE tenant_id = $1 ORDER BY created_at DESC"
    )
    .bind(&tenant_id)
    .fetch_all(&mut *tx)
    .await;

    match result {
        Ok(rows) => {
            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction: {:?}", e);
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(Vec::<crate::domain::repository::models::Opportunity>::new()),
                )
                    .into_response();
            }
            Json(rows).into_response()
        }
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
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateOpportunityStageRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_default();
    if tenant_id.is_empty() {
        return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response();
    }

    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {:?}", e);
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response();
        }
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        tracing::error!("Failed to set org context: {:?}", e);
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response();
    }

    match sqlx::query("UPDATE opportunities SET stage = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3")
        .bind(&payload.stage)
        .bind(&payload.opportunity_id)
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
    {
        Ok(_) => {
            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction: {:?}", e);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "failed to commit"}))).into_response();
            }
            (axum::http::StatusCode::OK, Json(serde_json::json!({"status": "success"}))).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to update opportunity stage: {:?}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to update"}))).into_response()
        }
    }
}
