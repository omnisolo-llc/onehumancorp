use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::services::chat::service::ChatService;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CreateCannedResponseRequest {
    pub short_code: String,
    pub content: String,
}

pub struct CannedResponsesAppState {
    pub chat_service: Arc<ChatService>,
}

pub fn canned_responses_router(chat_service: Arc<ChatService>) -> Router {
    let state = Arc::new(CannedResponsesAppState { chat_service });
    Router::new()
        .route(
            "/api/v1/inbox/tenants/:tenant_id/canned_responses",
            post(create_canned_response).get(get_canned_responses),
        )
        .with_state(state)
}

pub async fn create_canned_response(
    State(state): State<Arc<CannedResponsesAppState>>,
    Path(tenant_id): Path<Uuid>,
    Json(payload): Json<CreateCannedResponseRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let r = state
        .chat_service
        .create_canned_response(tenant_id, payload.short_code, payload.content)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::CREATED, Json(r)))
}

pub async fn get_canned_responses(
    State(state): State<Arc<CannedResponsesAppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let rs = state
        .chat_service
        .get_canned_responses(tenant_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::OK, Json(rs)))
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, StatusCode};
    use axum::body::Body;
    use tower::ServiceExt;

    use uuid::Uuid;

    #[tokio::test]
    async fn test_canned_responses_routes_exist() {
        let pool = crate::db::create_dummy_pg_pool().await;
        let chat_service = Arc::new(ChatService::new(pool));
        let app = canned_responses_router(chat_service);

        let tenant_id = Uuid::new_v4();
        let request = Request::builder()
            .uri(format!("/api/v1/inbox/tenants/{}/canned_responses", tenant_id))
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }
}
