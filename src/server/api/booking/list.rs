use axum::{
    extract::{State, Json, Query},
    response::IntoResponse,
    http::StatusCode,
    routing::get,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::db::DB;

#[derive(Serialize)]
pub struct ListBookingsResponse {
    pub bookings: Vec<Booking>,
}

#[derive(Serialize)]
pub struct Booking {
    pub id: String,
    pub product_id: String,
    pub customer_id: String,
    pub start_time: String,
    pub end_time: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct ListQuery {
    pub tenant_id: Option<String>,
}

pub fn router<S>(db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(handle_list_bookings))
        .with_state(db)
}

async fn handle_list_bookings(
    State(db): State<Arc<DB>>,
    headers: axum::http::HeaderMap,
    Query(query): Query<ListQuery>,
) -> impl IntoResponse {
    let tenant_id = headers.get("x-tenant-id").and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .or(query.tenant_id)
        .unwrap_or_default();

    if tenant_id.is_empty() {
        return (axum::http::StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({"error": "unauthorized"}))).into_response();
    }

    let pool = db.pool.clone();

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("failed to begin tx: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ListBookingsResponse {
                    bookings: vec![],
                }),
            )
                .into_response();
        }
    };

    let _ = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await;

    let res = sqlx::query!(
        r#"
        SELECT id, product_id, customer_id, start_time, end_time, status
        FROM bookings
        WHERE tenant_id = $1
        ORDER BY start_time ASC
        "#,
        tenant_id
    )
    .fetch_all(&mut *tx)
    .await;

    let rows = match res {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Failed to fetch bookings: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ListBookingsResponse {
                    bookings: vec![],
                }),
            )
                .into_response();
        }
    };

    let mut bookings = Vec::new();
    for row in rows {
        bookings.push(Booking {
            id: row.id,
            product_id: row.product_id.unwrap_or_default(),
            customer_id: row.customer_id.unwrap_or_default(),
            start_time: row.start_time.to_rfc3339(),
            end_time: row.end_time.to_rfc3339(),
            status: row.status.unwrap_or_else(|| "pending".to_string()),
        });
    }

    (
        StatusCode::OK,
        Json(ListBookingsResponse { bookings }),
    ).into_response()
}
