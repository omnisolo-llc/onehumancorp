use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::db::DB;
use crate::domain::repository::crm_repo::CrmRepository;
use crate::domain::repository::models::{Lead, Opportunity};

pub fn router<S: Clone + Send + Sync + 'static>(db: DB) -> Router<S> {
    let repo = Arc::new(CrmRepository::new(db));

    Router::new()
        .route("/leads/:tenant_id", get(list_leads))
        .route("/leads", post(create_lead))
        .route("/opportunities/:tenant_id", get(list_opportunities))
        .route("/opportunities", post(create_opportunity))
        .route("/opportunities/:tenant_id/:id/stage", put(update_opportunity_stage))
        .with_state(repo)
}

#[derive(Deserialize)]
pub struct CreateLeadReq {
    pub tenant_id: String,
    pub source: String,
    pub contact_info: String,
    pub context: Option<String>,
}

async fn list_leads(
    State(repo): State<Arc<CrmRepository>>,
    Path(tenant_id): Path<String>,
) -> Result<Json<Vec<Lead>>, StatusCode> {
    match repo.list_leads(&tenant_id).await {
        Ok(leads) => Ok(Json(leads)),
        Err(e) => {
            tracing::error!("Failed to list leads: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_lead(
    State(repo): State<Arc<CrmRepository>>,
    Json(req): Json<CreateLeadReq>,
) -> Result<Json<Lead>, StatusCode> {
    let lead = Lead {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: req.tenant_id,
        source: req.source,
        contact_info: req.contact_info,
        context: req.context,
        created_at: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now()),
    };

    match repo.create_lead(&lead).await {
        Ok(_) => Ok(Json(lead)),
        Err(e) => {
            tracing::error!("Failed to create lead: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct CreateOpportunityReq {
    pub tenant_id: String,
    pub lead_id: Option<String>,
    pub title: String,
    pub stage: String,
    pub estimated_value: f64,
    pub priority: String,
}

async fn list_opportunities(
    State(repo): State<Arc<CrmRepository>>,
    Path(tenant_id): Path<String>,
) -> Result<Json<Vec<Opportunity>>, StatusCode> {
    match repo.list_opportunities(&tenant_id).await {
        Ok(opps) => Ok(Json(opps)),
        Err(e) => {
            tracing::error!("Failed to list opportunities: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_opportunity(
    State(repo): State<Arc<CrmRepository>>,
    Json(req): Json<CreateOpportunityReq>,
) -> Result<Json<Opportunity>, StatusCode> {
    let opp = Opportunity {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: req.tenant_id,
        lead_id: req.lead_id,
        title: req.title,
        stage: req.stage,
        estimated_value: req.estimated_value,
        priority: req.priority,
        created_at: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now()),
    };

    match repo.create_opportunity(&opp).await {
        Ok(_) => Ok(Json(opp)),
        Err(e) => {
            tracing::error!("Failed to create opportunity: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateStageReq {
    pub stage: String,
}

async fn update_opportunity_stage(
    State(repo): State<Arc<CrmRepository>>,
    Path((tenant_id, id)): Path<(String, String)>,
    Json(req): Json<UpdateStageReq>,
) -> Result<StatusCode, StatusCode> {
    match repo.update_opportunity_stage(&id, &tenant_id, &req.stage).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            tracing::error!("Failed to update opportunity stage: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
