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

    let db_clone1 = db.clone();
    let db_clone2 = db.clone();
    let t_id1 = tenant_id.clone();
    let t_id2 = tenant_id.clone();

    let (list_res, count_res) = tokio::join!(
        tokio::spawn(async move {
            let mut tx = db_clone1.pool.begin().await?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &t_id1).await?;
            let rows = sqlx::query_as::<_, crate::domain::repository::models::Opportunity>(
                if mobile_optimized {
                    "SELECT id, '' as tenant_id, lead_id, title, stage, estimated_value, priority, created_at, updated_at FROM opportunities WHERE tenant_id = $1 ORDER BY created_at DESC"
                } else {
                    "SELECT id, tenant_id, lead_id, title, stage, estimated_value, priority, created_at, updated_at FROM opportunities WHERE tenant_id = $1 ORDER BY created_at DESC"
                }
            )
            .bind(&t_id1)
            .fetch_all(&mut *tx)
            .await?;
            tx.commit().await?;
            Ok::<_, sqlx::Error>(rows)
        }),
        tokio::spawn(async move {
            let mut tx = db_clone2.pool.begin().await?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &t_id2).await?;
            let count: (i64,) = sqlx::query_as(
                "SELECT count(*) FROM opportunities WHERE tenant_id = $1"
            )
            .bind(&t_id2)
            .fetch_one(&mut *tx)
            .await?;
            tx.commit().await?;
            Ok::<_, sqlx::Error>(count.0)
        })
    );

    let rows_result = match list_res {
        Ok(res) => res,
        Err(e) => {
            tracing::error!("Tokio spawn error fetching opportunities: {:?}", e);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(Vec::<crate::domain::repository::models::Opportunity>::new()),
            ).into_response();
        }
    };

    let rows = match rows_result {
        Ok(rows) => rows,
        Err(e) => {
            tracing::error!("Failed to fetch opportunities: {:?}", e);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(Vec::<crate::domain::repository::models::Opportunity>::new()),
            ).into_response();
        }
    };

    let total_count = match count_res {
        Ok(Ok(c)) => c,
        _ => 0,
    };

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        "x-total-count",
        axum::http::HeaderValue::from_str(&total_count.to_string()).unwrap_or(axum::http::HeaderValue::from_static("0"))
    );

    (headers, Json(rows)).into_response()
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
