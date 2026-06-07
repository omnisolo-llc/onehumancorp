use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct OfflineMutation {
    pub transaction_id: String,
    pub product_id: String,
    pub quantity_deducted: i32,
    pub amount: Option<i64>, // amount in cents
    pub payment_method: Option<String>,
    pub payment_intent_id: Option<String>,
    pub currency: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OfflineSyncRequest {
    pub mutations: Vec<OfflineMutation>,
}

#[derive(Serialize)]
pub struct OfflineSyncResponse {
    pub success: bool,
}

pub async fn offline_sync_handler(
    State((db, mesh)): State<(sqlx::PgPool, Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>)>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<OfflineSyncRequest>,
) -> impl IntoResponse {
    tracing::info!("Received {} offline mutations for CRDT-based edge sync.", payload.mutations.len());

    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(OfflineSyncResponse { success: false }),
        ).into_response();
    }

    let cache = crate::builder::edge::get_edge_cache();
    cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

    for mutation in &payload.mutations {
        cache.invalidate_by_tag(&format!("entity:product:{}", mutation.product_id)).await;

        let mut tx = match db.begin().await {
            Ok(t) => t,
            Err(e) => {
                tracing::error!("CRDT sync failed to begin db tx: {}", e);
                continue;
            }
        };

        let tx_id = if mutation.transaction_id.is_empty() { Uuid::new_v4().to_string() } else { mutation.transaction_id.clone() };

        // Append to pos_offline_transactions (grow-only set for transactions)
        let insert_res = sqlx::query(
            "INSERT INTO pos_offline_transactions (id, tenant_id, client_id, amount_cents, currency, payload, status)
             VALUES ($1, $2, 'offline_sync', $3, $4, $5::jsonb, 'PENDING')
             ON CONFLICT (id) DO NOTHING"
        )
        .bind(&tx_id)
        .bind(&tenant_id)
        .bind(mutation.amount.unwrap_or(0))
        .bind(mutation.currency.as_deref().unwrap_or("USD"))
        .bind(serde_json::to_string(mutation).unwrap_or_else(|_| "{}".to_string()))
        .execute(&mut *tx)
        .await;

        if let Err(e) = insert_res {
            tracing::error!("CRDT sync failed to insert offline transaction: {}", e);
            let _ = tx.rollback().await;
            continue;
        }

        // Queue job for asynchronous processing
        let job_id = Uuid::new_v4().to_string();
        let job_payload = serde_json::json!({
            "transaction_id": tx_id,
            "product_id": mutation.product_id,
            "quantity_deducted": mutation.quantity_deducted,
            "amount": mutation.amount,
            "payment_method": mutation.payment_method,
            "payment_intent_id": mutation.payment_intent_id,
            "currency": mutation.currency,
        }).to_string();

        let job_res = sqlx::query(
            "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload)
             VALUES ($1, $2, 'offline_pos_sync', $3::jsonb)"
        )
        .bind(&job_id)
        .bind(&tenant_id)
        .bind(&job_payload)
        .execute(&mut *tx)
        .await;

        if let Err(e) = job_res {
            tracing::error!("CRDT sync failed to enqueue offline_pos_sync job: {}", e);
        }

        let _ = tx.commit().await;

        // Publish mesh event for realtime updates across devices
        let event = ::server_ohc::orchestration::TeammateMeshEvent {
            action: "InventoryUpdated".to_string(),
            agent_id: "system".to_string(),
            status: "".to_string(),
            msg_id: uuid::Uuid::new_v4().to_string(),
            payload: serde_json::json!({
                "product_id": mutation.product_id,
                "transaction_id": tx_id,
                "quantity_deducted": mutation.quantity_deducted,
                "tenant_id": tenant_id
            }).to_string().into_bytes(),
        };
        let _ = mesh.publish("mesh:inventory:updated", event).await;
    }

    (
        StatusCode::OK,
        Json(OfflineSyncResponse { success: true }),
    ).into_response()
}
