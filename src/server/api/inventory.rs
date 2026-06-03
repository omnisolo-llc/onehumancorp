use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use axum::http::StatusCode;
use ohc_builtin_agent::mesh::transport::MeshTransport;

#[derive(Deserialize)]
pub struct InventoryStatusChangeRequest {
    pub product_id: String,
    pub inventory_count: i32,
}

#[derive(Serialize)]
pub struct InventoryStatusChangeResponse {
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

pub async fn handle_inventory_status_change(
    State((db, mesh)): State<(sqlx::PgPool, Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>)>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<InventoryStatusChangeRequest>,
) -> impl IntoResponse {
    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse { error: "UNAUTHORIZED".to_string(), message: "Missing or invalid SPIFFE ID".to_string() }),
        ).into_response();
    }

    let query = "
        UPDATE products
        SET inventory_count = $1
        WHERE id = $2 AND tenant_id = $3
        RETURNING id
    ";

    let result = sqlx::query(query)
        .bind(payload.inventory_count)
        .bind(&payload.product_id)
        .bind(&tenant_id)
        .fetch_optional(&db)
        .await;

    match result {
        Ok(Some(_)) => {
            // Publish mesh event for agents
            let event = ::server_ohc::orchestration::TeammateMeshEvent {
                action: "InventoryStatusChanged".to_string(),
                agent_id: "system".to_string(),
                status: "".to_string(),
                msg_id: uuid::Uuid::new_v4().to_string(),
                payload: serde_json::json!({
                    "product_id": payload.product_id,
                    "inventory_count": payload.inventory_count,
                    "tenant_id": tenant_id
                }).to_string().into_bytes(),
            };
            let _ = mesh.publish("mesh:inventory:status_changed", event).await;

            (StatusCode::OK, Json(InventoryStatusChangeResponse { success: true, message: Some("Inventory updated".to_string()) })).into_response()
        }
        Ok(None) => {
            // Might not exist or wrong tenant
            (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "NOT_FOUND".to_string(), message: "Product not found".to_string() })).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to update inventory for product {}: {}", payload.product_id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "DATABASE_ERROR".to_string(), message: "Failed to update inventory".to_string() })).into_response()
        }
    }
}

pub fn router<S: Clone + Send + Sync + 'static>(db_pool: sqlx::PgPool, mesh: Arc<dyn MeshTransport>) -> Router<S> {
    Router::new()
        .route("/status", post(handle_inventory_status_change))
        .with_state((db_pool, mesh))
}
