use axum::{
    extract::{State, Json, Path},
    response::IntoResponse,
    http::StatusCode,
    routing::get,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::db::DB;

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
    headers: axum::http::HeaderMap,
    Path(service_id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (axum::http::StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let pool = db.pool.clone();
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("failed to begin tx: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "internal error"})),
            )
                .into_response();
        }
    };

    let _ = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await;

    // Use sqlx instead of missing db.store method
    let rows = match sqlx::query(
        "SELECT id, start_time, end_time FROM availability_blocks WHERE tenant_id = $1 AND service_id = $2 AND is_available = true ORDER BY start_time ASC",
    )
    .bind(&tenant_id)
    .bind(&service_id)
    .fetch_all(&mut *tx)
    .await
    {
        Ok(r) => r,
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
    for row in rows {
        use sqlx::Row;

        let st: chrono::DateTime<chrono::Utc> = match row.try_get("start_time") {
            Ok(v) => v,
            Err(_) => continue,
        };
        let et: chrono::DateTime<chrono::Utc> = match row.try_get("end_time") {
            Ok(v) => v,
            Err(_) => continue,
        };

        response_slots.push(Slot {
            id: row.try_get("id").unwrap_or_default(),
            start_time: st.to_rfc3339(),
            end_time: et.to_rfc3339(),
        });
    }

    (
        StatusCode::OK,
        Json(AvailableSlotsResponse { slots: response_slots }),
    ).into_response()
}
