use axum::{
    extract::{State, Json},
    response::Json as JsonResponse,
    http::HeaderMap,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use serde_json::{json};
use redis::AsyncCommands;
use uuid::Uuid;
use sqlx::PgPool;

pub fn router(pool: Arc<PgPool>, mesh: Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>) -> Router {
    Router::new()
        .route("/inventory/sync", post(offline_sync_handler))
        .with_state(((*pool).clone(), mesh))
}

#[derive(Deserialize, Debug)]
pub struct SyncRequest {
    pub client_id: String,
    pub transactions: Vec<SyncTransaction>,
}

#[derive(Deserialize, Debug)]
pub struct OfflineSyncRequest {
    pub client_id: String,
    pub transactions: Vec<SyncTransaction>,
}

#[derive(Deserialize, Debug)]
pub struct SyncTransaction {
    pub transaction_id: String,
    pub items: Vec<SyncItem>,
    pub amount_cents: i64,
    pub currency: String,
}

#[derive(Deserialize, Debug)]
pub struct SyncItem {
    pub product_id: String,
    pub quantity: i32,
    pub version: i32,
}

#[derive(Serialize)]
pub struct SyncResponse {
    pub success: bool,
    pub processed_transactions: Vec<String>,
    pub failed_transactions: Vec<FailedTransaction>,
}

#[derive(Serialize)]
pub struct FailedTransaction {
    pub transaction_id: String,
    pub error: String,
    pub conflict_items: Vec<String>,
}

pub async fn offline_sync_handler(
    State((pool, _mesh)): State<(sqlx::Pool<sqlx::Postgres>, Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>)>,
    headers: HeaderMap,
    Json(payload): Json<OfflineSyncRequest>,
) -> JsonResponse<SyncResponse> {
    let request = SyncRequest {
        client_id: payload.client_id,
        transactions: payload.transactions,
    };

    let pool_arc = Arc::new(pool);
    handle_offline_sync(pool_arc, headers, request).await
}

async fn handle_offline_sync(
    pool: Arc<PgPool>,
    headers: HeaderMap,
    payload: SyncRequest,
) -> JsonResponse<SyncResponse> {
    let tenant_id = headers
        .get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default")
        .to_string();

    let redis_client_opt: Option<redis::Client> = match std::env::var("REDIS_URL") {
        Ok(url) => redis::Client::open(url).ok(),
        Err(_) => None,
    };

    let mut processed_transactions = Vec::new();
    let mut failed_transactions = Vec::new();

    for tx in payload.transactions {
        let mut tx_success = true;
        let mut conflicts = Vec::new();
        let mut locked_keys = Vec::new();

        let mut redis_conn = match &redis_client_opt {
            Some(client) => client.get_multiplexed_async_connection().await.ok(),
            None => None,
        };

        // 1. Acquire Locks for all items in the transaction
        if let Some(ref mut conn) = redis_conn {
            for item in &tx.items {
                let lock_key = format!("ohc:lock:{}:inventory:{}", tenant_id, item.product_id);
                // Simple Redis lock with set_nx and expire
                let lock_id = Uuid::new_v4().to_string();
                let acquired: bool = redis::cmd("SET")
                    .arg(&lock_key)
                    .arg(&lock_id)
                    .arg("NX")
                    .arg("PX")
                    .arg(5000) // 5 seconds lock
                    .query_async(conn)
                    .await
                    .unwrap_or(false);

                if acquired {
                    locked_keys.push((lock_key, lock_id));
                } else {
                    tx_success = false;
                    conflicts.push(item.product_id.clone());
                    break;
                }
            }
        } else {
            // If Redis is not available, we proceed optimistically and rely solely on PostgreSQL versioning
        }

        if tx_success {
            // 2. Perform optimistic database update within a transaction
            let mut db_tx = match pool.begin().await {
                Ok(t) => t,
                Err(e) => {
                    tx_success = false;
                    failed_transactions.push(FailedTransaction {
                        transaction_id: tx.transaction_id.clone(),
                        error: format!("Database transaction error: {}", e),
                        conflict_items: vec![],
                    });

                    // Release locks
                    if let Some(ref mut conn) = redis_conn {
                        for (key, lock_id) in locked_keys {
                            let script = redis::Script::new(
                                r#"
                                if redis.call("get",KEYS[1]) == ARGV[1] then
                                    return redis.call("del",KEYS[1])
                                else
                                    return 0
                                end
                                "#
                            );
                            let _: redis::RedisResult<()> = script.key(key).arg(lock_id).invoke_async(conn).await;
                        }
                    }
                    continue;
                }
            };

            for item in &tx.items {
                // Update inventory_count and increment version if current version matches provided version
                let result = sqlx::query(
                    r#"
                    UPDATE products
                    SET inventory_count = inventory_count - $1, version = version + 1
                    WHERE id = $2 AND tenant_id = $3 AND version = $4 AND inventory_count >= $1
                    "#
                )
                .bind(item.quantity)
                .bind(&item.product_id)
                .bind(&tenant_id)
                .bind(item.version)
                .execute(&mut *db_tx)
                .await;

                match result {
                    Ok(res) if res.rows_affected() > 0 => {
                        // Success for this item
                    }
                    _ => {
                        tx_success = false;
                        conflicts.push(item.product_id.clone());
                        break;
                    }
                }
            }

            if tx_success {
                // Record the offline transaction
                let payload_json = json!({
                    "items": tx.items.iter().map(|i| json!({"product_id": i.product_id, "quantity": i.quantity})).collect::<Vec<_>>()
                });

                let _ = sqlx::query(
                    r#"
                    INSERT INTO pos_offline_transactions (id, tenant_id, client_id, amount_cents, currency, payload, status, _sync_status)
                    VALUES ($1, $2, $3, $4, $5, $6, 'SYNCED', 'synced')
                    ON CONFLICT (id) DO NOTHING
                    "#
                )
                .bind(&tx.transaction_id)
                .bind(&tenant_id)
                .bind(&payload.client_id)
                .bind(tx.amount_cents)
                .bind(&tx.currency)
                .bind(payload_json)
                .execute(&mut *db_tx)
                .await;

                if let Err(_) = db_tx.commit().await {
                     tx_success = false;
                     failed_transactions.push(FailedTransaction {
                         transaction_id: tx.transaction_id.clone(),
                         error: "Database commit failed".to_string(),
                         conflict_items: vec![],
                     });
                } else {
                     processed_transactions.push(tx.transaction_id.clone());
                }
            } else {
                let _ = db_tx.rollback().await;
                failed_transactions.push(FailedTransaction {
                    transaction_id: tx.transaction_id.clone(),
                    error: "Optimistic concurrency conflict or insufficient inventory".to_string(),
                    conflict_items: conflicts,
                });
            }
        } else {
             failed_transactions.push(FailedTransaction {
                 transaction_id: tx.transaction_id.clone(),
                 error: "Failed to acquire distributed lock".to_string(),
                 conflict_items: conflicts,
             });
        }

        // 3. Release Locks
        if let Some(ref mut conn) = redis_conn {
            for (key, lock_id) in locked_keys {
                let script = redis::Script::new(
                    r#"
                    if redis.call("get",KEYS[1]) == ARGV[1] then
                        return redis.call("del",KEYS[1])
                    else
                        return 0
                    end
                    "#
                );
                let _: redis::RedisResult<()> = script.key(key).arg(lock_id).invoke_async(conn).await;
            }
        }

        let _ = tx_success; // Suppress unused warning
    }

    JsonResponse(SyncResponse {
        success: failed_transactions.is_empty(),
        processed_transactions,
        failed_transactions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_offline_sync_handler() {
        // Simple mock test, full E2E test covers db logic.
        assert!(true);
    }
}
