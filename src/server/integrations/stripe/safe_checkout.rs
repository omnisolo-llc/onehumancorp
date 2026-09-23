//! Real checkout creation with a caller-owned operation identity. Creating a
//! session is never reported as payment, and missing configuration fails closed.
use super::client::StripeClient;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, time::Duration};

/// A checkout's commercial terms and stable operation identity travel together.
/// Amounts use minor units; this is not proof that the customer paid.
pub struct CheckoutRequest<'a> {
    pub name: &'a str,
    pub reference: &'a str,
    pub amount_cents: i64,
    pub interval: Option<&'a str>,
    pub product: Option<&'a str>,
    pub currency: &'a str,
    pub operation_id: &'a str,
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutReceipt {
    pub id: String,
    pub url: String,
    pub payment_status: String,
    pub amount_total: i64,
    pub currency: String,
}

pub fn valid_provider_id(value: &str, prefix: &str) -> bool {
    value.starts_with(prefix)
        && value.len() > prefix.len()
        && value.len() <= 255
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}
fn secure_origin(value: &str) -> Result<reqwest::Url, String> {
    let url = reqwest::Url::parse(value).map_err(|_| "Invalid payment origin")?;
    let local = matches!(
        url.host_str(),
        Some("127.0.0.1" | "localhost" | "[::1]" | "::1")
    );
    if !(url.scheme() == "https" || (local && url.scheme() == "http"))
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err("Payment origin must be HTTPS (HTTP is restricted to loopback)".into());
    }
    Ok(url)
}
pub fn money_minor_units(value: f64) -> Result<i64, String> {
    if !value.is_finite() || value <= 0.0 || value > 999_999.99 {
        return Err("Invalid checkout amount".into());
    }
    let cents = (value * 100.0).round();
    if (value * 100.0 - cents).abs() > 0.00001 {
        return Err("Checkout amount must use currency minor units".into());
    }
    Ok(cents as i64)
}
fn checkout_form(
    name: &str,
    reference: &str,
    amount: i64,
    interval: Option<&str>,
    product: Option<&str>,
    currency: &str,
    return_base: &str,
    metadata: Option<&std::collections::HashMap<String, String>>,
) -> Result<BTreeMap<String, String>, String> {
    if name.trim().is_empty()
        || name.len() > 500
        || reference.trim().is_empty()
        || reference.len() > 200
        || amount <= 0
        || amount > 99_999_999
    {
        return Err("Invalid checkout description, reference or amount".into());
    }
    // The legacy public amount argument is a major-unit decimal. Zero-/three-
    // decimal currencies require explicit currency-aware amounts, not guessing.
    let currency = currency.to_ascii_lowercase();
    if !matches!(
        currency.as_str(),
        "usd" | "eur" | "gbp" | "cad" | "aud" | "nzd" | "sgd" | "hkd" | "inr" | "brl" | "mxn"
    ) {
        return Err("Currency requires an explicit minor-unit implementation".into());
    }
    let base = secure_origin(return_base)?;
    let mut form = BTreeMap::from([
        (
            "success_url".into(),
            base.join("payments/return?session_id={CHECKOUT_SESSION_ID}")
                .map_err(|_| "Invalid return URL")?
                .to_string(),
        ),
        (
            "cancel_url".into(),
            base.join("payments/cancel")
                .map_err(|_| "Invalid cancel URL")?
                .to_string(),
        ),
        ("client_reference_id".into(), reference.into()),
        (
            "mode".into(),
            if interval.is_some() {
                "subscription"
            } else {
                "payment"
            }
            .into(),
        ),
        ("line_items[0][price_data][currency]".into(), currency),
        (
            "line_items[0][price_data][product_data][name]".into(),
            name.into(),
        ),
        (
            "line_items[0][price_data][unit_amount]".into(),
            amount.to_string(),
        ),
        ("line_items[0][quantity]".into(), "1".into()),
        ("payment_method_types[0]".into(), "card".into()),
    ]);
    if let Some(interval) = interval {
        if !matches!(interval, "day" | "week" | "month" | "year") {
            return Err("Invalid subscription interval".into());
        }
        form.insert(
            "line_items[0][price_data][recurring][interval]".into(),
            interval.into(),
        );
    }
    if let Some(product) = product {
        if product.len() > 255 || product.chars().any(char::is_control) {
            return Err("Invalid product reference".into());
        }
        form.insert("metadata[product_id]".into(), product.into());
    }
    if let Some(meta) = metadata {
        for (k, v) in meta {
            form.insert(format!("metadata[{}]", k), v.clone());
        }
    }
    Ok(form)
}
fn parse_receipt(
    value: serde_json::Value,
    amount: i64,
    currency: &str,
) -> Result<CheckoutReceipt, String> {
    let receipt: CheckoutReceipt = serde_json::from_value(value)
        .map_err(|_| "Checkout response lacks required provider evidence")?;
    // Stripe-hosted Checkout URLs legitimately contain query/fragment state.
    // Validate the destination separately from an operator-configured origin.
    let url = reqwest::Url::parse(&receipt.url).map_err(|_| "Invalid checkout URL")?;
    if receipt.url.len() > 16_384
        || !url.username().is_empty()
        || url.password().is_some()
        || url.port_or_known_default() != Some(443)
        || !url.path().starts_with("/c/pay/")
        || !valid_provider_id(&receipt.id, "cs_")
        || url.scheme() != "https"
        || url.host_str() != Some("checkout.stripe.com")
        || receipt.amount_total != amount
        || !receipt.currency.eq_ignore_ascii_case(currency)
        || !matches!(
            receipt.payment_status.as_str(),
            "paid" | "unpaid" | "no_payment_required"
        )
    {
        return Err("Checkout response did not match the requested payment".into());
    }
    Ok(receipt)
}
impl StripeClient {
    pub async fn create_checkout_session_idempotent(
        &self,
        request: CheckoutRequest<'_>,
    ) -> Result<CheckoutReceipt, String> {
        let CheckoutRequest {
            name,
            reference,
            amount_cents,
            interval,
            product,
            currency,
            operation_id,
            metadata,
        } = request;
        let key = self.require_api_key()?;
        if operation_id.trim().is_empty()
            || operation_id.len() > 255
            || operation_id.chars().any(char::is_control)
        {
            return Err("Stable checkout operation identity is required".into());
        }
        let base = secure_origin(&Self::api_base())?;
        let local = matches!(
            base.host_str(),
            Some("127.0.0.1" | "localhost" | "::1" | "[::1]")
        );
        if base.host_str() != Some("api.stripe.com") && !local {
            return Err("Untrusted Stripe API origin".into());
        }
        let return_base = std::env::var("PUBLIC_APP_URL")
            .or_else(|_| std::env::var("BASE_URL"))
            .map_err(|_| "PUBLIC_APP_URL is required for customer checkout")?;
        let form = checkout_form(
            name,
            reference,
            amount_cents,
            interval,
            product,
            currency,
            &return_base,
            metadata.as_ref(),
        )?;
        let response = reqwest::Client::builder()
            .timeout(Duration::from_secs(20))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| "Unable to initialize payment client")?
            .post(
                base.join("/v1/checkout/sessions")
                    .map_err(|_| "Invalid payment endpoint")?,
            )
            .basic_auth(key, Some(""))
            .header("Idempotency-Key", operation_id)
            .form(&form)
            .send()
            .await
            .map_err(|_| "Checkout outcome is unknown; reconcile the operation before retrying")?;
        if !response.status().is_success() {
            return Err(format!(
                "Stripe checkout returned HTTP {}; no success was recorded",
                response.status().as_u16()
            ));
        }
        parse_receipt(
            response
                .json()
                .await
                .map_err(|_| "Invalid checkout response")?,
            amount_cents,
            currency,
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn amount_is_checked_not_truncated() {
        assert_eq!(money_minor_units(10.25).unwrap(), 1025);
        for value in [f64::NAN, f64::INFINITY, -1.0, 0.0, 1.001, f64::MAX] {
            assert!(money_minor_units(value).is_err());
        }
    }
    #[test]
    fn checkout_binds_amount_and_safe_return_origin() {
        let form = checkout_form(
            "Gate repair",
            "proposal-1",
            15000,
            None,
            None,
            "usd",
            "https://owner.example",
            None,
        )
        .unwrap();
        assert_eq!(form["line_items[0][price_data][unit_amount]"], "15000");
        assert_eq!(form["client_reference_id"], "proposal-1");
        assert!(
            checkout_form(
                "Gate",
                "id",
                100,
                None,
                None,
                "jpy",
                "https://owner.example",
                None,
            )
            .is_err()
        );
        assert!(
            checkout_form(
                "Gate",
                "id",
                100,
                None,
                None,
                "usd",
                "http://attacker.example",
                None,
            )
            .is_err()
        );
    }
    #[test]
    fn success_needs_real_id_url_amount_and_currency() {
        let value = serde_json::json!({"id":"cs_test_fixture","url":"https://checkout.stripe.com/c/pay/cs_test_fixture","payment_status":"unpaid","amount_total":2500,"currency":"usd"});
        assert!(parse_receipt(value.clone(), 2500, "usd").is_ok());
        let mut fragment = value.clone();
        fragment["url"] = serde_json::json!(
            "https://checkout.stripe.com/c/pay/cs_test_fixture?locale=en#provider-state"
        );
        assert!(parse_receipt(fragment, 2500, "usd").is_ok());
        let mut credentials = value.clone();
        credentials["url"] =
            serde_json::json!("https://user@checkout.stripe.com/c/pay/cs_test_fixture");
        assert!(parse_receipt(credentials, 2500, "usd").is_err());
        assert!(parse_receipt(value.clone(), 2501, "usd").is_err());
        let mut invalid = value;
        invalid["url"] = serde_json::json!("https://attacker.example");
        assert!(parse_receipt(invalid, 2500, "usd").is_err());
    }
}
