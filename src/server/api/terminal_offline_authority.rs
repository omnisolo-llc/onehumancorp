//! Only a tenant-owned committed request proves that offline cash/inventory work
//! was applied. Uploading a queue envelope never proves a payment happened.
use serde_json::Value;
use sqlx::PgPool;

fn decoded(value: &Value) -> Option<Value> {
    match value {
        Value::String(text) => serde_json::from_str(text).ok(),
        Value::Object(_) | Value::Array(_) => Some(value.clone()),
        _ => None,
    }
}

/// Explicit classification is necessary, but never sufficient authority.
/// Reject contradictions anywhere in the original nested/serialized evidence.
fn consistent_evidence(value: &Value, kind: &str, depth: usize) -> bool {
    if depth > 16 {
        return false;
    }
    match value {
        Value::Array(items) => items
            .iter()
            .all(|v| consistent_evidence(v, kind, depth + 1)),
        Value::Object(fields) => {
            for key in ["mutation_type", "type"] {
                if let Some(value) = fields.get(key).filter(|v| !v.is_null()) {
                    if value.as_str() != Some(kind) {
                        return false;
                    }
                }
            }
            if fields
                .get("payment_intent_id")
                .is_some_and(|v| !v.is_null() && v.as_str() != Some(""))
            {
                return false;
            }
            if let Some(method) = fields.get("payment_method").filter(|v| !v.is_null()) {
                if kind != "cash_sale" || !matches!(method.as_str(), Some("cash" | "cash_sale")) {
                    return false;
                }
            }
            for (key, value) in fields {
                if value.is_null() {
                    continue;
                }
                if matches!(key.as_str(), "payload" | "mutation" | "items") {
                    let Some(value) = decoded(value) else {
                        return false;
                    };
                    if !consistent_evidence(&value, kind, depth + 1) {
                        return false;
                    }
                } else if (value.is_object() || value.is_array())
                    && !consistent_evidence(value, kind, depth + 1)
                {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

pub fn explicit_kind(identity: &Value) -> Option<&str> {
    identity
        .get("mutation_type")
        .and_then(Value::as_str)
        .filter(|kind| matches!(*kind, "cash_sale" | "inventory_sale"))
        .filter(|kind| consistent_evidence(identity, kind, 0))
}

fn transaction_id(payload: &Value) -> Option<&str> {
    let id = payload
        .get("transaction_id")
        .or_else(|| payload.get("pos_transaction_id"))?
        .as_str()?;
    if id.is_empty() || id.trim() != id || id.len() > 512 {
        return None;
    }
    if payload
        .get("transaction_id")
        .is_some_and(|v| v.as_str() != Some(id))
        || payload
            .get("pos_transaction_id")
            .is_some_and(|v| v.as_str() != Some(id))
    {
        return None;
    }
    Some(id)
}

/// Complete only effects already committed by a canonical producer. This worker
/// never collects cards, creates sales, deducts stock, or invents ledger credit.
/// Missing/foreign/legacy/contradictory evidence returns Err; the queue keeps it.
pub async fn complete_committed_operation(
    pool: &PgPool,
    tenant: &str,
    payload: &Value,
) -> Result<(), String> {
    if tenant.trim().is_empty() || tenant.trim() != tenant || tenant.eq_ignore_ascii_case("system")
    {
        return Err("Verified tenant required for offline reconciliation".into());
    }
    let id = transaction_id(payload)
        .ok_or("Offline transaction identity is missing; retain the original job")?;
    let mut tx = pool
        .begin()
        .await
        .map_err(|_| "Offline storage unavailable")?;
    ::server_common::auth_utils::set_org_context(&mut *tx, tenant)
        .await
        .map_err(|_| "Offline tenant context unavailable")?;
    if let Some(receipt_id) = payload.get("receipt_id").and_then(Value::as_str) {
        let saved: Option<(Value,String,String,String)> = sqlx::query_as("SELECT request_identity,receipt_status,action_type,receipt_route FROM sync_events WHERE id=$1 AND tenant_id=$2 AND request_identity IS NOT NULL AND receipt_status IS NOT NULL AND receipt_route IS NOT NULL FOR UPDATE")
            .bind(receipt_id).bind(tenant).fetch_optional(&mut *tx).await.map_err(|_| "Offline receipt storage unavailable")?;
        if let Some((identity, status, action, route)) = saved {
            if matches!(status.as_str(), "acknowledged" | "reconciliation")
                && route == "/api/v1/sync/offline"
                && identity.get("transaction_id").and_then(Value::as_str) == Some(id)
                && explicit_kind(&identity) == Some(action.as_str())
                && payload.get("mutation") == Some(&identity)
                && payload.get("mutation_type").and_then(Value::as_str) == Some(action.as_str())
                && consistent_evidence(payload, &action, 0)
            {
                tx.commit()
                    .await
                    .map_err(|_| "Offline receipt completion is unconfirmed")?;
                return Ok(());
            }
        }
        // A missing POS row is normal for the durable inventory producer. The
        // original queue job remains the reconciliation record in this case.
    } else {
        let saved: Option<(Option<Value>,Option<String>,Value,i64,String,String)> = sqlx::query_as("SELECT request_identity,request_status,payload,amount_cents,currency,client_id FROM pos_offline_transactions WHERE id=$1 AND tenant_id=$2 FOR UPDATE")
            .bind(id).bind(tenant).fetch_optional(&mut *tx).await.map_err(|_| "Offline transaction storage unavailable")?;
        if let Some((Some(identity), Some(status), original, amount, currency, client)) = saved {
            if matches!(status.as_str(), "acknowledged" | "reconciliation")
                && explicit_kind(&identity) == Some("cash_sale")
                && identity.get("tenant_id").and_then(Value::as_str) == Some(tenant)
                && identity.get("id").and_then(Value::as_str) == Some(id)
                && identity.get("client_id").and_then(Value::as_str) == Some(client.as_str())
                && identity.get("amount_cents").and_then(Value::as_i64) == Some(amount)
                && identity.get("currency").and_then(Value::as_str) == Some(currency.as_str())
                && identity.get("payload") == Some(&original)
                && payload.get("client_id").and_then(Value::as_str) == Some(client.as_str())
                && payload.get("amount_cents").and_then(Value::as_i64) == Some(amount)
                && payload.get("currency").and_then(Value::as_str) == Some(currency.as_str())
                && payload.get("payload").and_then(decoded).as_ref() == Some(&original)
                && payload.get("mutation_type").and_then(Value::as_str) == Some("cash_sale")
                && consistent_evidence(payload, "cash_sale", 0)
            {
                let changed = sqlx::query("UPDATE pos_offline_transactions SET status='RESOLVED',_sync_status='synced',updated_at=CURRENT_TIMESTAMP WHERE id=$1 AND tenant_id=$2")
                    .bind(id).bind(tenant).execute(&mut *tx).await.map_err(|_| "Offline cash receipt update failed")?;
                if changed.rows_affected() != 1 {
                    return Err("Offline cash receipt disappeared; retain the original job".into());
                }
                tx.commit()
                    .await
                    .map_err(|_| "Offline cash completion is unconfirmed")?;
                return Ok(());
            }
        }
    }
    // Zero rows is intentional for missing/foreign IDs: preserve the queue job,
    // never fabricate a durable transaction or touch another tenant's record.
    sqlx::query("UPDATE pos_offline_transactions SET status='RECONCILIATION_REQUIRED',_sync_status='pending',updated_at=CURRENT_TIMESTAMP WHERE id=$1 AND tenant_id=$2")
        .bind(id).bind(tenant).execute(&mut *tx).await.map_err(|_| "Unable to retain offline transaction")?;
    tx.commit()
        .await
        .map_err(|_| "Offline reconciliation state is unconfirmed")?;
    Err("Offline payment or unknown operation requires reconciliation; no provider, inventory, order or ledger effect was made".into())
}
