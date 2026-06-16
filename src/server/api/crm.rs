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
    pub mobile_optimized: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdateOpportunityStageRequest {
    pub opportunity_id: String,
    pub stage: String,
}

pub async fn list_opportunities_handler(
    State(db): State<Arc<crate::db::DB>>,
    Query(query): Query<OpportunitiesQuery>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);

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

    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit context transaction: {:?}", e);
    }

    let tenant_id_clone = tenant_id.clone();
    let db_clone = db.clone();

    let pool1 = db_clone.pool.clone();
    let pool2 = db_clone.pool.clone();

    let (opps_result, stats_result) = tokio::join!(
        tokio::spawn(async move {
            sqlx::query_as::<_, crate::domain::repository::models::Opportunity>(
                "SELECT id, tenant_id, lead_id, title, stage, estimated_value, priority, created_at, updated_at FROM opportunities WHERE tenant_id = $1 ORDER BY created_at DESC"
            )
            .bind(&tenant_id)
            .fetch_all(&pool1)
            .await
        }),
        tokio::spawn(async move {
            sqlx::query_scalar::<_, i64>("SELECT count(*) FROM opportunities WHERE tenant_id = $1")
                .bind(&tenant_id_clone)
                .fetch_one(&pool2)
                .await
        })
    );

    let opps_result = opps_result.unwrap_or_else(|_| Err(sqlx::Error::RowNotFound));
    let stats_result = stats_result.unwrap_or_else(|_| Err(sqlx::Error::RowNotFound));

    match opps_result {
        Ok(mut rows) => {
            if mobile_optimized {
                for row in &mut rows {
                    row.tenant_id = String::new();
                    // lead_id is essential for mobile navigation, so we keep it.
                }
            }

            let total_count = stats_result.unwrap_or(0);

            Json(serde_json::json!({
                "opportunities": rows,
                "total_count": total_count
            })).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch opportunities: {:?}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "opportunities": Vec::<crate::domain::repository::models::Opportunity>::new(),
                    "total_count": 0
                })),
            )
                .into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repository::models::Opportunity;

    #[test]
    fn test_opportunity_mobile_optimization() {
        let opp = Opportunity {
            id: "opt_1".to_string(),
            tenant_id: "tenant_1".to_string(),
            lead_id: Some("lead_1".to_string()),
            title: "Test".to_string(),
            stage: "NEW".to_string(),
            estimated_value: Some(100.0),
            priority: Some(1),
            created_at: None,
            updated_at: None,
        };

        let mut rows = vec![opp];

        // Simulate mobile_optimized = true logic from handler
        for row in &mut rows {
            row.tenant_id = String::new();
        }

        assert_eq!(rows[0].tenant_id, "");
        assert_eq!(rows[0].lead_id, Some("lead_1".to_string()), "lead_id must be preserved for navigation");
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
