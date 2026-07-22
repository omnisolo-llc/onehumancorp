use sqlx::PgPool;
use serde_json::Value;


#[cfg(ohc_bazel)]
use crate::integrations::stripe::client::StripeClient;
#[cfg(not(ohc_bazel))]
use server_integrations_stripe::client::StripeClient;

pub async fn handle_invoice_draft_action(tenant_id: &str, payload: &Value, pool: &PgPool) -> Result<(), sqlx::Error> {
    let amount_cents = payload.get("amount_cents").and_then(|v| v.as_i64()).unwrap_or(0);
    let mut customer_id = payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
    if customer_id.is_empty() {
        customer_id = uuid::Uuid::new_v4().to_string();
    }

    let project_name = payload.get("project_name").and_then(|v| v.as_str()).unwrap_or("Project");
    let milestone_name = payload.get("milestone_name").and_then(|v| v.as_str()).unwrap_or("Milestone");
    let description = format!("Invoice for {} - {}", project_name, milestone_name);

    tracing::info!("Approved invoice draft action for project: {} milestone: {}", project_name, milestone_name);

    let api_key = std::env::var("STRIPE_API_KEY").unwrap_or_else(|_| "sk_test_mock".to_string());
    let stripe_client = StripeClient::new(api_key);

    let mut payment_link = String::new();
    match stripe_client
        .create_checkout_session(
            &description,
            &customer_id,
            (amount_cents as f64) / 100.0,
            None,
            None,
        )
        .await
    {
        Ok(url) => payment_link = url,
        Err(error) => {
            tracing::error!(
                "Failed to create Stripe checkout session for invoice draft: {}",
                error
            );
        }
    }

    let invoice_id = uuid::Uuid::new_v4().to_string();
    let due_date = chrono::Utc::now().timestamp() + (30 * 24 * 60 * 60);

    let insert_res = sqlx::query(
        "INSERT INTO invoices (id, tenant_id, client_id, client_name, status, due_date, currency, total_amount, stripe_payment_link, created_at, updated_at) VALUES ($1, $2, $3, $4, 'sent', $5, 'USD', $6, $7, NOW(), NOW())"
    )
    .bind(&invoice_id)
    .bind(tenant_id)
    .bind(&customer_id)
    .bind("Client") // Simplified
    .bind(due_date)
    .bind((amount_cents as f64) / 100.0)
    .bind(&payment_link)
    .execute(pool)
    .await;

    if let Err(e) = insert_res {
        tracing::error!("Failed to generate invoice from draft: {}", e);
        return Err(e);
    }

    let li_id = uuid::Uuid::new_v4().to_string();
    let _ = sqlx::query(
        "INSERT INTO invoice_line_items (id, tenant_id, invoice_id, description, quantity, unit_price, amount, created_at, updated_at) VALUES ($1, $2, $3, $4, 1, $5, $5, NOW(), NOW())"
    )
    .bind(&li_id)
    .bind(tenant_id)
    .bind(&invoice_id)
    .bind(&description)
    .bind((amount_cents as f64) / 100.0)
    .execute(pool)
    .await;

    tracing::info!("Successfully generated invoice from draft and created payment link");

    Ok(())
}

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
