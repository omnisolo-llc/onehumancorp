use crate::db::DB;
use axum::{
    Router,
    extract::{Extension, Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct AvailableSlotsResponse {
    pub slots: Vec<Slot>,
}

#[derive(Serialize)]
pub struct Slot {
    pub id: String,
    pub start_time: String,
    pub end_time: String,
}

pub fn router<S>(db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/{service_id}", get(handle_get_available_slots))
        .with_state(db)
}

async fn handle_get_available_slots(
    State(db): State<Arc<DB>>,
    claims: Option<Extension<::server_common::Claims>>,
    Path(service_id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match claims
        .as_ref()
        .and_then(|Extension(claims)| ::server_common::auth_utils::signed_tenant_id(claims))
    {
        Some(tenant_id) => tenant_id,
        _ => {
            return (
                axum::http::StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({"error": "unauthorized"})),
            )
                .into_response();
        }
    };
    if service_id.trim().is_empty() || service_id.chars().count() > 128 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "invalid service_id"})),
        )
            .into_response();
    }

    let slots = match db.query_available_slots(&tenant_id, &service_id).await {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Failed to query available slots: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to fetch available slots"})),
            )
                .into_response();
        }
    };

    let mut response_slots = Vec::new();
    for slot in slots {
        response_slots.push(Slot {
            id: slot.id,
            start_time: slot.start_time.to_rfc3339(),
            end_time: slot.end_time.to_rfc3339(),
        });
    }

    (
        StatusCode::OK,
        Json(AvailableSlotsResponse {
            slots: response_slots,
        }),
    )
        .into_response()
}
