use axum::{
    extract::{Extension, Path, State},
    response::IntoResponse,
    Json,
    routing::{get, post, put},
    Router,
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;
use crate::{db::DB, domain::repository::action_required_queue_repo::ActionRequiredQueueRepo};

pub struct AppState {
    pub db: Arc<DB>,
}

pub fn router(db: Arc<DB>, auth_store: Arc<::server_auth::Store>) -> Router {
    Router::new()
        .route("/", get(list_pending_drafts))
        .route("/{id}/approve", post(approve_draft))
        .route("/{id}/edit", put(edit_draft))
        .layer(axum::middleware::from_fn_with_state(
            auth_store,
            ::server_auth::strict_bearer_auth_middleware,
        ))
        .with_state(Arc::new(AppState { db }))
}

fn claim_tenant_id(claims: &::server_common::Claims) -> Result<Uuid, axum::response::Response> {
    claims
        .organization_id
        .as_deref()
        .map(str::trim)
        .filter(|tenant_id| !tenant_id.is_empty() && !tenant_id.eq_ignore_ascii_case("system"))
        .and_then(|tenant_id| Uuid::parse_str(tenant_id).ok())
        .ok_or_else(|| {
            (
                axum::http::StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Unauthorized"})),
            )
                .into_response()
        })
}

fn can_manage_drafts(claims: &::server_common::Claims) -> bool {
    claims.roles.iter().any(|role| {
        role.eq_ignore_ascii_case("admin")
            || role.eq_ignore_ascii_case("owner")
            || role.eq_ignore_ascii_case("operator")
    })
}

async fn list_pending_drafts(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    if !can_manage_drafts(&claims) {
        return axum::http::StatusCode::FORBIDDEN.into_response();
    }
    let tenant_id = match claim_tenant_id(&claims) {
        Ok(tenant_id) => tenant_id,
        Err(response) => return response,
    };

    let repo = ActionRequiredQueueRepo::new(state.db.clone());
    match repo.get_pending_drafts(tenant_id).await {
        Ok(drafts) => (axum::http::StatusCode::OK, Json(drafts)).into_response(),
        Err(error) => {
            tracing::error!("Failed to list action-required drafts: {error:?}");
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Drafts unavailable"}))).into_response()
        },
    }
}

async fn approve_draft(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(draft_id_str): Path<String>,
) -> impl IntoResponse {
    if !can_manage_drafts(&claims) {
        return axum::http::StatusCode::FORBIDDEN.into_response();
    }
    let tenant_id = match claim_tenant_id(&claims) {
        Ok(tenant_id) => tenant_id,
        Err(response) => return response,
    };

    let draft_id = match Uuid::parse_str(&draft_id_str) {
        Ok(t) => t,
        Err(_) => return (axum::http::StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid draft ID"}))).into_response(),
    };

    let repo = ActionRequiredQueueRepo::new(state.db.clone());
    match repo.approve_draft(draft_id, tenant_id).await {
        Ok(true) => {
            (axum::http::StatusCode::OK, Json(json!({"status": "approved"}))).into_response()
        },
        Ok(false) => (axum::http::StatusCode::NOT_FOUND, Json(json!({"error": "Draft not found"}))).into_response(),
        Err(error) => {
            tracing::error!("Failed to approve action-required draft: {error:?}");
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Draft update failed"}))).into_response()
        },
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EditDraftPayload {
    pub response: String,
}

async fn edit_draft(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(draft_id_str): Path<String>,
    Json(payload): Json<EditDraftPayload>,
) -> impl IntoResponse {
    if !can_manage_drafts(&claims) {
        return axum::http::StatusCode::FORBIDDEN.into_response();
    }
    let tenant_id = match claim_tenant_id(&claims) {
        Ok(tenant_id) => tenant_id,
        Err(response) => return response,
    };

    let draft_id = match Uuid::parse_str(&draft_id_str) {
        Ok(t) => t,
        Err(_) => return (axum::http::StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid draft ID"}))).into_response(),
    };
    if payload.response.trim().is_empty() || payload.response.chars().count() > 16_000 {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid response"})),
        )
            .into_response();
    }

    let repo = ActionRequiredQueueRepo::new(state.db.clone());
    match repo.update_draft_response(draft_id, tenant_id, &payload.response).await {
        Ok(true) => {
            (axum::http::StatusCode::OK, Json(json!({"status": "edited"}))).into_response()
        },
        Ok(false) => (axum::http::StatusCode::NOT_FOUND, Json(json!({"error": "Draft not found"}))).into_response(),
        Err(error) => {
            tracing::error!("Failed to edit action-required draft: {error:?}");
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Draft update failed"}))).into_response()
        },
    }
}
