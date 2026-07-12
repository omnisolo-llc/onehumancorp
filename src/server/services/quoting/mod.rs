use crate::api::quotes::{Quote, QuoteItem};
use crate::integrations::taxjar::provider::TaxJarProvider;
use crate::integrations::taxjar::client::TaxJarParams;

pub async fn build_quote_summary(tenant_id: &str, mut items: Vec<QuoteItem>) -> Result<Quote, String> {
    let mut total_pre_tax_cents = 0;
    for item in &items {
        total_pre_tax_cents += item.unit_price_cents * item.quantity as i32;
    }

    let total_pre_tax_usd = (total_pre_tax_cents as f64) / 100.0;
    let mut tax_amount_usd = 0.0;

    if let Ok(key) = std::env::var("TAXJAR_API_KEY") {
        let provider = TaxJarProvider::new(key);
        let params = TaxJarParams {
            amount: total_pre_tax_usd,
            shipping: 0.0,
            to_country: "US",
            to_zip: "90002",
            to_state: "CA",
            from_country: "US",
            from_zip: "92093",
            from_state: "CA",
        };
        if let Ok(tax_rate) = provider.calculate_tax(params).await {
            tax_amount_usd = tax_rate.amount_to_collect;
        }
    }

    let tax_amount_cents = (tax_amount_usd * 100.0).round() as i32;
    let final_total_cents = total_pre_tax_cents + tax_amount_cents;

    Ok(Quote {
        id: format!("qt-{}", uuid::Uuid::new_v4()),
        tenant_id: tenant_id.to_string(),
        items,
        total_cents: final_total_cents,
        tax_amount_cents,
        status: "DRAFT".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    })
}
