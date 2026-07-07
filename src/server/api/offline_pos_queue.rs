use axum::{extract::State, Json, http::{StatusCode, HeaderMap}, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{error, info};
use crate::auth::parse_spiffe_id;

#[derive(Deserialize, Debug)]
pub struct PosTransaction {
    pub transaction_id: String,
    pub payload: serde_json::Value,
    pub status: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct InventoryDelta {
    pub product_id: String,
    pub quantity_delta: i32,
}

#[derive(Deserialize, Debug)]
pub struct PosQueueSyncRequest {
    pub transactions: Vec<PosTransaction>,
    pub inventory_deltas: Vec<InventoryDelta>, // simplified: all deltas associated with this sync batch
}

#[derive(Serialize)]
pub struct PosQueueSyncResponse {
    pub success: bool,
    pub synced_count: i32,
}

pub async fn offline_pos_queue_handler(
    State(db): State<PgPool>,
    headers: HeaderMap,
    Json(payload): Json<PosQueueSyncRequest>,
) -> impl IntoResponse {
    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        return (StatusCode::UNAUTHORIZED, Json(PosQueueSyncResponse { success: false, synced_count: 0 })).into_response();
    }

    let mut tx = match db.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            error!("Failed to begin transaction: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(PosQueueSyncResponse { success: false, synced_count: 0 })).into_response();
        }
    };

    let mut synced_count = 0;

    for t in payload.transactions {
        let transaction_id = uuid::Uuid::parse_str(&t.transaction_id).unwrap_or_else(|_| uuid::Uuid::new_v4());
        let status = t.status.unwrap_or_else(|| "synced".to_string());

        let res = sqlx::query(
            "INSERT INTO local_transaction_queue (transaction_id, tenant_id, status, payload) VALUES ($1, $2, $3, $4) ON CONFLICT (transaction_id) DO NOTHING"
        )
        .bind(transaction_id)
        .bind(tenant_id.clone())
        .bind(status)
        .bind(t.payload)
        .execute(&mut *tx).await;

        match res {
            Ok(_) => synced_count += 1,
            Err(e) => error!("Failed to insert to local_transaction_queue: {}", e)
        }
    }

    for d in payload.inventory_deltas {
        let ledger_id = uuid::Uuid::new_v4();
        let product_id = uuid::Uuid::parse_str(&d.product_id).unwrap_or_else(|_| uuid::Uuid::new_v4());
        let dummy_tx_id = uuid::Uuid::new_v4(); // simplified mapping for demo

        let res = sqlx::query(
            "INSERT INTO inventory_ledger (ledger_id, tenant_id, product_id, quantity_delta, transaction_id) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(ledger_id)
        .bind(tenant_id.clone())
        .bind(product_id)
        .bind(d.quantity_delta)
        .bind(dummy_tx_id)
        .execute(&mut *tx).await;

        if let Err(e) = res {
             error!("Failed to insert to inventory_ledger: {}", e);
        }
    }

    if let Err(e) = tx.commit().await {
        error!("Failed to commit POS queue sync: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(PosQueueSyncResponse { success: false, synced_count: 0 })).into_response();
    }

    info!("Simulating trigger for AI Ops Agent for tenant {}", tenant_id);

    (StatusCode::OK, Json(PosQueueSyncResponse { success: true, synced_count })).into_response()
}
