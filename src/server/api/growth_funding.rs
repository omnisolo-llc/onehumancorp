use axum::{extract::State, Json};
use std::sync::Arc;
use sqlx::Row;
use crate::hub::Hub;
use server_proto::growth::{
    FundingOpportunity, ListFundingOpportunitiesRequest, ListFundingOpportunitiesResponse,
    SubmitFundingOpportunityRequest, SubmitFundingOpportunityResponse,
};

pub async fn list_funding_opportunities(
    State(hub): State<Arc<Hub>>,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
    Json(mut req): Json<ListFundingOpportunitiesRequest>,
) -> Json<ListFundingOpportunitiesResponse> {
    if !auth_info.spiffe_id.is_empty() {
        req.tenant_id = auth_info.spiffe_id;
    }
    let tenant_id = req.tenant_id;
    let pool = &hub.pool;

    let rows = match sqlx::query(
        "SELECT id, tenant_id, grant_name, amount, draft_proposal_text, status, deadline FROM funding_opportunities WHERE tenant_id = $1"
    )
    .bind(&tenant_id)
    .fetch_all(pool)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            tracing::error!("Failed to fetch funding opportunities: {}", e);
            return Json(ListFundingOpportunitiesResponse { opportunities: vec![] });
        }
    };

    let mut opportunities = Vec::new();
    for row in rows {
        let deadline: chrono::DateTime<chrono::Utc> = row.try_get("deadline").unwrap_or_else(|_| chrono::Utc::now());
        let amount: sqlx::types::BigDecimal = row.try_get("amount").unwrap_or_default();

        opportunities.push(FundingOpportunity {
            id: row.try_get::<uuid::Uuid, _>("id").unwrap_or_default().to_string(),
            tenant_id: row.try_get::<uuid::Uuid, _>("tenant_id").unwrap_or_default().to_string(),
            grant_name: row.try_get("grant_name").unwrap_or_default(),
            amount: format!("{}", amount).parse::<f64>().unwrap_or(0.0),
            draft_proposal_text: row.try_get("draft_proposal_text").unwrap_or_default(),
            status: row.try_get("status").unwrap_or_default(),
            deadline_unix: deadline.timestamp(),
        });
    }

    Json(ListFundingOpportunitiesResponse { opportunities })
}

pub async fn submit_funding_opportunity(
    State(hub): State<Arc<Hub>>,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
    Json(mut req): Json<SubmitFundingOpportunityRequest>,
) -> Json<SubmitFundingOpportunityResponse> {
    if !auth_info.spiffe_id.is_empty() {
        req.tenant_id = auth_info.spiffe_id;
    }
    let tenant_id = req.tenant_id;
    let opportunity_id = match uuid::Uuid::parse_str(&req.id) {
        Ok(id) => id,
        Err(_) => return Json(SubmitFundingOpportunityResponse { opportunity: None }),
    };

    let pool = &hub.pool;

    // Update status to 'Submitted'
    let update_result = sqlx::query(
        "UPDATE funding_opportunities SET status = 'Submitted' WHERE id = $1 AND tenant_id = $2 RETURNING id, tenant_id, grant_name, amount, draft_proposal_text, status, deadline"
    )
    .bind(&opportunity_id)
    .bind(&tenant_id)
    .fetch_optional(pool)
    .await;

    match update_result {
        Ok(Some(row)) => {
            let deadline: chrono::DateTime<chrono::Utc> = row.try_get("deadline").unwrap_or_else(|_| chrono::Utc::now());
            let amount: sqlx::types::BigDecimal = row.try_get("amount").unwrap_or_default();

            let opp = FundingOpportunity {
                id: row.try_get::<uuid::Uuid, _>("id").unwrap_or_default().to_string(),
                tenant_id: row.try_get::<uuid::Uuid, _>("tenant_id").unwrap_or_default().to_string(),
                grant_name: row.try_get("grant_name").unwrap_or_default(),
                amount: format!("{}", amount).parse::<f64>().unwrap_or(0.0),
                draft_proposal_text: row.try_get("draft_proposal_text").unwrap_or_default(),
                status: row.try_get("status").unwrap_or_default(),
                deadline_unix: deadline.timestamp(),
            };
            Json(SubmitFundingOpportunityResponse { opportunity: Some(opp) })
        }
        Ok(None) => Json(SubmitFundingOpportunityResponse { opportunity: None }),
        Err(e) => {
            tracing::error!("Failed to submit funding opportunity: {}", e);
            Json(SubmitFundingOpportunityResponse { opportunity: None })
        }
    }
}
