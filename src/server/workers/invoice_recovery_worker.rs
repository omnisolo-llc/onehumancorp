use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, error};

use crate::db::DB;

/// Worker that periodically checks for overdue invoices and creates agent feed items
/// for the owner to review and approve follow-up messages.
pub async fn start_invoice_recovery_worker(db: Arc<DB>) {
    info!("Starting Invoice Recovery worker");
    loop {
        match run_invoice_recovery_cycle(&db).await {
            Ok(_) => {}
            Err(e) => {
                error!("Error in invoice recovery cycle: {:?}", e);
            }
        }
        sleep(Duration::from_secs(3600)).await; // Run once an hour
    }
}

async fn run_invoice_recovery_cycle(db: &DB) -> Result<(), sqlx::Error> {
    // 1. Find invoices that are overdue (e.g., status is 'draft' or 'sent' and created > 3 days ago, or due_date passed)
    // We will use SKIP LOCKED to safely process them concurrently if multiple workers exist
    // and payment_status != 'paid' and status != 'paid'

    // For simplicity, we look for unpaid invoices older than 3 days
    let mut tx = db.pool.begin().await?;

    let overdue_invoices = sqlx::query!(
        r#"
        SELECT id, tenant_id, customer_id, total_amount_cents, amount_paid_cents
        FROM invoices
        WHERE (payment_status = 'draft' OR payment_status = 'sent' OR payment_status IS NULL)
          AND (status = 'draft' OR status = 'sent' OR status IS NULL)
          AND created_at < NOW() - INTERVAL '3 days'
          -- Basic check to ensure we haven't already drafted a follow-up recently
          AND id NOT IN (SELECT invoice_id FROM invoice_communication_events WHERE created_at > NOW() - INTERVAL '7 days')
        LIMIT 50
        FOR UPDATE SKIP LOCKED
        "#
    )
    .fetch_all(&mut *tx)
    .await?;

    if overdue_invoices.is_empty() {
        tx.commit().await?;
        return Ok(());
    }

    info!("Found {} overdue invoices for recovery", overdue_invoices.len());

    for invoice in overdue_invoices {
        let remaining_balance = invoice.total_amount_cents - invoice.amount_paid_cents;
        if remaining_balance <= 0 {
            continue; // Already paid
        }

        // Generate the draft message
        let drafted_content = format!(
            "Hi there! Just a friendly reminder that there is an outstanding balance of ${:.2} on your recent invoice. Here is a quick link to settle it: [Payment Link]. Let us know if you have any questions!",
            remaining_balance as f64 / 100.0
        );

        // Record the communication event
        let event_id = uuid::Uuid::new_v4().to_string();
        sqlx::query!(
            r#"
            INSERT INTO invoice_communication_events (id, tenant_id, invoice_id, status, channel, drafted_content)
            VALUES ($1, $2, $3, 'drafted', 'email', $4)
            "#,
            event_id,
            invoice.tenant_id,
            invoice.id,
            drafted_content
        )
        .execute(&mut *tx)
        .await?;

        // Push to the agent feed
        let feed_item_id = uuid::Uuid::new_v4().to_string();
        let event_source = "Finance Agent";
        let context_payload = serde_json::json!({
            "invoice_id": invoice.id,
            "customer_id": invoice.customer_id,
            "balance_cents": remaining_balance,
            "draft_id": event_id
        }).to_string();
        let proposed_action = "Approve payment reminder";

        sqlx::query!(
            r#"
            INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state)
            VALUES ($1, $2, $3, $4, $5, 'PENDING_APPROVAL')
            "#,
            feed_item_id,
            invoice.tenant_id,
            event_source,
            context_payload,
            proposed_action
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}
