use axum::{
    extract::{Extension, Json, State},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use axum::http::StatusCode;

#[derive(Deserialize)]
pub struct InventoryStatusRequest {
    pub tenant_id: String,
    pub product_id: String,
    pub stock_count: i32,
    pub product_name: Option<String>,
}

#[derive(Serialize)]
pub struct InventoryStatusResponse {
    pub success: bool,
}

async fn handle_inventory_status(
    Extension(_hub): Extension<Arc<crate::hub::Hub>>,
    State(mesh): State<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>>,
    Json(payload): Json<InventoryStatusRequest>,
) -> impl IntoResponse {
    let event = ::server_ohc::orchestration::TeammateMeshEvent {
        action: "StatusChanged".to_string(),
        agent_id: "system".to_string(),
        status: "".to_string(),
        msg_id: uuid::Uuid::new_v4().to_string(),
        payload: serde_json::json!({
            "product_id": payload.product_id,
            "stock_count": payload.stock_count,
            "tenant_id": payload.tenant_id,
            "product_name": payload.product_name,
        }).to_string().into_bytes(),
    };
    let _ = mesh.publish("mesh:inventory:status_changed", event).await;

    (StatusCode::OK, Json(InventoryStatusResponse { success: true })).into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<crate::hub::Hub>, mesh: Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>) -> Router<S> {
    Router::new()
        .route("/status", post(handle_inventory_status))
        .layer(Extension(hub))
        .with_state(mesh)
}
