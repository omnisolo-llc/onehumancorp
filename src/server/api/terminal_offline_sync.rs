use super::{
    PosOfflineTransaction, SyncOfflineTransactionsRequest, SyncOfflineTransactionsResponse,
};
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::{Value, json};
use sqlx::{PgPool, Postgres, Transaction};

use super::super::sync_transaction::SyncError;

#[path = "terminal_offline_authority.rs"]
mod offline_authority;

const ROUTE: &str = "/api/v1/payments/terminal/sync_offline";
#[derive(serde::Serialize)]
pub struct TerminalOutcome {
    id: String,
    route: &'static str,
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<&'static str>,
}
struct Applied {
    id: String,
    replay: bool,
    status: &'static str,
    reason: Option<&'static str>,
    conflicts: Vec<Value>,
    products: Vec<String>,
}
fn blocked(id: &str, reason: &'static str) -> Applied {
    Applied {
        id: id.into(),
        replay: false,
        status: "blocked",
        reason: Some(reason),
        conflicts: vec![],
        products: vec![],
    }
}
fn reconciliation(id: &str, reason: &'static str) -> Applied {
    Applied {
        status: "reconciliation",
        ..blocked(id, reason)
    }
}

pub(super) async fn sync_offline_response(
    pool: &PgPool,
    tenant: &str,
    request: &SyncOfflineTransactionsRequest,
    redis: Option<redis::Client>,
) -> axum::response::Response {
    if tenant.trim().is_empty() || tenant.trim() != tenant || tenant.eq_ignore_ascii_case("system")
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error":"Tenant-scoped authentication required"})),
        )
            .into_response();
    }
    let mut response = SyncOfflineTransactionsResponse {
        success: true,
        synced_count: 0,
        failed_transaction_ids: vec![],
        pending_reconciliation: None,
        acknowledged_transaction_ids: vec![],
        already_processed_transaction_ids: vec![],
        reconciliation_required_transaction_ids: vec![],
        outcomes: vec![],
    };
    for item in &request.transactions {
        let id = item.id.as_deref().unwrap_or("");
        let applied = match apply(pool, tenant, request.session_id.as_deref(), item).await {
            Ok(result) => result,
            Err(error) => {
                tracing::warn!(%error,"Offline terminal transaction did not commit");
                Applied {
                    status: error.status(),
                    ..blocked(id, error.reason())
                }
            }
        };
        match applied.status {
            "acknowledged" => {
                response.synced_count += 1;
                response
                    .acknowledged_transaction_ids
                    .push(applied.id.clone());
                if applied.replay {
                    response
                        .already_processed_transaction_ids
                        .push(applied.id.clone());
                }
            }
            "reconciliation" => {
                response.success = false;
                response
                    .reconciliation_required_transaction_ids
                    .push(applied.id.clone());
            }
            _ => {
                response.success = false;
                response.failed_transaction_ids.push(applied.id.clone());
            }
        }
        if !applied.conflicts.is_empty() {
            response
                .pending_reconciliation
                .get_or_insert_default()
                .extend(applied.conflicts);
        }
        response.outcomes.push(TerminalOutcome {
            id: applied.id,
            route: ROUTE,
            status: applied.status,
            reason: applied.reason,
        });
        // Cache/event notifications happen only for newly committed effects.
        if !applied.products.is_empty()
            && let Some(client) = &redis
            && let Ok(mut conn) = client.get_multiplexed_async_connection().await
        {
            for product in applied.products {
                let payload=json!({"event":"inventory.updated","tags":[format!("tenant-id:{tenant}"),format!("entity:product:{product}")]}).to_string();
                let _: Result<(), _> = redis::cmd("PUBLISH")
                    .arg("cache_invalidation_events")
                    .arg(&payload)
                    .query_async(&mut conn)
                    .await;
            }
        }
    }
    (StatusCode::OK, Json(response)).into_response()
}

