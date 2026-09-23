use serde_json::Value;
use sqlx::PgPool;

pub async fn handle_invoice_action(
    tenant_id: &str,
    payload: &Value,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    if let Some(invoice_id) = payload.get("invoice_id").and_then(|v| v.as_str()) {
        // Verify invoice is still unpaid/open before sending
        let invoice_res: Option<(String, Option<String>, f64, Option<i32>)> = sqlx::query_as(
            r#"SELECT status, payment_status, total_amount, amount_paid_cents
               FROM invoices WHERE id = $1 AND tenant_id = $2"#
        )
        .bind(invoice_id)
        .bind(tenant_id)
        .fetch_optional(pool)
        .await?;

        let should_send = if let Some((status, payment_status_opt, total_amount, amount_paid_cents_opt)) = invoice_res {
            let status = status.to_lowercase();
            let payment_status = payment_status_opt.unwrap_or_default().to_lowercase();
            let total_cents = (total_amount * 100.0).round() as i64;
            let amount_paid_cents = amount_paid_cents_opt.unwrap_or(0) as i64;

            !["paid", "refunded", "void", "cancelled", "canceled"].contains(&payment_status.as_str())
                && ["open", "sent", "pending", "overdue", "draft"].contains(&status.as_str())
                && total_cents > amount_paid_cents
        } else {
            false
        };

        if !should_send {
            tracing::info!("Skipping reminder for invoice {}: already paid, cancelled, or not found", invoice_id);
            return Ok(());
        }

        // Check for deduplication: has a reminder been sent very recently?
        let recent_reminder: Option<(String,)> = sqlx::query_as(
            "SELECT id FROM invoice_communication_events WHERE invoice_id = $1 AND tenant_id = $2 AND status IN ('sent', 'delivered') AND created_at > CURRENT_TIMESTAMP - INTERVAL '1 day'"
        )
        .bind(invoice_id)
        .bind(tenant_id)
        .fetch_optional(pool)
        .await?;

        if recent_reminder.is_some() {
            tracing::info!("Skipping reminder for invoice {}: reminder already sent recently", invoice_id);
            return Ok(());
        }

        tracing::info!(
            "Approved invoice followup action for invoice: {}",
            invoice_id
        );

        let draft_content = payload
            .get("generated_response")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let id = uuid::Uuid::new_v4().to_string();

        let mut tx = pool.begin().await?;

        // 1. Persist as drafted
        sqlx::query(
            "INSERT INTO invoice_communication_events (id, tenant_id, invoice_id, status, channel, drafted_content) VALUES ($1, $2, $3, 'drafted', 'email', $4)"
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(invoice_id)
        .bind(draft_content)
        .execute(&mut *tx)
        .await?;

        // 2. Mark as sent
        sqlx::query(
            "UPDATE invoice_communication_events SET status = 'sent', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2"
        )
        .bind(&id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

        // Update view_count/communication metric loosely tracked on the invoice
        sqlx::query(
            "UPDATE invoices SET updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2",
        )
        .bind(invoice_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        // Simulate sending email through omnichannel dispatcher
        tracing::info!(
            "Omnichannel Dispatcher sent invoice reminder: {}",
            draft_content
        );

        // Simulate delivery
        sqlx::query(
            "UPDATE invoice_communication_events SET status = 'delivered', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2"
        )
        .bind(&id)
        .bind(tenant_id)
        .execute(pool)
        .await?;
    }
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct InvoiceDraft {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub project_name: String,
    pub milestone_name: String,
    pub total_amount_cents: i64,
    pub line_items: Vec<InvoiceDraftLineItem>,
    pub message_context: InvoiceMessageContext,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct InvoiceDraftLineItem {
    pub description: String,
    pub amount_cents: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct InvoiceMessageContext {
    pub generated_message: String,
    pub suggested_channel: String,
}
