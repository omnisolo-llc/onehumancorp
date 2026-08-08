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
pub struct CreateMacroRequest {
    pub name: String,
    pub visibility: String,
    pub actions: serde_json::Value,
}

#[derive(Deserialize)]
pub struct ExecuteMacroRequest {
    pub conversation_id: Uuid,
}

pub struct MacrosAppState {
    pub chat_service: Arc<ChatService>,
}

pub fn macros_router(chat_service: Arc<ChatService>) -> Router {
    let state = Arc::new(MacrosAppState { chat_service });
    Router::new()
        .route("/api/v1/inbox/tenants/:tenant_id/macros", post(create_macro).get(get_macros))
        .route("/api/v1/inbox/tenants/:tenant_id/macros/:macro_id/execute", post(execute_macro))
        .with_state(state)
}

pub async fn create_macro(
    State(state): State<Arc<MacrosAppState>>,
    Path(tenant_id): Path<Uuid>,
    Json(payload): Json<CreateMacroRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let m = state
        .chat_service
        .create_macro(tenant_id, payload.name, payload.visibility, payload.actions)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::CREATED, Json(m)))
}

pub async fn get_macros(
    State(state): State<Arc<MacrosAppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let macros = state
        .chat_service
        .get_macros(tenant_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::OK, Json(macros)))
}

pub async fn execute_macro(
    State(state): State<Arc<MacrosAppState>>,
    Path((tenant_id, macro_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<ExecuteMacroRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    state
        .chat_service
        .execute_macro(tenant_id, macro_id, payload.conversation_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, StatusCode};
    use axum::body::Body;
    use tower::ServiceExt;

    use uuid::Uuid;

    #[tokio::test]
    async fn test_macros_routes_exist() {
        // Just testing if the router registers properly and responds (even with 500 without a real DB).
        // A real DB test would setup PgPool. For now, testing 404 vs 500 vs 400 is enough to prove it's wired.
        let pool = crate::db::create_dummy_pg_pool().await;
        let chat_service = Arc::new(ChatService::new(pool));
        let app = macros_router(chat_service);

        let tenant_id = Uuid::new_v4();
        let request = Request::builder()
            .uri(format!("/api/v1/inbox/tenants/{}/macros", tenant_id))
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        // Without a valid connected database, it'll likely return 500, but it shouldn't be 404
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }
}