async fn apply(
    pool: &PgPool,
    tenant: &str,
    session: Option<&str>,
    item: &PosOfflineTransaction,
) -> Result<Applied, SyncError> {
    let id = item.id.as_deref().unwrap_or("");
    if id.trim().is_empty() || id.trim() != id || id.len() > 512 {
        return Ok(blocked(id, "stable_transaction_id_required"));
    }
    // This is the existing non-cryptographic transport check. Every request,
    // including a replay, must pass it; it does not attest device ownership.
    if item
        .device_signature
        .as_deref()
        .is_none_or(|s| !s.starts_with("sig_") || s.len() <= 4 || s.trim() != s)
    {
        return Ok(blocked(id, "invalid_device_signature"));
    }
    let client = item.client_id.as_deref().unwrap_or("");
    if client.trim().is_empty()
        || item.amount_cents < 0
        || item.currency.len() != 3
        || !item.currency.bytes().all(|b| b.is_ascii_alphabetic())
    {
        return Ok(blocked(id, "invalid_transaction"));
    }
    let kind = item.mutation_type.as_deref().unwrap_or("payment");
    if !matches!(kind, "cash_sale" | "tap_to_pay" | "payment") {
        return Ok(blocked(id, "unsupported_operation"));
    }
    if kind == "tap_to_pay" && !item.currency.eq_ignore_ascii_case("USD") {
        return Ok(reconciliation(id, "unsupported_offline_card_currency"));
    }
    let payload: Value = match serde_json::from_str(&item.payload) {
        Ok(v) => v,
        Err(_) => return Ok(blocked(id, "invalid_payload")),
    };
    if !payload.is_object() && !payload.is_array() {
        return Ok(blocked(id, "invalid_payload"));
    }
    let mut quantities = std::collections::BTreeMap::<String, i32>::new();
    if let Some(items) = payload.as_array() {
        for line in items {
            let product = line.get("product_id").and_then(Value::as_str).unwrap_or("");
            let quantity = line
                .get("quantity")
                .and_then(Value::as_i64)
                .and_then(|q| i32::try_from(q).ok());
            let Some(quantity) = quantity.filter(|q| *q > 0) else {
                return Ok(blocked(id, "invalid_quantity"));
            };
            if product.trim().is_empty() {
                return Ok(blocked(id, "invalid_product"));
            }
            let previous = quantities.get(product).copied().unwrap_or(0);
            let Some(total) = previous.checked_add(quantity) else {
                return Ok(blocked(id, "invalid_quantity"));
            };
            quantities.insert(product.into(), total);
        }
    }
    let identity = json!({"tenant_id":tenant,"id":id,"client_id":client,"amount_cents":item.amount_cents,"currency":item.currency,"payload":payload,"timestamp":item.timestamp,"mutation_type":item.mutation_type,"terminal_id":item.terminal_id});
    if kind == "cash_sale" && offline_authority::explicit_kind(&identity) != Some("cash_sale") {
        return Ok(reconciliation(id, "contradictory_cash_payment_evidence"));
    }
    let mut tx = pool.begin().await?;
    ::server_common::auth_utils::set_org_context(&mut *tx, tenant).await?;
    let inserted:Option<String>=sqlx::query_scalar("INSERT INTO pos_offline_transactions (id,tenant_id,client_id,amount_cents,currency,payload,status,_sync_status,device_signature,terminal_id,request_identity,request_status) VALUES ($1,$2,$3,$4,$5,$6,'PENDING','pending',$7,$8,$9,'pending') ON CONFLICT (id) DO NOTHING RETURNING id")
        .bind(id).bind(tenant).bind(client).bind(item.amount_cents).bind(&item.currency).bind(&payload).bind(&item.device_signature).bind(&item.terminal_id).bind(&identity).fetch_optional(&mut *tx).await?;
    if inserted.is_none() {
        let old:Option<(Option<Value>,Option<String>,Option<Value>)>=sqlx::query_as("SELECT request_identity,request_status,request_reconciliation FROM pos_offline_transactions WHERE id=$1 AND tenant_id=$2 FOR UPDATE").bind(id).bind(tenant).fetch_optional(&mut *tx).await?;
        if let Some((Some(old), Some(status), conflicts)) = old
            && old == identity
            && (status == "acknowledged" || status == "reconciliation")
        {
            return Ok(Applied {
                id: id.into(),
                replay: true,
                status: if status == "acknowledged" {
                    "acknowledged"
                } else {
                    "reconciliation"
                },
                reason: if status == "reconciliation" {
                    Some("inventory_shortage")
                } else {
                    None
                },
                conflicts: conflicts
                    .and_then(|v| v.as_array().cloned())
                    .unwrap_or_default(),
                products: vec![],
            });
        }
        return Ok(reconciliation(id, "request_identity_changed_or_unverified"));
    }
    let mut conflicts = vec![];
    for (product, quantity) in &quantities {
        let stock: Option<(i32,bool)> = sqlx::query_as(
            "SELECT available_quantity, (to_jsonb(products) ? 'pn_counter_p' AND to_jsonb(products) ? 'pn_counter_n') FROM products WHERE id=$1 AND tenant_id=$2 FOR UPDATE",
        )
        .bind(product)
        .bind(tenant)
        .fetch_optional(&mut *tx)
        .await?;
        let Some((stock, has_counters)) = stock else {
            return Ok(blocked(id, "product_not_found_in_tenant"));
        };
        if stock < *quantity {
            let expected_stock = i64::from(*quantity);
            let actual_stock = i64::from(stock);
            conflicts.push(json!({"transaction_id":id,"product_id":product,"shortage":expected_stock-actual_stock}));

            let conflict_id = format!("sync_conflict_{}_{}", id, product);
            let notification_payload = json!({
                "transaction_id": id,
                "product_id": product,
                "expected_stock": expected_stock,
                "actual_stock": actual_stock,
                "message": format!("Inventory Sync Conflict: {} sold out offline, causing an online shortage. Operations is resolving this.", product)
            });
            sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, source, agent_type, action_type, payload, status) VALUES ($1, $2, 'terminal_offline', 'operations', 'inventory.sync.conflict', $3::jsonb, 'PENDING') ON CONFLICT DO NOTHING")
                .bind(&conflict_id)
                .bind(tenant)
                .bind(notification_payload)
                .execute(&mut *tx).await?;
        }
        sqlx::query(if has_counters {
            "UPDATE products SET pn_counter_n=COALESCE(pn_counter_n,0)+$1,inventory_count=GREATEST(0,COALESCE(pn_counter_p,0)-(COALESCE(pn_counter_n,0)+$1)),available_quantity=GREATEST(0,available_quantity-$1),updated_at=clock_timestamp() WHERE id=$2 AND tenant_id=$3"
        } else {
            "UPDATE products SET inventory_count=GREATEST(0,inventory_count-$1),available_quantity=GREATEST(0,available_quantity-$1),updated_at=clock_timestamp() WHERE id=$2 AND tenant_id=$3"
        }).bind(quantity).bind(product).bind(tenant).execute(&mut *tx).await?;
    }
    let job = terminal_offline_job(id, client, item.amount_cents, &item.currency, item.mutation_type.as_deref(), &payload);
    sqlx::query("INSERT INTO ohc_job_queue (id,tenant_id,job_type,payload) VALUES ($1,$2,'offline_pos_sync',$3)").bind(uuid::Uuid::new_v4().to_string()).bind(tenant).bind(job).execute(&mut *tx).await?;
    if matches!(kind, "cash_sale" | "tap_to_pay") {
        let order = uuid::Uuid::new_v4().to_string();
        // Cash is an owner-recorded sale; a queued card payment is still pending.
        let status = if kind == "cash_sale" {
            "completed"
        } else {
            "pending_payment"
        };
        sqlx::query("INSERT INTO orders (id,tenant_id,customer_id,total_amount,status) VALUES ($1,$2,NULL,$3::bigint::numeric / 100,$4)").bind(&order).bind(tenant).bind(item.amount_cents).bind(status).execute(&mut *tx).await?;
        for (product, quantity) in &quantities {
            sqlx::query("INSERT INTO order_items (id,tenant_id,order_id,product_id,quantity,price) SELECT $1,$2,$3,id,$4,price FROM products WHERE id=$5 AND tenant_id=$2")
                .bind(uuid::Uuid::new_v4().to_string()).bind(tenant).bind(&order).bind(quantity).bind(product).execute(&mut *tx).await?;
        }
        for department in ["sales_and_revenue", "operations"] {
            sqlx::query("INSERT INTO agent_action_requests (id,tenant_id,source,agent_type,action_type,payload,status) VALUES ($1,$2,'terminal_offline',$3,'record_pos_transaction',$4,'pending')")
                .bind(uuid::Uuid::new_v4().to_string()).bind(tenant).bind(department).bind(json!({"event":"pos_transaction_synced","transaction_id":id,"order_id":order,"amount_cents":item.amount_cents})).execute(&mut *tx).await?;
        }
    }
    record_session(&mut tx, tenant, session, client, &conflicts).await?;
    let status = if conflicts.is_empty() {
        "acknowledged"
    } else {
        "reconciliation"
    };
    sqlx::query("UPDATE pos_offline_transactions SET request_status=$1,request_reconciliation=$2 WHERE id=$3 AND tenant_id=$4").bind(status).bind(json!(conflicts)).bind(id).bind(tenant).execute(&mut *tx).await?;
    tx.commit().await.map_err(SyncError::Commit)?;
    Ok(Applied {
        id: id.into(),
        replay: false,
        status,
        reason: if conflicts.is_empty() {
            None
        } else {
            Some("inventory_shortage")
        },
        conflicts,
        products: quantities.into_keys().collect(),
    })
}

