use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct MeshMutation {
    pub id: String,
    pub action_type: String,
    pub payload: serde_json::Value,
    #[serde(default)]
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct MeshSyncRequest {
    pub batch_id: Option<String>,
    pub mutations: Vec<MeshMutation>,
}

#[derive(Serialize)]
pub struct MeshSyncResponse {
    pub success: bool,
    pub synced_count: usize,
}

pub async fn sync_mesh_handler(
    State((db, _mesh)): State<(sqlx::PgPool, Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>)>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<MeshSyncRequest>,
) -> impl IntoResponse {
    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(MeshSyncResponse { success: false, synced_count: 0 }),
        ).into_response();
    }

    let batch_id = payload.batch_id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let mut synced_count = 0;

    let mut db_tx = match db.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(MeshSyncResponse { success: false, synced_count: 0 }),
            ).into_response();
        }
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *db_tx, &tenant_id).await {
        tracing::error!("Failed to set org context: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MeshSyncResponse { success: false, synced_count: 0 }),
        ).into_response();
    }

    for mutation in &payload.mutations {
        let payload_str = serde_json::to_string(&mutation.payload).unwrap_or_else(|_| "{}".to_string());

        let insert_mutation_res = sqlx::query(
            "INSERT INTO mutation_queue (id, tenant_id, action_type, payload, status)
             VALUES ($1, $2, $3, $4::jsonb, 'synced')
             ON CONFLICT (id) DO UPDATE SET status = 'synced', payload = excluded.payload"
        )
        .bind(&mutation.id)
        .bind(&tenant_id)
        .bind(&mutation.action_type)
        .bind(&payload_str)
        .execute(&mut *db_tx)
        .await;

        if let Err(e) = insert_mutation_res {
            tracing::error!("Failed to insert mutation_queue: {}", e);
            continue;
        }

        let sync_event_id = Uuid::new_v4().to_string();
        let insert_sync_event_res = sqlx::query(
            "INSERT INTO sync_events (id, tenant_id, batch_id, action_type, payload)
             VALUES ($1, $2, $3, $4, $5::jsonb)"
        )
        .bind(&sync_event_id)
        .bind(&tenant_id)
        .bind(&batch_id)
        .bind(&mutation.action_type)
        .bind(&payload_str)
        .execute(&mut *db_tx)
        .await;

        if let Err(e) = insert_sync_event_res {
            tracing::error!("Failed to insert sync_events: {}", e);
            continue;
        }

        // Trigger AI Agent through department_tasks
        if mutation.action_type == "JobCompleted" {
            let ai_task_id = Uuid::new_v4().to_string();
            let ai_payload = serde_json::json!({
                "batch_id": batch_id,
                "sync_event_id": sync_event_id,
                "action_type": mutation.action_type,
                "payload": mutation.payload
            }).to_string();

            let _ = sqlx::query(
                "INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status)
                 VALUES ($1, $2, 'operations', 'SyncEvent:JobCompleted', $3::jsonb, 'PENDING')"
            )
            .bind(&ai_task_id)
            .bind(&tenant_id)
            .bind(&ai_payload)
            .execute(&mut *db_tx)
            .await;
        }

        synced_count += 1;
    }

    if let Err(e) = db_tx.commit().await {
        tracing::error!("Failed to commit sync mesh transaction: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MeshSyncResponse { success: false, synced_count: 0 }),
        ).into_response();
    }

    (
        StatusCode::OK,
        Json(MeshSyncResponse { success: true, synced_count }),
    ).into_response()
}
