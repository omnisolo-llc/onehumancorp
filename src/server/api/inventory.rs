use axum::{
    extract::{Extension, Json, State},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use axum::http::StatusCode;
use crate::hub::Hub;

#[derive(Deserialize, Debug)]
pub struct InventoryStatusUpdateRequest {
    pub item_id: String,
    pub is_sold_out: bool,
}

#[derive(Serialize)]
pub struct InventoryStatusUpdateResponse {
    pub success: bool,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

pub async fn inventory_status_handler(
    Extension(hub): Extension<Arc<Hub>>,
    State(mesh): State<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<InventoryStatusUpdateRequest>,
) -> impl IntoResponse {
    // Get tenant ID from token/claims
    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse { error: "UNAUTHORIZED".to_string(), message: "Missing or invalid spiffe id".to_string() }),
        ).into_response();
    }

    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to acquire DB connection: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "DATABASE_ERROR".to_string(),
                    message: "Failed to connect to database".to_string(),
                }),
            ).into_response();
        }
    };

    let result = sqlx::query("UPDATE products SET is_sold_out = $1 WHERE id = $2 AND tenant_id = $3 RETURNING name, description")
        .bind(payload.is_sold_out)
        .bind(&payload.item_id)
        .bind(&tenant_id)
        .fetch_optional(&mut *conn)
        .await;

    match result {
        Ok(Some(row)) => {
            use sqlx::Row;
            let product_name: String = row.try_get("name").unwrap_or_else(|_| "Unknown Product".to_string());
            let description: String = row.try_get("description").unwrap_or_else(|_| "".to_string());

            // Publish mesh event for inventory status changed
            let event = ::server_ohc::orchestration::TeammateMeshEvent {
                action: "InventoryStatusChanged".to_string(),
                agent_id: "system".to_string(),
                status: "".to_string(),
                msg_id: uuid::Uuid::new_v4().to_string(),
                payload: serde_json::json!({
                    "product_id": payload.item_id,
                    "is_sold_out": payload.is_sold_out,
                    "tenant_id": tenant_id,
                    "name": product_name,
                    "description": description,
                    "images": []
                }).to_string().into_bytes(),
            };

            if let Err(e) = mesh.publish("mesh:inventory:status_changed", event).await {
                tracing::error!("Failed to publish mesh:inventory:status_changed event: {}", e);
            }

            (
                StatusCode::OK,
                Json(InventoryStatusUpdateResponse { success: true }),
            ).into_response()
        }
        Ok(None) => {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse { error: "NOT_FOUND".to_string(), message: "Product not found or unauthorized".to_string() }),
            ).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to update product sold out status {}: {}", payload.item_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: "DATABASE_ERROR".to_string(), message: "Failed to update status".to_string() }),
            ).into_response()
        }
    }
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>, mesh: Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>) -> Router<S> {
    Router::new()
        .route("/status", post(inventory_status_handler))
        .layer(Extension(hub))
        .with_state(mesh)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;
    use ohc_builtin_agent::mesh::transport::{InProcessTransport, MeshTransport};
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_inventory_status_unauthorized() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }
        let db_url = std::env::var("OHC_DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new().connect_lazy(&db_url).unwrap();
        let (tx, _) = mpsc::channel(100);
        let hub = std::sync::Arc::new(Hub::new(tx, pool));

        let mesh: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());

        let req = InventoryStatusUpdateRequest { item_id: "test".to_string(), is_sold_out: true };
        let headers = HeaderMap::new();

        let response = inventory_status_handler(Extension(hub), State(mesh), headers, Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