async fn record_session(
    tx: &mut Transaction<'_, Postgres>,
    tenant: &str,
    session: Option<&str>,
    client: &str,
    conflicts: &[Value],
) -> Result<(), sqlx::Error> {
    let session = session
        .map(str::to_owned)
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    sqlx::query("INSERT INTO pos_terminal_sessions (id,tenant_id,device_id,status,started_at,last_synced_at,offline_changes_count,sync_status,pending_reconciliation) VALUES ($1,$2,$3,'ACTIVE',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP,1,$4,$5) ON CONFLICT (tenant_id,device_id) DO UPDATE SET last_synced_at=CURRENT_TIMESTAMP,offline_changes_count=pos_terminal_sessions.offline_changes_count+1,sync_status=CASE WHEN $4='CONFLICTS_PENDING' THEN $4 ELSE pos_terminal_sessions.sync_status END,pending_reconciliation=COALESCE(pos_terminal_sessions.pending_reconciliation,'[]'::jsonb)||$5")
        .bind(session).bind(tenant).bind(client).bind(if conflicts.is_empty(){"SYNCED"}else{"CONFLICTS_PENDING"}).bind(json!(conflicts)).execute(&mut **tx).await?;
    Ok(())
}

#[cfg(test)]
#[path = "terminal_offline_sync_test.rs"]
mod tests;

fn terminal_offline_job(id: &str, client: &str, amount: i64, currency: &str, kind: Option<&str>, payload: &Value) -> Value {
    json!({"pos_transaction_id":id,"client_id":client,"amount_cents":amount,
        "currency":currency,"payload":payload.to_string(),"mutation_type":kind,
        "inventory_already_deducted":true})
}
