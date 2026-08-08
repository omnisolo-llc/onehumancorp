use axum::{
    Json, Router,
    extract::{Path, State, Extension},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde_json::json;
use std::sync::Arc;

use crate::orchestration::dynamic_workflows::{DynamicWorkflowManager, DynamicWorkflowRequest};
use crate::domain::repository::agent_feed_repo::{AgentFeedRepository, AgentFeedItem};

pub fn router<S>(manager: Arc<DynamicWorkflowManager>, db: Arc<crate::db::DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(start_workflow))
        .route("/{id}", get(get_workflow))
        .route("/{id}/confirm", post(confirm_workflow))
        .with_state((manager, db))
}

async fn start_workflow(
    State((manager, db)): State<(Arc<DynamicWorkflowManager>, Arc<crate::db::DB>)>,
    claims: Option<Extension<::server_common::Claims>>,
    auth_info: Option<Extension<::server_auth::orchestration::AuthInfo>>,
    Json(mut request): Json<DynamicWorkflowRequest>,
) -> axum::response::Response {
    if request.prompt.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "prompt is required" })),
        )
            .into_response();
    }

    let tenant_id = if let Some(Extension(c)) = claims {
        c.organization_id.clone().unwrap_or_else(|| "default".to_string())
    } else if let Some(Extension(a)) = auth_info {
        if !a.org_id.is_empty() {
            a.org_id.clone()
        } else {
            "default".to_string()
        }
    } else {
        "default".to_string()
    };

    request.tenant_id = tenant_id.clone();

    match manager.start_workflow(request).await {
        Ok(start) => {
            if start.plan.requires_confirmation {
                let repo = AgentFeedRepository::new(db.clone());
                let feed_item = AgentFeedItem {
                    id: start.plan.id.clone(),
                    tenant_id: tenant_id.clone(),
                    event_source: "dynamic_workflow".to_string(),
                    context_payload: Some(sqlx::types::Json(json!({
                        "description": format!("Approve Action Plan: {}", start.plan.prompt),
                        "feature_type": "action_plan",
                        "plan": start.plan,
                    }))),
                    proposed_action: Some(sqlx::types::Json(json!({
                        "feature_type": "action_plan",
                        "plan": start.plan,
                    }))),
                    lifecycle_state: "PENDING_APPROVAL".to_string(),
                    created_at: Some(chrono::Utc::now()),
                    updated_at: Some(chrono::Utc::now()),
                };
                let _ = repo.create(feed_item).await;
            }

            (StatusCode::OK, Json(json!(start))).into_response()
        }
        Err(error) => (StatusCode::BAD_REQUEST, Json(json!({ "error": error }))).into_response(),
    }
}

async fn confirm_workflow(
    State((manager, db)): State<(Arc<DynamicWorkflowManager>, Arc<crate::db::DB>)>,
    claims: Option<Extension<::server_common::Claims>>,
    auth_info: Option<Extension<::server_auth::orchestration::AuthInfo>>,
    Path(id): Path<String>,
) -> axum::response::Response {
    let tenant_id = if let Some(Extension(c)) = claims {
        c.organization_id.clone().unwrap_or_else(|| "default".to_string())
    } else if let Some(Extension(a)) = auth_info {
        if !a.org_id.is_empty() {
            a.org_id.clone()
        } else {
            "default".to_string()
        }
    } else {
        "default".to_string()
    };

    match manager.confirm_workflow(&id).await {
        Ok(start) => {
            let repo = AgentFeedRepository::new(db.clone());
            let _ = repo.update_state(&tenant_id, &id, "APPROVED").await;
            (StatusCode::OK, Json(json!(start))).into_response()
        }
        Err(error) => (StatusCode::NOT_FOUND, Json(json!({ "error": error }))).into_response(),
    }
}

async fn get_workflow(
    State((manager, _db)): State<(Arc<DynamicWorkflowManager>, Arc<crate::db::DB>)>,
    Path(id): Path<String>,
) -> axum::response::Response {
    match manager.get_workflow(&id) {
        Ok(Some(plan)) => (StatusCode::OK, Json(json!(plan))).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "workflow not found" })),
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": error })),
        )
            .into_response(),
    }
}
