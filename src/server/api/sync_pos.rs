use axum::{
    extract::{State, Extension, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use crate::auth::Claims;
use crate::orchestration::locks::{DistributedLock, StandaloneLock};

#[derive(Deserialize, Debug)]
pub struct SyncPosRequest {
    pub client_id: String,
    pub transactions: Vec<OfflineTransaction>,
}

#[derive(Deserialize, Debug)]
pub struct OfflineTransaction {
    pub id: String,
    pub amount_cents: i64,
    pub currency: String,
    pub payload: serde_json::Value,
    pub timestamp: String,
    pub product_id: Option<String>,
    pub quantity: Option<i32>,
}

#[derive(Serialize)]
pub struct SyncPosResponse {
    pub success: bool,
    pub applied_count: i32,
    pub conflict_count: i32,
}

pub async fn sync_pos_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<SyncPosRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, axum::Json(SyncPosResponse { success: false, applied_count: 0, conflict_count: 0 })).into_response(),
    };

    let lock_manager = StandaloneLock::new();
    let mut applied_count = 0;
    let mut conflict_count = 0;

    for tx in payload.transactions {
        let _guard = match tx.product_id.as_ref() {
            Some(pid) => {
                match lock_manager.acquire_resource(&tenant_id, "inventory", pid).await {
                    Ok(guard) => Some(guard),
                    Err(_) => {
                        conflict_count += 1;
                        continue;
                    }
                }
            },
            None => None,
        };

        // Simplified logic: push offline transaction to the DB
        let res = sqlx::query(
            "INSERT INTO pos_offline_transactions (id, tenant_id, client_id, amount_cents, currency, payload, status)
             VALUES ($1, $2, $3, $4, $5, $6, 'SYNCED')
             ON CONFLICT DO NOTHING"
        )
        .bind(&tx.id)
        .bind(&tenant_id)
        .bind(&payload.client_id)
        .bind(&tx.amount_cents)
        .bind(&tx.currency)
        .bind(&tx.payload)
        .execute(&pool)
        .await;

        match res {
            Ok(_) => {
                // If it was an inventory reduction, check conflicts
                if let (Some(pid), Some(qty)) = (&tx.product_id, tx.quantity) {
                     let res2 = sqlx::query(
                        "UPDATE products SET available_quantity = available_quantity - $1
                         WHERE id = $2 AND tenant_id = $3 AND available_quantity >= $1"
                     )
                     .bind(qty)
                     .bind(pid)
                     .bind(&tenant_id)
                     .execute(&pool)
                     .await;

                     if let Ok(result) = res2 {
                         if result.rows_affected() == 0 {
                             // Conflict! Send to Operations Agent
                             let alert_payload = serde_json::json!({
                                 "event_type": "offline_inventory_conflict",
                                 "transaction_id": tx.id,
                                 "product_id": pid,
                                 "requested_quantity": qty,
                                 "resolution_options": ["cancel_online_order", "refund_pos_order"]
                             });

                             let _ = sqlx::query(
                                "INSERT INTO agent_feed (id, tenant_id, title, content, priority, context)
                                 VALUES ($1, $2, 'Offline POS Inventory Conflict', 'An offline POS transaction resulted in negative inventory.', 'HIGH', $3)"
                             )
                             .bind(uuid::Uuid::new_v4().to_string())
                             .bind(&tenant_id)
                             .bind(&alert_payload)
                             .execute(&pool)
                             .await;

                             conflict_count += 1;
                         } else {
                             applied_count += 1;
                         }
                     }
                } else {
                    applied_count += 1;
                }
            },
            Err(_) => conflict_count += 1,
        }
    }

    (StatusCode::OK, axum::Json(SyncPosResponse { success: true, applied_count, conflict_count })).into_response()
}
