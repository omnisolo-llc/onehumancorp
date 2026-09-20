//! Durable, source-grounded reminder drafts. No implicit model spend, messages,
//! invented cash-flow updates or provider delivery claims are made by this worker.
use crate::db::{DB, DbStore};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::time::{Duration, interval};

/// Spawn immediately and return a handle for supervision or cancellation.
/// An async constructor here previously let callers silently discard startup.
pub fn start_invoice_followup_worker(db: Arc<DB>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(24 * 60 * 60));
        loop {
            ticker.tick().await;
            // Cloud workers must be provisioned with their permitted tenant list.
            // Never bypass row-level security to discover another customer's data.
            let tenants: Vec<String> = match std::env::var("OMNISOLO_REMINDER_TENANTS") {
                Ok(value) => value
                    .split(',')
                    .map(str::trim)
                    .filter(|id| !id.is_empty())
                    .take(1000)
                    .map(str::to_owned)
                    .collect(),
                Err(_) => match &db.store {
                    DbStore::Sqlite(pool) => sqlx::query_scalar(
                        "SELECT DISTINCT tenant_id FROM invoices ORDER BY tenant_id LIMIT 1000",
                    )
                    .fetch_all(pool)
                    .await
                    .unwrap_or_default(),
                    DbStore::Postgres => {
                        tracing::warn!("Invoice reminders require an explicit tenant schedule");
                        Vec::new()
                    }
                },
            };
            for tenant in tenants {
                if let Err(error) =
                    prepare_reminders(&db, &tenant, chrono::Utc::now().timestamp()).await
                {
                    tracing::error!(reason=%error, "Invoice reminder draft sweep failed");
                }
            }
        }
    })
}

pub async fn prepare_reminders(db: &DB, tenant: &str, now: i64) -> Result<u64, String> {
    if tenant.trim().is_empty() || tenant.len() > 255 || tenant.chars().any(char::is_control) {
        return Err("Invalid reminder tenant".into());
    }
    macro_rules! prepare {
        ($tx:ident, $cancel_sql:expr, $due:literal) => {{
            sqlx::query($cancel_sql).bind(tenant).bind(now).execute(&mut *$tx).await.map_err(|_| "Unable to retire stale reminder drafts")?;
            let rows: Vec<(String,String,String,f64,i64,i64)> = sqlx::query_as(
                concat!("SELECT id,COALESCE(client_id,''),COALESCE(currency,'USD'),total_amount,CAST(COALESCE(amount_paid_cents,0) AS BIGINT),", $due,
                    " FROM invoices WHERE tenant_id=$1 AND LOWER(COALESCE(payment_status,'')) NOT IN ('paid','refunded','void','cancelled','canceled') AND LOWER(status) IN ('open','sent','pending') AND ", $due, " < $2 ORDER BY due_date,id LIMIT 100"))
                .bind(tenant).bind(now).fetch_all(&mut *$tx).await.map_err(|_| "Unable to read overdue invoices")?;
            let mut inserted = 0;
            for (invoice,customer,currency,total,paid,due) in rows {
                if !total.is_finite() || total <= 0.0 || total > 999_999.99 || paid < 0 { continue; }
                let balance = (total * 100.0).round() as i64 - paid;
                if balance <= 0 { continue; }
                let id = format!("reminder:{:x}",Sha256::digest(format!("{tenant}:{invoice}:{due}:{balance}")));
                let context=serde_json::json!({"description":format!("Review overdue invoice {invoice}"),"source_invoice":invoice});
                let payload=serde_json::json!({"feature_type":"invoice_followup","invoice_id":invoice,"customer_id":customer,
                    "balance_cents":balance,"currency":currency,"suggested_channel":"email","delivery_status":"not_sent",
                    "generated_response":format!("Hello, this is a reminder that invoice {invoice} has an outstanding balance of {currency} {}.{:02}. Please let us know if you have questions.",balance/100,balance%100)});
                inserted += sqlx::query("INSERT INTO agent_feed_items(id,tenant_id,event_source,context_payload,proposed_action,lifecycle_state,created_at,updated_at) VALUES($1,$2,'finance_reminder',$3,$4,'PENDING_APPROVAL',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP) ON CONFLICT(id) DO NOTHING")
                    .bind(&id).bind(tenant).bind(sqlx::types::Json(context)).bind(sqlx::types::Json(payload))
                    .execute(&mut *$tx).await.map_err(|_| "Unable to persist reminder draft")?.rows_affected();
            }
            $tx.commit().await.map_err(|_| "Unable to commit reminder drafts")?;
            Ok(inserted)
        }};
    }
    match &db.store {
        DbStore::Postgres => {
            let mut tx = db
                .pool
                .begin()
                .await
                .map_err(|_| "Reminder database unavailable")?;
            ::server_common::auth_utils::set_org_context(&mut *tx, tenant)
                .await
                .map_err(|_| "Reminder tenant context unavailable")?;
            prepare!(
                tx,
                "UPDATE agent_feed_items f SET lifecycle_state='CANCELLED',updated_at=CURRENT_TIMESTAMP WHERE f.tenant_id=$1 AND f.event_source='finance_reminder' AND f.lifecycle_state='PENDING_APPROVAL' AND NOT EXISTS(SELECT 1 FROM invoices i WHERE i.tenant_id=$1 AND i.id=f.proposed_action->>'invoice_id' AND LOWER(i.status) IN ('open','sent','pending') AND LOWER(COALESCE(i.payment_status,'')) NOT IN ('paid','refunded','void','cancelled','canceled') AND CAST(EXTRACT(EPOCH FROM i.due_date) AS BIGINT)<$2 AND CAST(ROUND(i.total_amount*100) AS BIGINT)-COALESCE(i.amount_paid_cents,0)=CAST(f.proposed_action->>'balance_cents' AS BIGINT))",
                "CAST(EXTRACT(EPOCH FROM due_date) AS BIGINT)"
            )
        }
        DbStore::Sqlite(pool) => {
            let mut tx = pool
                .begin()
                .await
                .map_err(|_| "Reminder database unavailable")?;
            prepare!(
                tx,
                "UPDATE agent_feed_items AS f SET lifecycle_state='CANCELLED',updated_at=CURRENT_TIMESTAMP WHERE f.tenant_id=$1 AND f.event_source='finance_reminder' AND f.lifecycle_state='PENDING_APPROVAL' AND NOT EXISTS(SELECT 1 FROM invoices i WHERE i.tenant_id=$1 AND i.id=json_extract(f.proposed_action,'$.invoice_id') AND LOWER(i.status) IN ('open','sent','pending') AND LOWER(COALESCE(i.payment_status,'')) NOT IN ('paid','refunded','void','cancelled','canceled') AND i.due_date<$2 AND CAST(ROUND(i.total_amount*100) AS INTEGER)-COALESCE(i.amount_paid_cents,0)=json_extract(f.proposed_action,'$.balance_cents'))",
                "due_date"
            )
        }
    }
}
