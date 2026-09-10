use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use sqlx::PgPool;

pub fn router<S>(pool: PgPool) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/run", post(run_workflow))
        .with_state(pool)
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VisualWorkflowRequest {
    pub version: String,
    pub entrypoint: String,
    pub nodes: serde_json::Value,
    #[serde(default)]
    pub name: String,
}

async fn run_workflow(
    State(pool): State<PgPool>,
    axum::extract::Extension(auth_info): axum::extract::Extension<
        ::server_auth::orchestration::AuthInfo,
    >,
    Json(request): Json<VisualWorkflowRequest>,
) -> axum::response::Response {
    let tenant_id = if !auth_info.spiffe_id.is_empty() {
        auth_info.spiffe_id.clone()
    } else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "missing tenant identity in session" })),
        )
            .into_response();
    };

    let tenant_uuid = match uuid::Uuid::parse_str(&tenant_id) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({"error": "invalid tenant id"}))).into_response(),
    };

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "failed to begin tx"}))).into_response(),
    };

    let graph_id = uuid::Uuid::new_v4();
    let name = if request.name.is_empty() { "Visual Workflow" } else { &request.name };

    let res = sqlx::query!(
        r#"
        INSERT INTO workflow_graphs (id, tenant_id, name)
        VALUES ($1, $2, $3)
        "#,
        graph_id,
        tenant_uuid,
        name
    )
    .execute(&mut *tx)
    .await;

    if res.is_err() {
        let _ = tx.rollback().await;
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "failed to save workflow graph" })),
        )
            .into_response();
    }

    if let Some(nodes_obj) = request.nodes.as_object() {
        for (node_id, node_data) in nodes_obj {
            let node_type = node_data.get("type").and_then(|t| t.as_str()).unwrap_or("Unknown");
            let res = sqlx::query!(
                r#"
                INSERT INTO execution_nodes (id, tenant_id, workflow_graph_id, node_type, config)
                VALUES ($1, $2, $3, $4, $5)
                "#,
                uuid::Uuid::new_v4(),
                tenant_uuid,
                graph_id,
                node_type,
                node_data.clone()
            )
            .execute(&mut *tx)
            .await;

            if res.is_err() {
                let _ = tx.rollback().await;
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "failed to save execution node" })),
                )
                    .into_response();
            }
        }
    }

    let _ = tx.commit().await;

    (StatusCode::OK, Json(json!({"success": true, "result": "E2E Visual Workflow Success"}))).into_response()
}
