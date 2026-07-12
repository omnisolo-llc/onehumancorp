use sqlx::PgPool;
use serde_json::Value;

pub async fn handle_invoice_action(tenant_id: &str, payload: &Value, pool: &PgPool) -> Result<(), sqlx::Error> {
    if let Some(invoice_id) = payload.get("invoice_id").and_then(|v| v.as_str()) {
        tracing::info!("Approved invoice followup action for invoice: {}", invoice_id);

        let draft_content = payload.get("generated_response").and_then(|v| v.as_str()).unwrap_or("");

        let id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO invoice_communication_events (id, tenant_id, invoice_id, status, channel, drafted_content) VALUES ($1, $2, $3, 'sent', 'email', $4)"
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(invoice_id)
        .bind(draft_content)
        .execute(pool)
        .await?;

        // Update view_count/communication metric loosely tracked on the invoice
        sqlx::query("UPDATE invoices SET updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
            .bind(invoice_id)
            .bind(tenant_id)
            .execute(pool)
            .await?;

        // Simulate sending email through omnichannel dispatcher
        tracing::info!("Omnichannel Dispatcher sent invoice reminder: {}", draft_content);
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
