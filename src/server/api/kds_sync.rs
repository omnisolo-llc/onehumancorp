use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct KdsEventPayload {
    pub item_id: Option<String>,
    pub is_sold_out: Option<bool>,
    pub order_id: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct KdsEvent {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub event_type: String,
    pub payload: KdsEventPayload,
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct KdsSyncRequest {
    pub events: Vec<KdsEvent>,
}

#[derive(Serialize)]
pub struct KdsSyncResponse {
    pub success: bool,
}

pub async fn kds_sync_handler(
    State((db, _mesh)): State<(sqlx::PgPool, Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>)>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<KdsSyncRequest>,
) -> impl IntoResponse {
    tracing::info!("Received {} offline KDS events for sync.", payload.events.len());

    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(KdsSyncResponse { success: false }),
        ).into_response();
    }

    let cache = crate::builder::edge::get_edge_cache();
    cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

    for event in &payload.events {
        if event.event_type == "TOGGLE_SOLD_OUT" {
            if let (Some(item_id), Some(is_sold_out)) = (&event.payload.item_id, event.payload.is_sold_out) {
                cache.invalidate_by_tag(&format!("entity:product:{}", item_id)).await;

                let query = "
                    UPDATE products
                    SET is_sold_out = $1
                    WHERE id = $2 AND tenant_id = $3
                ";

                let result = sqlx::query(query)
                    .bind(is_sold_out)
                    .bind(item_id)
                    .bind(&tenant_id)
                    .execute(&db)
                    .await;

                if let Err(e) = result {
                    tracing::error!("Failed to toggle sold out for product {}: {}", item_id, e);
                }
            }
        } else if event.event_type == "UPDATE_ORDER_STATUS" {
            if let (Some(order_id), Some(status)) = (&event.payload.order_id, &event.payload.status) {
                let query = "
                    UPDATE orders
                    SET status = $1
                    WHERE id = $2 AND tenant_id = $3
                ";

                let result = sqlx::query(query)
                    .bind(status)
                    .bind(order_id)
                    .bind(&tenant_id)
                    .execute(&db)
                    .await;

                if let Err(e) = result {
                    tracing::error!("Failed to update order status for order {}: {}", order_id, e);
                }
            }
        }
    }

    (
        StatusCode::OK,
        Json(KdsSyncResponse { success: true }),
    ).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;
    use ohc_builtin_agent::mesh::transport::{InProcessTransport, MeshTransport};
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_kds_sync_unauthorized() {
        let pool = PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap();
        let mesh: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
        let state = State((pool, mesh));

        let req = KdsSyncRequest { events: vec![] };
        let headers = HeaderMap::new();

        let response = kds_sync_handler(state, headers, Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
