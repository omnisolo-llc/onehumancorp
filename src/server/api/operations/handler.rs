use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;

use crate::domain::operations::operations_manager::{OperationsManager, ActionIntent, OperationsError};

#[derive(Deserialize)]
pub struct ActionRequest {
    pub tenant_id: String,
    pub action_type: String,
    pub payload: Value,
}

#[derive(Serialize)]
pub struct ActionResponse {
    pub success: bool,
    pub message: Option<String>,
    pub error: Option<String>,
}

pub fn router() -> Router<PgPool> {
    Router::new().route("/approve", post(approve_action_card))
}

pub async fn approve_action_card(
    State(pool): State<PgPool>,
    Json(req): Json<ActionRequest>,
) -> impl IntoResponse {
    let manager = OperationsManager::new(pool);
    let intent = ActionIntent {
        tenant_id: req.tenant_id.clone(),
        action_type: req.action_type.clone(),
        payload: req.payload.clone(),
    };

    match manager.execute_action(intent).await {
        Ok(_) => {
            let res = ActionResponse {
                success: true,
                message: Some(format!("Action {} approved and executed.", req.action_type)),
                error: None,
            };
            (StatusCode::OK, Json(res))
        }
        Err(e) => {
            let status = match e {
                OperationsError::Unauthorized => StatusCode::UNAUTHORIZED,
                _ => StatusCode::BAD_REQUEST,
            };
            let res = ActionResponse {
                success: false,
                message: None,
                error: Some(e.to_string()),
            };
            (status, Json(res))
        }
    }
}
